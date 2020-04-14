use crate::{db::Store, models::registration};
use actix_web::{
    web::{Data, Json, Query},
    Error, HttpResponse,
};
use serde_json::json;

/// Checks to see if a username is available, and valid, for the server.
///
/// The server should check to ensure that, at the time of the request, the username
/// requested is available for use. This includes verifying that an application service
/// has not claimed the username and that the username fits the server's desired
/// requirements (for example, a server could dictate that it does not permit usernames
/// with underscores).
///
/// Matrix clients may wish to use this API prior to attempting registration, however
/// the clients must also be aware that using this API does not normally reserve the username.
/// This can mean that the username becomes unavailable between checking its availability
/// and attempting to register it.
pub async fn get_available<T: Store>(
    params: Query<registration::AvailableParams>,
    storage: Data<T>,
) -> Result<HttpResponse, Error> {
    // TODO: !!!Validate Username:
    // M_INVALID_USERNAME : The desired username is not a valid user name.
    // M_EXCLUSIVE : The desired username is in the exclusive namespace claimed by an application service.

    let res = storage.check_username_exists(&params.username).await;

    match res {
        Ok(exists) if !exists => Ok(HttpResponse::Ok().json(json!({"avaiable": true}))),
        //TODO: Should Use Matrix errors, but likely they should be moved to top level mod
        Ok(_unavailable) => Ok(HttpResponse::BadRequest().json(
            json!({"errorcode":"M_USER_IN_USE", "error": "Desired user ID is already taken."}),
        )),
        _ => Ok(HttpResponse::InternalServerError().json("")),
    }
}

/// This API endpoint uses the User-Interactive Authentication API_, except in the
/// cases where a guest account is being registered.
///
/// Register for an account on this homeserver.
///
/// There are two kinds of user account:
///
///     user accounts. These accounts may use the full API described in this
/// specification.
///
///     guest accounts. These accounts may have limited permissions and may not be
/// supported by all servers.
///
/// If registration is successful, this endpoint will issue an access token the client
/// can use to authorize itself in subsequent requests.
///
/// If the client does not supply a device_id, the server must auto-generate one.
///
/// The server SHOULD register an account with a User ID based on the username provided,
/// if any. Note that the grammar of Matrix User ID localparts is restricted, so the
/// server MUST either map the provided username onto a user_id in a logical manner, or
/// reject username\s which do not comply to the grammar, with M_INVALID_USERNAME.
///
/// Matrix clients MUST NOT assume that localpart of the registered user_id matches the
/// provided username.
///
/// The returned access token must be associated with the device_id supplied by the client
///  or generated by the server. The server may invalidate any access token previously
/// associated with that device. See Relationship between access tokens and devices_.
///
/// When registering a guest account, all parameters in the request body with the exception
/// of initial_device_display_name MUST BE ignored by the server. The server MUST pick a
/// device_id for the account regardless of input.
///
/// Any user ID returned by this API must conform to the grammar given in the Matrix specification_.
pub async fn post_register<T: Store>(
    params: Query<registration::RequestParams>,
    mut req: Json<registration::Request>,
    storage: Data<T>,
) -> Result<HttpResponse, Error> {
    req.kind = params.kind.clone();
    println!("{}", storage.get_type());

    unimplemented!()
}
