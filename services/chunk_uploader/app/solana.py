from solana.rpc.api import Client

from app.config import AppConfig

class Solana:
    def __init__(self, config: AppConfig):
        self._config = config
        self._solana = Client(self._config.SOLANA_RPC_URL)
    
    def read_uri_from_asset_account(self, asset_pubkey: bytes):
        ...
    
    def update_cose_asset_account_uri(self):
        ...
