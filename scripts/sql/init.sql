CREATE DATABASE rust_postgres_server;

CREATE TABLE users (
   ID serial NOT NULL PRIMARY KEY,
   email VARCHAR NOT NULL,
   input JSONB NOT NULL
);
