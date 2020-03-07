CREATE DATABASE rust_postgres_server;

CREATE TABLE json_table (
   ID serial NOT NULL PRIMARY KEY,
   email VARCHAR NOT NULL,
   input JSONB NOT NULL
);

CREATE TABLE users (
   id serial PRIMARY KEY,
   username VARCHAR UNIQUE NOT NULL,
   password VARCHAR  NOT NULL,
   email VARCHAR UNIQUE NOT NULL,
   created_on TIMESTAMP DEFAULT Now() NOT NULL
);

