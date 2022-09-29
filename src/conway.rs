
#[derive(Debug)]
pub enum Cell {
    Live,
    Dead,
}

#[derive(Debug)]
pub struct World<'a> {
    height: isize,
    width: isize,
    cells: &'a [Cell],
}

impl<'a> World<'a> {
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
        println!("getting neighbors for {}, {}", x, y);
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
            println!("{},{} is valid coordinates", n1.0, n1.1);
            neighbors.push(self.get_index_from_coordinates(n1.0, n1.1));
            println!("{} index is neighbor", self.get_index_from_coordinates(n1.0, n1.1))
        }
        if self.is_valid_cell_coordinates(n2.0, n2.1) {
            println!("{},{} is valid coordinates", n2.0, n2.1);
            neighbors.push(self.get_index_from_coordinates(n2.0, n2.1));
            println!("{} index is neighbor", self.get_index_from_coordinates(n2.0, n2.1))
        }
        if self.is_valid_cell_coordinates(n3.0, n3.1) {
            println!("{},{} is valid coordinates", n3.0, n3.1);
            neighbors.push(self.get_index_from_coordinates(n3.0, n3.1));
            println!("{} index is neighbor", self.get_index_from_coordinates(n3.0, n3.1))
        }
        if self.is_valid_cell_coordinates(n4.0, n4.1) {
            println!("{},{} is valid coordinates", n4.0, n4.1);
            neighbors.push(self.get_index_from_coordinates(n4.0, n4.1));
            println!("{} index is neighbor", self.get_index_from_coordinates(n4.0, n4.1))
        }
        if self.is_valid_cell_coordinates(n5.0, n5.1) {
            println!("{},{} is valid coordinates", n5.0, n5.1);
            neighbors.push(self.get_index_from_coordinates(n5.0, n5.1));
            println!("{} index is neighbor", self.get_index_from_coordinates(n5.0, n5.1))
        }
        if self.is_valid_cell_coordinates(n6.0, n6.1) {
            println!("{},{} is valid coordinates", n6.0, n6.1);
            neighbors.push(self.get_index_from_coordinates(n6.0, n6.1));
            println!("{} index is neighbor", self.get_index_from_coordinates(n6.0, n6.1))
        }
        if self.is_valid_cell_coordinates(n7.0, n7.1) {
            println!("{},{} is valid coordinates", n7.0, n7.1);
            neighbors.push(self.get_index_from_coordinates(n7.0, n7.1));
            println!("{} index is neighbor", self.get_index_from_coordinates(n7.0, n7.1))
        }
        if self.is_valid_cell_coordinates(n8.0, n8.1) {
            println!("{},{} is valid coordinates", n8.0, n8.1);
            neighbors.push(self.get_index_from_coordinates(n8.0, n8.1));
            println!("{} index is neighbor", self.get_index_from_coordinates(n8.0, n8.1))
        }

        return neighbors;
    }


    pub fn new(cells: &'a [Cell], height: isize, width: isize) -> Self {
        Self { cells, height, width }
    }

    /*
    The universe of the Game of Life is an infinite, two-dimensional orthogonal grid of square cells, 
    each of which is in one of two possible states, 
    live or dead (or populated and unpopulated, respectively). 
    Every cell interacts with its eight neighbours, which are the cells that are horizontally, vertically, or diagonally adjacent. 
    At each step in time, the following transitions occur:

        Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        Any live cell with two or three live neighbours lives on to the next generation.
        Any live cell with more than three live neighbours dies, as if by overpopulation.
        Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    These rules, which compare the behavior of the automaton to real life, can be condensed into the following:

        Any live cell with two or three live neighbours survives.
        Any dead cell with three live neighbours becomes a live cell.
        All other live cells die in the next generation. Similarly, all other dead cells stay dead.
    */

    pub fn forward_time(&mut self) {
        
    }
}
