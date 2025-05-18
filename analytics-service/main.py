import uvicorn
from fastapi import FastAPI
from app.api import router
from app.config import settings

# Create FastAPI app
app = FastAPI(
    title="Analytics Service",
    description="Analytics service for polyglot persistence system",
    version="1.0.0",
)

# Include API routes
app.include_router(router)

if __name__ == "__main__":
    uvicorn.run(
        "main:app",
        host=settings.host,
        port=settings.port,
        reload=settings.debug
    )