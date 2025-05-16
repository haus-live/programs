import os.path
import subprocess
import tempfile
import logging
from apscheduler.schedulers.base import BaseScheduler
from datetime import datetime

from app.config import AppConfig
from app.pinata import Pinata
from app.solana import Solana
from app.base import StreamId, IpfsUri

class Task:
    __slots__ = '_config', '_solana', '_pinata', '_scheduler', '_logger'

    def __init__(self, config: AppConfig, solana: Solana, pinata: Pinata, scheduler: BaseScheduler):
        self._config = config
        self._solana = solana
        self._pinata = pinata
        self._logger = logging.getLogger(name='app.Task')
        self._scheduler = scheduler

    def _run_ffmpeg(self, *, stream_id: StreamId, file_name: str):
        # ffmpeg -i http://localhost:8080/memfs/617f7575-c9f8-42f2-b706-344c0d997afb.m3u8 -t 15 -c copy test-15.mkv
        hls_uri = f"{self._config.RESTREAMER_HLS_BASE_URI}/{stream_id}.m3u8"
        self._logger.info(f'capturing HLS stream {hls_uri}')
        subprocess.run(
            [
                "ffmpeg",
                "-y",  # auto overwrite (just in case)
                "-i",
                f"{self._config.RESTREAMER_HLS_BASE_URI}/{stream_id}.m3u8",
                "-t",
                f"{self._config.CHUNK_DURATION}",
                "-c",
                "copy",
                f"{file_name}",
            ],
            stderr=subprocess.DEVNULL,
            stdout=subprocess.DEVNULL
        )
        self._logger.info(f'stream chunk saved to {file_name}')

    @staticmethod
    def _patch_asset_content(*, content: dict, chunk: IpfsUri, chunk_id: int):
        content['properties']['chunks'][str(chunk_id)] = dict(
            uri=chunk,
            type='video/mp4'
        )
    
    def _process_chunk(self, *, chunk_id: int, stream_id: StreamId, asset_pubkey: bytes):
        with tempfile.TemporaryDirectory(
            suffix=f'_haus.live.asset.{asset_pubkey}.stream.{stream_id}.chunk.{chunk_id}'
        ) as td:
            self._logger.info(f'created a temporary directory {td}')
            chunk_file_name = f'file.mp4'
            chunk_file_path = os.path.join(td, chunk_file_name)
            with open(chunk_file_path, 'wb') as file:
                self._run_ffmpeg(stream_id=stream_id, file_name=file.name)
            with open(chunk_file_path, 'rb') as file:
                chunk_uri = self._pinata.write_file(file)
        # with tempfile.NamedTemporaryFile(suffix=f'{chunk_id}.{stream_id}.mp4', mode='wb') as tmp:
        #     print(f"tmp file created {tmp.name}")
        #     self._run_ffmpeg(stream_id=stream_id, file_name=tmp.name)
        #     chunk_uri = self._pinata.write_file(tmp)
            # asset_uri = self._solana.read_uri_from_asset_account(asset_pubkey)
            # asset_content = self._pinata.read_json(asset_uri)
            # self._patch_asset_content(content=asset_content, chunk=chunk_uri, chunk_id=chunk_id)
            # new_asset_uri = self._pinata.write_json(asset_content)
            # self._solana.update_core_asset_account_uri(asset_pubkey, new_asset_uri)
        print("tmp file removed")
    
    def _process_chunk_noexcept(self, *, chunk_id, stream_id, asset_pubkey):
        try:
            self._process_chunk(chunk_id=chunk_id, stream_id=stream_id, asset_pubkey=asset_pubkey)
        except BaseException as e:
            self._logger.error(e)

    def process_stream(self, *, stream_id, asset_pubkey, event_begin_timestamp, event_end_timestamp):
        _ = list(map(
            lambda id_and_timestamp: (
                self._scheduler.add_job(
                    func=self._process_chunk_noexcept,
                    kwargs=dict(
                        chunk_id=id_and_timestamp[0], 
                        stream_id=stream_id, 
                        asset_pubkey=asset_pubkey
                    ),
                    trigger='date',
                    run_date=datetime.fromtimestamp(float(id_and_timestamp[1]))  # timezone!
                )
            ),
            enumerate(
                range(
                    event_begin_timestamp, 
                    event_end_timestamp, 
                    self._config.CHUNK_DURATION
                )
            )
        ))
