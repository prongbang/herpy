# herpy

![Stauts](https://github.com/prongbang/herpy/actions/workflows/rust.yml/badge.svg)

Herpy API Gateway write in Rust

## Configuration

```yaml
---
authorization_api_url: "http://auth-service/posts/1"
services:
  - path: "/users"
    target_service: "http://user-service/users"
    target_port: "80"
  - path: "/orders"
    target_service: "http://order-service/posts"
    target_port: "80"
```
