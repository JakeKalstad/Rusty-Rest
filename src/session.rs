extern crate iron;

extern crate bodyparser;
extern crate persistent;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate serde;
extern crate serde_json;
extern crate uuid;
extern crate r2d2_redis;
extern crate redis;

use r2d2_redis::RedisConnectionManager;

use redis::Commands;

use std;
use std::ops::Deref;

use uuid::Uuid;
use serde::{Deserialize, Deserializer};

use sql;
use product;
use db;

use cache;

use persistent::Read;
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use std::default::Default;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    id: String,
    user_id: String,
    data: String,
    created: String,
}
impl Session {
    pub fn new(id: String, user_id: String, data: String, created: String) -> Session {
        Session {
            id: id,
            user_id: user_id,
            data: data,
            created: created,
        }
    }
}

pub fn put(req: &mut Request) -> IronResult<Response> {
    let session = req.get::<bodyparser::Struct<Session>>();
    let mut stat = status::Ok;
    let mut response = String::from("");
    match session {
        Ok(Some(session)) => {
            let pool = req.get::<Read<db::AppDb>>().unwrap();
            let redis = req.get::<Read<cache::AppRedis>>().unwrap();
            let rConn = redis.get().unwrap();
            let conn = pool.get().unwrap();
            let isNew = session.id.len() == 0;
            let new_id = match isNew {
                true => Uuid::new_v4().to_string(),
                _ => session.id.to_owned(),
            };
            conn.execute(sql::SQL_UPSERTSESSION,
                         &[&new_id, &session.user_id, &session.data])
                .unwrap();
            let resp = Session::new(new_id, session.user_id, session.data, session.created);
            response = serde_json::to_string(&resp).unwrap();

            let _: std::result::Result<String, redis::RedisError> =
                rConn.set(resp.id.as_str(), response.as_str());
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
    let redis = req.get::<Read<cache::AppRedis>>().unwrap();
    let path = req.url.path();
    let mut stat = status::Ok;
    let mut response = String::from("");
    match path.len() >= 2 {
        true => {
            let id = req.url.path()[1];
            let conn = pool.get().unwrap();
            let rConn = redis.get().unwrap();
            let r: std::result::Result<String, redis::RedisError> = rConn.get(id);
            match r {
                Ok(value) => response = value,
                Err(_) => {
                    stat = status::BadRequest;
                    response = String::from("Redis error retrieving session")
                }
            }
            // for row in conn.query(sql::SQL_GETSESSION, &[&id]).unwrap().iter() {
            //     let session = Session::new(row.get(0), row.get(1), row.get(2), row.get(3));
            //     response = serde_json::to_string(&session).unwrap();
            // }
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
    let mut sessions = Vec::new();
    for row in &conn.query(sql::SQL_GETSESSIONS, &[]).expect("Shit the bed") {
        let session = Session::new(row.get(0), row.get(1), row.get(2), row.get(3));
        sessions.push(session);
    }
    Ok(Response::with((status::Ok, serde_json::to_string(&sessions).unwrap())))
}
