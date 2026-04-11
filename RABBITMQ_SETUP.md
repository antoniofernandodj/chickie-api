# RabbitMQ Setup - Chickie API

## Overview

RabbitMQ is now integrated into the Chickie ecosystem for asynchronous message handling between services.

## Architecture

```
┌─────────────┐      ┌──────────────┐      ┌─────────────┐
│   API       │─────>│  RabbitMQ    │─────>│   Worker    │
│ (Publisher) │      │  (Broker)    │      │ (Consumer)  │
└─────────────┘      └──────────────┘      └─────────────┘
                           │
                     ┌─────────────┐
                     │ Scheduler   │
                     │ (Publisher) │
                     └─────────────┘
```

## Ports

| Service | Port | Purpose |
|---------|------|---------|
| **AMQP** | `5672` | Message protocol (used by application) |
| **Management UI** | `15672` | Web interface for monitoring |

## Quick Start

### Start All Services

```bash
docker compose up -d
```

### Access RabbitMQ Management UI

1. Open browser: `http://localhost:15672`
2. Login credentials:
   - **Username:** `chickie`
   - **Password:** `chickie_password`

### Check RabbitMQ Status

```bash
# Check if RabbitMQ is running
docker compose exec rabbitmq rabbitmq-diagnostics check_running

# View queue statistics
docker compose exec rabbitmq rabbitmqctl list_queues name messages consumers

# View exchange statistics
docker compose exec rabbitmq rabbitmqctl list_exchanges name type durable
```

## Environment Variables

### Local Development (`database.secrets.env`)

```env
RABBITMQ_DEFAULT_USER=chickie
RABBITMQ_DEFAULT_PASS=chickie_password
RABBITMQ_DEFAULT_VHOST=/
RABBITMQ_URL=amqp://chickie:chickie_password@rabbitmq:5672
```

### Production (`.env`)

```env
RABBITMQ_USER=chickie_production
RABBITMQ_PASSWORD=your_secure_password
RABBITMQ_VHOST=/
RABBITMQ_URL=amqp://chickie_production:your_secure_password@rabbitmq:5672
```

## Configuration in Services

### API Service
```yaml
environment:
  RABBITMQ_URL: amqp://chickie:chickie_password@rabbitmq:5672
```

### Worker Service
```yaml
environment:
  RABBITMQ_URL: amqp://chickie:chickie_password@rabbitmq:5672
```

### Scheduler Service
```yaml
environment:
  RABBITMQ_URL: amqp://chickie:chickie_password@rabbitmq:5672
```

## Health Checks

All services wait for RabbitMQ to be healthy before starting:

```yaml
depends_on:
  rabbitmq:
    condition: service_healthy
```

## Useful Commands

### View Logs

```bash
# RabbitMQ logs
docker compose logs -f rabbitmq

# Worker logs (consumer)
docker compose logs -f worker

# API logs (publisher)
docker compose logs -f api
```

### Restart Services

```bash
# Restart only RabbitMQ
docker compose restart rabbitmq

# Restart all services
docker compose restart
```

### Clear RabbitMQ Data

```bash
# Stop services
docker compose down

# Remove RabbitMQ volume (clears all queues/exchanges)
docker volume rm chickie_rabbitmq_data

# Restart
docker compose up -d
```

## Production Deployment

```bash
docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

### Production Notes

- ✅ Management UI port `15672` is **NOT** exposed
- ✅ Uses environment variables instead of env_file
- ✅ Persistent volume for message durability
- ✅ Health checks ensure proper startup order

## Integration with Rust Code

The `lapin` crate is already configured in `Cargo.toml`:

```toml
[workspace.dependencies]
lapin = "2"
```

### Example Usage (Worker)

```rust
use lapin::{
    Connection, ConnectionProperties, ExchangeKind, BasicProperties,
    options::*, types::FieldTable,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rabbitmq_url = std::env::var("RABBITMQ_URL")
        .unwrap_or_else(|_| "amqp://localhost:5672".into());
    
    let conn = Connection::connect(&rabbitmq_url, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;
    
    // Declare queue
    channel.queue_declare(
        "chickie_tasks",
        QueueDeclareOptions::default(),
        FieldTable::default(),
    ).await?;
    
    // Consume messages
    let mut consumer = channel
        .basic_consume(
            "chickie_tasks",
            "chickie_worker",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;
    
    while let Some(delivery) = consumer.next().await {
        let delivery = delivery?;
        println!("Received: {:?}", std::str::from_utf8(&delivery.data)?);
        delivery.ack(BasicAckOptions::default()).await?;
    }
    
    Ok(())
}
```

## Troubleshooting

### RabbitMQ Won't Start

```bash
# Check logs
docker compose logs rabbitmq

# Verify port availability
lsof -i :5672
lsof -i :15672
```

### Connection Refused

```bash
# Verify RabbitMQ is healthy
docker compose exec rabbitmq rabbitmq-diagnostics check_running

# Check network
docker compose exec api ping rabbitmq
```

### Messages Not Being Consumed

```bash
# Check queue consumers
docker compose exec rabbitmq rabbitmqctl list_consumers

# Check queue contents
docker compose exec rabbitmq rabbitmqctl list_queues name messages
```

## Resources

- [RabbitMQ Documentation](https://www.rabbitmq.com/documentation.html)
- [Lapin Crate](https://crates.io/crates/lapin)
- [AMQP 0-9-1 Protocol](https://www.rabbitmq.com/tutorials/amqp-concepts.html)
