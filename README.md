# Rs Watchlist
the Rust Watchlist is a application written in Rust is a CRUD using surrealdb to store a list of media's name.

It's divided into 2 binaries: `api` and `client`.
The API is resposible for interacting with the database, while the client connects to the API.

# Deploy
* Don't forget to configure the `/.env` file!

You can deploy the application's backend using Docker:
```sh
# Docker-compose
docker-compose up --build;

# Docker
docker network create watchlist;

source .env;
docker run --network watchlist -v ./data:/data:rw --name watchlist-db surrealdb/surrealdb:latest start file:/data --auth --user $DATABASE_USER --pass $DATABASE_PASS --log $DATABASE_LOG;
docker run --env-file .env --network watchlist -p 3000:3000/tcp --name watchlist gsaiki26/watchlist-api:latest;
```
