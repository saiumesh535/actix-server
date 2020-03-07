# actix-server
Web server using actix-web

## Database setup
Run the database script which is available [here](https://github.com/saiumesh535/actix-server/blob/master/scripts/sql/init.sql)

We will be using [Deadpool postgres ](https://crates.io/crates/deadpool-postgres) for connecting postgres database


## Working with Postgres JSONB
For JSON Postgres example [click here](https://github.com/saiumesh535/actix-server/tree/master/src/Json)

## Roadmap

| Feature  | Status | Source |
| ------------- | ------------- | ------------- |
| Postgres Pool  | :heavy_check_mark:  | [Link](https://github.com/saiumesh535/actix-server/blob/master/src/main.rs#L15)
| Postgres JSON(B)  | :heavy_check_mark:  | [Link](https://github.com/saiumesh535/actix-server/tree/master/src/Json) |
| Error handling using Snafu | :x:  | :x: |
| Authentication | :heavy_check_mark:  | [Link](https://github.com/saiumesh535/actix-server/pull/6) |
| Authorization | :x:  | :x: |
| Middleware | :x:  | :x: |
| Redis | :heavy_check_mark:  | [Link](https://github.com/saiumesh535/actix-server/pull/6) |
| Email | :x:  | :x: |
| Download Files | :x:  | :x: |
| Validations | :x:  | :x: |

## Contribution
I'm not sure what to write here, will update later :smiley:
