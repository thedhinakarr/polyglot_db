import os
from pydantic import BaseSettings
from dotenv import load_dotenv

# Load environment variables from .env file
load_dotenv()

class Settings(BaseSettings):
    # Server settings
    host: str = os.getenv("HOST", "0.0.0.0")
    port: int = int(os.getenv("PORT", "5000"))
    
    # InfluxDB settings
    influxdb_url: str = os.getenv("INFLUXDB_URL", "http://localhost:8086")
    influxdb_token: str = os.getenv("INFLUXDB_TOKEN", "mytoken")
    influxdb_org: str = os.getenv("INFLUXDB_ORG", "analytics")
    influxdb_bucket: str = os.getenv("INFLUXDB_BUCKET", "metrics")
    
    # App settings
    debug: bool = os.getenv("DEBUG", "False").lower() in ("true", "1", "t")
    
    class Config:
        env_file = ".env"

# Create settings instance
settings = Settings()