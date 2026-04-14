use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    body::{Bytes, Body},
    extract::{FromRequest, Request},
};
use prost::Message;

pub struct Protobuf<T>(pub T);

impl<T> IntoResponse for Protobuf<T>
where
    T: Message,
{
    fn into_response(self) -> Response {
        let mut buf = Vec::new();
        if let Err(err) = self.0.encode(&mut buf) {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to encode protobuf: {}", err),
            )
                .into_response();
        }

        (
            [(header::CONTENT_TYPE, "application/x-protobuf")],
            Bytes::from(buf),
        )
            .into_response()
    }
}

impl<S, T> FromRequest<S> for Protobuf<T>
where
    T: Message + Default,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
        let message = T::decode(bytes)
            .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
        Ok(Protobuf(message))
    }
}
