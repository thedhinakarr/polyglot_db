# Auth Service API Documentation

The Auth Service provides user authentication and management capabilities.

## Base URL
http://localhost:4000/api/auth

## Endpoints

### Register User

Creates a new user account.

- **URL**: `/register`
- **Method**: `POST`
- **Request Body**:
  ```json
  {
    "name": "User Name",
    "email": "user@example.com",
    "password": "password123"
  }

Success Response:

Code: 201 Created
Content:
json{
  "id": 1,
  "name": "User Name",
  "email": "user@example.com",
  "created_at": "2023-05-18T14:30:00Z",
  "updated_at": "2023-05-18T14:30:00Z"
}



Error Response:

Code: 400 Bad Request
Content:
json{
  "message": "User with this email already exists"
}




Login
Authenticates a user and returns a JWT token.

URL: /login
Method: POST
Request Body:
json{
  "email": "user@example.com",
  "password": "password123"
}

Success Response:

Code: 200 OK
Content:
json{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "sessionId": "a1b2c3d4-e5f6-g7h8-i9j0-k1l2m3n4o5p6",
  "user": {
    "id": 1,
    "name": "User Name",
    "email": "user@example.com"
  }
}



Error Response:

Code: 401 Unauthorized
Content:
json{
  "message": "Invalid credentials"
}




Get User Profile
Retrieves the authenticated user's profile.

URL: /profile
Method: GET
Headers:

Authorization: Bearer <JWT_TOKEN>


Success Response:

Code: 200 OK
Content:
json{
  "id": 1,
  "name": "User Name",
  "email": "user@example.com",
  "created_at": "2023-05-18T14:30:00Z",
  "updated_at": "2023-05-18T14:30:00Z"
}



Error Response:

Code: 401 Unauthorized
Content:
json{
  "message": "No token provided"
}


OR

Code: 401 Unauthorized
Content:
json{
  "message": "Invalid token"
}




Logout
Invalidates the user's session.

URL: /logout
Method: POST
Request Body:
json{
  "sessionId": "a1b2c3d4-e5f6-g7h8-i9j0-k1l2m3n4o5p6"
}

Success Response:

Code: 200 OK
Content:
json{
  "message": "Logged out successfully"
}



Error Response:

Code: 500 Internal Server Error
Content:
json{
  "message": "Error message"
}
