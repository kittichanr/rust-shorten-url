# Rust URL Shortener

A high-performance URL shortener service built with Rust and Actix Web, featuring Redis for caching and rate limiting.

## 🚀 Features

- **URL Shortening**: Convert long URLs into short, manageable links
- **Custom Short URLs**: Support for custom short URL aliases
- **Expiration Control**: Set custom expiration times for shortened URLs
- **Rate Limiting**: IP-based rate limiting to prevent abuse
- **Redis Caching**: Fast URL resolution with Redis backend
- **Base62 Encoding**: Efficient short URL generation
- **HTTP Redirects**: Proper 301 redirects for URL resolution
- **Analytics**: Click counter tracking for shortened URLs

## 🛠️ Tech Stack

- **Framework**: Actix Web 4.11.0
- **Database**: Redis (with r2d2 connection pooling)
- **Configuration**: YAML-based configuration
- **Serialization**: Serde for JSON handling
- **URL Validation**: Built-in URL parsing and validation
- **Random Generation**: Rand crate for unique ID generation

## 📋 Prerequisites

- Rust 1.70+ (2024 edition)
- Redis server
- Docker (optional, for Redis)

## 🚀 Quick Start

### 1. Clone the Repository

```bash
git clone <repository-url>
cd rust-shorten-url
```

### 2. Install Dependencies

```bash
cargo build
```

### 3. Start Redis

**Option A: Using Docker**
```bash
docker compose up -d
```

**Option B: Using Homebrew (macOS)**
```bash
brew install redis
brew services start redis
```

**Option C: Manual Installation**
```bash
# Download and install Redis from https://redis.io/download
redis-server
```

### 4. Configure the Application

Edit `configuration/base.yaml`:

```yaml
application:
  host: "127.0.0.1"
  base_url: "http://127.0.0.1"
  port: 8000
  domain: "localhost:8000"  # Your domain for short URLs
  api_quota: 10            # Rate limit per IP
redis_uri: "redis://127.0.0.1:6379"
```

### 5. Run the Application

```bash
cargo run
```

The server will start on `http://localhost:8000`

## 📚 API Documentation

### Shorten URL

**Endpoint**: `POST /shorten`

**Request Body**:
```json
{
  "url": "https://example.com",
  "custom_short_url": "my-custom-link",  // Optional
  "expire_at": 24                        // Optional, hours (default: 24)
}
```

**Response**:
```json
{
  "url": "https://example.com",
  "custom_short_url": "localhost:8000/abc123",
  "expiry": 24,
  "x_rate_remaining": 9,
  "x_rate_limit_reset": 30
}
```

**Example**:
```bash
curl -X POST http://localhost:8000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com", "expire_at": 48}'
```

### Resolve URL

**Endpoint**: `GET /resolve/{short_url}`

**Response**: HTTP 301 redirect to the original URL

**Example**:
```bash
curl -v http://localhost:8000/resolve/abc123
```

### Health Check

**Endpoint**: `GET /hello`

**Response**: Basic health check with Redis connectivity test

## 🔧 Configuration

The application uses YAML configuration files located in the `configuration/` directory.

### Configuration Options

| Field | Description | Default |
|-------|-------------|---------|
| `application.host` | Server host | `127.0.0.1` |
| `application.port` | Server port | `8000` |
| `application.domain` | Domain for short URLs | `localhost:8000` |
| `application.api_quota` | Rate limit per IP | `10` |
| `redis_uri` | Redis connection string | `redis://127.0.0.1:6379` |

## 🏗️ Project Structure

```
rust-shorten-url/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library exports
│   ├── configuration.rs    # Configuration management
│   ├── startup.rs          # Server startup logic
│   ├── helpers.rs          # Utility functions (Base62, URL validation)
│   └── routes/
│       ├── mod.rs          # Route module exports
│       ├── shorten/
│       │   ├── mod.rs      # Shorten module
│       │   └── post.rs     # POST /shorten handler
│       └── resolve/
│           ├── mod.rs      # Resolve module
│           └── get.rs      # GET /resolve/{url} handler
├── configuration/
│   └── base.yaml           # Application configuration
├── docker-compose.yml      # Redis Docker setup
└── Cargo.toml             # Dependencies and metadata
```

## 🔒 Rate Limiting

The application implements IP-based rate limiting:

- Each IP address gets a quota of requests (configurable)
- Quota resets every 30 minutes
- Rate limit headers are included in responses
- Exceeded limits return HTTP 503 Service Unavailable

## 🎯 URL Generation

- **Random URLs**: Uses Base62 encoding with random 64-bit integers
- **Custom URLs**: Users can specify custom short URL aliases
- **Collision Detection**: Checks for existing custom URLs before creation
- **Alphabet**: `abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789`

## 📊 Analytics

- **Click Counter**: Tracks total clicks across all shortened URLs
- **Redis Storage**: All analytics data stored in Redis
- **Real-time**: Counter updates happen on each URL resolution

## 🧪 Testing

### Manual Testing

1. **Create a short URL**:
   ```bash
   curl -X POST http://localhost:8000/shorten \
     -H "Content-Type: application/json" \
     -d '{"url": "https://github.com", "expire_at": 24}'
   ```

2. **Resolve the short URL**:
   ```bash
   curl -v http://localhost:8000/resolve/{short_url}
   ```

3. **Test error cases**:
   ```bash
   # Non-existent URL
   curl http://localhost:8000/resolve/nonexistent
   
   # Invalid URL
   curl -X POST http://localhost:8000/shorten \
     -H "Content-Type: application/json" \
     -d '{"url": "invalid-url"}'
   ```

## 🚀 Deployment

### Production Considerations

1. **Environment Variables**: Use environment variables for sensitive configuration
2. **Redis Persistence**: Enable Redis persistence for data durability
3. **Load Balancing**: Deploy behind a load balancer for high availability
4. **Monitoring**: Add logging and monitoring for production use
5. **HTTPS**: Use HTTPS in production for security

### Docker Deployment

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/rust-shorten-url /usr/local/bin/
COPY --from=builder /app/configuration /app/configuration
EXPOSE 8000
CMD ["rust-shorten-url"]
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Actix Web](https://actix.rs/)
- Redis for caching and session storage
- Inspired by modern URL shortener services

## 📞 Support

For support, please open an issue in the GitHub repository or contact the maintainers.

---

**Happy URL Shortening! 🎉**