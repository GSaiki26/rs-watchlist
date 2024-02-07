// Libs
use axum::{extract::Path, http::StatusCode, Json};
use axum_auth::AuthBasic;
use surrealdb::sql::{Datetime, Id};
use tracing::{error, info, warn};

use super::response_body::ResponseBody;
use crate::models::{model_trait::ModelTrait, user_model::User};
use crate::security::is_valid_field;

// Types
type Response = (StatusCode, Json<ResponseBody>);

// Functions
/**
 * POST /user
 * A method to create a new user.
*/
pub async fn post_user(Json(mut user): Json<User>) -> Response {
    // Check if the username is valid.
    if !is_valid_field(&user.username, 20) {
        return (
            StatusCode::BAD_REQUEST,
            ResponseBody::error("The username is invalid. Check the parameters and try again."),
        );
    }

    // Check if the username already exists.
    if let Ok(Some(_)) = User::from_username(&user.username).await {
        warn!("Username already exists.");
        return (
            StatusCode::BAD_REQUEST,
            ResponseBody::error("The username already exists. Try another one."),
        );
    }

    // Try to synchronize the given user in the database.
    match user.create().await {
        Err(e) => {
            warn!("Couldn\'t create the user. {}", e);
            (
                StatusCode::BAD_REQUEST,
                ResponseBody::error(
                    "Couldn\'t create the user. Check the parameters and try again.",
                ),
            )
        }
        Ok(_) => {
            user.clear_password();
            (StatusCode::CREATED, ResponseBody::success(user))
        }
    }
}

/**
 * PATCH /user/
 * Authorization: Basic
 * A method to update an user.
*/
pub async fn patch_user(
    AuthBasic(user_auth): AuthBasic,
    Json(mut new_user_content): Json<User>,
) -> Response {
    // Check if the authorization is valid.
    let provided_user = match login_user(user_auth, false).await {
        Err(res) => return res,
        Ok(user) => user,
    };

    // Define the content that the response doesn't have/can't modify.
    new_user_content.id = provided_user.id;
    new_user_content.created_at = provided_user.created_at;
    new_user_content.updated_at = Some(Datetime::default());

    // Try to synchronize the user in the database.
    match new_user_content.sync().await {
        Err(e) => {
            warn!("Couldn\'t update the user. {}", e);
            (
                StatusCode::BAD_REQUEST,
                ResponseBody::error(
                    "Couldn\'t create the user. Check the parameters and try again.",
                ),
            )
        }
        Ok(_) => {
            new_user_content.clear_password();
            (StatusCode::OK, ResponseBody::success(new_user_content))
        }
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
        Ok(mut user) => {
            user.clear_password();
            (StatusCode::OK, ResponseBody::success(user))
        }
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
        Ok(mut user) => {
            user.clear_password();
            (StatusCode::OK, ResponseBody::success(user))
        }
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
        true => Ok(user_db),
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
