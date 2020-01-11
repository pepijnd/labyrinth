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
use crate::labyrinth_error;

use crate::error::LabyrinthErrorBase;

#[derive(Debug)]
enum ResourceError {
    Find(String, &'static str),
    Get(&'static str),
    Type(String, &'static str),
    Loading(Box<dyn LabyrinthErrorBase>, String, &'static str),
    Render(Box<dyn std::error::Error>, String, &'static str),
}

labyrinth_error!(ResourceError, |e| match e {
        ResourceError::Find(name, kind) => {
            format!("[Find] Unable to find resource \"{}\" ({})", name, kind)
        },
        ResourceError::Get(kind) => {
            format!("[Get] Unable to get resource ({})", kind)
        },
        ResourceError::Loading(e, name, kind) => {
            format!("[Loading] Error occured while loading resource \"{}\" ({}): {}", name, kind, e)
        },
        ResourceError::Type(name, kind) => {
            format!("[Type] Item \"{}\" is not of type ({})", name, kind)
        },
        ResourceError::Render(e, name, kind) => {
            format!("[Render] Error occured while loading resource \"{}\" ({}): {}", name, kind, e)
        }
    }
);

pub trait ResourceBase: Downcast + std::fmt::Debug {
    fn get_identifier(&self) -> &str;
}
impl_downcast!(ResourceBase);

pub trait Loadable {
    type Source;

    fn load<F>(asset: &Self::Source, facade: &F, context: &mut LabyrinthContext) -> crate::LabyrinthResult<Index>
    where
        F: Facade;
}

pub trait Findable<T: ResourceBase> {
    fn get(context: &LabyrinthContext, index: Index) -> crate::LabyrinthResult<&T> {
        let item = context.resources.get(index);
        match item {
            Some(item) => {
                item.downcast_ref::<T>().ok_or_else(|| ResourceError::Type(item.get_identifier().to_string(), Self::get_type()).into())
            },
            None => {
                Err(ResourceError::Get(Self::get_type()).into())
            }
        }
    }
    
    fn find(context: &LabyrinthContext, ident: &str) -> crate::LabyrinthResult<Index> {
        match context.resources.iter().find(|x| {
            match x.1.downcast_ref::<T>() {
                Some(res) => res.get_identifier() == ident,
                _ => false
            }
        }) {
            Some(n) => Ok(n.0),
            None => Err(ResourceError::Find(ident.to_string(), Self::get_type()).into()),
        }
    }

    fn get_type() -> &'static str;
}

#[macro_export]
macro_rules! impl_resource {
    ($ty:ty, $id:ident) => {
        impl crate::resources::ResourceBase for $ty {
            fn get_identifier(&self) -> &str {
                &self.$id
            }
        }
        impl crate::resources::Findable<Self> for $ty {
            fn get_type() -> &'static str {
                stringify!($ty)
            }
        }
    };
}