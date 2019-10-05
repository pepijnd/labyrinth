use crate::game::context::LabyrinthContext;

use glium::backend::Facade;

use generational_arena::Index;

use labyrinth_assets::assets::Program;

pub struct ProgramBuffer {
    pub name: String,
    pub program: glium::Program,
}

impl ProgramBuffer {
    pub fn load<F>(program: &Program, facade: &F, context: &mut LabyrinthContext) -> Index
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
            .unwrap(),
        };
        context.programs.insert(buffer)
    }

    pub fn get(context: &LabyrinthContext, index: Index) -> Option<&ProgramBuffer> {
        context.programs.get(index)
    }

    pub fn find(context: &LabyrinthContext, name: &str) -> Option<Index> {
        match context.programs.iter().find(|x| x.1.name == name) {
            Some(n) => Some(n.0),
            None => None,
        }
    }
}
