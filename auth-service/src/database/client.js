const { Pool } = require('pg');
const Redis = require('ioredis');

class DatabaseFactory {
  static createPostgresClient(config) {
    return new PostgresClient(config);
  }
  
  static createRedisClient(config) {
    return new RedisClient(config);
  }
}

class PostgresClient {
  constructor(config) {
    this.pool = new Pool(config);
  }
  
  async query(text, params) {
    try {
      const result = await this.pool.query(text, params);
      return result;
    } catch (error) {
      console.error('Database query error:', error);
      throw error;
    }
  }
  
  async getClient() {
    const client = await this.pool.connect();
    return client;
  }
  
  async healthCheck() {
    try {
      await this.pool.query('SELECT NOW()');
      return true;
    } catch (error) {
      console.error('Database health check failed:', error);
      return false;
    }
  }
}

class RedisClient {
  constructor(config) {
    this.client = new Redis(config);
  }
  
  async get(key) {
    try {
      return await this.client.get(key);
    } catch (error) {
      console.error('Redis get error:', error);
      throw error;
    }
  }
  
  async set(key, value, ...options) {
    try {
      return await this.client.set(key, value, ...options);
    } catch (error) {
      console.error('Redis set error:', error);
      throw error;
    }
  }
  
  async del(key) {
    try {
      return await this.client.del(key);
    } catch (error) {
      console.error('Redis del error:', error);
      throw error;
    }
  }
  
  async healthCheck() {
    try {
      await this.client.ping();
      return true;
    } catch (error) {
      console.error('Redis health check failed:', error);
      return false;
    }
  }
}

module.exports = {
  DatabaseFactory,
  PostgresClient,
  RedisClient
};