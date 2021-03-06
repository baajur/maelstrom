use super::handlers;
use crate::{db::Store, server::middleware::auth_checker::AuthChecker};
use actix_web::web::ServiceConfig;
use actix_web::web::{get, post, resource, scope};

/// Configures the routes/services for Server
pub fn config<T: Store + 'static>(cfg: &mut ServiceConfig) {
    cfg.route(
        "/.well-known/matrix/client",
        get().to(handlers::admin::get_wellknown),
    )
    .route(
        "/_matrix/client/versions",
        get().to(handlers::admin::get_versions),
    )
    .service(
        scope("/_matrix/client/r0")
            .service(
                resource("/register").route(post().to(handlers::registration::post_register::<T>)),
            )
            .service(
                resource("/register/available")
                    .route(get().to(handlers::registration::get_available::<T>)),
            )
            .service(
                scope("/logout")
                    .service(
                        resource("/all")
                            .route(post().to(handlers::auth::logout_all::<T>))
                            .wrap(AuthChecker::<T>::new()),
                    )
                    .service(
                        resource("")
                            .route(post().to(handlers::auth::logout::<T>))
                            .wrap(AuthChecker::<T>::new()),
                    ),
            )
            .service(
                resource("/login")
                    .route(get().to(handlers::auth::login_info))
                    .route(post().to(handlers::auth::login::<T>)),
            )
            .service(
                resource("/account/whoami")
                    .route(get().to(handlers::account::whoami))
                    .wrap(AuthChecker::<T>::new()),
            )
            .service(
                scope("/profile")
                    .service(
                        resource("/{userId}/displayname")
                            .route(get().to(handlers::profile::get_displayname::<T>))
                    )
            ),
    );
}
