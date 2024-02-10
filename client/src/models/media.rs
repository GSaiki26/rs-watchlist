// Libs
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{
    api_model_trait::ApiModelTrait, config::Config, response_body::ResponseBody, user::UserRequest,
};

// Structs
#[derive(Clone, Deserialize)]
pub struct Media {
    id: String,
    title: String,
    description: String,
    watchlist: String,
    watched: bool,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize)]
struct MediaRequest {
    title: String,
    description: String,
    watchlist: String,
    watched: bool,
}

// Implementations
impl ApiModelTrait for Media {
    async fn get_by_id(auth: UserRequest, id: &str) -> reqwest::Result<Box<Self>> {
        // Get the uri.
        let server_addr = Config::new().get_server_addr();
        let uri = format!("{}/media/{}", server_addr, id);

        // Make the request.
        let (user, pass) = auth.extract_auth();
        let req = Client::new().get(uri).basic_auth(user, pass);

        // Do the request.
        let res: ResponseBody<Media> = req.send().await?.error_for_status()?.json().await?;
        Ok(Box::new(Media::from(res.data)))
    }

    async fn create(&mut self, auth: UserRequest) -> reqwest::Result<()> {
        // Get the uri.
        let server_addr = Config::new().get_server_addr();
        let uri = format!("{}/media", server_addr);

        // Make the request.
        let (user, pass) = auth.extract_auth();
        let req_body = MediaRequest::new(
            &self.title,
            &self.description,
            &self.watchlist,
            self.watched,
        );
        let req = Client::new()
            .post(uri)
            .json(&req_body)
            .basic_auth(user, pass);

        // Do the request.
        let res: ResponseBody<Media> = req.send().await?.error_for_status()?.json().await?;
        self.merge(Media::from(res.data));
        Ok(())
    }

    async fn update(&mut self, auth: UserRequest) -> reqwest::Result<()> {
        // Get the uri.
        let server_addr = Config::new().get_server_addr();
        let uri = format!("{}/media/{}", server_addr, self.id.clone());

        // Make the request.
        let (user, pass) = auth.extract_auth();
        let req_body = MediaRequest::new(
            &self.title,
            &self.description,
            &self.watchlist,
            self.watched,
        );
        let req = Client::new()
            .patch(uri)
            .json(&req_body)
            .basic_auth(user, pass);

        // Do the request.
        let res: ResponseBody<Watchlist> = req.send().await?.error_for_status()?.json().await?;
        self.merge(Watchlist::from(res.data));
        Ok(())
    }

    async fn delete(self, auth: UserRequest) -> reqwest::Result<()> {
        // Get the uri.
        let server_addr = Config::new().get_server_addr();
        let uri = format!("{}/media/{}", server_addr, self.id);

        // Make the request.
        let (user, pass) = auth.extract_auth();
        let req = Client::new().delete(uri).basic_auth(user, pass);

        // Make the request.
        req.send().await?.error_for_status()?;

        Ok(())
    }

    fn merge(&mut self, value: Self) {
        self.id = value.id;
        self.username = value.username;
        self.password = value.password;
        self.created_at = value.created_at;
        self.updated_at = value.updated_at;
    }
}

impl MediaRequest {
    fn new(title: &str, description: &str, watchlist: &str, watched: bool) -> MediaRequest {
        MediaRequest {
            title: title.to_string(),
            description: description.to_string(),
            watchlist: watchlist.to_string(),
            watched,
        }
    }
}
