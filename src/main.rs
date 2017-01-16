#![feature(custom_attribute)]
#![feature(custom_derive, plugin)]
#![feature(optin_builtin_traits)]
#![feature(proc_macro)]
#![feature(core_intrinsics)]

mod sql;
mod base;
mod user;
mod db;
mod cache;
mod product;
mod session;
mod echo;
mod image;
mod price;
mod appointment;
#[macro_use]
extern crate serde_derive;

extern crate iron;
extern crate staticfile;
extern crate router;

extern crate bodyparser;
extern crate persistent;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate r2d2_redis;
extern crate redis;

extern crate serde;
extern crate serde_json;
extern crate uuid;
extern crate futures;
use std::ops::Deref;
use futures::Future;
use redis::Commands;
use r2d2_redis::RedisConnectionManager;
use uuid::Uuid;
use serde::{Deserialize, Deserializer};
use std::default::Default;
use std::thread;
use std::sync::Arc;
use std::sync::RwLock;

use router::Router;
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use std::fs::File;
use std::env;
use persistent::{Read, Write};
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use iron::AfterMiddleware;
extern crate mount;
struct App {
    name: String,
    version: String,
}

impl App {
    pub fn new(name: String) -> App {
        let version: String = match env::var("APP_VERSION") {
            Ok(val) => val,
            Err(_) => "0.0.0".to_string(),
        };
        App {
            name: name,
            version: version,
        }
    }

    pub fn info(&self) -> String {
        return self.name.to_string() + " | " + &self.version;
    }
}

struct Location {
    latitude: f64,
    longitude: f64,
    timestamp: f64,
}

fn reset(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<db::AppDb>>().unwrap();
    let conn = pool.get().unwrap();
    conn.query(sql::SQL_RESET, &[]);
    Ok(Response::with((status::Ok, "DB-RESET")))

}
fn root(req: &mut Request, app: App) -> IronResult<Response> {
    match req.url.path()[0] {
        "health" => return health(req),
        "reset" => return reset(req),
        _ => return Ok(Response::with((status::Ok, app.info()))),
    }
}
fn health(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Alive and kickin it")))
}


fn get_router() -> fn(&mut Request) -> IronResult<Response> {
    fn router(req: &mut Request) -> IronResult<Response> {
        println!("{:?}", req.url.path());
        let app = App::new("!HairParty!".to_string());
        match base::url_fmt(req.method.as_ref(), req.url.path()[0]).as_str() {
            "post_product" => return product::put(req),
            "get_product" => return product::get(req),
            "get_products" => return product::get_all(req),
            "post_session" => return session::put(req),
            "get_session" => return session::get(req),
            "get_sessions" => return session::get_all(req),
            "post_appointment" => return appointment::put(req),
            "get_appointment" => return appointment::get(req),
            "get_appointments" => return appointment::get_all(req),
            "post_price" => return price::put(req),
            "get_price" => return price::get(req),
            "get_prices" => return price::get_all(req),
            "post_image" => return image::put(req),
            "get_image" => return image::get(req),
            "get_images" => return image::get_all(req),
            "post_user" => return user::put(req),
            "get_user" => return user::get(req),
            "get_users" => return user::get_all(req),
            "post_login" => return user::login(req),
            "echo" => return echo::echo(req),
            _ => return root(req, app),
        }
    }
    return router;
}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

struct CorsMiddleware;
impl AfterMiddleware for CorsMiddleware {
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
        let _ = res.headers.set_raw("access-control-allow-origin".to_string(),
                                    vec![b"*".to_vec()]);
        if req.method == iron::method::Method::Options {
            let _ = res.headers.set_raw("access-control-allow-headers".to_string(),
                                        vec![b"accept, content-type".to_vec()]);
            let _ = res.headers.set_raw("access-control-allow-methods".to_string(),
                                        vec![b"GET,POST,DELETE,OPTIONS,PATCH".to_vec()]);
        }
        res.headers.set(iron::headers::AccessControlAllowOrigin::Any);
        Ok(res)
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut port = String::from("");
    let mut conn_string = String::from("");
    if args.len() > 1 {
        port = args[1].as_str().to_string();
        conn_string = args[2].as_str().to_string();
    } else {
        panic!("{:?}", args);
    }
    println!("Connecting to postgres: {}", conn_string);
    let postgres_pool = db::setup_connection_pool(&conn_string, 6);
    let conn = postgres_pool.get().unwrap();
    println!("Connected to postgres");
    db::scaffold_db(&conn);
    let redis_pool = cache::setup_connection_pool("redis://localhost", 6);
    let mut assets_mount = Mount::new();
    assets_mount.mount("/", get_router())
        .mount("/assets/", Static::new(Path::new("../hairparty_web/")));
    let mut middleware = Chain::new(assets_mount);
    middleware.link(Read::<db::AppDb>::both(postgres_pool));
    middleware.link(Read::<cache::AppRedis>::both(redis_pool));
    middleware.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    middleware.link_after(CorsMiddleware);
    let listen = format!("0.0.0.0:{}", port);
    println!("{:?}", listen);
    Iron::new(middleware).http(listen.as_str()).unwrap();
}
