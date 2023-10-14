mod ping;
mod user;
mod access_token;

use crate::api::response::*;
use axum::{routing::{get, post}, Router};
use sqlx::PgPool;
use tracing::{info, instrument};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

// Centralized builder for all API routes.
// Actual routes are also under this module,
// and not public the rest of the crate.
#[instrument]
pub fn build_router(pool: PgPool) -> Router {
    info!("Building axum routes and API documentation");

    #[derive(OpenApi)]
    #[openapi(
        paths(
            ping::handle,
        ),
        modifiers(&SecurityAddon),
        tags(
            (name = "Liftlog", description = "Web application to record exercise sets and repetitions.")
        ),
        components(schemas(RouteSuccessString))
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "token",
                    SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
                )
            }
        }
    }

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .route("/api/ping", get(ping::handle))
        .route("/api/user", post(user::create_user))
        .with_state(pool)
}
