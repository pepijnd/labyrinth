use std::error::Error;

use downcast_rs::{
    impl_downcast,
    Downcast
};


pub trait LabyrinthErrorBase: Downcast + std::fmt::Debug {
    fn msg(&self) -> String { 
        String::from("[LabyrinthErrorBase]")    
    }
}
impl_downcast!(LabyrinthErrorBase);

impl std::error::Error for dyn LabyrinthErrorBase {}

impl std::fmt::Display for dyn LabyrinthErrorBase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let backtrace = self.backtrace();
        if cfg!(debug_assertions) && backtrace.is_some() {
            write!(f, "{}\n{}", backtrace.unwrap(), self.msg())
        } else {
            write!(f, "{}", self.msg())
        }
    }
}

#[macro_export]
macro_rules! labyrinth_error {
    ($ty:ty, |$id:ident| $closure:expr) => {
        impl $crate::error::LabyrinthErrorBase for $ty {
            fn msg(&self) -> String {
                format!("[{}] {}", stringify!($ty), (|$id: &Self| $closure)(self))
            }
        }

        impl From<$ty> for Box<dyn $crate::error::LabyrinthErrorBase> {
            fn from(error: $ty) -> Self {
                box error
            }
        }
    }
}

