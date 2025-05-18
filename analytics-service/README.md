# Analytics Service

This service handles data processing and analysis for the Polyglot Persistence System.

## Features

- Time-series data processing
- Reporting functionality
- Data analysis

## Technologies

- Python
- InfluxDB for time-series data
- FastAPI for API endpoints

## Setup

1. Create and activate virtual environment:
   ```bash
   python -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate
   ```

2. Install dependencies:
   ```bash
   pip install -r requirements.txt
   ```

3. Create `.env` file (use `.env.example` as a template when available)

4. Run the service:
   ```bash
   python app.py
   ```

## Testing

```bash
python -m pytest
```

## API Documentation

See [Analytics Service API Documentation](../docs/api/analytics-service.md) for detailed API information when available.
