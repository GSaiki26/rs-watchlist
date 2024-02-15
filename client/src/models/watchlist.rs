// Libs
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{
    api_model_trait::ApiModelTrait, config::Config, media::Media, response_body::ResponseBody,
    user::UserRequest,
};

// Structs
#[derive(Clone, Deserialize, PartialEq)]
pub struct Watchlist {
    pub id: String,
    pub owner: String,
    pub title: String,
    pub description: String,
    pub members: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct WatchlistRequest {
    title: String,
    description: String,
    members: Vec<String>,
}

// Implementations
impl Watchlist {
    /**
     * A method to get the media list in the watchlist.
     */
    async fn get_media_list(&self, auth: &UserRequest) -> reqwest::Result<Vec<Media>> {
        // Get the uri.
        let server_addr = Config::get_server_addr().await;
        let uri = format!("{}/watchlist/media", server_addr);

        // Make the request.
        let (user, pass) = auth.extract_auth();
        let req = Client::new().post(uri).basic_auth(user, pass);

        // Do the request.
        let res: ResponseBody<Vec<Media>> = req.send().await?.error_for_status()?.json().await?;
        Ok(res.data)
    }
}

impl ApiModelTrait for Watchlist {
    async fn get_by_id(auth: UserRequest, id: &str) -> reqwest::Result<Box<Self>> {
        // Get the uri.
        let server_addr = Config::get_server_addr().await;
        let uri = format!("{}/watchlist/{}", server_addr, id);

        // Make the request.
        let (user, pass) = auth.extract_auth();
        let req = Client::new().get(uri).basic_auth(user, pass);

        // Do the request.
        let res: ResponseBody<Watchlist> = req.send().await?.error_for_status()?.json().await?;
        Ok(Box::new(res.data))
    }

    async fn create<U: Serialize>(auth: UserRequest, content: U) -> reqwest::Result<Box<Self>> {
        // Get the uri.
        let server_addr = Config::get_server_addr().await;
        let uri = format!("{}/watchlist", server_addr);

        // Make the request.
        let (user, pass) = auth.extract_auth();
        let req = Client::new()
            .post(uri)
            .json(&content)
            .basic_auth(user, pass);

        // Do the request.
        let res: ResponseBody<Watchlist> = req.send().await?.error_for_status()?.json().await?;
        Ok(Box::new(res.data))
    }

    async fn update(&mut self, auth: UserRequest) -> reqwest::Result<()> {
        // Get the uri.
        let server_addr = Config::get_server_addr().await;
        let uri = format!("{}/watchlist/{}", server_addr, self.id.clone());

        // Make the request.
        let (user, pass) = auth.extract_auth();
        let req_body: WatchlistRequest = self.into();
        let req = Client::new()
            .patch(uri)
            .json(&req_body)
            .basic_auth(user, pass);

        // Do the request.
        let res: ResponseBody<Watchlist> = req.send().await?.error_for_status()?.json().await?;
        self.merge(res.data);
        Ok(())
    }

    async fn delete(self, auth: UserRequest) -> reqwest::Result<()> {
        // Get the uri.
        let server_addr = Config::get_server_addr().await;
        let uri = format!("{}/watchlist/{}", server_addr, self.id);

        // Make the request.
        let (user, pass) = auth.extract_auth();
        let req = Client::new().delete(uri).basic_auth(user, pass);

        // Do the request.
        req.send().await?.error_for_status()?;

        Ok(())
    }

    fn merge(&mut self, value: Self) {
        self.title = value.title;
        self.description = value.description;
        self.members = value.members;
        self.created_at = value.created_at;
    }
}

impl From<&mut Watchlist> for WatchlistRequest {
    fn from(val: &mut Watchlist) -> Self {
        WatchlistRequest::new(&val.title, &val.description, &val.members)
    }
}

impl WatchlistRequest {
    fn new(title: &str, description: &str, members: &[String]) -> WatchlistRequest {
        WatchlistRequest {
            title: String::from(title),
            description: String::from(description),
            members: members.to_vec(),
        }
    }
}
