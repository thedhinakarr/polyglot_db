class BaseRepository {
  constructor(dbClient) {
    this.dbClient = dbClient;
  }
  
  async findOne(id) {
    throw new Error('Method not implemented');
  }
  
  async findMany(criteria) {
    throw new Error('Method not implemented');
  }
  
  async create(entity) {
    throw new Error('Method not implemented');
  }
  
  async update(id, entity) {
    throw new Error('Method not implemented');
  }
  
  async delete(id) {
    throw new Error('Method not implemented');
  }
}

module.exports = BaseRepository;