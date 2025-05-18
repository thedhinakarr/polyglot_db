const BaseRepository = require('./base.repository');

class SessionRepository extends BaseRepository {
  constructor(redisClient) {
    super(redisClient);
  }
  
  async findOne(sessionId) {
    try {
      const session = await this.dbClient.get(`session:${sessionId}`);
      return session ? JSON.parse(session) : null;
    } catch (error) {
      console.error('Error in findOne:', error);
      throw error;
    }
  }
  
  async create(session) {
    try {
      await this.dbClient.set(
        `session:${session.id}`, 
        JSON.stringify(session),
        'EX',
        session.expiresIn || 3600 // Default 1 hour
      );
      return session;
    } catch (error) {
      console.error('Error in create:', error);
      throw error;
    }
  }
  
  async delete(sessionId) {
    try {
      await this.dbClient.del(`session:${sessionId}`);
      return true;
    } catch (error) {
      console.error('Error in delete:', error);
      throw error;
    }
  }
}

module.exports = SessionRepository;