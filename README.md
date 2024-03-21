# herpy

![Stauts](https://github.com/prongbang/herpy/actions/workflows/rust.yml/badge.svg)

Herpy API Gateway write in Rust

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/prongbang)

```shell
rewrk -h http://127.0.0.1:8080/hello -t 12 -c 100 -d 60s

Benchmarking 100 connections @ http://127.0.0.1:8080/hello for 1 minute(s)
  Latencies:
    Avg      Stdev    Min      Max
    2.44ms   0.96ms   0.17ms   29.52ms
  Requests:
    Total: 2459428 Req/Sec: 40990.92
  Transfer:
    Total: 213.44 MB Transfer Rate: 3.56 MB/Sec
```

## Configuration

- herpy.yaml

```yaml
port: 8080
authorization:
  authorize_token:
    host: "https://httpbin.org"
    path: "/post"
    method: POST
services:
  - endpoint: "/users"
    method: POST
    backends:
      - host: "https://jsonplaceholder.typicode.com"
        path: "/users"
        method: GET
        authorization: authorize_token
  - endpoint: "/posts"
    method: POST
    backends:
      - host: "https://httpbin.org"
        path: "/post"
        method: POST
  - endpoint: "/hello"
    method: POST
    backends:
      - host: "http://localhost:8000"
        path: "/v1/hello"
        method: POST
```
