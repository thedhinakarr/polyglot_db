const BaseRepository = require('./base.repository');

class UserRepository extends BaseRepository {
  constructor(dbClient) {
    super(dbClient);
    this.tableName = 'users';
  }
  
  async findOne(id) {
    try {
      const result = await this.dbClient.query(
        `SELECT * FROM ${this.tableName} WHERE id = $1`,
        [id]
      );
      return result.rows[0] || null;
    } catch (error) {
      console.error('Error in findOne:', error);
      throw error;
    }
  }
  
  async findByEmail(email) {
    try {
      const result = await this.dbClient.query(
        `SELECT * FROM ${this.tableName} WHERE email = $1`,
        [email]
      );
      return result.rows[0] || null;
    } catch (error) {
      console.error('Error in findByEmail:', error);
      throw error;
    }
  }
  
  async create(user) {
    try {
      const { name, email, password } = user;
      const result = await this.dbClient.query(
        `INSERT INTO ${this.tableName} (name, email, password) 
         VALUES ($1, $2, $3) 
         RETURNING *`,
        [name, email, password]
      );
      return result.rows[0];
    } catch (error) {
      console.error('Error in create:', error);
      throw error;
    }
  }
  
  async update(id, user) {
    try {
      const { name, email } = user;
      const result = await this.dbClient.query(
        `UPDATE ${this.tableName} 
         SET name = $1, email = $2, updated_at = NOW() 
         WHERE id = $3 
         RETURNING *`,
        [name, email, id]
      );
      return result.rows[0] || null;
    } catch (error) {
      console.error('Error in update:', error);
      throw error;
    }
  }
  
  async delete(id) {
    try {
      await this.dbClient.query(
        `DELETE FROM ${this.tableName} WHERE id = $1`,
        [id]
      );
      return true;
    } catch (error) {
      console.error('Error in delete:', error);
      throw error;
    }
  }
}

module.exports = UserRepository;