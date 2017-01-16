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
struct Image {
    id: String,
    name: String,
    url: String,
}

impl Image {
    pub fn new(id: String, name: String, url: String) -> Image {
        Image {
            id: id,
            name: name,
            url: url,
        }
    }
}
pub fn put(req: &mut Request) -> IronResult<Response> {
    let image = req.get::<bodyparser::Struct<Image>>();
    let mut stat = status::Ok;
    let mut response = String::from("");
    match image {
        Ok(Some(image)) => {
            let pool = req.get::<Read<db::AppDb>>().unwrap();
            let conn = pool.get().unwrap();
            let isNew = image.id.len() == 0;
            let new_id = match isNew {
                true => Uuid::new_v4().to_string(),
                _ => image.id.to_owned(),
            };
            conn.execute(sql::SQL_UPSERTIMAGE, &[&new_id, &image.name, &image.url])
                .unwrap();
            let resp = Image::new(new_id, image.name, image.url);
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
            for row in conn.query(sql::SQL_GETIMAGE, &[&id]).unwrap().iter() {
                let image = Image::new(row.get(0), row.get(1), row.get(2));
                response = serde_json::to_string(&image).unwrap();
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
    let mut images = Vec::new();
    for row in &conn.query(sql::SQL_GETIMAGES, &[]).expect("Shit the bed") {
        let image = Image::new(row.get(0), row.get(1), row.get(2));
        images.push(image);
    }
    Ok(Response::with((status::Ok, serde_json::to_string(&images).unwrap())))
}
