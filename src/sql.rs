
pub static SQL_USERTABLE: &'static str = "
CREATE TABLE IF NOT EXISTS users (
    user_id varchar PRIMARY KEY,
    name varchar NOT NULL,
    lname varchar NOT NULL,
    dob bigint NOT NULL,
    password varchar NOT NULL,
    email varchar NOT NULL,
    phone varchar NOT NULL,
    active integer NOT NULL DEFAULT '1',
    created bigint NULL DEFAULT extract(epoch from now()),
    updated bigint NULL
);
";
pub static SQL_UPSERTUSER: &'static str =
    "
INSERT INTO users (user_id, name, lname, dob, password, email, phone, active) 
VALUES ($1, \
     $2, $3, cast($4 as bigint), crypt($5, gen_salt('md5')), $6, $7, $8) 
ON CONFLICT (user_id) DO \
     UPDATE SET name=$2,lname=$3,dob=cast($4 as bigint), password=crypt($5, gen_salt('md5')), \
     email=$6, phone=$7, active=$8, updated=extract(epoch from now());
";
pub static SQL_GETUSER: &'static str = "SELECT u.user_id, name, lname, dob, email, phone, active, \
                                        string_agg(up.product_id, ',') as products from users u \
                                        left join user_to_product up on up.user_id=u.user_id \
                                        where u.user_id=$1 group by u.user_id;";
pub static SQL_GETUSERS: &'static str = "SELECT user_id, name, lname, dob, email, phone, active \
                                         from users;";
pub static SQL_USERAUTH: &'static str = "SELECT user_id, name, lname, dob, email, phone, active \
                                         from users where password=crypt($1, password) AND \
                                         email=$2;";

pub static SQL_IMAGETABLE: &'static str = "
CREATE TABLE IF NOT EXISTS images (
    id varchar PRIMARY KEY,
    name varchar NOT NULL,
    url varchar NOT NULL,
    created bigint NOT NULL DEFAULT extract(epoch from now()),
    updated bigint NULL
);
";
pub static SQL_UPSERTIMAGE: &'static str = "
INSERT INTO images (id, name, url) 
VALUES ($1, $2, $3) 
ON CONFLICT (id) DO UPDATE SET name=$2,url=$3,updated=extract(epoch from now());
";
pub static SQL_GETIMAGE: &'static str = "SELECT id, name, url from images where id =$1;";
pub static SQL_GETIMAGES: &'static str = "SELECT id, name, url from images;";

pub static SQL_IMAGE_PRODUCTTABLE: &'static str = "
CREATE TABLE IF NOT EXISTS image_product (
    image_id varchar PRIMARY KEY,
    product_id varchar NOT NULL,
    created bigint NOT NULL DEFAULT extract(epoch from now()),
    updated bigint NULL
);
";
pub static SQL_UPSERTIMAGE_PRODUCT: &'static str = "
INSERT INTO image_product (image_id, product_id) 
VALUES ($1, $2) 
ON CONFLICT (image_id) DO UPDATE SET image_id=$1,product_id=$2,updated=extract(epoch from now());
";

pub static SQL_PRODUCTTABLE: &'static str = "
CREATE TABLE IF NOT EXISTS products (
    id varchar PRIMARY KEY,
    name varchar NOT NULL,
    price_id varchar NOT NULL,
    created bigint NOT NULL DEFAULT extract(epoch from now()),
    updated bigint NULL
);
";

pub static SQL_UPSERTPRODUCT: &'static str = "
INSERT INTO products (id, name, price_id) 
VALUES ($1, $2, $3) 
ON CONFLICT (id) DO UPDATE SET name=$2,price_id=$3,updated=extract(epoch from now());
";
pub static SQL_GETPRODUCT: &'static str =
    "SELECT p.id, name, price_id, COALESCE(string_agg(ip.image_id, ','), '') as images from \
     products p left join image_product ip ON ip.product_id = p.id where p.id =$1 group by p.id;";
pub static SQL_GETPRODUCTS: &'static str =
    "SELECT p.id, name, price_id, string_agg(ip.image_id, ',') as images from products p left \
     join image_product ip ON ip.product_id = p.id group by p.id;";

