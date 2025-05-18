const express = require('express');
const AuthController = require('../controllers/auth.controller');
const authMiddleware = require('../middlewares/auth.middleware');

const createAuthRouter = (authController) => {
  const router = express.Router();
  
  // Public routes
  router.post('/register', authController.register);
  router.post('/login', authController.login);
  router.post('/logout', authController.logout);
  
  // Protected routes
  router.get('/profile', authMiddleware, authController.getProfile);
  
  return router;
};

module.exports = createAuthRouter;