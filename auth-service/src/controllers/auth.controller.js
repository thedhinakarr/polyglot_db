class AuthController {
  constructor(userService) {
    this.userService = userService;
  }
  
  register = async (req, res) => {
    try {
      const userData = req.body;
      const user = await this.userService.register(userData);
      res.status(201).json(user);
    } catch (error) {
      console.error('Registration error:', error);
      res.status(400).json({ message: error.message });
    }
  };
  
  login = async (req, res) => {
    try {
      const { email, password } = req.body;
      const result = await this.userService.login(email, password);
      res.status(200).json(result);
    } catch (error) {
      console.error('Login error:', error);
      res.status(401).json({ message: error.message });
    }
  };
  
  logout = async (req, res) => {
    try {
      const { sessionId } = req.body;
      await this.userService.logout(sessionId);
      res.status(200).json({ message: 'Logged out successfully' });
    } catch (error) {
      console.error('Logout error:', error);
      res.status(500).json({ message: error.message });
    }
  };
  
  getProfile = async (req, res) => {
    try {
      const userId = req.user.id; // This will come from auth middleware
      const user = await this.userService.getUserById(userId);
      res.status(200).json(user);
    } catch (error) {
      console.error('Get profile error:', error);
      res.status(404).json({ message: error.message });
    }
  };
}

module.exports = AuthController;