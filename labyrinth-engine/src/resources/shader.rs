use glium::backend::Facade;
use generational_arena::Index;

use labyrinth_assets::assets::Program;

use crate::resources::ResourceError;
use crate::resources::Resource;

use crate::game::context::LabyrinthContext;
use crate::impl_resource;
use crate::resources::Loadable;

#[derive(Debug)]
pub struct ProgramBuffer {
    pub name: String,
    pub program: glium::Program,
}

impl_resource!(ProgramBuffer, name);

impl Loadable for ProgramBuffer {
    type Source = Program;

    fn load<F>(program: &Program, facade: &F, context: &mut LabyrinthContext) -> crate::LabyrinthResult<Index>
    where
        F: Facade,
    {
        let buffer = ProgramBuffer {
            name: program.name.clone(),
            program: glium::Program::from_source(
                facade,
                &program.vertex.code,
                &program.fragment.code,
                None,
            )
            .map_err(|e| ResourceError::Render(e.into(), program.name.clone(), Self::get_type()))?
        };
        
        Ok(context.resources.insert(Box::new(buffer)))
    }
}