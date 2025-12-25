use std::env;

use dotenvy::dotenv;
use server::{ bootstrap::build_dependencies, server::build_server };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Env variables
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    // Database stuff

    // Building Dependencies
    let dependencies = build_dependencies(());

    build_server(dependencies).await
}
