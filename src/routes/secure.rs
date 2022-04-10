use tide::Response;
use tide::prelude::*;

pub async fn secure(req: tide::Request<()>) -> tide::Result  {
    Ok(Response::builder(200)
    .content_type("application/json")
    .body(json!(req.session()))
    .build()
    )
}