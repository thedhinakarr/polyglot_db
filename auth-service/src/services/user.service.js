const bcrypt = require('bcrypt');
const jwt = require('jsonwebtoken');
const { v4: uuidv4 } = require('uuid');

class UserService {
  constructor(userRepository, sessionRepository) {
    this.userRepository = userRepository;
    this.sessionRepository = sessionRepository;
  }
  
  async register(userData) {
    // Check if user already exists
    const existingUser = await this.userRepository.findByEmail(userData.email);
    if (existingUser) {
      throw new Error('User with this email already exists');
    }
    
    // Hash password
    const salt = await bcrypt.genSalt(10);
    const hashedPassword = await bcrypt.hash(userData.password, salt);
    
    // Create new user
    const newUser = await this.userRepository.create({
      ...userData,
      password: hashedPassword
    });
    
    // Remove password from response
    const { password, ...userWithoutPassword } = newUser;
    
    return userWithoutPassword;
  }
  
  async login(email, password) {
    // Find user by email
    const user = await this.userRepository.findByEmail(email);
    if (!user) {
      throw new Error('Invalid credentials');
    }
    
    // Verify password
    const isPasswordValid = await bcrypt.compare(password, user.password);
    if (!isPasswordValid) {
      throw new Error('Invalid credentials');
    }
    
    // Generate JWT token
    const token = jwt.sign(
      { id: user.id, email: user.email },
      process.env.JWT_SECRET,
      { expiresIn: '1h' }
    );
    
    // Create session
    const sessionId = uuidv4();
    await this.sessionRepository.create({
      id: sessionId,
      userId: user.id,
      expiresIn: 3600 // 1 hour
    });
    
    return {
      token,
      sessionId,
      user: {
        id: user.id,
        name: user.name,
        email: user.email
      }
    };
  }
  
  async logout(sessionId) {
    return this.sessionRepository.delete(sessionId);
  }
  
  async getUserById(userId) {
    const user = await this.userRepository.findOne(userId);
    if (!user) {
      throw new Error('User not found');
    }
    
    // Remove password from response
    const { password, ...userWithoutPassword } = user;
    
    return userWithoutPassword;
  }
}

module.exports = UserService;