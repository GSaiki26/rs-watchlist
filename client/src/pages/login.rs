// Libs
use web_sys::{
    console::{error_1 as error, info_1 as info},
    HtmlElement, HtmlInputElement,
};
use yew::prelude::*;

use crate::components::login::Login as LoginComponent;

// Functions
/**
 * An event to be executed on register submit.
*/
// fn on_register_submit_event(
//     error_message: HtmlElement,
//     username: HtmlInputElement,
//     password: HtmlInputElement,
//     cpassword: HtmlInputElement,
// ) -> Callback<SubmitEvent> {
//     Callback::from(move |e: SubmitEvent| {
//         e.prevent_default();

//         // Check if the properties are valid.
//         let username = username.inner_text();
//         let password = password.inner_text();
//         let cpassword = cpassword.inner_text();
//         if !is_field_valid(&username) {
//             error_message.set_inner_text(
//                 "The username is invalid. Check if has valid characters and 3-20 length.",
//             );
//             return;
//         }
//         if !is_field_valid(&password) {
//             error_message.set_inner_text(
//                 "The password is invalid. Check if has valid characters and 3-20 length.",
//             );
//             return;
//         }

//         // Try to login in the API.
//         let error_message = error_message.clone();
//         spawn_local(async move {
//             let user = UserRequest::new(&username, &password);
//             match UserModel::login(user).await {
//                 Err(e) => {
//                     error(&format!("Couldn\'t login the user. {}", e).into());
//                     error_message.set_inner_text(
//                         "The username or password is invalid. Check the fields and try again.",
//                     );
//                 }
//                 Ok(user) => {
//                     info(&format!("User#{} successfully logged in.", &user.id).into());
//                 }
//             };
//         });
//     })
// }

// Structs
#[derive(Default)]
pub struct Login {
    error_message: NodeRef,
    login_username: NodeRef,
    login_password: NodeRef,
    // register_username: NodeRef,
    // register_password: NodeRef,
    // register_cpassword: NodeRef,
}

// Implementations
impl Component for Login {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Render the html.
        html! {
            <body>
            <span ref={&self.error_message} id="error-message" class="hidden"> {"Error"}</span>
            <div id="login-container">
                <LoginComponent error_message_ref={&self.error_message}/>
                // <form class="login-form hidden">
                //     <h1>{"Register"}</h1>
                //     <input class="login-input" type="text"      name="username" placeholder="Username"          required={true} autocomplete="username"/>
                //     <input class="login-input" type="password"  name="password" placeholder="Password"          required={true} autocomplete="new-password"/>
                //     <input class="login-input" type="password"  name="password" placeholder="Confirm password"  required={true} autocomplete="new-password"/>
                //     <input class="login-input" type="submit" value="Register"/>
                //     <span class="login-change-form">
                //         {"Already have an account?"}
                //         <a class="login-a">{"Click here to login"}</a>
                //     </span>
                // </form>
            </div>
            </body>
        }
    }
}
