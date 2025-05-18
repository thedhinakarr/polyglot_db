from influxdb_client import InfluxDBClient
from influxdb_client.client.write_api import SYNCHRONOUS
from app.config import settings

class InfluxDBClientSingleton:
    """Singleton class for InfluxDB client."""
    
    _instance = None
    
    def __new__(cls):
        if cls._instance is None:
            cls._instance = super(InfluxDBClientSingleton, cls).__new__(cls)
            cls._instance.client = InfluxDBClient(
                url=settings.influxdb_url,
                token=settings.influxdb_token,
                org=settings.influxdb_org,
            )
            cls._instance.write_api = cls._instance.client.write_api(write_options=SYNCHRONOUS)
            cls._instance.query_api = cls._instance.client.query_api()
            cls._instance.bucket = settings.influxdb_bucket
            cls._instance.org = settings.influxdb_org
        return cls._instance

    def close(self):
        """Close the client connection."""
        if self.client:
            self.client.close()