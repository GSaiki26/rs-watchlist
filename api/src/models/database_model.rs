// Libs
use surrealdb::sql::Id;

// Traits
pub trait DatabaseModel<T> {
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
     * A method to delete the T from the database.
     */
    async fn delete(self) -> surrealdb::Result<()>;
}
