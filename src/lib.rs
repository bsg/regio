mod tests;

extern crate macros;

pub use macros::component;
pub use macros::init;
pub use macros::inject;

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::OnceLock,
};

pub trait ToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Component> ToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// TODO get rid of ToAny
pub trait Component: Send + Sync + ToAny {
    fn new() -> Self
    where
        Self: Sized;
    fn init(&self) {}
}

pub struct Registry {
    components: HashMap<TypeId, Box<dyn Component>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            components: HashMap::new(),
        }
    }

    pub fn put<T: Component + 'static>(&mut self, component: T) {
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, Box::new(component));
    }

    pub fn get<T: Component>(&'static self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components.get(&type_id).and_then(|c| {
            c.as_any().downcast_ref()
        })
    }
}

static COMPONENTS: OnceLock<Registry> = OnceLock::new();

pub fn init(registry: Registry) {
    match COMPONENTS.set(registry) {
        Ok(_) => {}
        Err(_) => panic!("Failed to initialize registry"),
    }
    for component in COMPONENTS.get().unwrap().components.values().into_iter() {
        component.as_ref().init();
    }
}

// TODO init_async

pub fn get<T: Component>() -> &'static T {
    match COMPONENTS.get() {
        Some(r) => match r.get::<T>() {
            Some(c) => c,
            None => panic!("Unregistered component {}", std::any::type_name::<T>()),
        },
        None => panic!("Registry uninitialized"),
    }
}
