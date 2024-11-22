use axum::{
    routing::{delete, get, post, put},
    Router,
};
use diesel::r2d2::Pool;
use diesel::PgConnection;
use std::sync::Arc;

use crate::handlers::todos_handlers;

pub fn todos_routes(
    db_connection: Arc<Pool<diesel::r2d2::ConnectionManager<PgConnection>>>,
) -> Router {
    Router::new()
        .route("/todos", post(todos_handlers::create_todo))
        .route("/todos", get(todos_handlers::get_todos))
        .route("/todos/:id", get(todos_handlers::get_todo))
        .route("/todos/:id", put(todos_handlers::update_todo))
        .route("/todos/:id", delete(todos_handlers::delete_todo))
        .with_state(db_connection)
}
