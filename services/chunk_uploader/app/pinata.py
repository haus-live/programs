import tempfile
import io

import json
import requests

from app.config import AppConfig
from app.base import IpfsUri


_PINATA_PUBLIC_GATEWAY_FMT = 'https://gateway.pinata.cloud/ipfs/{CID}'


class Pinata:
    __slots__ = '_config',

    def __init__(self, config: AppConfig):
        self._config = config
    
    def write_file(self, file: io.FileIO) -> IpfsUri:
        # context: tempfile
        response = requests.post(
            f'{self._config.PINATA_BASE_URL}/files',
            files=dict(
                file=file,
                name=file.name,
                network='public',
            ),
            headers={
                'Authorization': f'Bearer {self._config.PINATA_JWT}',
                'Content-Type': 'multipart/form-data'
            }
        )
        response.raise_for_status()
        json_response = response.json()
        # https://docs.pinata.cloud/api-reference/endpoint/upload-a-file#response-data-cid
        cid = json_response['data']['cid']
        # https://docs.pinata.cloud/ipfs-101/what-are-cids
        return _PINATA_PUBLIC_GATEWAY_FMT.format(cid)

    def read_json(self, uri: str) -> dict:
        response = requests.get(uri)
        response.raise_for_status()
        return response.json()

    def write_json(self, payload: dict) -> IpfsUri:
        with tempfile.TemporaryFile() as tmp:
            json.dump(payload)
            return self.write_file(tmp)
