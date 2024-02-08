// Libs
use axum::{extract::Path, http::StatusCode, Json};
use axum_auth::AuthBasic;
use surrealdb::sql::Id;
use tracing::{error, info, warn};

use super::response_body::ResponseBody;
use crate::models::user_model::UserRequest;
use crate::models::{model_trait::ModelTrait, user_model::User};
use crate::security::is_valid_field;

// Types
type Response = (StatusCode, Json<ResponseBody>);

// Functions
/**
 * POST /user
 * A method to create a new user.
*/
pub async fn post_user(Json(new_user): Json<UserRequest>) -> Response {
    // Check if the username is valid.
    if !is_valid_field(&new_user.username, 20) {
        return (
            StatusCode::BAD_REQUEST,
            ResponseBody::error("The username is invalid. Check the parameters and try again."),
        );
    }

    // Check if the username already exists.
    if let Ok(Some(_)) = User::from_username(&new_user.username).await {
        warn!("Username already exists.");
        return (
            StatusCode::BAD_REQUEST,
            ResponseBody::error("The username already exists. Try another one."),
        );
    }

    let mut new_user = User::from(new_user);
    // Try to synchronize the given user in the database.
    match new_user.sync().await {
        Err(e) => {
            warn!("Couldn\'t create the user. {}", e);
            (
                StatusCode::BAD_REQUEST,
                ResponseBody::error(
                    "Couldn\'t create the user. Check the parameters and try again.",
                ),
            )
        }
        Ok(_) => (
            StatusCode::CREATED,
            ResponseBody::success(new_user.to_user_response()),
        ),
    }
}

/**
 * PATCH /user/
 * Authorization: Basic
 * A method to update an user.
*/
pub async fn patch_user(
    AuthBasic(user_auth): AuthBasic,
    Json(new_user_content): Json<UserRequest>,
) -> Response {
    // Check if the authorization is valid.
    let mut logged_user = match login_user(user_auth, false).await {
        Err(res) => return res,
        Ok(user) => user,
    };

    // Define the content that the response doesn't have/can't modify.
    let new_user_content = User::from(new_user_content);
    logged_user.merge(new_user_content);

    // Try to synchronize the user in the database.
    match logged_user.sync().await {
        Err(e) => {
            warn!("Couldn\'t update the user. {}", e);
            (
                StatusCode::BAD_REQUEST,
                ResponseBody::error(
                    "Couldn\'t create the user. Check the parameters and try again.",
                ),
            )
        }
        Ok(_) => (
            StatusCode::OK,
            ResponseBody::success(logged_user.to_user_response()),
        ),
    }
}

/**
 * DELETE /user/
 * Authorization: Basic
 * A method to delete an user.
*/
pub async fn delete_user(AuthBasic(user_auth): AuthBasic) -> Response {
    // Check if the authorization is valid.
    let provided_user = match login_user(user_auth, false).await {
        Err(res) => return res,
        Ok(user) => user,
    };

    // Try to delete the user.
    if let Err(e) = provided_user.delete().await {
        error!("Couldn\'t delete the user. {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ResponseBody::error("Couldn\'t get the user. Please call the admin."),
        );
    }

    (StatusCode::OK, ResponseBody::success_no_data())
}

/**
 * GET /user/{user_id}
 * A method to get an user.
*/
pub async fn get_user(Path(user_id): Path<String>) -> Response {
    // Try to get the user.
    match get_user_from_id(Id::from(user_id)).await {
        Err(response) => response,
        Ok(user) => (
            StatusCode::OK,
            ResponseBody::success(user.to_user_response()),
        ),
    }
}

/**
 * POST /user/login
 * Authorization: Basic
 * A method to login an user. Uses its username and password. It's needed to get the user id.
*/
pub async fn post_user_login(AuthBasic(user_auth): AuthBasic) -> Response {
    // Try to login the user.
    match login_user(user_auth, true).await {
        Err(response) => response,
        Ok(user) => (
            StatusCode::OK,
            ResponseBody::success(user.to_user_response()),
        ),
    }
}

// Utils
/**
 * A method to login an user. Uses its id and password.
*/
pub async fn login_user(
    user_auth: (String, Option<String>),
    is_username: bool,
) -> Result<User, Response> {
    info!("Trying to login the user...");

    // Define the default error_message.
    let response_error = (
        StatusCode::UNAUTHORIZED,
        ResponseBody::error("Check the credentials and try again."),
    );

    // Check the authorization.
    let (user_identifier, password) = match user_auth {
        (user_identifier, Some(pass)) => (user_identifier, pass),
        (_, None) => {
            info!("Password not included.");
            return Err(response_error);
        }
    };

    // Get the user from the database and check the password.
    let user_db = if is_username {
        get_user_from_username(&user_identifier).await?
    } else {
        get_user_from_id(Id::from(user_identifier)).await?
    };
    match user_db.is_login_valid(password) {
        false => Err(response_error),
        true => {
            info!("User successfully logged in.");
            Ok(user_db)
        }
    }
}

/**
 * A method to get some user in the database using his id.
*/
pub async fn get_user_from_id(user_id: Id) -> Result<User, Response> {
    // Try to get the user using his id.
    match User::from_id(user_id).await {
        Err(e) => {
            error!("Couldn\'t get the user. {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseBody::error("Couldn\'t get the user. Please call the admin."),
            ))
        }
        Ok(None) => Err((
            StatusCode::UNAUTHORIZED,
            ResponseBody::error("User not found. Check the id and try again."),
        )),
        Ok(Some(user)) => Ok(user),
    }
}

/**
 * A method to get some user in the database using his username.
*/
pub async fn get_user_from_username(username: &str) -> Result<User, Response> {
    // Check if the username is valid.
    if !is_valid_field(username, 20) {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseBody::error("The username is invalid. Check the parameters and try again."),
        ));
    }

    // try to get the user by his username.
    match User::from_username(username).await {
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            ResponseBody::error("Couldn\'t get the user. Please call the admin."),
        )),
        Ok(None) => Err((
            StatusCode::UNAUTHORIZED,
            ResponseBody::error("User not found. Check the username and try again."),
        )),
        Ok(Some(user)) => Ok(user),
    }
}
