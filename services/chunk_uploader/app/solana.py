from app.config import AppConfig

class SolanaWrapper:
    def __init__(self, config: AppConfig):
        self._config = config
    
    def read_uri_from_asset_account(self):
        ...
    
    def update_cose_asset_account_uri(self):
        ...
