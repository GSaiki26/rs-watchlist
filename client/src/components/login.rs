// Libs
use web_sys::{
    console::{error_1 as error, info_1 as info},
    HtmlElement, HtmlInputElement,
};
use yew::{platform::spawn_local, prelude::*};

use super::utils::is_field_valid;
use crate::models::user::{User as UserModel, UserRequest};

// Structs
#[derive(PartialEq, Properties)]
pub struct LoginProps {
    pub error_message_ref: NodeRef,
}

#[derive(Default)]
pub struct Login {
    input_username_ref: NodeRef,
    input_password_ref: NodeRef,
}

// Implementations
impl Login {
    /**
     * An event to be executed on login submit.
     */
    fn on_login_submit_event(&self, error_message: NodeRef) -> Callback<SubmitEvent> {
        let username_ref = self.input_username_ref.clone();
        let password_ref = self.input_password_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            // Treat the refs.
            e.prevent_default();
            let error_message = error_message.cast::<HtmlElement>().unwrap();
            let username = username_ref.cast::<HtmlInputElement>().unwrap();
            let password = password_ref.cast::<HtmlInputElement>().unwrap();

            // Check if the properties are valid.
            if !is_field_valid(&username.value()) {
                error_message.set_inner_text(
                    "The username is invalid. Check if has valid characters and 3-20 length.",
                );
                return;
            }
            if !is_field_valid(&password.value()) {
                error_message.set_inner_text(
                    "The password is invalid. Check if has valid characters and 3-20 length.",
                );
                return;
            }

            // Try to login in the API.
            let error_message = error_message.clone();
            let (username, password) = (username.value(), password.value());
            spawn_local(async move {
                let user = UserRequest::new(&username, &password);
                match UserModel::login(user).await {
                    Err(e) => {
                        error(&format!("Couldn\'t login the user. {}", e).into());
                        error_message.set_inner_text(
                            "The username or password is invalid. Check the fields and try again.",
                        );
                    }
                    Ok(user) => {
                        info(&format!("User#{} successfully logged in.", &user.id).into());
                    }
                };
            });
        })
    }
}

impl Component for Login {
    type Message = ();

    type Properties = LoginProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Events
        let onsubmit = self.on_login_submit_event(ctx.props().error_message_ref.clone());

        // Render
        html! {
            <form class="login-form">
                <h1>{"Login"}</h1>
                <input ref={&self.input_username_ref} class="login-input" type="text" name="username" placeholder="Username" required={true} autocomplete="username" />
                <input ref={&self.input_password_ref} class="login-input" type="password" name="password" placeholder="Password" required={true} autocomplete="current-password" />
                <input onsubmit={onsubmit} class="login-input" type="submit" value="Login"/>
                <span class="login-change-form">
                    {"Don't have an account?"}
                    <a class="login-a">{"Click here to register"}</a>
                </span>
            </form>
        }
    }
}
