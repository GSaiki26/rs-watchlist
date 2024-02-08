// Libs
use axum::{extract::Path, http::StatusCode, Json};
use axum_auth::AuthBasic;
use surrealdb::sql::{Id, Thing};
use tracing::{error, info, warn};

use super::response_body::ResponseBody;
use super::user_controler::login_user;
use crate::models::{
    model_trait::ModelTrait,
    user_model::User,
    watchlist_model::{Watchlist, WatchlistRequest, WatchlistResponse},
};

// Types
type Response = (StatusCode, Json<ResponseBody>);

// Functions
/**
 * POST /watchlist
 * Authorization: Basic
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

    // Get owned watchlists.
    let mut watchlists = match logged_user.get_watchlists_as_owner().await {
        Err(e) => {
            error!("Couldn\'t get the owned watchlists. {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseBody::error("Couldn\'t get the watchlists. Please contact the admin."),
            );
        }
        Ok(watchlists) => watchlists,
    };

    // Get the watchlists where the user is member.
    let mut member_watchlists = match logged_user.get_watchlists_as_member().await {
        Err(e) => {
            error!("Couldn\'t get the as member watchlists. {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseBody::error("Couldn\'t get the watchlists. Please contact the admin."),
            );
        }
        Ok(member_watchlists) => member_watchlists,
    };

    watchlists.append(&mut member_watchlists);

    info!("The watchlists were successfully retrieved.");
    let watchlists = watchlists
        .iter()
        .map(|wl| wl.to_watchlist_response())
        .collect::<Vec<WatchlistResponse>>();
    (StatusCode::OK, ResponseBody::success(watchlists))
}

/**
 * GET /watchlist/{watchlist_id}
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
 * PATCH /watchlist/{watchlist_id}
 * Authorization: Basic
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

// Utils
/**
 * A method to get some watchlist in the database using its id.
*/
pub async fn get_watchlist_from_id(watchlist_id: Id) -> Result<Watchlist, Response> {
    // Try to get the watchlist using his id.
    match Watchlist::from_id(watchlist_id).await {
        Err(e) => {
            error!("Couldn\'t get the watchlist. {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseBody::error("Couldn\'t get the watchlist. Please contact the admin."),
            ))
        }
        Ok(None) => {
            info!("watchlist not found.");
            Err((
                StatusCode::NOT_FOUND,
                ResponseBody::error("watchlist not found. Check the id and try again."),
            ))
        }
        Ok(Some(watchlist)) => {
            info!("watchlist found.");
            Ok(watchlist)
        }
    }
}

/**
 * A method to check if all members are valid.
*/
pub async fn are_members_valid(owner: &Thing, members: &Vec<String>) -> Result<(), Response> {
    info!("Checking if all members from the watchlist are valid.");
    if members.contains(&owner.id.to_string()) {
        warn!("The owner is a member.");
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseBody::error("Some member is valid. Check the parameters and try again."),
        ));
    }

    // Check if all members are valid.
    for member_id in members {
        match User::from_id(Id::from(member_id)).await {
            Err(e) => {
                error!("Couldn\'t check if all members are valid. {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseBody::error(
                        "Couldn\'t create the watchlist. Please contact the admin.",
                    ),
                ));
            }
            Ok(None) => {
                info!("Some member is invalid.");
                return Err((
                    StatusCode::BAD_REQUEST,
                    ResponseBody::error(
                        "Some member is valid. Check the parameters and try again.",
                    ),
                ));
            }
            Ok(Some(_)) => (),
        }
    }

    info!("All members are valid.");
    Ok(())
}
