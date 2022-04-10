use tide::{Response, StatusCode};
use tide::prelude::*;

pub async fn session(req: tide::Request<()>) -> tide::Result  {
    let user_id: String = req.session().get("user_id").unwrap_or("".to_string());
    Ok(Response::builder(StatusCode::Ok)
    .content_type("application/json")
    .body(json!({"user_id": user_id}))
    .build()
    )
}

pub async fn login(mut req: tide::Request<()>) -> tide::Result  {

    // doesn't check if user is already logged in
    let Login {username, password} = req.body_json().await?;

    // assumes here that password is good as long as it exists and not empty
    if password.is_empty() {
        return {
            Ok(Response::builder(StatusCode::Unauthorized)
            .content_type("application/json")
            .body(json!({"result": "unauthorized"}))
            .build()
            )
        }
    }

    // password good then set user_id in session
    req.session_mut().insert("user_id", username)?;
    
    Ok(Response::builder(StatusCode::Ok)
    .content_type("application/json")
    .body(json!({"result": "ok"}))
    .build()
    )
}

pub async fn logout(mut req: tide::Request<()>) -> tide::Result  {
    // destroy session
    req.session_mut().destroy();
    Ok(Response::builder(StatusCode::Ok)
    .content_type("application/json")
    .body(json!({"result": "ok"}))
    .build()
    )
}

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}