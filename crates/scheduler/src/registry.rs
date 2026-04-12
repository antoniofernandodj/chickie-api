use std::{collections::HashMap, sync::Arc};
use crate::jobs::{
    CronJob,
    backup::BackupJob,
    cleanup::CleanupJob,
    health_check::HealthCheckJob,
    soft_delete_cleanup::SoftDeleteCleanupJob,
};

pub fn create_job_registry() -> HashMap<&'static str, Arc<dyn CronJob>> {
    let mut registry: HashMap<&'static str, Arc<dyn CronJob>> = HashMap::new();
    registry.insert("backup_job", Arc::new(BackupJob));
    registry.insert("cleanup_job", Arc::new(CleanupJob));
    registry.insert("health_check_job", Arc::new(HealthCheckJob));
    registry.insert("soft_delete_cleanup_job", Arc::new(SoftDeleteCleanupJob));
    registry
}