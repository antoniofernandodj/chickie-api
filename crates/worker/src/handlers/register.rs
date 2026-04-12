use std::pin::Pin;
use std::future::Future;
use std::sync::Arc;
use anyhow::Result;

use crate::handlers;


/// Representa o registro de um handler para uma fila RabbitMQ
pub struct QueueRegistration {
    pub queue: String,
    pub routing_key: String,
    pub handler: Arc<
        dyn Fn(String) ->
            Pin<Box<dyn Future<Output = Result<()> > + Send>> +
            Send +
            Sync
    >
}

impl QueueRegistration {
    fn new<F, Fut>(
        queue: String,
        routing_key: String,
        handler: F,
    ) -> Self
    where
        F: Fn(String) ->
            Fut +
            Send +
            Sync +
            'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        Self {
            queue,
            routing_key,
            handler: Arc::new(move |body| Box::pin(handler(body))),
        }
    }
}


pub fn register_handlers() -> Vec<QueueRegistration> {
    vec![
        QueueRegistration::new(
            "emails".to_string(),
            "task.email.#".to_string(),
            |body| handlers::handle_email(body),
        ),

        QueueRegistration::new(
            "notifications".to_string(),
            "task.notification.#".to_string(),
            |body| handlers::handle_notification(body),
        ),

        QueueRegistration::new(
            "reports".to_string(),
            "task.report.#".to_string(),
            |body| handlers::handle_report(body),
        ),
    ]
}
