from typing import Optional
from pydantic import BaseSettings


class AppConfig(BaseSettings):
    PINATA_JWT: str
    PINATA_BASE_URL: str = "https://uploads.pinata.cloud/v3"
    PINATA_DOMAIN: Optional[str]
    PINATA_API_KEY: Optional[str]
    PINATA_API_SECRET: Optional[str]

    SOLANA_RPC_URL: str
    MPL_CORE_ASSET_PROGRAM_ID: str = "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"

    RESTREAMER_HLS_BASE_URI: str
    CHUNK_DURATION: int = 60

    SCHEDULER_MAX_INSTANCES: int = 100

    APP_PORT: int =8888
