use crate::conway::{Cell};

mod conway;

fn main() {
    let cells = [
        Cell::Live, // 0 0
        Cell::Dead, // 1 0
        Cell::Live, // 2 0

        Cell::Dead, // 0 1
        Cell::Dead, // 1 1
        Cell::Live, // 2 1
 
        Cell::Live,
        Cell::Dead,
        Cell::Live,

    ];
    let mut world = conway::World::new(&cells[..], 3, 3);
    let mut neighbors = world.get_neighbors(1);
    neighbors.sort();
    dbg!(neighbors);
}
