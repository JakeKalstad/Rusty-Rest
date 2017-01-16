extern crate iron;

extern crate bodyparser;
extern crate persistent;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde;
extern crate serde_json;
extern crate uuid;

use uuid::Uuid;
use serde::{Deserialize, Deserializer};

use sql;
use product;
use db;

use persistent::Read;
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Price {
    id: String,
    user_id: String,
    product_id: String,
    currency_code: String,
    price: i32,
}
impl Price {
    pub fn new(id: String,
               user_id: String,
               product_id: String,
               currency_code: String,
               price: i32)
               -> Price {
        Price {
            id: id,
            user_id: user_id,
            product_id: product_id,
            currency_code: currency_code,
            price: price,
        }
    }
}

pub fn put(req: &mut Request) -> IronResult<Response> {
    let price = req.get::<bodyparser::Struct<Price>>();
    let mut stat = status::Ok;
    let mut response = String::from("");
    match price {
        Ok(Some(price)) => {
            let pool = req.get::<Read<db::AppDb>>().unwrap();
            let conn = pool.get().unwrap();
            let isNew = price.id.len() == 0;
            let new_id = match isNew {
                true => Uuid::new_v4().to_string(),
                _ => price.id.to_owned(),
            };
            conn.execute(sql::SQL_UPSERTPRICE,
                         &[&new_id,
                           &price.user_id,
                           &price.product_id,
                           &price.currency_code,
                           &price.price])
                .unwrap();
            let resp = Price::new(new_id,
                                  price.user_id,
                                  price.product_id,
                                  price.currency_code,
                                  price.price);
            response = serde_json::to_string(&resp).unwrap();
            // upsert images
        }
        Ok(None) => {
            stat = status::BadRequest;
        }
        Err(err) => {
            println!("{:?}", err);
            stat = status::InternalServerError;
        }
    }
    Ok(Response::with((stat, response)))
}

pub fn get(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<db::AppDb>>().unwrap();
    let path = req.url.path();
    let mut stat = status::Ok;
    let mut response = String::from("");
    match path.len() >= 2 {
        true => {
            let id = req.url.path()[1];
            let conn = pool.get().unwrap();
            for row in conn.query(sql::SQL_GETPRICE, &[&id]).unwrap().iter() {
                let price = Price::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4));
                response = serde_json::to_string(&price).unwrap();
            }
        }
        _ => {
            stat = status::BadRequest;
            response = String::from("No key found")
        }
    }
    Ok(Response::with((stat, response)))
}

pub fn get_all(req: &mut Request) -> IronResult<Response> {
    let conn = req.get::<Read<db::AppDb>>().unwrap().get().unwrap();
    let path = req.url.path();
    let mut prices = Vec::new();
    for row in &conn.query(sql::SQL_GETPRICES, &[]).expect("Shit the bed") {
        let price = Price::new(row.get(0), row.get(1), row.get(2), row.get(3), row.get(4));
        prices.push(price);
    }
    Ok(Response::with((status::Ok, serde_json::to_string(&prices).unwrap())))
}
