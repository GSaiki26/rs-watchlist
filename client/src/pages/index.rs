// Libs
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::watchlist::Watchlist as WatchlistComponent;
use crate::models::{
    user::{User, UserRequest},
    watchlist::Watchlist as WatchlistModel,
};
use crate::router::Route;

// Functions
#[function_component(Index)]
pub fn index() -> Html {
    // Redirect to login if not logged in.
    let nav = use_navigator().unwrap();
    nav.push(&Route::Login);

    // Get wacthlists.
    let watchlists = use_state(Vec::new);

    spawn_local({
        console::log_1(&"Getting watchlists...".into());
        let watchlists = watchlists.clone();
        async move {
            let user_auth = UserRequest::new("danidani", "password");
            let user = User::login(user_auth).await;
            watchlists.set(
                user.unwrap()
                    .get_watchlists()
                    .await
                    .expect("Could not get watchlists."),
            );
            console::log_1(&"Got watchlists.".into());
        }
    });
    // let watchlists = watchlists.clone();
    // spawn_local(async move {

    // });

    let watchlists = watchlists.iter().map(|watchlist| {
        html! {
            <WatchlistComponent model={watchlist.clone()}/>
        }
    });

    html! {
        <body>
            <span id="error-message" class="hidden">{"Error"}</span>
            <div id="index-container">
                <div id="index-watchlists-list">
                    <div class="index-searchbox">
                        <div class="index-sync index-syncing"></div>
                        <input class="index-submit index-create" type="submit" value="+"/>
                        <input class="index-search" type="text" placeholder="Search watchlist"/>
                        <input class="index-submit" type="submit" value="Search"/>
                    </div>
                    {for watchlists}
                </div>
                <div id="index-watchlist-content">
                    <div id="index-watchlist-selected">
                        <span id="index-watchlist-selected-title">{"watchlist 1"}</span>
                        <span id="index-watchlist-selected-description">{"description"}</span>
                    </div>
                    <div class="index-searchbox">
                        <div class="index-sync index-syncing"></div>
                        <input class="index-submit index-create" type="submit" value="+"/>
                        <input class="index-search" type="text" placeholder="Search media"/>
                        <input class="index-submit" type="submit" value="Search"/>
                    </div>
                    <div class="index-media-entry">
                        <div class="index-entry-title">
                            {"media 1"}
                            <span class="index-media-entry-status">{"Watched"}</span>
                        </div>
                        <span>{"description"}</span>
                        <span class="index-entry-delete">{"X"}</span>
                    </div>
                </div>
            </div>
        </body>
    }
}
