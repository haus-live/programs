from typing import Optional
from pydantic_settings import BaseSettings


class AppConfig(BaseSettings):
    class Config:
        env_file = '.env'

    LOG_LEVEL: str = 'INFO'

    PINATA_JWT: str
    PINATA_UPLOAD_BASE_URL: str = "https://uploads.pinata.cloud/v3"
    PINATA_API_BASE_URL: str = "https://api.pinata.cloud/v3"
    PINATA_DOMAIN: Optional[str]
    PINATA_API_KEY: Optional[str]
    PINATA_API_SECRET: Optional[str]

    SOLANA_RPC_URL: str = "https://api.devnet.solana.com"
    HAUS_PROGRAM_ID: str = "GZtbVznhmHTqn6PbiSN6PdJNPBboMW5gkCYszq9caNQ1"
    MPL_CORE_ASSET_PROGRAM_ID: str = "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
    CHUNK_UPDATER_PUBLIC_KEY: str
    CHUNK_UPDATER_SECRET_KEY: list[int]

    RESTREAMER_HLS_BASE_URI: str
    # CHUNK_DURATION: int = 60
    CHUNK_DURATION: int = 60

    SCHEDULER_MAX_INSTANCES: int = 100

    APP_PORT: int = 8888
