use std::env;

// route handlers
mod routes;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    // Configuration
    let port = env::var("SERVER_PORT")
        .ok()
        .unwrap_or_else(|| "8080".to_string());
    let host = env::var("SERVER_HOST")
        .ok()
        .unwrap_or_else(|| "127.0.0.1".to_string());
    let secret = env::var("SERVER_SECRET")
        .ok()
        .unwrap_or_else(|| "recommended that you set Secret instead of fixed value".to_string());

    let addr = format!("{}:{}", host, port);
    
    let mut app = tide::new();

    // add middleware to manage sessions
    // using simple given MemoryStore for dev
    app.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        secret.as_bytes()
    ));

    // serve Svelte App
    app.at("/").serve_dir("public/")?;
    app.at("/").serve_file("public/index.html")?;

    // configure Routes
    app.at("/auth/login").post(routes::login);
    app.at("/auth/logout").get(routes::logout);
    app.at("/auth/session").get(routes::get_session);

    // private area
    app.at("/secure").get(routes::secure);


    // start server
    app.listen(addr).await?;
    
    Ok(())
}

