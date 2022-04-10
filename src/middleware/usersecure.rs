use tide::{Next, Request, Response, Result, StatusCode};
use std::future::Future;
use std::pin::Pin;
use tide::prelude::*;

/// middleware function to check if user_id exists in session and therefore access to secure
pub fn user_secure<State: Clone + Send + Sync + 'static>(
    req: Request<State>,
    next: Next<'_, State>,
) -> Pin<Box<dyn Future<Output = Result> + Send + '_>> {
    Box::pin(async {

        // check if session and user exist
        let user_id: Option<String> = req.session().get("user_id");

        match user_id {
            Some(_) => Ok(next.run(req).await),
            None => Ok(Response::builder(StatusCode::Unauthorized)
                .content_type("application/json")
                .body(json!({"result": "unauthorized"}))
                .build()
                )
        }
    })
}
