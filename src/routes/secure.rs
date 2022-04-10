use tide::{Response, StatusCode};
use tide::prelude::*;

pub async fn secure(req: tide::Request<()>) -> tide::Result  {
    Ok(Response::builder(StatusCode::Ok)
    .content_type("application/json")
    .body(json!(req.session()))
    .build()
    )
}