use tide::Response;
use tide::prelude::*;

pub async fn get_session(req: tide::Request<()>) -> tide::Result  {
    let user_id: String = req.session().get("user_id").unwrap_or("".to_string());
    dbg!(&req.session());
    Ok(Response::builder(200)
    .content_type("application/json")
    .body(json!({"user_id": user_id}))
    .build()
    )
}

pub async fn login(mut req: tide::Request<()>) -> tide::Result  {

    // doesn't check if user is already logged in

    let Login {username, password: _} = req.body_json().await?;

    // password ignored - assumes here that password is good (ie doesn't check)

    req.session_mut().insert("user_id", username)?;
    Ok(Response::builder(200)
    .content_type("application/json")
    .body(json!({"result": "ok"}))
    .build()
    )
}

pub async fn logout(mut req: tide::Request<()>) -> tide::Result  {

    // destroy session
    req.session_mut().destroy();
    Ok(Response::builder(200)
    .content_type("application/json")
    .body(json!({"result": "ok"}))
    .build()
    )
}

pub async fn secure(req: tide::Request<()>) -> tide::Result  {

    // check if session and user exist
    let user_id: String = req.session().get("user_id").unwrap_or("".to_string());
    dbg!(&req.session());

    if !user_id.is_empty() {
        Ok(Response::builder(200)
        .content_type("application/json")
        .body(json!(req.session()))
        .build()
        )
    } else {
        Ok(Response::builder(401)
        .content_type("application/json")
        .body(json!({"result": "unauthorized"}))
        .build()
        )
    }

}

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}