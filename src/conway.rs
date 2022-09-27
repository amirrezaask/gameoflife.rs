
#[derive(Debug)]
pub enum Cell {
    Live,
    Dead,
}

#[derive(Debug)]
pub struct World<'a> {
    height: usize,
    width: usize,
    cells: &'a [Cell],
}

impl<'a> World<'a> {
    /*
        we get a slice of cells as input in a consecutive order,
        so we need to convert it to a two-dimensional index so 
        we can then do processing on it based on it's neighbors.
    */
    fn get_coordinates_from_index(&self, index: usize) -> (usize, usize) {
        todo!()
    }
    /* 
    each cell has 8 neighbors
    1 2 3
    4 5 6
    7 8 9
    so if we take 5,
    1 2 3 4 6 7 8 9 are all it's neighbors.
    */ 
    fn get_neighbors(&self, index: usize) -> [Cell; 8] {
        todo!()
    }
    pub fn new(cells: &'a [Cell], height: usize, width: usize) -> Self {
        Self { cells, height, width }
    }

    pub fn forward_time(&mut self) {}
}
