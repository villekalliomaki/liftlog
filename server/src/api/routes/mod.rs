mod access_token;
mod exercise;
mod exercise_instance;
mod fallback;
mod ping;
mod session;
mod set;
mod user;

use crate::{
    api::{
        response::*,
        routes::{self, fallback::fallback404},
    },
    models,
};
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use serde::{Deserialize, Deserializer};
use sqlx::PgPool;
use tracing::{info, instrument};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

// For serde...
pub fn default_as_false() -> bool {
    false
}

// Used for nested options: https://github.com/serde-rs/serde/issues/904
// Only way to have optional fields, which differentiate between the field being missing
// and being set to null js JSON
pub fn deserialize_optional_option<'de, T, D>(
    deserializer: D,
) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}

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
            exercise::create_exercise,
            exercise::edit_exercise,
            exercise::get_exercise_by_id,
            exercise::delete_exercise_by_id,
            exercise::get_user_exercises,
            exercise::get_user_exercises_by_kind,
            session::create_session,
            session::edit_session,
            session::delete_session_by_id,
            session::get_session_by_id,
            session::finish_session,
            session::get_all_user_sessions,
            exercise_instance::create_exercise_instance,
            exercise_instance::get_exercise_instance_by_id,
            exercise_instance::delete_exercise_instance_by_id,
            exercise_instance::edit_exercise_instance,
            exercise_instance::add_exercise_instance_comment,
            exercise_instance::set_exercise_instance_comment,
            exercise_instance::delete_exercise_instance_comment,
            set::create_set,
            set::get_set_by_id,
            set::delete_set,
            set::edit_set,
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
            models::exercise::Exercise,
            models::exercise::ExerciseKind,
            models::session::Session,
            models::exercise_instance::ExerciseInstance,
            models::set::Set,
            routes::user::CreateUserInput,
            routes::user::ChangeUsernameInput,
            routes::user::ChangePasswordInput,
            routes::access_token::CreateAccessTokenInput,
            routes::exercise::CreateExerciseInput,
            routes::exercise::EditExerciseInput,
            routes::session::CreateSessionInput,
            routes::session::EditSessionInput,
            routes::exercise_instance::CreateExerciseInstanceInput,
            routes::exercise_instance::CreateExerciseInstanceCommentInput,
            routes::exercise_instance::SetExerciseInstanceCommentInput,
            routes::exercise_instance::EditExerciseInstanceInput,
            routes::set::CreateSetInput,
            routes::set::EditSetInput,
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

    let exercise_router = Router::new()
        .route("/", post(exercise::create_exercise))
        .route("/all", get(exercise::get_user_exercises))
        .route(
            "/all/:exercise_kind",
            get(exercise::get_user_exercises_by_kind),
        )
        .route("/:exercise_id", patch(exercise::edit_exercise))
        .route("/:exercise_id", get(exercise::get_exercise_by_id))
        .route("/:exercise_id", delete(exercise::delete_exercise_by_id));

    let session_router = Router::new()
        .route("/", post(session::create_session))
        .route("/", get(session::get_all_user_sessions))
        .route("/:session_id", patch(session::edit_session))
        .route("/:session_id", delete(session::delete_session_by_id))
        .route("/:session_id", get(session::get_session_by_id))
        .route("/:session_id/finish", patch(session::finish_session));

    let exercise_instance_router = Router::new()
        .route("/", post(exercise_instance::create_exercise_instance))
        .route(
            "/:exercise_instance_id",
            get(exercise_instance::get_exercise_instance_by_id),
        )
        .route(
            "/:exercise_instance_id",
            delete(exercise_instance::delete_exercise_instance_by_id),
        )
        .route(
            "/:exercise_instance_id",
            patch(exercise_instance::edit_exercise_instance),
        )
        .route(
            "/:exercise_instance_id/comment",
            post(exercise_instance::add_exercise_instance_comment),
        )
        .route(
            "/:exercise_instance_id/comment/:comment_index",
            patch(exercise_instance::set_exercise_instance_comment),
        )
        .route(
            "/:exercise_instance_id/comment/:comment_index",
            delete(exercise_instance::delete_exercise_instance_comment),
        );

    let set_router = Router::new()
        .route("/", post(set::create_set))
        .route("/:set_id", get(set::get_set_by_id))
        .route("/:set_id", delete(set::delete_set))
        .route("/:set_id", patch(set::edit_set));

    let api_router = Router::new()
        .route("/ping", get(ping::handle))
        .nest("/user", user_router)
        .nest("/access_token", access_token_router)
        .nest("/exercise", exercise_router)
        .nest("/session", session_router)
        .nest("/set", set_router)
        .nest("/exercise_instance", exercise_instance_router);

    Router::new()
        .merge(SwaggerUi::new("/docs/swagger_ui").url("/docs/spec/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/docs/redoc", ApiDoc::openapi()))
        .merge(RapiDoc::new("/docs/spec/openapi.json").path("/docs/rapidoc"))
        .nest("/api", api_router)
        .with_state(pool)
        .fallback(fallback404)
}
