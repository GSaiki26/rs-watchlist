// Libs
use axum::{extract::Path, http::StatusCode, Json};
use axum_auth::AuthBasic;
use surrealdb::sql::Id;
use tracing::{error, info, warn};

use super::controllers_utils::*;
use super::response_body::ResponseBody;
use crate::models::{
    media_model::MediaResponse,
    model_trait::ModelTrait,
    watchlist_model::{Watchlist, WatchlistRequest, WatchlistResponse},
};

// Functions
/**
 * POST /watchlist
 * Authorization: Basic
 * BODY: WatchlistRequest
 * A method to create a new watchlist.
*/
pub async fn post_watchlist(
    AuthBasic(user_auth): AuthBasic,
    Json(watchlist): Json<WatchlistRequest>,
) -> Response {
    // Get the user and make it the owner of the watchlist.
    let logged_user = match login_user(user_auth, false).await {
        Err(res) => return res,
        Ok(logged_user) => logged_user,
    };

    // Check if all provided members are valid.
    if let Err(res) = are_members_valid(logged_user.id.as_ref().unwrap(), &watchlist.members).await
    {
        return res;
    }

    // Convert the watchlist request to a watchlist.
    let mut watchlist = Watchlist::from(watchlist);
    watchlist.owner = logged_user.id;

    // Try to synchronize the given watchlist in the database.
    match watchlist.sync().await {
        Err(e) => {
            warn!("Couldn\'t create the watchlist. {}", e);
            (
                StatusCode::BAD_REQUEST,
                ResponseBody::error(
                    "Couldn\'t create the watchlist. Check the parameters and try again.",
                ),
            )
        }
        Ok(_) => (
            StatusCode::CREATED,
            ResponseBody::success(watchlist.to_watchlist_response()),
        ),
    }
}

/**
 * GET /watchlists
 * Authorization: Basic
 * A method to get all the watchlist from the user.
*/
pub async fn get_watchlists(AuthBasic(user_auth): AuthBasic) -> Response {
    // Check if the authorization is valid.
    let logged_user = match login_user(user_auth, false).await {
        Err(res) => return res,
        Ok(logged_user) => logged_user,
    };

    // Get all the watchlists from the user.
    let watchlists = match get_all_watchlist_from_user(&logged_user).await {
        Err(res) => return res,
        Ok(watchlists) => watchlists,
    };

    let watchlists = watchlists
        .iter()
        .map(|wl| wl.to_watchlist_response())
        .collect::<Vec<WatchlistResponse>>();
    (StatusCode::OK, ResponseBody::success(watchlists))
}

/**
 * GET /watchlist/{watchlist_id}
 * Authorization: Basic
 * A method to get an watchlist.
*/
pub async fn get_watchlist(
    AuthBasic(user_auth): AuthBasic,
    Path(watchlist_id): Path<String>,
) -> Response {
    // Check if the authorization is valid.
    let logged_user = match login_user(user_auth, false).await {
        Err(res) => return res,
        Ok(user) => user,
    };

    // Get the watchlist.
    let watchlist = match get_watchlist_from_id(Id::from(watchlist_id)).await {
        Err(res) => return res,
        Ok(watchlist) => watchlist,
    };

    // Check if the user is the owner or is a members of the watchlist.
    let id = logged_user.id.as_ref().unwrap();
    if !watchlist.is_owner(id) && !watchlist.has_member(id) {
        return (
            StatusCode::FORBIDDEN,
            ResponseBody::error("You don\'t have permission to access this watchlist."),
        );
    }

    (
        StatusCode::OK,
        ResponseBody::success(watchlist.to_watchlist_response()),
    )
}

