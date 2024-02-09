// Libs
use axum::{extract::Path, http::StatusCode, Json};
use axum_auth::AuthBasic;
use surrealdb::sql::Id;
use tracing::{error, warn};

use super::controllers_utils::*;
use super::response_body::ResponseBody;
use crate::models::user_model::UserRequest;
use crate::models::{model_trait::ModelTrait, user_model::User};
use crate::security::is_valid_field;

// Functions
/**
 * POST /user
 * BODY: UserRequest
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
            ResponseBody::error("The username already exists. Try other."),
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
 * BODY: UserRequest
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

    // Check if the provided username is different from the database.
    if new_user_content.username != logged_user.username {
        // Check if the username is valid.
        if !is_valid_field(&new_user_content.username, 20) {
            return (
                StatusCode::BAD_REQUEST,
                ResponseBody::error("The username is invalid. Check the parameters and try again."),
            );
        }

        // Check if the username already exists.
        if let Ok(Some(_)) = User::from_username(&new_user_content.username).await {
            warn!("Username already exists.");
            return (
                StatusCode::BAD_REQUEST,
                ResponseBody::error("The username already exists. Try other."),
            );
        }
    }

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
