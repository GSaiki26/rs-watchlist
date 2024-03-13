// Libs
use web_sys::{
    console::{error_1 as error, info_1 as info},
    HtmlElement, HtmlInputElement,
};
use yew::{platform::spawn_local, prelude::*};

use crate::{
    components::utils::is_field_valid,
    models::user::{User as UserModel, UserRequest},
};

// Structs
#[derive(Default)]
pub struct Login {
    error_message_ref: NodeRef,
    login_username_ref: NodeRef,
    login_password_ref: NodeRef,
    register_username_ref: NodeRef,
    register_password_ref: NodeRef,
    register_cpassword_ref: NodeRef,
}

// Implementations
impl Login {
    /**
     * An event to be executed on register submit.
     */
    fn on_register_submit_event(&self) -> Callback<SubmitEvent> {
        let username = self.register_username_ref.clone();
        let password = self.register_password_ref.clone();
        let cpassword = self.register_cpassword_ref.clone();
        let error_message = self.error_message_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            // Check if the properties are valid.
            let username = username.cast::<HtmlInputElement>().unwrap();
            let password = password.cast::<HtmlInputElement>().unwrap();
            let cpassword = cpassword.cast::<HtmlInputElement>().unwrap();
            let error_message = error_message.cast::<HtmlElement>().unwrap();

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
            if !is_field_valid(&password.value()) || password.value() != cpassword.value() {
                error_message.set_inner_text(
                    "The password is invalid. Check if has valid characters and 3-20 length.",
                );
                return;
            }

            // Try to login in the API.
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

    /**
     * An event to be executed on login submit.
     */
    fn on_login_submit_event(&self) -> Callback<SubmitEvent> {
        let username_ref = self.login_username_ref.clone();
        let password_ref = self.login_password_ref.clone();
        let error_message_ref = self.error_message_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            // Treat the refs.
            e.prevent_default();
            let error_message = error_message_ref.cast::<HtmlElement>().unwrap();
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
            // let error_message = error_message.clone();
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

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let on_login_submit = self.on_login_submit_event();
        let on_register_submit = self.on_register_submit_event();
        let login_form = NodeRef::default();
        let register_form = NodeRef::default();
        let change_form = {
            let login_form = login_form.clone();
            let register_form = register_form.clone();
            Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                let login_form = login_form.cast::<HtmlElement>().unwrap();
                let register_form = register_form.cast::<HtmlElement>().unwrap();
                info(&login_form.class_name().into());
                // if login_form.class_list().contains("hidden") {
                //     login_form.class_list().remove_1("hidden").unwrap();
                //     register_form.class_list().add_1("hidden").unwrap();
                // } else {
                //     login_form.class_list().add_1("hidden").unwrap();
                //     register_form.class_list().remove_1("hidden").unwrap();
                // }
            })
        };

        // Render the html.
        html! {
            <body>
            <span ref={&self.error_message_ref} id="error-message" class="hidden"> {"Error"}</span>
            <div id="login-container">
                <form class="login-form">
                    <h1>{"Login"}</h1>
                    <input ref={&self.login_username_ref} class="login-input" type="text" name="username" placeholder="Username" required={true} autocomplete="username" />
                    <input ref={&self.login_password_ref} class="login-input" type="password" name="password" placeholder="Password" required={true} autocomplete="current-password" />
                    <input onsubmit={on_login_submit} class="login-input" type="submit" value="Login"/>
                    <span class="login-change-form">
                        {"Don't have an account?"}
                        <a onclick={&change_form} class="login-a">{"Click here to register"}</a>
                    </span>
                </form>

                <form class="login-form hidden">
                    <h1>{"Register"}</h1>
                    <input ref={&self.register_username_ref} class="login-input" type="text" name="username" placeholder="Username" required={true} autocomplete="username"/>
                    <input ref={&self.register_password_ref} class="login-input" type="password" name="password" placeholder="Password" required={true} autocomplete="new-password"/>
                    <input ref={&self.register_cpassword_ref} class="login-input" type="password" name="password" placeholder="Confirm password" required={true} autocomplete="new-password"/>
                    <input onsubmit={on_register_submit} class="login-input" type="submit" value="Register"/>
                    <span class="login-change-form">
                        {"Already have an account?"}
                        <a onclick={change_form} class="login-a">{"Click here to login"}</a>
                    </span>
                </form>
            </div>
            </body>
        }
    }
}
