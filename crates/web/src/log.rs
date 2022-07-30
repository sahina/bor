use axum::http::{Method, Uri};
use chrono::Utc;
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::ctx::Ctx;
use crate::error::ClientError;
use crate::{Error, Result};

#[skip_serializing_none]
#[derive(Debug, Serialize)]
struct RequestLogLine {
    uuid: String,
    timestamp: String,

    user_id: Option<u64>,

    // - http request attributes
    req_path: String,
    req_method: String,

    // -- error attributes
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}

pub async fn log_request(
    uuid: Uuid,
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    service_error: Option<&Error>,
    client_error: Option<ClientError>,
) -> Result<()> {
    let timestamp = Utc::now().timestamp_millis();

    let error_type = service_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    // Create the RequestLogLine
    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),

        req_path: uri.to_string(),
        req_method: req_method.to_string(),

        user_id: ctx.map(|c| c.user_id()),

        client_error_type: client_error.map(|e| e.as_ref().to_string()),

        error_type,
        error_data,
    };

    println!("   ->> log_request: \n{}", json!(log_line));

    // TODO - Send to cloud-watch.

    Ok(())
}
