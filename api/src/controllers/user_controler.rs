// Libs
use axum::{extract::Path, http::StatusCode, Json};
use axum_auth::AuthBasic;
use surrealdb::sql::{Datetime, Id};
use tracing::{error, info};

use super::response_body::ResponseBody;
use crate::{
    models::{model_trait::ModelTrait, user_model::User},
    security::get_sha512,
};

// Types
type Response<T> = (StatusCode, Json<ResponseBody<T>>);

// Functions
/**
 * POST /user
 * A method to create a new user.
*/
pub async fn create_user(Json(mut user): Json<User>) -> Response<User> {
    // Try to synchronize the given user in the database.
    match user.create().await {
        Err(e) => {
            info!("Couldn\'t create the user. {}", e);
            (
                StatusCode::BAD_REQUEST,
                ResponseBody::error(
                    "Couldn\'t create the user. Check the parameters and try again.",
                ),
            )
        }
        Ok(_) => {
            info!("{} created successfully.", user.id.clone().unwrap());
            (StatusCode::CREATED, ResponseBody::success(user))
        }
    }
}

/**
 * GET /user/:user_id
 * A method to get an user.
*/
pub async fn get_user_by_id(Path(user_id): Path<String>) -> Response<User> {
    match get_user(Id::from(user_id)).await {
        Err(response) => response,
        Ok(user) => (StatusCode::OK, ResponseBody::success(user)),
    }
}

/**
 * PATCH /user/
 * HEADER Authorization
 * A method to update an user.
*/
pub async fn update_user_by_id(
    AuthBasic(user_auth): AuthBasic,
    Json(mut new_user_content): Json<User>,
) -> Response<User> {
    // Check if the login is valid.
    let user_db = match is_login_valid(user_auth).await {
        Err(res) => return res,
        Ok(user) => user,
    };

    // Define the content that the response doesn't have/can't modify.
    new_user_content.id = user_db.id;
    new_user_content.created_at = user_db.created_at;
    new_user_content.updated_at = Some(Datetime::default());

    // Try to synchronize the user in the database.
    match new_user_content.sync().await {
        Err(e) => {
            info!("Couldn\'t update the user. {}", e);
            (
                StatusCode::BAD_REQUEST,
                ResponseBody::error(
                    "Couldn\'t create the user. Check the parameters and try again.",
                ),
            )
        }
        Ok(_) => {
            info!(
                "{} updated successfully.",
                new_user_content.id.clone().unwrap()
            );
            (StatusCode::OK, ResponseBody::success(new_user_content))
        }
    }
}

/**
 * DELETE /user/:user_id
 * A method to delete an user.
*/
pub async fn delete_user(Path(user_id): Path<String>) -> Response<User> {
    match get_user(Id::from(&user_id)).await {
        Err(response) => response,
        Ok(user) => {
            if let Err(e) = user.delete().await {
                error!("Couldn\'t delete the user. {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseBody::error("Couldn\'t get the user. Please call the admin."),
                );
            }
            info!("The {user_id} was successfully deleted.");
            (StatusCode::OK, ResponseBody::success_no_data())
        }
    }
}

/**
 * A method to check if the login is valid.
*/
async fn is_login_valid(user_auth: (String, Option<String>)) -> Result<User, Response<User>> {
    info!("Trying to login the user...");

    // Define the default error_message.
    let response_error = (
        StatusCode::UNAUTHORIZED,
        ResponseBody::error("Check the credentials and try again."),
    );

    // Check the authorization.
    let (user_id, user_pass) = match user_auth {
        (id, Some(pass)) => (id, pass),
        (_, None) => {
            info!("Password not included.");
            return Err(response_error);
        }
    };

    // try to get the user by his id.
    let user_db = match get_user(Id::from(user_id)).await {
        Ok(user) => user,
        Err(_) => {
            info!("User not found.");
            return Err(response_error);
        }
    };

    // Check if the password are equals.
    if user_db.password != get_sha512(user_pass.as_bytes()) {
        info!("The user exists but the password is wrong.");
        return Err(response_error);
    }

    Ok(user_db)
}

/**
 * A method to get some user in the database using
*/
async fn get_user(user_id: Id) -> Result<User, Response<User>> {
    // Try to get the user using his id.
    match User::from_id(user_id).await {
        Err(e) => {
            error!("Couldn\'t get the user. {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseBody::error("Couldn\'t get the user. Please call the admin."),
            ))
        }
        Ok(None) => {
            info!("User not found.");
            Err((
                StatusCode::BAD_REQUEST,
                ResponseBody::error("User not found. Check the id and try again."),
            ))
        }
        Ok(Some(user)) => {
            info!("User found.");
            Ok(user)
        }
    }
}
