use std::collections::HashMap;
use std::sync::{Arc, Weak};

/// Used to hold a reference on a type of resource to be read by multiple
/// users.
/// K is the Configuration type of V representing its parameters as an unique
/// instance
pub struct ResourceManager<K, V> {
    pub resources: HashMap<K, Weak<V>>,
}

impl<K: std::cmp::Eq + std::hash::Hash, V: ManagedResource<K>> ResourceManager<K, V> {
    pub fn new() -> ResourceManager<K, V> {
        ResourceManager {
            resources: HashMap::new(),
        }
    }

    /// Instanciate the demanded resource if possible, return an Arc on it if
    /// instanciation is a success or a similar resource already exist.
    /// Can panic if no resource can be created with specified configuration.
    pub fn get_resource(&mut self, config: &K) -> Arc<V> {
        match self.resources.get(config) {
            Some(t) => t.upgrade().or_else(|| self.load_from_config(config)).unwrap(),
            None => self.load_from_config(config).unwrap()
        }
    }

    /// Try to load a resource and return it.
    /// It need the resource type K to implement ManagedResource Trait.
    fn load_from_config(&mut self, config: &K) -> Option<Arc<V>> {
        let res = V::load_from_config(config);
        match res {
            Some(r) => {
                // add it to the managed resources
                let copy: K = *config;
                self.resources.insert(copy, Arc::downgrade(&r));
                res
            }
        }
    }
}

pub trait ManagedResource<Config> {
    /// Return an instanciated resource if the config is correct
    fn load_from_config(config: &Config) -> Option<Arc<Self>>;
}