# herpy

![Stauts](https://github.com/prongbang/herpy/actions/workflows/rust.yml/badge.svg)

Herpy API Gateway write in Rust

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/prongbang)

## Install

- Install with Homebrew

```shell
brew update
brew tap prongbang/homebrew-formulae
brew install herpy
```

- Install with Cargo

```shell
cargo install herpy --git https://github.com/prongbang/herpy.git
```

- Install with Docker

```shell
docker pull prongbang/herpy:latest
```

## Benchmark

- MacBook Pro (14-inch, 2021)
- Chip Apple M1 Pro
- Memory 16 GB

```shell
rewrk -h http://127.0.0.1:8080/hello -t 12 -c 100 -d 60s
```

### Comparisons

[Source](https://github.com/prongbang/herpy-bench)

| Name | Latency.Avg | Latency.Stdev | Latency.Min | Latency.Max | Request.Total | Request.Req/Sec | Transfer.Total | Transfer.Rate |
|----------------|---|---|---|---|---|---|---|---|
| Direct         |1.96ms|1.51ms|0.03ms|39.83ms|3053635|50894.22|381.49 MB|6.36 MB/Sec|
| **Herpy**          |**3.03ms**|**1.88ms**|**0.11ms**|**33.97ms**|**1978253**|**32971.05**|**186.77 MB**|**3.11 MB/Sec**|
| KrakenD        |3.90ms|1.62ms|0.06ms|65.20ms|1539334|25656.21|344.99 MB|5.75 MB/Sec|

## Configuration

- herpy.yaml

```yaml
version: "1"
metadata:
  port: 8080
services:
  - endpoint: "/users"
    method: POST
    backends:
      - host: "https://jsonplaceholder.typicode.com"
        path: "/users"
        method: GET
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
      - host: "http://localhost:8000"
        path: "/v1/hello"
        method: POST
```

## Run

- Native

```shell
herpy -c herpy.yaml
```

- Docker

```shell
docker run \
    -p 8080:8080 \
    -v "./herpy.yaml:/etc/herpy/herpy.yaml" \
    --name herpy-api-gateway \
    prongbang/herpy:latest
```

- Listen

```shell
2024-03-23T16:20:32.967967Z  INFO herpy::server: starting server on '0.0.0.0:8080' listen=0.0.0.0:8080
```
