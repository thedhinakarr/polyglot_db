from typing import List, Optional, Dict
from datetime import datetime
from fastapi import APIRouter, Depends, HTTPException, Query
from app.models.metric import Metric, MetricQuery
from app.services.analytics_service import AnalyticsService

router = APIRouter(prefix="/api/analytics", tags=["analytics"])

def get_analytics_service():
    """Dependency for getting the analytics service."""
    return AnalyticsService()

@router.post("/metrics", response_model=Metric, status_code=201)
async def record_metric(
    metric: Metric, 
    service: AnalyticsService = Depends(get_analytics_service)
):
    """Record a new metric."""
    return await service.record_metric(metric)

@router.get("/metrics", response_model=List[Metric])
async def get_metrics(
    name: Optional[str] = None,
    start_time: Optional[datetime] = None,
    end_time: Optional[datetime] = None,
    aggregation: Optional[str] = None,
    service: AnalyticsService = Depends(get_analytics_service),
    tag_host: Optional[str] = Query(None, description="Filter by host tag"),
    tag_region: Optional[str] = Query(None, description="Filter by region tag"),
    tag_unit: Optional[str] = Query(None, description="Filter by unit tag")
):
    """Get metrics based on query parameters."""
    # Build tags dictionary from individual parameters
    tags = {}
    if tag_host:
        tags["host"] = tag_host
    if tag_region:
        tags["region"] = tag_region
    if tag_unit:
        tags["unit"] = tag_unit
    
    query = MetricQuery(
        name=name,
        tags=tags,
        start_time=start_time,
        end_time=end_time,
        aggregation=aggregation
    )
    return await service.get_metrics(query)

@router.get("/metrics/recent/{name}", response_model=List[Metric])
async def get_recent_metrics(
    name: str,
    hours: int = Query(24, gt=0, lt=168),  # 1 to 168 hours (1 week)
    service: AnalyticsService = Depends(get_analytics_service)
):
    """Get metrics from the last N hours."""
    return await service.get_recent_metrics(name, hours)

@router.get("/metrics/aggregated/{name}", response_model=List[Metric])
async def get_aggregated_metrics(
    name: str,
    aggregation: str = Query(..., regex="^(mean|sum|count|min|max)$"),
    hours: int = Query(24, gt=0, lt=168),  # 1 to 168 hours (1 week)
    service: AnalyticsService = Depends(get_analytics_service),
    tag_host: Optional[str] = Query(None, description="Filter by host tag"),
    tag_region: Optional[str] = Query(None, description="Filter by region tag"),
    tag_unit: Optional[str] = Query(None, description="Filter by unit tag")
):
    """Get aggregated metrics."""
    # Build tags dictionary from individual parameters
    tags = {}
    if tag_host:
        tags["host"] = tag_host
    if tag_region:
        tags["region"] = tag_region
    if tag_unit:
        tags["unit"] = tag_unit
    
    return await service.get_aggregated_metrics(name, aggregation, tags, hours)

@router.get("/health")
async def health_check():
    """Health check endpoint."""
    return {"status": "ok", "message": "Analytics service is running"}
