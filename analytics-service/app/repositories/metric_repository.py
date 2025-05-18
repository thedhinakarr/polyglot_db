from typing import List, Optional, Dict, Any
import uuid
from datetime import datetime, timedelta
from influxdb_client.client.write_api import Point
from app.repositories.base_repository import BaseRepository
from app.repositories.influxdb_client import InfluxDBClientSingleton
from app.models.metric import Metric, MetricQuery

class MetricRepository(BaseRepository[Metric, str]):
    """Repository for handling metrics in InfluxDB."""
    
    def __init__(self):
        self.influxdb = InfluxDBClientSingleton()
    
    async def find_by_id(self, id: str) -> Optional[Metric]:
        """Find a metric by ID (not typically used with InfluxDB)."""
        # InfluxDB doesn't have a natural concept of IDs like relational DBs
        # This is a placeholder implementation
        return None
    
    async def find_all(self) -> List[Metric]:
        """Find all metrics (limited to recent metrics for practicality)."""
        # For InfluxDB, fetching "all" metrics without filters would be impractical
        # So we'll limit to the last 24 hours
        query = MetricQuery(
            start_time=datetime.utcnow() - timedelta(days=1),
            end_time=datetime.utcnow()
        )
        return await self.query_metrics(query)
    
    async def create(self, metric: Metric) -> Metric:
        """Create a new metric."""
        # Ensure metric has an ID
        if not metric.id:
            metric.id = str(uuid.uuid4())
        
        # Create a Point
        point = Point(metric.name)
        
        # Add value
        point = point.field("value", metric.value)
        
        # Add tags
        for key, value in metric.tags.items():
            point = point.tag(key, value)
        
        # Add timestamp if provided
        if metric.timestamp:
            point = point.time(metric.timestamp)
        
        # Write to InfluxDB
        self.influxdb.write_api.write(
            bucket=self.influxdb.bucket,
            record=point
        )
        
        return metric
    
    async def update(self, id: str, metric: Metric) -> Metric:
        """Update a metric (not typically used with InfluxDB)."""
        # InfluxDB is append-only, so "updating" is actually creating a new point
        # This is a placeholder implementation that simply creates a new metric
        return await self.create(metric)
    
    async def delete(self, id: str) -> None:
        """Delete a metric (not typically used with InfluxDB)."""
        # InfluxDB doesn't support deleting individual points easily
        # This would require retention policies or predicate-based deletion
        # This is a placeholder implementation
        pass
    
    async def query_metrics(self, query: MetricQuery) -> List[Metric]:
        """Query metrics based on the provided parameters."""
        # Start building the Flux query
        flux_query = f'from(bucket: "{self.influxdb.bucket}")'
        
        # Add time range
        if query.start_time and query.end_time:
            flux_query += f' |> range(start: {query.start_time.isoformat()}Z, stop: {query.end_time.isoformat()}Z)'
        elif query.start_time:
            flux_query += f' |> range(start: {query.start_time.isoformat()}Z)'
        else:
            # Default to last 24 hours if no time range is specified
            flux_query += ' |> range(start: -24h)'
        
        # Filter by measurement name if provided
        if query.name:
            flux_query += f' |> filter(fn: (r) => r._measurement == "{query.name}")'
        
        # Filter by tags if provided
        for key, value in query.tags.items():
            flux_query += f' |> filter(fn: (r) => r.{key} == "{value}")'
        
        # Apply aggregation if provided
        if query.aggregation:
            # Group by appropriate fields
            flux_query += ' |> group(columns: ["_measurement"'
            for tag in query.tags.keys():
                flux_query += f', "{tag}"'
            flux_query += '])'
            
            # Apply the aggregation function
            flux_query += f' |> {query.aggregation}()'
        
        # Execute the query
        result = self.influxdb.query_api.query(query=flux_query, org=self.influxdb.org)
        
        # Process the results
        metrics = []
        for table in result:
            for record in table.records:
                tags = {}
                for key, value in record.values.items():
                    if key not in ['_time', '_value', '_measurement', '_field'] and not key.startswith('_'):
                        tags[key] = value
                
                metric = Metric(
                    id=str(uuid.uuid4()),  # InfluxDB doesn't return IDs, so generate a new one
                    name=record.get_measurement(),
                    value=record.get_value(),
                    tags=tags,
                    timestamp=record.get_time(),
                )
                metrics.append(metric)
        
        return metrics