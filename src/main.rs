mod routes;
mod models;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, App, HttpServer};
use sqlx::{Postgres, Pool, postgres::PgPoolOptions};
use dotenv::dotenv;
use routes::{health_route::health_checker_handler, config::config};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() ->  std::io::Result<()> {
    // set enviroment variable for RUST_LOG
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();
    // set database_url variable from enviroment variable
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // create our database pool
    let pool:Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the db is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database {:?}", err);
            std::process::exit(1);
        }

    };

    println!("Server started successfully!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_origin("http://localhost:3000")
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::CONTENT_ENCODING,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(actix_web::web::Data::new(AppState {db: pool.clone()}))
            .service(health_checker_handler)
            .configure(config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
