use axum::Router;
pub mod testing_routes;

pub fn make_common_routes() -> Router {
    Router::new()
        .nest("/", testing_routes::make_testing_routes())
}
