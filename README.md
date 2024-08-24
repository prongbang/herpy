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

Here's the updated table with the total values moved to the second column:

| **Test Scenario**        | **Total Requests** | **Requests/sec** | **Transfer/sec** | **Total Transfer** | **Avg Latency** | **Stdev Latency** | **Max Latency** | **Req/Sec Stdev** |
|--------------------------|--------------------|------------------|------------------|--------------------|-----------------|------------------|-----------------|------------------|
| **Without API Gateway**   | 9,403,288          | 156,474.47       | 19.55 MB         | 1.15 GB            | 5.51 ms         | 24.68 ms          | 395.91 ms       | 8.87k            |
| **Herpy API Gateway**     | 4,916,822          | 81,811.14        | 10.22 MB         | 614.27 MB          | 2.14 ms         | 8.55 ms           | 272.67 ms       | 1.14k            |
| **Zolly API Gateway**     | 4,545,014          | 75,621.61        | 9.45 MB          | 567.81 MB          | 2.44 ms         | 8.68 ms           | 253.62 ms       | 1.44k            |
| **KrakenD API Gateway**   | 2,647,991          | 44,109.90        | 9.89 MB          | 593.45 MB          | 3.03 ms         | 5.69 ms           | 111.44 ms       | 1.00k            |
| **Nginx API Gateway**     | 1,200,598          | 19,978.23        | 5.91 MB          | 355.27 MB          | 8.87 ms         | 76.93 ms          | 1.25 s          | 1.93k            |

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
