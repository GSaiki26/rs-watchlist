// Libs
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::index::Index;
use crate::pages::login::Login;

// Structs
#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Index/> },
        Route::Login => html! { <Login/> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
