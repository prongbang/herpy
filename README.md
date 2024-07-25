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

- Mac Studio 2022
- Chip Apple M1 Max
- Memory 32 GB

```shell
rewrk -h http://127.0.0.1:8080/hello -t 12 -c 100 -d 60s
```

### Comparisons

[Source](https://github.com/prongbang/herpy-bench)

| Name | Latency.Avg | Latency.Stdev | Latency.Min | Latency.Max | Request.Total | Request.Req/Sec | Transfer.Total | Transfer.Rate |
|----------------|---|---|---|---|---|---|---|---|
| Direct         |0.67ms|1.52ms|0.05ms|124.63ms|8890025|148165.69|1.08 GB|18.51 MB/Sec|
| **Herpy**          |**0.63ms**|**0.19ms**|**0.02ms**|**11.46ms**|**9528463**|**158806.60**|**826.92 MB**|**13.78 MB/Sec**|
| KrakenD        |1.11ms|0.92ms|0.02ms|48.48ms|5427454|90456.93|936.86 MB|15.61 MB/Sec|

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
