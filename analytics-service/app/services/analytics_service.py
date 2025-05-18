from datetime import datetime, timedelta
from typing import List, Dict, Any, Optional
from app.models.metric import Metric, MetricQuery
from app.repositories.metric_repository import MetricRepository

class AnalyticsService:
    """Service for handling analytics operations."""
    
    def __init__(self):
        self.repository = MetricRepository()
    
    async def get_metrics(self, query: MetricQuery) -> List[Metric]:
        """Get metrics based on query parameters."""
        return await self.repository.query_metrics(query)
    
    async def record_metric(self, metric: Metric) -> Metric:
        """Record a new metric."""
        return await self.repository.create(metric)
    
    async def get_recent_metrics(self, name: str, hours: int = 24) -> List[Metric]:
        """Get metrics from the last N hours."""
        query = MetricQuery(
            name=name,
            start_time=datetime.utcnow() - timedelta(hours=hours),
            end_time=datetime.utcnow()
        )
        return await self.repository.query_metrics(query)
    
    async def get_aggregated_metrics(
        self, 
        name: str, 
        aggregation: str, 
        tags: Optional[Dict[str, str]] = None, 
        hours: int = 24
    ) -> List[Metric]:
        """Get aggregated metrics."""
        query = MetricQuery(
            name=name,
            tags=tags or {},
            start_time=datetime.utcnow() - timedelta(hours=hours),
            end_time=datetime.utcnow(),
            aggregation=aggregation
        )
        return await self.repository.query_metrics(query)