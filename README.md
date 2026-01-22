# Soldier Management API

A REST API built with Rust and Actix-web for managing soldier data with MongoDB as the database backend.

## Features

- **RESTful API** for soldier management
- **MongoDB** integration with auto-incrementing IDs
- **Data validation** using validator crate
- **Swagger UI** for API documentation and testing
- **OpenAPI 3.0** specification
- **Environment-based configuration**

## Tech Stack

- **Rust** (2024 edition)
- **Actix-web 4** - Web framework
- **MongoDB 3.1** - Database driver
- **Serde** - Serialization/deserialization
- **Validator** - Data validation
- **Utoipa** - OpenAPI documentation
- **Tokio** - Async runtime

## Prerequisites

- Rust (2024 edition or later)
- MongoDB instance running on `localhost:27017`
- Cargo (Rust package manager)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd personal-project
```

2. Create a `.env` file in the project root:
```bash
MONGODB_URI=mongodb://root:example@localhost:27017
DB_NAME=personal
```

3. Install dependencies:
```bash
cargo build
```

## Running the Application

Start the server:
```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`

## API Documentation

Once the server is running, access the interactive Swagger UI at:
```
http://127.0.0.1:8080/swagger-ui/
```

## API Endpoints

### Get All Soldiers
```http
GET /soldiers
```

**Response:** `200 OK`
```json
[
  {
    "_id": 1,
    "name": "John Doe",
    "rank": "Sergeant",
    "squad": "Alpha"
  }
]
```

### Add a Soldier
```http
POST /soldiers
Content-Type: application/json
```

**Request Body:**
```json
{
  "name": "John Doe",
  "rank": "Sergeant",
  "squad": "Alpha"
}
```

**Responses:**
- `201 Created` - Soldier created successfully
- `400 Bad Request` - Validation failed
- `500 Internal Server Error` - Server error

## Data Model

### Soldier
| Field | Type | Validation | Description |
|-------|------|------------|-------------|
| `_id` | `i64` | Auto-generated | Unique identifier |
| `name` | `String` | 1-100 chars | Soldier's name |
| `rank` | `String` | 1-50 chars | Military rank |
| `squad` | `String` | 1-50 chars | Squad assignment |

## Database

The application uses MongoDB with the following collections:
- **soldiers** - Stores soldier records
- **counters** - Manages auto-incrementing IDs

### Auto-Increment Implementation

The API implements custom auto-incrementing IDs for soldiers using a `counters` collection, ensuring each soldier gets a unique sequential ID.

## Configuration

Environment variables (configured in `.env`):

| Variable | Default | Description |
|----------|---------|-------------|
| `MONGODB_URI` | `mongodb://root:example@localhost:27017` | MongoDB connection string |
| `DB_NAME` | `personal` | Database name |

## Development

### Project Structure
```
src/
├── main.rs       # Application entry point and server setup
├── handlers.rs   # API endpoint handlers
└── model.rs      # Data models and schemas
```

### Build for Release
```bash
cargo build --release
```

### Run Tests
```bash
cargo test
```

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]
