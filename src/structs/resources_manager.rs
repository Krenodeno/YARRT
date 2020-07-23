use std::collections::HashMap;
use std::sync::{Arc, Weak};

use super::texture::{SolidColor, Texture, TextureConfig, TextureKind};
/// Used to hold a reference on a type of resource to be read by multiple
/// users.
/// K is the Configuration type of V representing its parameters as an unique
/// instance
pub struct ResourceManager<'a> {
    pub resources: HashMap<TextureConfig<'a>, Weak<dyn Texture>>,
}

impl<'a> ResourceManager<'a> {
    pub fn new() -> Self {
        ResourceManager {
            resources: HashMap::new(),
        }
    }

    /// Instanciate the demanded resource if possible, return an Arc on it if
    /// instanciation is a success or a similar resource already exist.
    /// Can panic if no resource can be created with specified configuration.
    pub fn get_resource(&mut self, config: TextureConfig<'a>) -> Arc<dyn Texture> {
        match self.resources.get(&config) {
            Some(t) => t
                .upgrade()
                .or_else(|| self.load_from_config(&config))
                .unwrap(),
            None => self.load_from_config(&config).unwrap(),
        }
    }

    /// Try to load a resource and return it.
    /// It need the resource type K to implement ManagedResource Trait.
    fn load_from_config(&mut self, config: &TextureConfig<'a>) -> Option<Arc<dyn Texture>> {
        let res: Arc<dyn Texture> = match config.kind {
            TextureKind::Constant(v) => Arc::new(SolidColor::new(v)),
            _ => return None,
        };

        // add it to the managed resources
        let copy = *config;
        self.resources.insert(copy, Arc::downgrade(&res));
        Some(res)
    }
}

pub trait Resource {}
