# Webshop Software Architecture Example

This example shows how Spec42 can be used to model a small webshop as a software architecture rather than only as an interaction sketch.

It includes:

- a structural architecture for a microservice-based webshop
- explicit ports and connections for API, event, database, payment-provider, and email-provider integration
- a `GeneralView` for architecture structure
- an `InterconnectionView` for service and infrastructure wiring
- two `SequenceView` scenarios:
  - checkout request orchestration
  - asynchronous order-event fan-out

The goal is to give a compact but realistic starting point for software-architecture modeling in Spec42.
