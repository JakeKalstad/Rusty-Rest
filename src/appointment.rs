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
pub struct Appointment {
    id: String,
    name: String,
    time: i64,
    confirmed: bool,
    user_id: String,
    vendor_id: String,
    product_id: String,
    price_id: String,
}

impl Appointment {
    pub fn new(id: String,
               name: String,
               time: i64,
               confirmed: bool,
               user_id: String,
               vendor_id: String,
               product_id: String,
               price_id: String)
               -> Appointment {
        Appointment {
            id: id,
            name: name,
            time: time,
            confirmed: confirmed,
            user_id: user_id,
            vendor_id: vendor_id,
            product_id: product_id,
            price_id: price_id,
        }
    }
}
pub fn put(req: &mut Request) -> IronResult<Response> {
    let appointment = req.get::<bodyparser::Struct<Appointment>>();
    let mut stat = status::Ok;
    let mut response = String::from("");
    match appointment {
        Ok(Some(appointment)) => {
            let pool = req.get::<Read<db::AppDb>>().unwrap();
            let conn = pool.get().unwrap();
            let isNew = appointment.id.len() == 0;
            let new_id = match isNew {
                true => Uuid::new_v4().to_string(),
                _ => appointment.id.to_owned(),
            };
            conn.execute(sql::SQL_UPSERTAPPOINTMENT,
                         &[&new_id,
                           &appointment.name,
                           &appointment.time,
                           &appointment.confirmed,
                           &appointment.user_id,
                           &appointment.vendor_id,
                           &appointment.product_id,
                           &appointment.price_id])
                .unwrap();
            let resp = Appointment::new(new_id,
                                        appointment.name,
                                        appointment.time,
                                        appointment.confirmed,
                                        appointment.user_id,
                                        appointment.vendor_id,
                                        appointment.product_id,
                                        appointment.price_id);
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
            for row in conn.query(sql::SQL_GETAPPOINTMENT, &[&id]).unwrap().iter() {
                let appointment = Appointment::new(row.get(0),
                                                   row.get(1),
                                                   row.get(2),
                                                   row.get(3),
                                                   row.get(4),
                                                   row.get(5),
                                                   row.get(6),
                                                   row.get(7));
                response = serde_json::to_string(&appointment).unwrap();
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
    let mut appointments = Vec::new();
    for row in &conn.query(sql::SQL_GETAPPOINTMENTS, &[]).expect("Shit the bed") {
        let appointment = Appointment::new(row.get(0),
                                           row.get(1),
                                           row.get(2),
                                           row.get(3),
                                           row.get(4),
                                           row.get(5),
                                           row.get(6),
                                           row.get(7));
        appointments.push(appointment);
    }
    Ok(Response::with((status::Ok, serde_json::to_string(&appointments).unwrap())))
}
