// Clean Architecture layers
pub mod domain;     // Entities, enums, errors (pure domain, no infrastructure)
pub mod ports;      // Interface contracts (traits without sqlx)
pub mod adapters;   // Infrastructure implementations (sqlx adapters)

// Application layer — services and usecases depend on ports (traits)
pub mod services;
pub mod usecases;

// Data layer — concrete repository implementations (implement port traits)
pub mod repositories;

// Models — domain entities with sqlx FromRow + serde derives
pub mod models;

// Infrastructure — database pool and migrations
pub mod database;

// Utilities
pub mod utils;
