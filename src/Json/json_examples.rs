use actix_web::web::Data;
use actix_web::{web::Json, HttpResponse, Responder};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonInput {
    pub email: String,
    pub input: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub payload: Vec<JsonInput>,
}

/**
    Reading the valid json from request body
    email is of type string
    input is non-structure valid json value
*/
pub async fn json_body(body: Json<JsonInput>) -> impl Responder {
    let json_input = to_value(body.0.input).unwrap();
    HttpResponse::Ok().json(JsonInput {
        email: body.0.email,
        input: json_input,
    })
}

/**
    Reading the valid json from request body
    email is of type string
    input is non-structure valid json value
    inserting the json value into database as JSONB
*/
pub async fn insert_json_body(body: Json<JsonInput>, db: Data<Pool>) -> impl Responder {
    let json_input = to_value(body.0.input).unwrap();
    // get database pool
    let pool = db.get().await.unwrap();
    pool.query(
        "INSERT INTO json_table (email, input) VALUES ($1, $2)",
        &[&body.0.email, &json_input],
    )
    .await
    .unwrap();
    HttpResponse::Ok().json(JsonInput {
        email: body.0.email,
        input: json_input,
    })
}

/**
    Getting
    Email of type string from postgres
    Input of type JSONB from postgres
*/
pub async fn read_json_from_db(db: Data<Pool>) -> impl Responder {
    // get database pool
    let pool = db.get().await.unwrap();
    let rows = pool
        .query("SELECT email, input from json_table", &[])
        .await
        .unwrap();

    // as input is of Type JSONB, we need to convert that into json
    let mut response: Vec<JsonInput> = vec![];
    for row in rows {
        let json_input = JsonInput {
            email: row.get("email"),
            input: from_value(row.get("input")).unwrap(),
        };
        response.push(json_input);
    }
    HttpResponse::Ok().json(Response { payload: response })
}
