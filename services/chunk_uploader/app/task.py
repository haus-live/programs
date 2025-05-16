import subprocess
import tempfile
from apscheduler.schedulers.base import BaseScheduler
from datetime import datetime

from app.config import AppConfig
from app.pinata import Pinata
from app.solana import Solana
from app.base import StreamId, IpfsUri

class Task:
    __slots__ = '_config', '_solana', '_pinata', '_scheduler'

    def __init__(self, config: AppConfig, solana: Solana, pinata: Pinata, scheduler: BaseScheduler):
        self._config = config
        self._solana = solana
        self._pinata = pinata
        self._scheduler = scheduler

    def _run_ffmpeg(self, *, stream_id: StreamId, file_name: str):
        # ffmpeg -i http://localhost:8080/memfs/617f7575-c9f8-42f2-b706-344c0d997afb.m3u8 -t 15 -c copy test-15.mkv
        subprocess.run([
            'ffmpeg',
            '-i',
            f'{self._config.RESTREAMER_HLS_BASE_URI}/{stream_id}.m3u8',
            '-t',
            str(self._config.CHUNK_DURATION),
            '-c',
            'copy',
            file_name
        ])
    
    @staticmethod
    def _patch_asset_content(*, content: dict, chunk: IpfsUri, chunk_id: int):
        content['properties']['chunks'][str(chunk_id)] = dict(
            uri=chunk,
            type='video/mkv'
        )
    
    def _process_chunk(self, *, chunk_id: int, stream_id: StreamId, asset_pubkey: bytes):
        with tempfile.TemporaryFile() as tmp:
            self._run_ffmpeg(stream_id=stream_id, file_name=tmp.name)
            chunk_uri = self._pinata.write_file(tmp)
            asset_uri = self._solana.read_uri_from_asset_account(asset_pubkey)
            asset_content = self._pinata.read_json(asset_uri)
            self._patch_asset_content(content=asset_content, chunk=chunk_uri, chunk_id=chunk_id)
            new_asset_uri = self._pinata.write_json(asset_content)
            self._solana.update_core_asset_account_uri(new_asset_uri)

    def process_stream(self, *, stream_id, asset_pubkey, event_begin_timestamp, event_end_timestamp):
        map(
            lambda chunk_id, job_timestamp: (
                self._scheduler.add_job(
                    func=self._process_chunk,
                    kwargs=dict(
                        chunk_id=chunk_id, 
                        stream_id=stream_id, 
                        asset_pubkey=asset_pubkey
                    ),
                    trigger='date',
                    run_date=datetime.fromtimestamp(float(job_timestamp))  # timezone!
                )
            ),
            enumerate(
                range(
                    event_begin_timestamp, 
                    event_end_timestamp, 
                    self._config.CHUNK_DURATION
                )
            )
        )
