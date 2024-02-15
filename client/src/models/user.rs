// Libs
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

use super::{
    api_model_trait::ApiModelTrait, config::Config, response_body::ResponseBody,
    watchlist::Watchlist,
};

// Structs
#[derive(Clone, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/**
 * NOTE: The UserRequest is also used as an Auth struct.
 * The username is a reference from username in a Basic Auth. But in the API it is used as the id.
*/
#[derive(Default, Serialize)]
pub struct UserRequest {
    username: String,
    password: String,
}

// Implementations
impl User {
    /**
     * A method to check if the login is valid.
     */
    pub async fn login(user_auth: UserRequest) -> reqwest::Result<User> {
        // Get the uri.
        // let server_addr = Config::get_server_addr().await;
        let server_addr = "http://127.0.0.1:3000";
        let uri = format!("{}/user/login", server_addr);

        // Make the request.
        let (username, password) = user_auth.extract_auth();
        let req = Client::new()
            .request(Method::POST, uri)
            .basic_auth(username, password);

        // Do the request.
        let user = req
            .send()
            .await?
            .error_for_status()?
            .json::<ResponseBody<Self>>()
            .await?;
        Ok(user.data)
    }

    /**
     * A method to get all watchlists the user is involved.
     */
    pub async fn get_watchlists(&self) -> reqwest::Result<Vec<Watchlist>> {
        // Get the uri.
        let server_addr = Config::get_server_addr().await;
        let uri = format!("{}/watchlist", server_addr);

        // Make the request.
        let Self { id, password, .. } = self.clone();
        let req = Client::new()
            .request(Method::POST, uri)
            .basic_auth(id, password);

        // Do the request.
        let res: ResponseBody<Vec<Watchlist>> =
            req.send().await?.error_for_status()?.json().await?;
        Ok(res.data)
    }
}

impl ApiModelTrait for User {
    async fn get_by_id(_auth: UserRequest, id: &str) -> reqwest::Result<Box<Self>> {
        // Get the uri.
        let server_addr = Config::get_server_addr().await;
        let uri = format!("{}/user/{}", server_addr, id);

        // Make the request.
        let req = Client::new().request(Method::GET, uri);

        // Do the request.
        let res: ResponseBody<User> = req.send().await?.error_for_status()?.json().await?;
        Ok(Box::new(res.data))
    }

    async fn create<U: Serialize>(_auth: UserRequest, content: U) -> reqwest::Result<Box<Self>> {
        // Get the uri.
        let server_addr = Config::get_server_addr().await;
        let uri = format!("{}/user", server_addr);

        // Make the request.
        let req = Client::new().request(Method::POST, uri).json(&content);

        // Do the request
        let res: ResponseBody<User> = req.send().await?.error_for_status()?.json().await?;
        Ok(Box::new(res.data))
    }

    async fn update(&mut self, _auth: UserRequest) -> reqwest::Result<()> {
        // Get the uri.
        let server_addr = Config::get_server_addr().await;
        let uri = format!("{}/user/{}", server_addr, self.id.clone());

        // Make the request.
        let Self { id, password, .. } = self.clone();
        let req_body: UserRequest = self.into();
        let req = Client::new()
            .request(Method::PATCH, uri)
            .basic_auth(id, password)
            .json(&req_body);

        // Do the request.
        let res: ResponseBody<User> = req.send().await?.error_for_status()?.json().await?;
        self.merge(res.data);
        Ok(())
    }

    async fn delete(self, _auth: UserRequest) -> reqwest::Result<()> {
        // Get the uri.
        let server_addr = Config::get_server_addr().await;
        let uri = format!("{}/user/{}", server_addr, self.id);

        // Make the request.
        let Self { id, password, .. } = self.clone();
        let req = Client::new()
            .request(Method::DELETE, uri)
            .basic_auth(id, password);

        // Do the request.
        req.send().await?.error_for_status()?;
        Ok(())
    }

    fn merge(&mut self, value: Self) {
        self.username = value.username;
        self.updated_at = value.updated_at;
    }
}

impl From<&mut User> for UserRequest {
    fn from(val: &mut User) -> Self {
        UserRequest::new(
            &val.username,
            val.password.as_ref().expect("Password not defined."),
        )
    }
}

impl UserRequest {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: String::from(username),
            password: String::from(password),
        }
    }

    /**
     * A method to extract the username and the password from the UserRequest.
     */
    pub fn extract_auth(&self) -> (&str, Option<&str>) {
        (&self.username, Some(&self.password))
    }
}
