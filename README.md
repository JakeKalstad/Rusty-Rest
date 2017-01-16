# USE MORE RUST
  An implementation of a restful crud service in rust as a POC.
![](http://i.imgur.com/nZBHDNp.gif)
### Tech
* [Postgressql]
* [Redis] 

(cargo.toml is looking for r2d2-redis locally, snag a copy from -> https://github.com/sorccu/r2d2-redis)

# API 
### cargo run --verbose -- 3000 postgres://postgres:toor@127.0.0.1:5432/data 
## user
  User object represents both users and vendors
```
curl -XPOST 127.0.0.1:3000/user -H "application/json" -d '{"id":"","name":"jake","password":"super secret", "email":"jake@jake.com","phone":"1239567","active":1,"products":[]}'

curl 127.0.0.1:3000/user/key-goes-here

curl 127.0.0.1:3000/users
```

## product 
  Product object represents a ... product
```
curl -XPOST 127.0.0.1:3000/product -H "application/json" -d '{"id":"", "name":"MOUNTAIN DEWWWW", "price":"", "images":[]}'

curl 127.0.0.1:3000/product/key-goes-here

curl 127.0.0.1:3000/products
```

## image 
  Image objects for users and products
```
curl -XPOST 127.0.0.1:3000/image -H "application/json" -d '{"id":"", "name":"Super dope haircut", "url":"http://i.imgur.com/oOtZe.jpg"}'

curl 127.0.0.1:3000/image/key-goes-here

curl 127.0.0.1:3000/images
```

## price 
  Price for products
```
curl -XPOST 127.0.0.1:3000/price -H "application/json" -d '{"id":"", "user_id":"xxx", "product_id":"zzzz", "currency_code":"USD", "price":34}'

curl 127.0.0.1:3000/price/key-goes-here

curl 127.0.0.1:3000/prices
```

## appointment 
  Appointment to exchance goods/services
```
curl -XPOST 127.0.0.1:3000/appointment -H "application/json" -d '{"id":"", "name":"xxx", "time":147324123, "confirmed":true, "user_id":"xxxx", "vendor_id":"xxxx", "product_id":"xxxx", "price_id":"xxxx"}'

curl 127.0.0.1:3000/appointment/key-goes-here

curl 127.0.0.1:3000/appointments
``` 
