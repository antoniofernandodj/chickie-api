# Chickie API - Complete Architecture

## Docker Compose Services

```
┌──────────────────────────────────────────────────────────────────────┐
│                         Docker Network                                │
│                                                                      │
│  ┌────────────┐         ┌─────────────────┐                          │
│  │ PostgreSQL │────────>│     API         │                          │
│  │   :5432    │         │   :3000         │                          │
│  └────────────┘         └────────┬────────┘                          │
│        ▲                         │                                    │
│        │                         ▼                                    │
│  ┌────────────┐         ┌─────────────────┐                          │
│  │  RabbitMQ  │<───────>│    Worker       │                          │
│  │ :5672/:15672│        │  (background)   │                          │
│  └────────────┘         └─────────────────┘                          │
│        ▲                                                            │
│        │                         ┌─────────────────┐                │
│        └────────────────────────>│   Scheduler     │                │
│                                  │    :8080        │                │
│                                  └─────────────────┘                │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

## Service Dependencies

| Service | Depends On | Health Check |
|---------|-----------|--------------|
| **API** | PostgreSQL, RabbitMQ | ✅ Waits for both |
| **Worker** | PostgreSQL, RabbitMQ | ✅ Waits for both |
| **Scheduler** | PostgreSQL, RabbitMQ | ✅ Waits for both |
| **PostgreSQL** | None | ✅ pg_isready |
| **RabbitMQ** | None | ✅ rabbitmq-diagnostics |

## Port Mapping

| Service | Host Port | Container Port | Protocol | Purpose |
|---------|-----------|---------------|----------|---------|
| **API** | 3000 | 3000 | HTTP | REST API |
| **Scheduler** | 8080 | 8080 | HTTP | Task scheduler |
| **PostgreSQL** | 5432 | 5432 | TCP | Database |
| **RabbitMQ** | 5672 | 5672 | AMQP | Message broker |
| **RabbitMQ UI** | 15672 | 15672 | HTTP | Management interface |

## Volume Bindings

### Development

| Service | Host Path | Container Path | Mode | Purpose |
|---------|-----------|---------------|------|---------|
| **API** | `./migrations` | `/app/migrations` | RW | Hot-reload schema changes |
| **Worker** | `.` | `/app/src` | RO | Source code access |
| **Scheduler** | `./scheduler.toml` | `/app/scheduler.toml` | RO | Config hot-reload |
| **PostgreSQL** | `postgres_data` | `/var/lib/postgresql/data` | RW | Persistent data |
| **RabbitMQ** | `rabbitmq_data` | `/var/lib/rabbitmq` | RW | Persistent queues |

### Production

| Service | Volume | Purpose |
|---------|--------|---------|
| **PostgreSQL** | `postgres_data` | Persistent database |
| **RabbitMQ** | `rabbitmq_data` | Persistent queues |
| **API/Worker/Scheduler** | None | Ephemeral containers |

## Environment Variables

### Shared (via `database.secrets.env`)

```env
# PostgreSQL
POSTGRES_USER=myuser
POSTGRES_PASSWORD=mypassword
POSTGRES_DB=mydatabase
POSTGRES_HOST=dokploy-postgres
POSTGRES_PORT=5432

# RabbitMQ
RABBITMQ_DEFAULT_USER=chickie
RABBITMQ_DEFAULT_PASS=chickie_password
RABBITMQ_DEFAULT_VHOST=/
RABBITMQ_URL=amqp://chickie:chickie_password@rabbitmq:5672
```

### Service-Specific

| Service | Variable | Value | Purpose |
|---------|----------|-------|---------|
| **API** | `APP_PORT` | `3000` | Server port |
| **API** | `RUST_LOG` | `debug` | Log level |
| **API** | `JWT_SECRET` | `secret` | JWT signing key |
| **API** | `MODE` | `development` | App mode |
| **Scheduler** | `SCHEDULER_PORT` | `8080` | Scheduler HTTP port |
| **Worker** | `RUST_LOG` | `debug` | Log level |

## Message Flow

### Example: Order Created

```
1. Client → API: POST /api/pedidos/criar
2. API → PostgreSQL: INSERT INTO pedidos
3. API → RabbitMQ: Publish "order.created" event
4. API → Client: 201 Created { uuid: "..." }

