import subprocess
import tempfile

from app.config import AppConfig
from app.pinata import Pinata
from app.solana import Solana
from app.base import StreamId, IpfsUri

class Task:
    __slots__ = '_config', '_solana', '_pinata'

    def __init__(self, config: AppConfig, solana: Solana, pinata: Pinata):
        self._config = config
        self._solana = solana
        self._pinata = pinata

    def _run_ffmpeg(self, stream_id: StreamId, file_name: str):
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
    def _patch_asset_content(content: dict, chunk: IpfsUri, chunk_id: int):
        ...
    
    def process_chunk(self, chunk_id: int, stream_id: StreamId, asset_pubkey: bytes):
        with tempfile.TemporaryFile() as tmp:
            self._run_ffmpeg()
            chunk_uri = self._pinata.write_file(tmp)
            asset_uri = self._solana.read_uri_from_asset_account(asset_pubkey)
            asset_content = self._pinata.read_json(asset_uri)
            self._patch_asset_content(asset_content, chunk_uri, chunk_id)
            new_asset_uri = self._pinata.write_json(asset_content)
            self._solana.update_cose_asset_account_uri(new_asset_uri)
