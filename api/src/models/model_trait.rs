// Libs
use surrealdb::sql::{Id, Thing};
use tracing::info;

// Traits
pub trait ModelTrait<T> {
    /**
     * A method to get some T using a id.
     */
    async fn from_id(id: Id) -> surrealdb::Result<Option<T>>;

    /**
     * A method to run the migration.
     */
    async fn migration() -> surrealdb::Result<()>;

    /**
     * A method to syncronize the T in the database.
     */
    async fn sync(&mut self) -> surrealdb::Result<()>;

    /**
     * A method to create a new T in the database. MUST NOT BE CALLED DIRECTLY. USE sync() INSTEAD.
     */
    async fn create(&mut self) -> surrealdb::Result<()>;

    /**
     * A method to delete the T from the database.
     */
    async fn delete(self) -> surrealdb::Result<()>;

    /**
     * A method to generate a new ulid to the T.
     */
    async fn generate_new_ulid(tb: &str) -> surrealdb::Result<Thing> {
        info!("Generating a new ID for a new {}...", tb);
        loop {
            let id = Id::ulid();
            if Self::from_id(id.clone()).await?.is_none() {
                info!("New ID generated.");
                return Ok(Thing {
                    id,
                    tb: String::from(tb),
                });
            }
        }
    }
}
