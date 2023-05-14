mod models;
mod repository;

use crate::models::Entry;
use crate::repository::Error as RepoError;

use ::config::Config;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, ResponseError};
use derive_more::{Display, From};
use dotenv::dotenv;
use ormlite;
use repository::Repository;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub debug: bool,
    pub db_url: String,
}

#[derive(Display, From, Debug)]
pub enum Error {
    NotFound,
    RepoError(RepoError),
}
impl std::error::Error for Error {}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::NotFound => HttpResponse::NotFound().finish(),
            Error::RepoError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[get("/changelog")]
async fn get_changelog(repository: web::Data<Repository>) -> Result<HttpResponse, Error> {
    let changelog = repository.get_changelog().await?;

    if changelog.is_empty() {
        return Err(Error::NotFound);
    }

    Ok(HttpResponse::Ok().json(changelog))
}

#[derive(Deserialize)]
struct CreateEntry {
    pub text: String,
    pub tags: Vec<String>,
}

#[post("/changelog")]
async fn post_changelog(
    new_entry: web::Json<CreateEntry>,
    repository: web::Data<Repository>,
) -> Result<HttpResponse, Error> {
    let new_entry = new_entry.into_inner();
    let entry = repository
        .add_entry(Entry {
            tags: new_entry.tags,
            text: new_entry.text,
            ..Default::default()
        })
        .await?;

    Ok(HttpResponse::Created().json(entry))
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Alive and well.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config: AppConfig = Config::builder()
        .add_source(::config::Environment::default().separator("__"))
        .build()
        .expect("build config")
        .try_deserialize()
        .expect("load configuration");

    let conn = ormlite::postgres::PgPoolOptions::new()
        .connect(&config.db_url)
        .await
        .unwrap();

    let repository = Repository::new(conn);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(repository.clone()))
            .service(health)
            .service(
                web::scope("/v1")
                    .service(get_changelog)
                    .service(post_changelog),
            )
    })
    .shutdown_timeout(10)
    .bind(("0.0.0.0", 8080))?
    .run();

    server.await
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App};
    use testcontainers::{clients, images::postgres};

    use crate::models::Entry;

    use super::*;

    #[actix_web::test]
    async fn test_health() {
        let app = test::init_service(App::new().service(health)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    // #[actix_web::test]
    //     async fn test_changelog() {
    //         let docker = clients::Cli::default();
    //         let postgres_node = docker.run(postgres::Postgres::default());

    //         let pool = deadpool_postgres::Config {
    //             dbname: Some("postgres".into()),
    //             port: Some(postgres_node.get_host_port_ipv4(5432)),
    //             host: Some("127.0.0.1".into()),
    //             user: Some("postgres".into()),
    //             password: Some("postgres".into()),
    //             ..Default::default()
    //         }
    //         .create_pool(Some(Runtime::Tokio1), NoTls)
    //         .expect("create pg pool");

    //         let client: Client = pool.get().await.unwrap();

    //         client
    //             .batch_execute(include_str!("../sql/00_schema.sql"))
    //             .await
    //             .expect("create database schema");

    //         client
    //             .batch_execute(
    //                 r#"
    // INSERT INTO entries (id, tags, title, description) VALUES
    //   ('entry1', '{"test", "tag1"}', 'Test', '# Test Body\nAnd paragraph.'),
    //   ('entry2', '{"test", "tag2"}', 'Some other test', '# Some other test body\nAnd paragraph.')
    // ;
    // "#,
    //             )
    //             .await
    //             .expect("insert testing data");

    //         let repo = Repository::new(pool.clone());
    //         let app = test::init_service(
    //             App::new()
    //                 .app_data(web::Data::new(repo.clone()))
    //                 .service(get_changelog),
    //         )
    //         .await;

    //         let req = test::TestRequest::get().uri("/changelog").to_request();
    //         let resp = test::call_service(&app, req).await;
    //         assert!(resp.status().is_success());
    //         let resp: Vec<Entry> = test::read_body_json(resp).await;
    //         assert_eq!(
    //             resp,
    //             vec![
    //                 Entry {
    //                     id: "entry1".into(),
    //                     ..Default::default()
    //                 },
    //                 Entry {
    //                     id: "entry2".into(),
    //                     ..Default::default()
    //                 }
    //             ]
    //         );
    //     }
}
