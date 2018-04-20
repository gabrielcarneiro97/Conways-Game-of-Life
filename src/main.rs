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
    alives: Vec<Coords>,
    size: Coords,
    counter: usize
}

impl Map {
    fn new(size: Coords) -> Map {
        Map {
            alives: Vec::new(),
            size,
            counter: 0
        }
    }

    fn next_tick(&mut self) {

    }

    fn map(&self) {

    }
}

fn main() {
    let a = State::Alive;
    println!("{:?}", a);
}
