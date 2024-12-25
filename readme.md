# Simple http server in rust

This is a simple http server which return a "hello world" on a given route.
The route can be defined with env var `HTTP_ROUTE_PREFIX`.

For example, you could set `HTTP_ROUTE_PREFIX=/simplehttp`, and because the server is listening on `0.0.0.0:3000`,
you would be able to visit <http://localhost:3000/simplehttp> and get a "hello world" back.

## Build docker image for x64

See the `docker-build.yaml` github workflow.

## Build docker image for arm64

See the `build_arm64.bash` script.

## Run with docker

```bash
docker run --name simplehttp-rs --rm -p 3000:3000 kevinmidtown/simplehttp-rs:arm64
```

## Usage docker images

* for x64: ghcr.io/kindlychung/simplehttp-rs:latest
* for arm64: kevinmidtown/simplehttp-rs:arm64
