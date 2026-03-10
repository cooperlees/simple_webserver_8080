# simple_webserver_8080

A minimal Rust HTTP/1.1 webserver that listens on `[::]:8080`, accepting connections
on both IPv4 and IPv6.

## Features

- Dual-stack (IPv4 + IPv6) via a single `[::]:8080` binding
- Zero external dependencies – uses Rust's standard library only
- Small optimised release binary (`opt-level = "z"`, LTO, stripped)

## Building locally

```bash
cargo build --release
./target/release/simple_webserver_8080
# Listening on http://[::]:8080/ (IPv4 + IPv6)
```

Test with curl:

```bash
curl http://127.0.0.1:8080/   # IPv4
curl http://[::1]:8080/        # IPv6
```

## Docker

Two Docker images are provided:

| File | Base image | Notes |
|------|-----------|-------|
| `Dockerfile.alpine` | `alpine:latest` | Small image with a musl-based runtime |
| `Dockerfile.distroless` | `gcr.io/distroless/static:nonroot` | No shell, minimal attack surface |

Both use a multi-stage build: `rust:alpine` compiles a statically-linked musl binary
which is then copied into the slim runtime image.

### Build Alpine image

```bash
docker build -f Dockerfile.alpine -t simple_webserver_8080:alpine .
docker run --rm -p 8080:8080 simple_webserver_8080:alpine
```

### Build Distroless image

```bash
docker build -f Dockerfile.distroless -t simple_webserver_8080:distroless .
docker run --rm -p 8080:8080 simple_webserver_8080:distroless
```

### Multi-arch build (amd64 + arm64)

```bash
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -f Dockerfile.alpine \
  -t simple_webserver_8080:alpine .

docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -f Dockerfile.distroless \
  -t simple_webserver_8080:distroless .
```

## CI – GitHub Actions

The workflow in [`.github/workflows/docker-build.yml`](.github/workflows/docker-build.yml)
runs on every push and pull-request to `main`. It builds all four combinations:

| Image | Platform |
|-------|----------|
| alpine | `linux/amd64` (x86_64) |
| alpine | `linux/arm64` (aarch64) |
| distroless | `linux/amd64` (x86_64) |
| distroless | `linux/arm64` (aarch64) |

QEMU is used for cross-architecture emulation so all four jobs run on standard
`ubuntu-latest` GitHub-hosted runners.
