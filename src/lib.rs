pub use any_handle::AnyHandle;
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// A ServiceLocator is a simple service locator structure.
/// Multiple memory-managed 'services' can be attached to it and later retrieved by their type.
///
/// For example:
///
/// ```
/// use mini_service_locator::ServiceLocator;
///
/// struct MyUsefulService {
///     some_shared_thing: i32
/// }
///
/// let mut locator = ServiceLocator::default();
///
/// // Put a MyUsefulService into the locator.
/// locator.provide(MyUsefulService { some_shared_thing: 24 });
///
/// // Later, we can fetch this service.
/// // If we *don't* use store the resulting service as `mut`, we won't be allowed to write to it.
/// // We can run get<MyUsefulService> multiple times, and it will always provide the same service.
/// let mut useful_service = locator.get::<MyUsefulService>().unwrap();
///
/// assert_eq!(useful_service.read().some_shared_thing, 24);
/// useful_service.write().some_shared_thing = 32;
/// assert_eq!(useful_service.read().some_shared_thing, 32);
/// ```

/// A simple service locator, storing objects by their type.
#[derive(Default)]
pub struct ServiceLocator {
    map: HashMap<TypeId, AnyHandle<dyn Any>>,
}

impl ServiceLocator {
    /// Get a value from a service locator by its type, `T`.
    /// If no service exists on the locator of that type, returns None.
    /// Otherwise, returns a thread-safe handle to the service.
    pub fn get<T: Any>(&self) -> Option<AnyHandle<T>> {
        self.map.get(&TypeId::of::<T>())?.clone().into()
    }

    /// Provide the service locator with a service of type T.
    /// The locator will take ownership of this service.
    /// The service can then be retrieved with [get].
    ///
    /// If a service of type T already exists, this will replace it.
    /// Any code storing a reference to the previous value of this service
    /// will retain the old service, as the handle is not reused.
    ///
    /// A shared handle to the service is returned.
    pub fn provide<T: Any>(&mut self, object: T) -> AnyHandle<T> {
        self.map
            .insert(TypeId::of::<T>(), AnyHandle::new(Box::new(object)));
        self.get::<T>().unwrap()
    }
}
