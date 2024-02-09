// Libs
use axum::{extract::Path, http::StatusCode, Json};
use axum_auth::AuthBasic;
use surrealdb::sql::Id;
use tracing::{error, warn};

use super::controllers_utils::*;
use super::response_body::ResponseBody;
use crate::models::media_model::MediaRequest;
use crate::models::watchlist_model::Watchlist;
use crate::models::{media_model::Media, model_trait::ModelTrait};

// Functions
/**
 * POST /media
 * Authorization: Basic
 * BODY: MediaRequest
 * A method to create a new media.
*/
pub async fn post_media(
    AuthBasic(user_auth): AuthBasic,
    Json(new_media): Json<MediaRequest>,
) -> Response {
    // Check if the user is valid.
    let logged_user = match login_user(user_auth, false).await {
        Err(res) => return res,
        Ok(logged_user) => logged_user,
    };

    // Check if the provided watchlist is valid.
    let watchlist = match Watchlist::from_id(Id::from(&new_media.watchlist)).await {
        Ok(Some(watchlist)) => watchlist,
        Err(e) => {
            warn!("Couldn\'t get the watchlist. {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseBody::error("Couldn\'t create the media. Please call the admin."),
            );
        }
        Ok(None) => {
            warn!("Watchlist not found.");
            return (
                StatusCode::BAD_REQUEST,
                ResponseBody::error("Watchlist not found. Check the id and try again."),
            );
        }
    };

    // Check if the user has permission to add a media to the watchlist.
    let id = logged_user.id.as_ref().unwrap();
    if !watchlist.is_owner(id) && !watchlist.has_member(id) {
        warn!("User doesn\'t have permission to add a media to the watchlist.");
        return (
            StatusCode::FORBIDDEN,
            ResponseBody::error("You don\'t have permission to add a media to the watchlist."),
        );
    }

    // Try to synchronize the given media in the database.
    let mut new_media = Media::from(new_media);
    match new_media.sync().await {
        Err(e) => {
            warn!("Couldn\'t create the media. {}", e);
            (
                StatusCode::BAD_REQUEST,
                ResponseBody::error(
                    "Couldn\'t create the media. Check the parameters and try again.",
                ),
            )
        }
        Ok(_) => (
            StatusCode::CREATED,
            ResponseBody::success(new_media.to_media_response()),
        ),
    }
}

/**
 * PATCH /media/{media_id}
 * Authorization: Basic
 * BODY: MediaRequest
 * A method to update an media.
*/
pub async fn patch_media(
    AuthBasic(user_auth): AuthBasic,
    Path(media_id): Path<String>,
    Json(new_media): Json<MediaRequest>,
) -> Response {
    // Check if the user is valid.
    let logged_user = match login_user(user_auth, false).await {
        Err(res) => return res,
        Ok(logged_user) => logged_user,
    };
    // Check if the provided media is valid.
    let mut db_media = match get_media_from_id(Id::from(&media_id)).await {
        Err(res) => return res,
        Ok(media) => media,
    };
    // Check if the provided watchlist is valid.
    let db_watchlist = match get_watchlist_from_id(Id::from(&media_id)).await {
        Err(res) => return res,
        Ok(media) => media,
    };

    // Check if the user has permission to add the media to the watchlist.
    let id = logged_user.id.as_ref().unwrap();
    if !db_watchlist.is_owner(id) && !db_watchlist.has_member(id) {
        warn!("User doesn\'t have permission to add a media to the watchlist.");
        return (
            StatusCode::FORBIDDEN,
            ResponseBody::error("You don\'t have permission to add a media to the watchlist."),
        );
    }

    // Define the content that the response doesn't have/can't modify.
    let new_media = Media::from(new_media);
    db_media.merge(new_media);

    // Try to synchronize the media in the database.
    match db_media.sync().await {
        Err(e) => {
            warn!("Couldn\'t update the media. {}", e);
            (
                StatusCode::BAD_REQUEST,
                ResponseBody::error(
                    "Couldn\'t create the media. Check the parameters and try again.",
                ),
            )
        }
        Ok(_) => (
            StatusCode::OK,
            ResponseBody::success(db_media.to_media_response()),
        ),
    }
}

/**
 * DELETE /media/{media_id}
 * Authorization: Basic
 * A method to delete an media.
*/
pub async fn delete_media(
    AuthBasic(media_auth): AuthBasic,
    Path(media_id): Path<String>,
) -> Response {
    // Check if the authorization is valid.
    let logged_user = match login_user(media_auth, false).await {
        Err(res) => return res,
        Ok(logged_user) => logged_user,
    };

    // Check if the provided media is valid.
    let provided_media = match get_media_from_id(Id::from(media_id)).await {
        Err(res) => return res,
        Ok(media) => media,
    };

    // Get the media watchlist.
    let watchlist_id = provided_media.watchlist.clone().id;
    let watchlist = match get_watchlist_from_id(watchlist_id).await {
        Err(res) => return res,
        Ok(watchlist) => watchlist,
    };

    // Check if the user has permission in tthe watchlist.
    let id = logged_user.id.as_ref().unwrap();
    if !watchlist.is_owner(id) && !watchlist.has_member(id) {
        warn!("User doesn\'t have permission to delete the media.");
        return (
            StatusCode::FORBIDDEN,
            ResponseBody::error("You don\'t have permission to delete the media."),
        );
    }

    // Try to delete the media.
    if let Err(e) = provided_media.delete().await {
        error!("Couldn\'t delete the media. {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ResponseBody::error("Couldn\'t get the media. Please call the admin."),
        );
    }

    (StatusCode::OK, ResponseBody::success_no_data())
}

/**
 * GET /media/{media_id}
 * Authorization: Basic
 * A method to get an media.
*/
pub async fn get_media(Path(media_id): Path<String>) -> Response {
    // Try to get the media.
    match get_media_from_id(Id::from(media_id)).await {
        Err(response) => response,
        Ok(media) => (
            StatusCode::OK,
            ResponseBody::success(media.to_media_response()),
        ),
    }
}
