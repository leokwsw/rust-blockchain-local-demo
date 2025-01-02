# Simple Blockchain Server

A lightweight, JSON file-based blockchain backend server built with Rust. This application implements a simple
blockchain with basic functionality, such as adding blocks and retrieving the blockchain, and persists data in a JSON
file.

## Features

- **Basic Blockchain**: Supports adding and retrieving blocks.
- **Data Persistence**: Stores blockchain data in a JSON file for durability.
- **RESTful API**: Exposes endpoints for interacting with the blockchain.
- **Lightweight**: No external database or authentication.

## Technologies Used

- **Rust**: Core programming language.
- **Actix-web**: Web framework for handling RESTful APIs.
- **Serde**: For JSON serialization and deserialization.
- **UUID**: Generates unique IDs for blocks.
- **Chrono**: Handles timestamps.
- **MD5**: For generating block hashes.

## Endpoints

### 1. **Get All Blocks**

- **URL**: `/`
- **Method**: `GET`
- **Description**: Fetches the entire blockchain.
- **Response**:
  ```json
  [
      {
          "id": "1e7886f1-3a7f-4bcb-830d-fc4f21c6e0e8",
          "timestamp": 1728393021104,
          "data": "Genesis Block",
          "previous_hash": "0",
          "hash": "e7c6c52089c6d1d06282f0c20e3e5cfb"
      },
      {
          "id": "4e36f7c3-11d2-42a3-8ea4-20d7e7684d30",
          "timestamp": 1728393025115,
          "data": "Block Data 1",
          "previous_hash": "e7c6c52089c6d1d06282f0c20e3e5cfb",
          "hash": "42b8b90cb972f47b1763b65f5bcb3067"
      }
  ]
  ```

### 2. **Add a Block**

- **URL**: `/add`
- **Method**: `POST`
- **Description**: Adds a new block to the blockchain.
- **Request Body**:
  ```json
  {
      "data": "Your block data here"
  }
  ```
- **Response**: `Block added successfully`

## How to Run

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable version)

### Steps

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd blockchain-server
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Run the server
   ```bash
   cargo run
   ```

4. The server will be available at [http://127.0.0.1:8080](http://127.0.0.1:8080).

## Testing with curl

### Get all blocks:

```bash
curl http://127.0.0.1:8080/
```

### Add a block:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"data": "Block Data"}' http://127.0.0.1:8080/add
```

# Project Structure

```
.
├── src
│   ├── main.rs           # Main application logic
│   ├── blockchain.rs     # Blockchain and block structure
├── Cargo.toml            # Dependencies and metadata
└── blockchain.json       # JSON file to store blockchain data (auto-created)
```

# Future Improvements

- Add authentication (e.g., API key or JWT).
- Introduce more robust hashing algorithms (e.g., SHA-256).
- Implement consensus mechanisms like Proof of Work (PoW).
- Support distributed blockchain nodes.
- Optimize JSON file handling for larger datasets.

# License

This project is licensed under the MIT License.