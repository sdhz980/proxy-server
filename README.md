# Simple JWT Auth using Actix Web, Jsonwebtoken, Sqlx, and MySQL

This project demonstrates a simple JWT-based authentication system using the following technologies:
- [Actix Web](https://actix.rs/): A powerful, pragmatic, and extremely fast web framework for Rust.
- [jsonwebtoken](https://github.com/Keats/jsonwebtoken): A crate for creating and parsing JWTs.
- [SQLx](https://github.com/launchbadge/sqlx): An async, pure Rust SQL crate featuring compile-time checked queries without a DSL.
- [MySQL](https://www.mysql.com/): An open-source relational database management system.

## Features

- User registration and login
- JWT-based authentication
- Secure password hashing

## Prerequisites

- Rust (latest stable version)
- MySQL

## Getting Started

### Clone the repository

```sh
git clone https://github.com/yourusername/actix_simple_auth_jwt.git
cd actix_simple_auth_jwt
```

### Set up the database

1. Create a MySQL database:
    ```sql
    CREATE DATABASE actix_auth;
    ```

2. Create a `.env` file in the project root and add your database connection details:
    ```env
    DATABASE_URL=mysql://username:password@localhost/actix_auth
    ```

### Run the application

1. Install dependencies:
    ```sh
    cargo build
    ```

2. Run the application:
    ```sh
    cargo run
    ```

The server will start on `http://localhost:8080`.

## API Endpoints

### Register

- **URL:** `/register`
- **Method:** `POST`
- **Body:**
    ```json
    {
        "username": "your_username",
        "password": "your_password"
    }
    ```

### Login

- **URL:** `/login`
- **Method:** `POST`
- **Body:**
    ```json
    {
        "username": "your_username",
        "password": "your_password"
    }
    ```

### Protected Route

- **URL:** `/protected`
- **Method:** `GET`
- **Headers:**
    ```http
    Authorization: Bearer <your_jwt_token>
    ```

## License

This project is licensed under the MIT License.

## Acknowledgements

- [Actix Web](https://actix.rs/)
- [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
- [SQLx](https://github.com/launchbadge/sqlx)
- [MySQL](https://www.mysql.com/)
