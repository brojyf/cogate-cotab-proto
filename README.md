# cogate-cotab-proto

The single source of truth for cogate's internal gRPC contracts.

## Layout

- `proto/` contains the authoritative protobuf sources.
- `gen/go/` contains committed Go clients and servers generated from those
  sources.
- The root Rust crate exposes the same contracts under
  `cogate_cotab_proto::{cogate,rusti2}`.

Services consume a tagged release. They do not copy `.proto` files or generated
stubs into their own repositories.

The repository is public so Go and Rust consumers can resolve tagged releases
over HTTPS without repository-specific credentials.

## Compatibility policy

Packages are versioned as `<service>.v1`. Within a package, fields are added
only: never rename, retype, delete, or reuse a field number. Breaking changes
require a new protobuf package version.

Every enum has an explicit `*_UNSPECIFIED = 0`. RPC authentication and request
deadlines remain the responsibility of each service transport.

## Development

Install `buf`, `protoc-gen-go`, and `protoc-gen-go-grpc`, then run:

```sh
make proto
make test
```

Commit generated Go changes with their source change. CI rejects lint errors,
wire-breaking edits, and generated-code drift.

## Releasing

After CI passes, create a semantic version tag such as `v0.2.0`. Consumers pin
that tag explicitly and upgrade in their own deployment cycle.

Go:

```sh
go get github.com/brojyf/cogate-cotab-proto@v0.1.0
```

Rust:

```toml
cogate-cotab-proto = { git = "https://github.com/brojyf/cogate-cotab-proto", tag = "v0.1.0" }
```
