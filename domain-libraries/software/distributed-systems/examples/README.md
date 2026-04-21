# Examples

This directory contains small example models for the distributed-systems library.

- `webshop/` is intended to be structurally valid for the current library skeleton.
- `grpc-orders/` shows how a gRPC and protobuf-oriented service can use overlays without polluting the core library.
- `missing-deployment/` is intentionally incomplete and is meant to trigger a future `DS301`-style deployment diagnostic once domain-rule execution exists.

These examples are deliberately simple. Their main purpose is to give the library a concrete starting point for parser checks, editor navigation, and future validation tests.

The `webshop/` example demonstrates HTTP and Kubernetes overlays.
The `grpc-orders/` example demonstrates gRPC and serialization overlays.

The overlay rule catalogs provide a first implementation target for future validation passes. They are metadata only for now; no rule engine consumes them yet.
