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
pub struct MediaRequest {
    title: String,
    description: String,
    watchlist: String,
    watched: bool,
}

// Implementations
impl ApiModelTrait for Media {
    async fn get_by_id(auth: UserRequest, id: &str) -> reqwest::Result<Box<Self>> {
        // Get the uri.
        let server_addr = Config::get_server_addr().await;
        let uri = format!("{}/media/{}", server_addr, id);

        // Make the request.
        let (user, pass) = auth.extract_auth();
        let req = Client::new().get(uri).basic_auth(user, pass);

        // Do the request.
        let res: ResponseBody<Media> = req.send().await?.error_for_status()?.json().await?;
        Ok(Box::new(res.data))
    }

    async fn create<U: Serialize>(auth: UserRequest, content: U) -> reqwest::Result<Box<Self>> {
        // Get the uri.
        let server_addr = Config::get_server_addr().await;
        let uri = format!("{}/media", server_addr);

        // Make the request.
        let (user, pass) = auth.extract_auth();
        let req = Client::new()
            .post(uri)
            .json(&content)
            .basic_auth(user, pass);

        // Do the request.
        let res: ResponseBody<Media> = req.send().await?.error_for_status()?.json().await?;
        Ok(Box::new(res.data))
    }

    async fn update(&mut self, auth: UserRequest) -> reqwest::Result<()> {
        // Get the uri.
        let server_addr = Config::get_server_addr().await;
        let uri = format!("{}/media/{}", server_addr, self.id.clone());

        // Make the request.
        let (user, pass) = auth.extract_auth();
        let req_body: MediaRequest = self.into();
        let req = Client::new()
            .patch(uri)
            .json(&req_body)
            .basic_auth(user, pass);

        // Do the request.
        let res: ResponseBody<Media> = req.send().await?.error_for_status()?.json().await?;
        self.merge(res.data);
        Ok(())
    }

    async fn delete(self, auth: UserRequest) -> reqwest::Result<()> {
        // Get the uri.
        let server_addr = Config::get_server_addr().await;
        let uri = format!("{}/media/{}", server_addr, self.id);

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
        self.watchlist = value.watchlist;
        self.watched = value.watched;
        self.updated_at = value.updated_at;
    }
}

impl From<&mut Media> for MediaRequest {
    fn from(val: &mut Media) -> Self {
        MediaRequest::new(&val.title, &val.description, &val.watchlist, val.watched)
    }
}

impl MediaRequest {
    fn new(title: &str, description: &str, watchlist: &str, watched: bool) -> MediaRequest {
        MediaRequest {
            title: String::from(title),
            description: String::from(description),
            watchlist: String::from(watchlist),
            watched,
        }
    }
}
