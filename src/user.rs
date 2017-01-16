
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
struct Login {
    password: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: String,
    name: String,
    lname: String,
    dob: i64,
    password: String,
    email: String,
    phone: String,
    active: i32,
    products: Vec<String>,
}
impl User {
    pub fn new(user_id: String,
               name: String,
               lname: String,
               dob: i64,
               email: String,
               phone: String,
               active: i32,
               products: Vec<String>)
               -> User {
        User {
            id: user_id,
            name: name,
            lname: lname,
            dob: dob,
            password: String::from(""),
            email: email,
            phone: phone,
            active: active,
            products: products,
        }
    }
}


pub fn put(req: &mut Request) -> IronResult<Response> {
    let user = req.get::<bodyparser::Struct<User>>();
    let mut stat = status::Ok;
    let mut response = String::from("");
    match user {
        Ok(Some(user)) => {
            let pool = req.get::<Read<db::AppDb>>().unwrap();
            let conn = pool.get().unwrap();
            let isNew = user.id.len() == 0;
            let new_id = match isNew {
                true => Uuid::new_v4().to_string(),
                _ => user.id.to_owned(),
            };
            conn.execute(sql::SQL_UPSERTUSER,
                         &[&new_id,
                           &user.name,
                           &user.lname,
                           &user.dob,
                           &user.password,
                           &user.email,
                           &user.phone,
                           &user.active])
                .unwrap();
            let mut prodIds = user.products.to_owned();
            for prodId in prodIds {
                let new_lu_id = Uuid::new_v4().to_string();
                conn.execute(sql::SQL_UPSERT_USER_PRODUCT,
                             &[&new_lu_id, &new_id, &prodId])
                    .unwrap();
            }
            let resp = User::new(new_id,
                                 user.name,
                                 user.lname,
                                 user.dob,
                                 user.email,
                                 user.phone,
                                 user.active,
                                 user.products);
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
    println!("{:?}", req.url.path());
    match path.len() >= 2 {
        true => {
            let id = req.url.path()[1];
            let conn = pool.get().unwrap();
            for row in conn.query(sql::SQL_GETUSER, &[&id]).unwrap().iter() {
                let mut prodResArray: String = row.get(7);
                let mut productKeys = prodResArray.split(",").map(|s| s.to_string()).collect();
                let user = User::new(row.get(0),
                                     row.get(1),
                                     row.get(2),
                                     row.get(3),
                                     row.get(4),
                                     row.get(5),
                                     row.get(6),
                                     productKeys);
                response = serde_json::to_string(&user).unwrap();
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
    let mut users = Vec::new();
    for row in &conn.query(sql::SQL_GETUSERS, &[]).expect("Shit the bed") {
        let mut products = Vec::new();
        products.push("".to_string());
        let user = User::new(row.get(0),
                             row.get(1),
                             row.get(2),
                             row.get(3),
                             row.get(4),
                             row.get(5),
                             row.get(6),
                             products);
        users.push(user);
    }
    Ok(Response::with((status::Ok, serde_json::to_string(&users).unwrap())))
}

pub fn login(req: &mut Request) -> IronResult<Response> {
    let loginData = req.get::<bodyparser::Struct<Login>>();
    let mut stat = status::Ok;
    let mut response = String::from("");
    match loginData {
        Ok(Some(loginData)) => {
            let pool = req.get::<Read<db::AppDb>>().unwrap();
            let conn = pool.get().unwrap();
            stat = status::Unauthorized;
            for row in &conn.query(sql::SQL_USERAUTH, &[&loginData.password, &loginData.email])
                .expect("Shit the bed") {
                let mut products = Vec::new();
                products.push("".to_string());
                let user = User::new(row.get(0),
                                     row.get(1),
                                     row.get(2),
                                     row.get(3),
                                     row.get(4),
                                     row.get(5),
                                     row.get(6),
                                     products);
                stat = status::Ok;
                response = serde_json::to_string(&user).unwrap();
            }
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
