use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};




// Here is where I think i will be setting up the request I would be getting from the internet
#[derive(Deserialize, Debug)]
struct Request 
{
    name: String,
}

#[derive(Serialize, Debug)]
struct Response 
{
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> 
{
    lambda_runtime::run(service_fn(faas_practice)).await?;
    Ok(())
}

// This is a function that returs nothing -> is for return type

async fn faas_practice(event: LambdaEvent<Request>) -> Result<Value, Error>
{
    let name = event.payload.name;
    let message = format!("Hello, {}!", name);
    let resp = Response { message };

    Ok(json!(resp))
}


