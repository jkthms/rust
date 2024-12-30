use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    schema::FibonacciOptions,
    AppState,
};

pub async fn health_checker_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    const MESSAGE: &str = "Alive!";

    let json_response = serde_json::json!({
        "status": "OK",
        "message": MESSAGE
    });

    Ok(Json(json_response))
}

// Recursive Fibonacci calculation function
// Now it accepts a reference to a MutexGuard for access to the HashMap
fn fibonacci(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&result) = cache.get(&n) {
        return result;
    }

    let result = if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1, cache) + fibonacci(n - 2, cache)
    };

    cache.insert(n, result); // Store the result in the cache

    result
}

pub async fn fibonacci_handler(
    opts: Query<FibonacciOptions>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts;

    let n = opts.n;

    // Lock the cache and pass the MutexGuard to the fibonacci function
    let mut cache = data.fibonacci_cache.lock().unwrap();

    // Pass the locked HashMap (through MutexGuard) to fibonacci
    let result = fibonacci(n, &mut cache);

    let json_response = serde_json::json!({
        "status": "OK",
        "result": result
    });

    Ok(Json(json_response))
}
