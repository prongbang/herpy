# herpy

![Stauts](https://github.com/prongbang/herpy/actions/workflows/rust.yml/badge.svg)

Herpy API Gateway write in Rust

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/prongbang)

## Configuration

```yaml
---
---
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
