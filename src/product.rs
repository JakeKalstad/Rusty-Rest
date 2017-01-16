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
pub struct Product {
    id: String,
    name: String,
    images: Vec<String>,
    price: String,
}
impl Product {
    pub fn new(id: String, name: String, price: String, images: Vec<String>) -> Product {
        Product {
            id: id,
            name: name,
            images: images,
            price: price,
        }
    }
}
pub fn put(req: &mut Request) -> IronResult<Response> {
    let product = req.get::<bodyparser::Struct<Product>>();
    let mut stat = status::Ok;
    let mut response = String::from("");
    match product {
        Ok(Some(product)) => {
            let pool = req.get::<Read<db::AppDb>>().unwrap();
            let conn = pool.get().unwrap();
            let isNew = product.id.len() == 0;
            let new_id = match isNew {
                true => Uuid::new_v4().to_string(),
                _ => product.id.to_owned(),
            };
            conn.execute(sql::SQL_UPSERTPRODUCT,
                         &[&new_id, &product.name, &product.price])
                .unwrap();
            let mut imgIds = product.images.to_owned();
            for imgId in imgIds {
                conn.execute(sql::SQL_UPSERTIMAGE_PRODUCT, &[&imgId, &new_id])
                    .unwrap();
            }
            let resp = Product::new(new_id, product.name, product.price, product.images);
            response = serde_json::to_string(&resp).unwrap();
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
            for row in conn.query(sql::SQL_GETPRODUCT, &[&id]).unwrap().iter() {
                let mut prodResArray: String = row.get(3);
                let mut productKeys = prodResArray.split(",").map(|s| s.to_string()).collect();
                let product = Product::new(row.get(0), row.get(1), row.get(2), productKeys);
                response = serde_json::to_string(&product).unwrap();
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
    let mut products = Vec::new();
    for row in &conn.query(sql::SQL_GETPRODUCTS, &[]).expect("Shit the bed") {
        let mut prodResArray: String = row.get(3);
        let mut productKeys = prodResArray.split(",").map(|s| s.to_string()).collect();
        let product = Product::new(row.get(0), row.get(1), row.get(2), productKeys);
        products.push(product);
    }
    Ok(Response::with((status::Ok, serde_json::to_string(&products).unwrap())))
}
