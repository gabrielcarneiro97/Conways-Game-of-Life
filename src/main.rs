use std::rc::Rc;

#[derive(Debug)]
enum State {
    Alive,
    Dead
}

struct Coords {
    x: usize,
    y: usize
}

struct Cell {
    state: State,
    position: Coords,
    neighboors: Vec<Rc<Cell>>
}

impl Cell {
    fn new(state: State, position: Coords) -> Cell {
        let mut this = Cell {
            state,
            position,
            neighboors: Vec::new()
        };

        this.discover_neighbors();
        
        this
    }

    fn discover_neighbors(&mut self) {

    }
}

struct Map {
    cells: Vec<Rc<Cell>>,
    size: Coords,
    counter: usize
}

impl Map {
    fn new(size: Coords) -> Map {
        Map {
            cells: Map::populate(&size),
            size,
            counter: 0
        }
    }

    fn populate(size: &Coords) -> Vec<Rc<Cell>> {

        let x_max = size.x;
        let y_max = size.y;
        let mut vec = Vec::new();

        for x in 0..x_max {
            for y in 0..y_max {
                vec.push(Rc::new(Cell::new(State::Dead, Coords { x, y })));
            }
        }

        vec
    }

    fn next_tick(&mut self) {

    }

    fn map(&self) {

    }

    fn get_cell(&self, coord: &Coords) -> Rc<Cell> {
        let pos = (2 * coord.x) + coord.y;

        Rc::clone(&self.cells[pos])
    }
}

fn main() {
    let a = State::Alive;
    println!("{:?}", a);
}
