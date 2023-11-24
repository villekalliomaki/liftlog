mod access_token;
mod ping;
mod user;

use crate::{
    api::{response::*, routes},
    models,
};
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
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
            access_token::create_access_token,
            access_token::delete_token,
            user::create_user,
            user::get_self,
            user::delete_user,
            user::change_username,
            user::change_password,
        ),
        modifiers(&SecurityAddon),
        tags(
            (name = "Liftlog", description = "Web application to record exercise sets and repetitions.")
        ),
        components(schemas(
            RouteSuccessString,
            SingleRouteError,
            RouteError,
            models::access_token::AccessToken,
            models::user::User,
            routes::user::CreateUserInput,
            routes::user::ChangeUsernameInput,
            routes::user::ChangePasswordInput,
            routes::access_token::CreateAccessTokenInput
        ))
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "access_token",
                    SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
                )
            }
        }
    }

    let user_router = Router::new()
        .route(
            "/",
            get(user::get_self)
                .post(user::create_user)
                .delete(user::delete_user),
        )
        .route("/username", patch(user::change_username))
        .route("/password", patch(user::change_password));

    let access_token_router = Router::new()
        .route("/", post(access_token::create_access_token))
        .route("/:token", delete(access_token::delete_token));

    let api_router = Router::new()
        .route("/ping", get(ping::handle))
        .nest("/user", user_router)
        .nest("/access_token", access_token_router);

    Router::new()
        .merge(SwaggerUi::new("/docs/swagger_ui").url("/docs/spec/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/docs/redoc", ApiDoc::openapi()))
        .merge(RapiDoc::new("/docs/spec/openapi.json").path("/docs/rapidoc"))
        .nest("/api", api_router)
        .with_state(pool)
}
