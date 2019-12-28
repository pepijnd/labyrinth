pub mod loader;
pub mod material;
pub mod model;
pub mod object;
pub mod shader;
pub mod texture;
pub mod animation;

use generational_arena::Index;
use glium::backend::Facade;

use downcast_rs::{
    impl_downcast,
    Downcast
};

use crate::game::context::LabyrinthContext;


pub trait ResourceBase: Downcast + std::fmt::Debug {
    fn get_identifier(&self) -> &str;
}
impl_downcast!(ResourceBase);

pub trait Loadable {
    type Source;

    fn load<F>(asset: &Self::Source, facade: &F, context: &mut LabyrinthContext) -> Index
    where
        F: Facade;
}

pub trait Findable<T: ResourceBase> {
    fn get(context: &LabyrinthContext, index: Index) -> Option<&T> {
        if let Some(item) = context.resources.get(index) {
            item.downcast_ref::<T>()
        } else { None }
    }
    
    fn find(context: &LabyrinthContext, ident: &str) -> Option<Index> {
        match context.resources.iter().find(|x| {
            match x.1.downcast_ref::<T>() {
                Some(res) => res.get_identifier() == ident,
                _ => false
            }
        }) {
            Some(n) => Some(n.0),
            None => None,
        }
    }
}

#[macro_export]
macro_rules! impl_resource {
    ($ty:ty, $id:ident) => {
        impl crate::resources::ResourceBase for $ty {
            fn get_identifier(&self) -> &str {
                &self.$id
            }
        }
        impl crate::resources::Findable<Self> for $ty {}
    };
}