
#[derive(Debug)]
pub enum Cell {
    Live,
    Dead,
}

#[derive(Debug)]
pub struct World {
    height: isize,
    width: isize,
    cells: Vec<Cell>,
}

struct CountResult {
    live_cells: isize,
    dead_cells: isize,
}

impl World {
    /*
        we get a slice of cells as input in a consecutive order,
        so we need to convert it to a two-dimensional index so 
        we can then do processing on it based on it's neighbors.
    */
    pub fn get_coordinates_from_index(&self, index: isize) -> (isize, isize) {
        (index % self.width, index / self.width)
    }
    pub fn get_index_from_coordinates(&self, x: isize, y: isize) -> isize {
        (y * self.height) + x
    }
    fn is_valid_cell_coordinates(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        if x >= self.width as isize || y>=self.height as isize {
            return false;
        }

        return true;
    }
    /* 
    each cell has 8 neighbors
    1 2 3
    4 5 6
    7 8 9
    so if we take 5,
    1 2 3 4 6 7 8 9 are all it's neighbors.
    */ 
    pub fn get_neighbors(&self, index: isize) -> Vec<isize> {
        let (x, y) = self.get_coordinates_from_index(index);
        let n1 = (x-1, y-1);
        let n2 = (x, y-1);
        let n3 = (x-1, y);
        let n4 = (x+1, y+1);
        let n5 = (x+1, y);
        let n6 = (x, y+1);
        let n7 = (x-1, y+1);
        let n8 = (x+1, y-1);
        let mut neighbors: Vec<isize> = Vec::new();
        if self.is_valid_cell_coordinates(n1.0, n1.1) {
            neighbors.push(self.get_index_from_coordinates(n1.0, n1.1));
        }
        if self.is_valid_cell_coordinates(n2.0, n2.1) {
            neighbors.push(self.get_index_from_coordinates(n2.0, n2.1));
        }
        if self.is_valid_cell_coordinates(n3.0, n3.1) {
            neighbors.push(self.get_index_from_coordinates(n3.0, n3.1));
        }
        if self.is_valid_cell_coordinates(n4.0, n4.1) {
            neighbors.push(self.get_index_from_coordinates(n4.0, n4.1));
        }
        if self.is_valid_cell_coordinates(n5.0, n5.1) {
            neighbors.push(self.get_index_from_coordinates(n5.0, n5.1));
        }
        if self.is_valid_cell_coordinates(n6.0, n6.1) {
            neighbors.push(self.get_index_from_coordinates(n6.0, n6.1));
        }
        if self.is_valid_cell_coordinates(n7.0, n7.1) {
            neighbors.push(self.get_index_from_coordinates(n7.0, n7.1));
        }
        if self.is_valid_cell_coordinates(n8.0, n8.1) {
            neighbors.push(self.get_index_from_coordinates(n8.0, n8.1));
        }

        return neighbors;
    }

    fn count_live_or_dead_neighbors_of_index(&self, index: isize) -> CountResult {
        let neighbors = self.get_neighbors(index);
        let mut res = CountResult { live_cells: 0, dead_cells: 0 };
        for neighbor in neighbors {
            match self.cells[neighbor as usize] {
                Cell::Live => res.live_cells+=1,
                Cell::Dead => res.dead_cells+=1,
            }
        }

        res
    }


    pub fn new(cells: Vec<Cell>, height: isize, width: isize) -> Self {
        Self { cells, height, width }
    }

    /*
        Any live cell with two or three live neighbours survives.
        Any dead cell with three live neighbours becomes a live cell.
        All other live cells die in the next generation. Similarly, all other dead cells stay dead.
        https://en.wikipedia.org/wiki/Conway's_Game_of_Life
    */
    pub fn forward_time(&mut self) {
        let mut new_cells = Vec::new();
        for (index, cell) in self.cells.iter().enumerate() {
            let count_res = self.count_live_or_dead_neighbors_of_index(index as isize);
            match cell {
                Cell::Live => {
                    if !(count_res.live_cells == 2 || count_res.live_cells == 3) {
                        // cell should live now
                        new_cells.push(Cell::Live);
                    } else {
                        new_cells.push(Cell::Dead);
                    }
                },
                Cell::Dead => {
                    if count_res.dead_cells == 3 {
                        new_cells.push(Cell::Live);
                    } else {
                        new_cells.push(Cell::Dead);
                    }
                },
            }
        }
        self.cells = new_cells;
    }
}
