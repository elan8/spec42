# Webshop Software Example

This example is the single software-commerce scenario in `examples/software/`. It combines the readability of the original webshop model with a curated subset of the e-commerce-platform capabilities.

The entry model is `webshop.sysml`, which assembles:

- domain-backed software structure (`HttpService`, `SqlDatabase`, `KafkaTopic`, `ExternalSystem`)
- behavioral modeling (`OrderLifecycleStateMachine` and `CheckoutPipeline`)
- requirements with traceability (`requirement`, `satisfy`, and one illustrative `allocate`)
- interaction scenarios for synchronous checkout orchestration and asynchronous event fan-out

The corresponding views in `Views.sysml` are:

- `structure` (`GeneralView`)
- `connections` (`InterconnectionView`)
- `checkoutFlow` (`SequenceView`)
- `orderEventFanout` (`SequenceView`)
- `orderLifecycle` (`StateTransitionView`)
- `checkoutPipeline` (`ActionFlowView`)
- `requirements` (`GeneralView`)

This model intentionally omits the full e-commerce-platform operational depth (BFF/mobile edge split, gRPC pricing service, retry/DLQ topic set, and broader platform observability/delivery surface) to stay compact and easy to learn.