pub static SQL_PRICETABLE: &'static str = "
CREATE TABLE IF NOT EXISTS prices (
    id varchar PRIMARY KEY,
    user_id varchar NOT NULL,
    product_id varchar NOT NULL,
    currency_code varchar NOT NULL,
    price integer NOT NULL,
    created bigint NOT NULL DEFAULT extract(epoch from now()),
    updated bigint NULL
);
";
pub static SQL_UPSERTPRICE: &'static str =
    "
INSERT INTO prices (id, user_id, product_id, currency_code, price) 
VALUES ($1, $2, $3, $4, \
     $5) 
ON CONFLICT (id) DO UPDATE SET \
     user_id=$2,product_id=$3,currency_code=$4,price=$5,updated=extract(epoch from now());
";

pub static SQL_GETPRICE: &'static str = "SELECT id, user_id, product_id, currency_code, price \
                                         from prices where id =$1;";
pub static SQL_GETPRICES: &'static str = "SELECT id, user_id, product_id, currency_code, price \
                                          from prices;";

pub static SQL_APPOINTMENTTABLE: &'static str = "
CREATE TABLE IF NOT EXISTS appointments (
    id varchar PRIMARY KEY,
    name varchar NOT NULL,
    time bigint NOT NULL,
    confirmed bool NOT NULL,
    user_id varchar NOT NULL,
    vendor_id varchar NOT NULL,
    product_id varchar NOT NULL,
    price_id varchar NOT NULL,
    created bigint NOT NULL DEFAULT extract(epoch from now()),
    updated bigint NULL
);
";
pub static SQL_UPSERTAPPOINTMENT: &'static str =
    "
INSERT INTO appointments (id, name, time, confirmed, user_id, vendor_id, product_id, \
     price_id) 
VALUES ($1, $2, $3, $4, $5, $6, $7, $8) 
ON CONFLICT (id) DO UPDATE SET \
     name=$2,time=$3,confirmed=$4,user_id=$5,vendor_id=$6,product_id=$7,price_id=$8, \
     updated=extract(epoch from now());
";

pub static SQL_GETAPPOINTMENT: &'static str = "SELECT id, name, time, confirmed, user_id, \
                                               vendor_id, product_id, price_id from appointments \
                                               where id =$1;";
pub static SQL_GETAPPOINTMENTS: &'static str = "SELECT id, name, time, confirmed, user_id, \
                                                vendor_id, product_id, price_id from appointments;";

pub static SQL_SESSIONTABLE: &'static str = "
CREATE TABLE IF NOT EXISTS sessions (
    id varchar  PRIMARY KEY,
    user_id     varchar NOT NULL,
    data        varchar,
    created     bigint NOT NULL DEFAULT extract(epoch from now())
);
";
pub static SQL_UPSERTSESSION: &'static str = "
INSERT INTO sessions (id, user_id, data) 
VALUES ($1, $2, $3) 
ON CONFLICT (id) DO UPDATE SET user_id=$2, data=$3;
";
pub static SQL_GETSESSION: &'static str = "SELECT id, user_id, data, (cast (created as varchar)) \
                                           from sessions where id =$1;";
pub static SQL_GETSESSIONS: &'static str = "SELECT id, user_id, data, (cast (created as varchar)) \
                                            from sessions;";

pub static SQL_USER_PRODUCTTABLE: &'static str = "
CREATE TABLE IF NOT EXISTS user_to_product (
    id varchar  PRIMARY KEY,
    user_id varchar NOT NULL,
    product_id varchar NOT NULL,
    created bigint NOT NULL DEFAULT extract(epoch from now()),
    updated bigint NULL
);
";

pub static SQL_UPSERT_USER_PRODUCT: &'static str = "
INSERT INTO user_to_product (id, user_id, product_id) 
VALUES ($1, $2, $3) 
ON CONFLICT (id) DO UPDATE SET user_id=$2, product_id=$3, updated=extract(epoch from now());
";

pub static SQL_RESET: &'static str = "
TRUNCATE appointments, users, images, prices, products;
";
