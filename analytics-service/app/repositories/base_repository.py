from abc import ABC, abstractmethod
from typing import Generic, TypeVar, Optional, List, Dict, Any

T = TypeVar('T')
ID = TypeVar('ID')

class BaseRepository(Generic[T, ID], ABC):
    """Base repository interface."""
    
    @abstractmethod
    async def find_by_id(self, id: ID) -> Optional[T]:
        """Find an entity by its ID."""
        pass
    
    @abstractmethod
    async def find_all(self) -> List[T]:
        """Find all entities."""
        pass
    
    @abstractmethod
    async def create(self, entity: T) -> T:
        """Create a new entity."""
        pass
    
    @abstractmethod
    async def update(self, id: ID, entity: T) -> T:
        """Update an existing entity."""
        pass
    
    @abstractmethod
    async def delete(self, id: ID) -> None:
        """Delete an entity by its ID."""
        pass