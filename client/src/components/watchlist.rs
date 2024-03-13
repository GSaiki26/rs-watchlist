// Libs
use yew::prelude::*;

use crate::models::watchlist::Watchlist as WatchlistModel;

// Structs
#[derive(PartialEq)]
pub struct Watchlist {
    model: WatchlistModel,
}

#[derive(Clone, PartialEq, Properties)]
pub struct WatchlistPops {
    pub model: WatchlistModel,
}

impl Component for Watchlist {
    type Message = ();
    type Properties = WatchlistPops;

    fn create(ctx: &Context<Self>) -> Self {
        // The watchlist model is passed as a prop.
        let props = ctx.props();
        Self {
            model: props.model.clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let model_title = &self.model.title;
        let model_owner = &self.model.owner;
        let model_description = &self.model.description;
        html! {
            <div class="index-watchlist-entry">
                <div class="index-entry-title">
                    {model_title}
                    <span class="index-entry-owner">{model_owner}</span>
                </div>
                <span>{model_description}</span>
                <span class="index-entry-delete">{"X"}</span>
            </div>
        }
    }
}
