from solana.rpc.api import Client
from solders.pubkey import Pubkey
from solders.keypair import Keypair

from solders.system_program import TransferParams, transfer
from solders.account import Account

from app.config import AppConfig

class Solana:
    def __init__(self, config: AppConfig):
        self._config = config
        self._solana = Client(self._config.SOLANA_RPC_URL)
    
    def read_uri_from_asset_account(self, asset_pubkey: str):
        pubkey = Pubkey.from_string(asset_pubkey)
        account_info_json = self._solana.get_account_info_json_parsed(pubkey)
        return account_info_json['uri']
    
    def update_core_asset_account_uri(self):
        ...
