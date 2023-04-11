use crate::command_extractor::CommandExtractor;
use crate::config::cqrs_framework;
use crate::domain::aggregate::BankAccount;
use crate::queries::BankAccountView;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Extension, Json, Router};
use cqrs_es::persist::ViewRepository;
use postgres_es::{default_postgress_pool, PostgresCqrs, PostgresViewRepository};
use std::sync::Arc;

pub async fn create_routes() -> Router {
    // Configure the CQRS framework, backed by a Postgres database, along with two queries:
    // - a simply-query prints events to stdout as they are published
    // - `account_query` stores the current state of the account in a ViewRepository that we can access
    //
    // The needed database tables are automatically configured with `docker-compose up -d`,
    // see init file at `/db/init.sql` for more.
    let pool = default_postgress_pool("postgresql://demo_user:demo_pass@localhost:5432/demo").await;
    let (cqrs, account_query) = cqrs_framework(pool);

    // Configure the Axum routes and services.
    // For this example a single logical endpoint is used and the HTTP method
    // distinguishes whether the call is a command or a query.
    Router::new()
        .route(
            "/account/:account_id",
            get(query_handler).post(command_handler),
        )
        .layer(Extension(cqrs))
        .layer(Extension(account_query))
}

// Serves as our query endpoint to respond with the materialized `BankAccountView`
// for the requested account.
async fn query_handler(
    Path(account_id): Path<String>,
    Extension(view_repo): Extension<Arc<PostgresViewRepository<BankAccountView, BankAccount>>>,
) -> Response {
    let view = match view_repo.load(&account_id).await {
        Ok(view) => view,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response();
        }
    };
    match view {
        None => StatusCode::NOT_FOUND.into_response(),
        Some(account_view) => (StatusCode::OK, Json(account_view)).into_response(),
    }
}

// Serves as our command endpoint to make changes in a `BankAccount` aggregate.
async fn command_handler(
    Path(account_id): Path<String>,
    Extension(cqrs): Extension<Arc<PostgresCqrs<BankAccount>>>,
    CommandExtractor(metadata, command): CommandExtractor,
) -> Response {
    match cqrs
        .execute_with_metadata(&account_id, command, metadata)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
