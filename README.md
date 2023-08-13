# herpy

![Stauts](https://github.com/prongbang/herpy/actions/workflows/rust.yml/badge.svg)

Herpy API Gateway write in Rust

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/prongbang)

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
