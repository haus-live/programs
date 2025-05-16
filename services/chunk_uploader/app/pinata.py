import logging
import tempfile
import io
import os 

import json
import requests

from app.config import AppConfig
from app.base import IpfsUri


# _PINATA_PUBLIC_GATEWAY_FMT = 'https://gateway.pinata.cloud/ipfs/{cid}'
_PINATA_PUBLIC_GATEWAY_FMT = 'https://gray-random-lamprey-785.mypinata.cloud/ipfs/{cid}'


class Pinata:
    __slots__ = '_config', '_logger'

    def __init__(self, config: AppConfig):
        self._config = config
        self._logger = logging.getLogger('app.Pinata')

    # won't pin for now
    # def _pin_cid(self, cid: str) -> None:
    #     self._logger.info(f'pinning cid {cid}')
    #     request_uri = f'{self._config.PINATA_API_BASE_URL}/files/public/pin_by_cid'
    #     response = requests.post(
    #         f'{self._config.PINATA_API_BASE_URL}/files/public/pin_by_cid',
    #         headers={
    #             'Authorization': f'Bearer {self._config.PINATA_JWT}',
    #             'Content-Type': 'application/json'
    #         },
    #         data={
    #             'cid': cid
    #         }
    #     )
    #     self._logger.info(f'response from {request_uri} received {response.text}')
    #     response.raise_for_status()
    #     self._logger.info(f'pinned cid {cid}')
    
    def write_file(self, file: io.FileIO) -> IpfsUri:
        # context: tempfile
        self._logger.info(f'writing file {file.name}')
        request_uri = f'{self._config.PINATA_UPLOAD_BASE_URL}/files'
        response = requests.post(
            request_uri,
            files=dict(
                file=file,
            ),
            data=dict(
                name=f'{os.path.basename(os.path.dirname(file.name))}.mp4',
                network='public',
                # streamable='true'  # this feature is not activated
            ),
            headers={
                'Authorization': f'Bearer {self._config.PINATA_JWT}',
            }
        )
        self._logger.info(f'response from {request_uri} received {response.text}')
        response.raise_for_status()
        json_response = response.json()
        # https://docs.pinata.cloud/api-reference/endpoint/upload-a-file#response-data-cid
        cid = json_response['data']['cid']
        # https://docs.pinata.cloud/ipfs-101/what-are-cids
        public_uri = _PINATA_PUBLIC_GATEWAY_FMT.format(cid=cid)
        self._logger.info(f'created public uri {public_uri}')
        # self._pin_cid(cid)
        return public_uri

    def read_json(self, uri: str) -> dict:
        response = requests.get(uri)
        response.raise_for_status()
        return response.json()

    def write_json(self, payload: dict) -> IpfsUri:
        with tempfile.NamedTemporaryFile(suffix=".pinata") as tmp:
            json.dump(payload)
            return self.write_file(tmp)
