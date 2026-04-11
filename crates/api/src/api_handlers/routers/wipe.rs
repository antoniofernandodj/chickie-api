use axum::routing::{delete, MethodRouter};
use std::sync::Arc;

use crate::api_handlers::{AppState, wipe_database};

pub fn wipe_route(_s: &Arc<AppState>) -> MethodRouter<Arc<AppState>> {
    delete(wipe_database)
}
