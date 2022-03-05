use lambda_runtime::{handler_fn, Context, Error as LambdaError};
use serde::Deserialize;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[derive(Deserialize)]
struct CustomEvent {
    first_name: String,
    last_name: String,
}

async fn handler(event: CustomEvent, _: Context) -> Result<Value, LambdaError> {
    Ok(json!({ "message": "Record written!" }))
}