/**
 * GET /watchlist/{watchlist_id}/media
 * Authorization: Basic
 * A method to get an watchlist.
*/
pub async fn get_watchlist_medias(
    AuthBasic(user_auth): AuthBasic,
    Path(watchlist_id): Path<String>,
) -> Response {
    // Check if the authorization is valid.
    let logged_user = match login_user(user_auth, false).await {
        Err(res) => return res,
        Ok(user) => user,
    };

    // Get the watchlist.
    let watchlist = match get_watchlist_from_id(Id::from(watchlist_id)).await {
        Err(res) => return res,
        Ok(watchlist) => watchlist,
    };

    // Check if the user is the owner or is a members of the watchlist.
    let id = logged_user.id.as_ref().unwrap();
    if !watchlist.is_owner(id) && !watchlist.has_member(id) {
        return (
            StatusCode::FORBIDDEN,
            ResponseBody::error("You don\'t have permission to access this watchlist."),
        );
    }

    // Get the medias from the watchlist.
    match watchlist.get_media().await {
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ResponseBody::error("Couldn\'t get the medias. Please contact the admin."),
        ),
        Ok(medias) => {
            info!("The medias were successfully retrieved.");
            let medias: Vec<MediaResponse> = medias.iter().map(|m| m.to_media_response()).collect();
            (StatusCode::OK, ResponseBody::success(medias))
        }
    }
}

/**
 * PATCH /watchlist/{watchlist_id}
 * Authorization: Basic
 * BODY: WatchlistRequest
 * A method to update an watchlist.
*/
pub async fn patch_watchlist(
    AuthBasic(user_auth): AuthBasic,
    Path(watchlist_id): Path<String>,
    Json(new_watchlist): Json<WatchlistRequest>,
) -> Response {
    // Check if the authorization is valid.
    let logged_user = match login_user(user_auth, false).await {
        Err(res) => return res,
        Ok(logged_user) => logged_user,
    };

    // Try to get the watchlist.
    let mut db_watchlist = match get_watchlist_from_id(Id::from(watchlist_id)).await {
        Err(res) => return res,
        Ok(watchlist) => watchlist,
    };

    // Check if the user is the owner of the watchlist.
    if !db_watchlist.is_owner(logged_user.id.as_ref().unwrap()) {
        return (
            StatusCode::FORBIDDEN,
            ResponseBody::error("You don\'t have permission to update this watchlist."),
        );
    }

    // Check if all provided members are valid.
    if let Err(res) = are_members_valid(&logged_user.id.unwrap(), &new_watchlist.members).await {
        return res;
    }

    // Define the content that the response doesn't have/can't modify.
    db_watchlist.merge(Watchlist::from(new_watchlist));

    // Try to synchronize the watchlist in the database.
    match db_watchlist.sync().await {
        Err(e) => {
            warn!("Couldn\'t update the watchlist. {}", e);
            (
                StatusCode::BAD_REQUEST,
                ResponseBody::error(
                    "Couldn\'t update the watchlist. Check the parameters and try again.",
                ),
            )
        }
        Ok(_) => {
            info!(
                "{} updated successfully.",
                db_watchlist.id.as_ref().unwrap()
            );
            (
                StatusCode::OK,
                ResponseBody::success(db_watchlist.to_watchlist_response()),
            )
        }
    }
}

/**
 * DELETE /watchlist/{watchlist_id}
 * Authorization: Basic
 * A method to delete an watchlist.
*/
pub async fn delete_watchlist(
    AuthBasic(watchlist_auth): AuthBasic,
    Path(watchlist_id): Path<String>,
) -> Response {
    // Check if the authorization is valid.
    let logged_user = match login_user(watchlist_auth, false).await {
        Err(res) => return res,
        Ok(watchlist) => watchlist,
    };

    // Try to get the watchlist.
    let db_watchlist = match get_watchlist_from_id(Id::from(watchlist_id)).await {
        Err(res) => return res,
        Ok(watchlist) => watchlist,
    };

    // Check if the user is the owner of the watchlist.
    if !db_watchlist.is_owner(logged_user.id.as_ref().unwrap()) {
        return (
            StatusCode::FORBIDDEN,
            ResponseBody::error("You don\'t have permission to delete this watchlist."),
        );
    }

    // Try to delete the watchlist.
    let watchlist_id = db_watchlist.id.clone().unwrap();
    if let Err(e) = db_watchlist.delete().await {
        error!("Couldn\'t delete the watchlist. {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ResponseBody::error("Couldn\'t get the watchlist. Please contact the admin."),
        );
    }

    info!("The {} was successfully deleted.", watchlist_id);
    (StatusCode::OK, ResponseBody::success_no_data())
}
