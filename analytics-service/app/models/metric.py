from datetime import datetime
from typing import Optional, Dict, Any
from pydantic import BaseModel, Field
import uuid

class Metric(BaseModel):
    """Model representing a metric data point."""
    id: Optional[str] = Field(default_factory=lambda: str(uuid.uuid4()))
    name: str
    value: float
    tags: Dict[str, str] = {}
    timestamp: datetime = Field(default_factory=datetime.utcnow)
    
    class Config:
        json_schema_extra = {
            "example": {
                "name": "cpu_usage",
                "value": 75.5,
                "tags": {"host": "server-01", "region": "us-west"},
                "timestamp": "2023-09-25T12:00:00Z"
            }
        }

class MetricQuery(BaseModel):
    """Model for querying metrics."""
    name: Optional[str] = None
    tags: Dict[str, str] = {}
    start_time: Optional[datetime] = None
    end_time: Optional[datetime] = None
    aggregation: Optional[str] = None  # sum, mean, count, etc.
    
    class Config:
        json_schema_extra = {
            "example": {
                "name": "cpu_usage",
                "tags": {"host": "server-01"},
                "start_time": "2023-09-25T00:00:00Z",
                "end_time": "2023-09-25T23:59:59Z",
                "aggregation": "mean"
            }
        }