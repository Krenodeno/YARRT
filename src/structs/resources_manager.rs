use std::collections::HashMap;
use std::sync::{Arc, Weak};

pub trait Resource {}

pub trait ResourceConfig: std::cmp::Eq + std::hash::Hash + Sized + Clone {
    type AssociatedResource: ?Sized;

    fn create_resource(&self, res_mgr: &mut ResourceManager<Self>)
        -> Arc<Self::AssociatedResource>;
}

/// Used to hold a reference on a type of resource to be read by multiple
/// users.
/// K is the Configuration type of V representing its parameters as an unique
/// instance
pub struct ResourceManager<K: ResourceConfig> {
    pub resources: HashMap<K, Weak<<K as ResourceConfig>::AssociatedResource>>,
}

impl<'a, K: ResourceConfig> ResourceManager<K> {
    pub fn new() -> Self {
        ResourceManager {
            resources: HashMap::new(),
        }
    }

    /// Instanciate the demanded resource if possible, return an Arc on it if
    /// instanciation is a success or a similar resource already exist.
    /// Can panic if no resource can be created with specified configuration.
    pub fn get_resource(&mut self, config: &K) -> Arc<<K as ResourceConfig>::AssociatedResource> {
        match self.resources.get(&config) {
            Some(t) => t
                .upgrade()
                .or_else(|| self.load_from_config(&config))
                .unwrap(),
            None => self.load_from_config(&config).unwrap(),
        }
    }

    /// Try to load a resource and return it.
    /// It need the resource type K to implement Resource Trait.
    fn load_from_config(
        &mut self,
        config: &K,
    ) -> Option<Arc<<K as ResourceConfig>::AssociatedResource>> {
        let res = config.create_resource(self);

        // add it to the managed resources
        let copy = config.clone();
        self.resources.insert(copy, Arc::downgrade(&res));
        Some(res)
    }
}