5. Worker ← RabbitMQ: Consume "order.created"
6. Worker → External Service: Send notification email
7. Worker → RabbitMQ: Publish "notification.sent"
8. Worker → PostgreSQL: UPDATE order_status
```

### Example: Scheduled Task

```
1. Scheduler → Cron: Every day at 9 AM
2. Scheduler → RabbitMQ: Publish "daily.report"
3. Worker ← RabbitMQ: Consume "daily.report"
4. Worker → PostgreSQL: Generate daily report
5. Worker → External Service: Send report via email
```

## Startup Order

```
1. PostgreSQL starts → health check passes ✅
2. RabbitMQ starts → health check passes ✅
3. API starts (waits for both)
4. Worker starts (waits for both)
5. Scheduler starts (waits for both)
```

## Commands

### Start All Services

```bash
docker compose up -d
```

### View Logs

```bash
# All services
docker compose logs -f

# Specific service
docker compose logs -f api
docker compose logs -f worker
docker compose logs -f scheduler
docker compose logs -f rabbitmq
docker compose logs -f postgres
```

### Access RabbitMQ Management UI

```
URL: http://localhost:15672
Username: chickie
Password: chickie_password
```

### Run Migrations

```bash
docker compose exec api sqlx migrate run
```

### Stop All Services

```bash
docker compose down
```

### Stop and Remove Volumes (⚠️ Deletes all data)

```bash
docker compose down -v
```

## Production Deployment

```bash
docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

### Production Differences

| Feature | Development | Production |
|---------|-------------|------------|
| **Env File** | `database.secrets.env` | Environment variables only |
| **Log Level** | `debug` | `info` |
| **Volume Mounts** | Source code, configs | None (except data volumes) |
| **RabbitMQ UI** | Exposed (`:15672`) | Not exposed |
| **Mode** | `development` | `production` |

## Monitoring

### Check Service Health

```bash
# PostgreSQL
docker compose exec postgres pg_isready -U myuser -d mydatabase

# RabbitMQ
docker compose exec rabbitmq rabbitmq-diagnostics check_running

# API
curl http://localhost:3000/

# Scheduler
curl http://localhost:8080/health
```

### View Resource Usage

```bash
docker stats
```

### Queue Statistics

```bash
docker compose exec rabbitmq rabbitmqctl list_queues name messages consumers
```

## Security Notes

✅ **Development**: Credentials in `database.secrets.env` (gitignored)  
✅ **Production**: Use environment variables or secret manager  
✅ **Never commit** `.env` files to version control  
✅ **RabbitMQ UI** not exposed in production  
## Clean Architecture

### Layer Structure

```
Domain (errors, enums)
  ↑
Ports (23 traits — no sqlx)
  ↑
Repositories (20 impls — sqlx queries, implement ports)
  ↑
Services (15 — business rules, depend on ports)
  ↑
Usecases (9 — orchestrators for API)
  ↑
API Handlers (Axum — extract request, call usecase, return response)
```

### Key Rules

1. **Handlers never contain business logic** — they delegate to usecases
2. **Services depend on port traits** (`Arc<dyn XPort>`), not concrete repositories
3. **Ports never mention sqlx** — they are pure interface contracts
4. **Repositories implement ports** — `impl XPort for YRepository`
5. **DomainError is the domain error type** — `AppError` maps it to HTTP status codes

### Dependency Injection

All wiring happens in `AppState::new()` (api/src/handlers/state.rs):

```rust
// Concrete repo → trait object
let usuario_repo = Arc::clone(&usuario_repo) as Arc<dyn UsuarioRepositoryPort>;

// Service receives trait object
let usuario_service = Arc::new(
    UsuarioService::new(usuario_repo)
);
```

### Adding New Entities

See [`CLEAN_ARCHITECTURE_GUIDE.md`](./CLEAN_ARCHITECTURE_GUIDE.md) for a complete step-by-step tutorial using `Pagamento` as example.  
