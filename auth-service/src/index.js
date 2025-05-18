const express = require('express');
const dotenv = require('dotenv');
const { DatabaseFactory } = require('./database/client');
const { setupDatabase } = require('./database/migrations/setup');
const UserRepository = require('./repositories/user.repository');
const SessionRepository = require('./repositories/session.repository');
const UserService = require('./services/user.service');
const AuthController = require('./controllers/auth.controller');
const createAuthRouter = require('./routes/auth.routes');

// Load environment variables
dotenv.config();

// Initialize Express app
const app = express();
app.use(express.json());

// Initialize database connections
const pgConfig = {
  host: process.env.POSTGRES_HOST || 'localhost',
  port: process.env.POSTGRES_PORT || 5432,
  user: process.env.POSTGRES_USER || 'postgres',
  password: process.env.POSTGRES_PASSWORD || 'postgres',
  database: process.env.POSTGRES_DB || 'auth_service'
};

const redisConfig = {
  host: process.env.REDIS_HOST || 'localhost',
  port: process.env.REDIS_PORT || 6379
};

const pgClient = DatabaseFactory.createPostgresClient(pgConfig);
const redisClient = DatabaseFactory.createRedisClient(redisConfig);

// Initialize repositories
const userRepository = new UserRepository(pgClient);
const sessionRepository = new SessionRepository(redisClient);

// Initialize services
const userService = new UserService(userRepository, sessionRepository);

// Initialize controllers
const authController = new AuthController(userService);

// Set up routes
app.use('/api/auth', createAuthRouter(authController));

// Health check endpoint
app.get('/health', async (req, res) => {
  const pgHealth = await pgClient.healthCheck();
  const redisHealth = await redisClient.healthCheck();
  
  if (pgHealth && redisHealth) {
    return res.status(200).json({ status: 'ok', message: 'All systems operational' });
  }
  
  const issues = [];
  if (!pgHealth) issues.push('PostgreSQL connection failed');
  if (!redisHealth) issues.push('Redis connection failed');
  
  return res.status(500).json({ 
    status: 'error', 
    message: 'System issues detected', 
    issues 
  });
});

// Basic route
app.get('/', (req, res) => {
  res.json({ message: 'Auth service is running' });
});

// Setup database on startup
const startServer = async () => {
  try {
    // Set up database tables
    await setupDatabase();
    
    // Start server
    const PORT = process.env.PORT || 4000;
    app.listen(PORT, () => {
      console.log(`Auth service running on port ${PORT}`);
    });
  } catch (error) {
    console.error('Failed to start server:', error);
    process.exit(1);
  }
};

startServer();

module.exports = app; // For testing