use serde::Serialize;

// Libs
use super::user::UserRequest;

// Traits
pub trait ApiModelTrait {
    /**
     * A method to get some T using an id.
     */
    async fn get_by_id(auth: UserRequest, id: &str) -> reqwest::Result<Box<Self>>;

    /**
     *  A method to create a new T.
     */
    async fn create<U: Serialize>(auth: UserRequest, content: U) -> reqwest::Result<Box<Self>>;

    /**
     * A method to update the T.
     */
    async fn update(&mut self, auth: UserRequest) -> reqwest::Result<()>;

    /**
     * A method to delete the T.
     */
    async fn delete(self, auth: UserRequest) -> reqwest::Result<()>;

    /**
     * A method to merge the T and some other T.
     */
    fn merge(&mut self, other: Self);
}
