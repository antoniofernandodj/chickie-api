// Clean Architecture layers
pub mod domain;     // Entities, enums, errors (pure domain, no infrastructure)
pub mod ports;      // Interface contracts (traits without sqlx)
pub mod adapters;   // Infrastructure implementations (sqlx, aws, etc.)

// Legacy modules — being migrated to the new structure
pub mod models;
pub mod repositories;
pub mod services;
pub mod usecases;
pub mod utils;
