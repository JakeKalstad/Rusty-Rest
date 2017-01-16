var http = require('http');

var verbose = false;
if (process.argv.length >= 2) {
    verbose = process.argv[2] === "v" || process.argv[2] === "verbose";
}

function _get(host, path, callback) {
    return http.get({
        host: "127.0.0.1",
        port: "3000",
        path: "/" + path + "/",
    }, function(response) {
        var body = '';
        response.on('data', function(d) {
            body += d;
        });
        response.on('end', function() {
            callback({ 
                result: JSON.parse(body)
            });
        });
    });
};

function _post(host, path, data, callback) {
    var options = {
        host: "127.0.0.1",
        port: "3000",
        path: "/"+path+"/",
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        }
    };
    var postReq = http.request(options, function(res) {
        var body = '';
        res.on('data', function(d) {
            body += d;
        });
        res.on('end', function() {
            callback({ 
                result: JSON.parse(body)
            });
        });
    });
    postReq.write(JSON.stringify(data));
    postReq.end();
};

function client(path) {
    var ip = "localhost:3000";
    return {
        get: function(callback) {
            _get(ip, path, callback);
        },
        post: function(data, callback) {
            _post(ip, path, data, callback);
        }
    }
}

function result(pass, data, msg, raw) {
    this.pass = pass;
    this.data = data;
    this.msg = msg;
    this.raw = raw;
    this.format = function() {
        return (this.pass ? "Success ::" : "Fail    ::" ) +this.msg+ ":::"+
                (verbose ? ("data: " + JSON.stringify(this.data)) + 
                "raw: " + JSON.stringify(this.raw) : "");
    };
}

function testInsert(data, svc_name, cb) {
    var svc = client(svc_name);
    if (data.length > 0) {
        var results = [];
        for (var i=0;i<data.length;i++) {
            svc.post(data[i], function (res) {
                var test_result = new result(res.result.id.length > 0, res.result, "Put " + svc_name, res)
                results.push(test_result)
                if (results.length == data.length) {
                    cb({results:results})
                }
            })
        }
    } else {
        svc.post(data, function (res) {
            var test_result = new result(res.result.id.length > 0, res.result, "Put " + svc_name, res)
            cb(test_result)
        })
    }
}

function testGet(svc_name, key, cb) {
    var path = key.length == 0 ? svc_name+'s' : (svc_name + "/" + key )
    var svc = client(path);
    svc.get(function (res) {
        var pass = res.result.length > 0 || (res.result.id.length > 0 && res.result.id == key)
        var test_result = new result(pass, res.result, "Get " + svc_name, res)
        cb(test_result)
    })
} 



var user = {"id":"","name":"jake","lname":"jakeson","dob":"147324123","password":"super secret", "email":"jake@jake.com","phone":"1239567","active":1,"products":[]};
var product = {"id":"", "name":"MOUNTAIN DEWWWW", "price":"", "images":[]};
var products = [product, {"id":"", "name":"CRACK ROX", "price":"", "images":[]}];
var image = {"id":"", "name":"Super dope haircut", "url":"http://i.imgur.com/oOtZe.jpg"};
var price = {"id":"", "user_id":"xxx", "product_id":"zzzz", "currency_code":"USD", "price":34};
var appointment = {"id":"", "name":"xxx", "time":147324123, "confirmed":true, "user_id":"xxxx", "vendor_id":"xxxx", "product_id":"xxxx", "price_id":"xxxx"};
var session = {"id":"", "user_id":"1", "data":"{root:1}","created":""};

function T(res) { for (var i=0;i<res.length;i++) { console.log(res[i].format()); }} 
function TS(svc_name, data) { 
    return function(callback) {
        testInsert(data, svc_name, function (insertResult) {
            if (insertResult.results) {
                for (var i = 0; i < insertResult.results.length; i++) {
                    testGet(svc_name, insertResult.results[i].data.id, function (getResult) {
                        testGet(svc_name, "", function (getManyResult) {
                            if (i == insertResult.results.length) {
                                callback(insertResult.results.concat([getResult, getManyResult]))
                            }
                        });
                    });
                }
            } else {
                testGet(svc_name, insertResult.data.id, function (getResult) {
                    testGet(svc_name, "", function (getManyResult) {
                        callback([insertResult, getResult, getManyResult])
                    });
                });
            }
        });
    }
}

http.get({
        host: "127.0.0.1",
        port: "3000",
        path: "/reset",
    }, function(response) {
        TS("user", user)(T);
        TS("image", image)(T);
        TS("price", price)(T);
        TS("product", product)(T);
        TS("product", products)(T);
        TS("appointment", appointment)(T);
        TS("session", session)(T);
});

