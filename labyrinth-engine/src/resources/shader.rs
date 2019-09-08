use crate::game::context::Shared;
use crate::game::context::SharedContext;


pub struct Shader {
    pub name: String,
    pub source: String
}

impl Shader {
    pub fn new(name: String, source: String) -> Shader {
        Shader {
            name,
            source
        }
    }
}

pub struct Program {
    pub name: String,
    pub program: glium::Program
}

impl Program {
    pub fn new<F>(name: String, facade: &F, context: SharedContext, vertex: Shared<Shader>, fragment: Shared<Shader>, geometry: Option<Shared<Shader>>) -> Program where F: glium::backend::Facade {
        let vertex_source = vertex.borrow();
        let vertex = vertex_source.source.as_str();
        let fragment_source = fragment.borrow();
        let fragment = fragment_source.source.as_str();
        let geometry_source = geometry.as_ref().map(|x| x.borrow());
        let geometry = geometry_source.as_ref().map(|x| x.source.as_str());
        Program {
            name,
            program: glium::Program::from_source(facade, vertex, fragment, geometry).unwrap()
        }
    }
}