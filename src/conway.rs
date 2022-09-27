use std::fmt::Display;

#[derive(Debug)]
pub enum CellState {
    Live,
    Dead
}

#[derive(Debug)]
pub struct Cell {
    name: String,
    state: CellState,
}
impl Cell {
    pub fn new(name: &str, state: CellState) -> Self {
        Self {
            name: name.to_string(), state
        }
    }
}

#[derive(Debug)]
pub struct World {
    cells: Vec<Vec<Cell>>
}

impl World {
    pub fn new(world_height: usize, world_width: usize) -> Self {
        Self {
            cells: vec![]
        }
    }

    pub fn forward_time(&mut self) {}

}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}