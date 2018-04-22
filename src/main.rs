use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
#[derive(PartialEq)]
enum State {
    Alive,
    Dead
}

#[derive(Clone)]
#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct Cell {
    state: State,
    position: Coords,
    neighboors: Vec<Coords>
}

impl Cell {
    fn new(state: State, position: Coords, neighboors: Vec<Coords>) -> Cell {
        Cell {
            state,
            position,
            neighboors
        }
    }

    fn find_neighboors(coord: Coords, size: &Coords) -> Vec<Coords> {
        let x = coord.x;
        let y = coord.y;

        let mut neighboors : Vec<Coords> = Vec::new();

        if y == 0 {
            if x == 0 {
                neighboors.push(Coords { x: x, y: y + 1});
                neighboors.push(Coords { x: x + 1, y: y + 1});
                neighboors.push(Coords { x: x + 1, y: y});
            } else if x == size.x - 1 {
                neighboors.push(Coords { x: x - 1, y: y});
                neighboors.push(Coords { x: x - 1, y: y + 1});
                neighboors.push(Coords { x: x, y: y + 1});
            } else {
                neighboors.push(Coords { x: x - 1, y: y});
                neighboors.push(Coords { x: x - 1, y: y + 1});
                neighboors.push(Coords { x: x, y: y + 1});
                neighboors.push(Coords { x: x + 1, y: y + 1});
                neighboors.push(Coords { x: x + 1, y: y});
            }
        } else if y == size.y - 1 {
            if x == 0 {
                neighboors.push(Coords { x: x, y: y - 1});
                neighboors.push(Coords { x: x + 1, y: y - 1});
                neighboors.push(Coords { x: x + 1, y: y});
            } else if x == size.x - 1 {
                neighboors.push(Coords { x: x - 1, y: y});
                neighboors.push(Coords { x: x - 1, y: y - 1});
                neighboors.push(Coords { x: x, y: y - 1});
            } else {
                neighboors.push(Coords { x: x - 1, y: y});
                neighboors.push(Coords { x: x - 1, y: y - 1});
                neighboors.push(Coords { x: x, y: y - 1});
                neighboors.push(Coords { x: x + 1, y: y - 1});
                neighboors.push(Coords { x: x + 1, y: y});
            }
        } else {
            if x == 0 {
                neighboors.push(Coords { x: x, y: y - 1});
                neighboors.push(Coords { x: x + 1, y: y - 1});
                neighboors.push(Coords { x: x + 1, y: y});
                neighboors.push(Coords { x: x + 1, y: y + 1});
                neighboors.push(Coords { x: x + 1, y: y});
            } else if x == size.x - 1 {
                neighboors.push(Coords { x: x, y: y - 1});
                neighboors.push(Coords { x: x - 1, y: y - 1});
                neighboors.push(Coords { x: x - 1, y: y});
                neighboors.push(Coords { x: x - 1, y: y + 1});
                neighboors.push(Coords { x: x, y: y + 1});
            } else {
                neighboors.push(Coords { x: x - 1, y: y - 1});
                neighboors.push(Coords { x: x - 1, y: y});
                neighboors.push(Coords { x: x - 1, y: y + 1});
                neighboors.push(Coords { x: x, y: y + 1});
                neighboors.push(Coords { x: x + 1, y: y + 1});
                neighboors.push(Coords { x: x + 1, y: y});
                neighboors.push(Coords { x: x + 1, y: y - 1});
                neighboors.push(Coords { x: x, y: y - 1});
            }
        }

        neighboors
    }

    fn change_state(&mut self) {
        match self.state {
            State::Alive => self.state = State::Dead,
            State::Dead => self.state = State::Alive
        };
    }
}

#[derive(Debug)]
struct Map {
    cells: Vec<Rc<RefCell<Cell>>>,
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

    fn populate(size: &Coords) -> Vec<Rc<RefCell<Cell>>> {

        let mut vec = Vec::new();

        for x in 0..size.x {
            for y in 0..size.y {
                let cell = Cell::new(State::Dead, Coords { x, y }, Cell::find_neighboors(Coords { x, y }, &size));
                vec.push(Rc::new(RefCell::new(cell)));
            }
        }

        vec
    }

    fn next_tick(&mut self) {

    }

    fn map(&self) {

    }

    fn get_cell(&self, coord: &Coords) -> Rc<RefCell<Cell>> {
        let pos = coord.y + (coord.x * &self.size.y);

        Rc::clone(&self.cells[pos])
    }

}

fn main() {
    let mut map = Map::new(Coords {x: 3, y: 3});
    println!("{:?}", map);
}
