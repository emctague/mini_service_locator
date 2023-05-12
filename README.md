# mini_service_locator

[![Crates.io](https://img.shields.io/crates/v/mini_service_locator?style=for-the-badge)](https://crates.io/crates/mini_service_locator) [![docs.rs](https://img.shields.io/docsrs/mini_service_locator?style=for-the-badge)](https://docs.rs/mini_service_locator) [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/emctague/mini_service_locator/rust.yml?style=for-the-badge)](https://github.com/emctague/mini_service_locator) [![Crates.io](https://img.shields.io/crates/l/mini_service_locator?style=for-the-badge)](https://opensource.org/license/mit/)

`mini_service_locator` provides a simple thread-safe service locator: a container that
stores "service" objects of different types, allowing for retrieval of a service by its type.

## Example

```rust
use mini_service_locator::ServiceLocator;

struct MyUsefulService {
    some_shared_thing: i32
}

let mut locator = ServiceLocator::default();

// Put a MyUsefulService into the locator.
locator.provide(MyUsefulService { some_shared_thing: 24 });

// Later, we can fetch this service.
// If we *don't* use store the resulting service as `mut`, we won't be allowed to write to it.
// We can run get<MyUsefulService> multiple times, and it will always provide the same service.
let mut useful_service = locator.get::<MyUsefulService>().unwrap();

assert_eq!(useful_service.read().some_shared_thing, 24);
useful_service.write().some_shared_thing = 32;
assert_eq!(useful_service.read().some_shared_thing, 32);