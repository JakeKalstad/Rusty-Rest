use sql;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
use iron::typemap::Key;

use std;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;

pub type PostgresPool = Pool<PostgresConnectionManager>;
pub type PostgresPooledConnection = PooledConnection<PostgresConnectionManager>;

pub struct AppDb;
impl Key for AppDb {
    type Value = PostgresPool;
}
fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}
pub fn setup_connection_pool(cn_str: &str, pool_size: u32) -> PostgresPool {
    let manager = ::r2d2_postgres::PostgresConnectionManager::new(cn_str,
                                                                  ::r2d2_postgres::TlsMode::None)
        .unwrap();
    let config = ::r2d2::Config::builder().pool_size(pool_size).build();
    let pool = ::r2d2::Pool::new(config, manager).unwrap();
    let conn = pool.get().unwrap();
    return pool;
}

pub fn scaffold_db(conn: &PostgresPooledConnection) {
    conn.execute(sql::SQL_USERTABLE, &[]).unwrap();
    conn.execute(sql::SQL_IMAGETABLE, &[]).unwrap();
    conn.execute(sql::SQL_IMAGE_PRODUCTTABLE, &[]).unwrap();
    conn.execute(sql::SQL_PRODUCTTABLE, &[]).unwrap();
    conn.execute(sql::SQL_PRICETABLE, &[]).unwrap();
    conn.execute(sql::SQL_APPOINTMENTTABLE, &[]).unwrap();
    conn.execute(sql::SQL_SESSIONTABLE, &[]).unwrap();
    conn.execute(sql::SQL_USER_PRODUCTTABLE, &[]).unwrap();
}
