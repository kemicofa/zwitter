use actix_web::{ App, HttpServer, web };

use crate::{ bootstrap::{ Dependencies }, routes::create_user_handler };

pub async fn build_server(dependencies: Dependencies) -> std::io::Result<()> {
    // Data is a wrapper around Arc, so we can clone it.
    let data = web::Data::new(dependencies);
    HttpServer::new(move || { App::new().app_data(data.clone()).service(create_user_handler) })
        .bind(("127.0.0.1", 8080))?
        .run().await
}
