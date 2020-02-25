Building a web server using actix is fairly simple but, working with the Postgres database is not. when I started working with Postgres, I had some problems such as application getting hung and Database connection was only synchronous though the framework itself is async.

The other problems were mostly due to the JSON(B) nature of Postgres. I did go through some links which were helpful but, took me a lot of time. So, I decided to create a project which uses Postgres pool connection and also some working examples with JSONB

Some the links, I came across while trying to figure out about pool connection and JSON(B)

[actixweb_10_and_tokiopostgres_04_deadlock](https://www.reddit.com/r/rust/comments/bytqu5/actixweb_10_and_tokiopostgres_04_deadlock/) </br>
[rust-postgres/issues/112](https://github.com/sfackler/rust-postgres/issues/112) </br>
[how_to_deserialize_queried_postgres_rows_into_json](https://www.reddit.com/r/rust/comments/clv6ou/how_to_deserialize_queried_postgres_rows_into_json/) </br>

## refer json-url.http for example URL(s)
[click here for json-urls](https://github.com/saiumesh535/actix-server/blob/master/http/json-url.http)
