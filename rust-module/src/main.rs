use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use std::thread;


#[derive(PartialEq, Debug, Clone)]
pub enum State {
    Alive,
    Dead
}

#[derive(Debug, Clone)]
pub struct Coords {
    x: usize,
    y: usize
}

#[derive(Debug)]
pub struct Cell {
    state: State,
    position: Coords,
    neighboors: Vec<Coords>,
    neighboors_alive: i32
}

impl Cell {
    pub fn new(state: State, position: Coords, neighboors: Vec<Coords>) -> Cell {
        Cell {
            state,
            position,
            neighboors,
            neighboors_alive: 0
        }
    }

    pub fn find_neighboors(coord: Coords, size: &Coords) -> Vec<Coords> {
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
                neighboors.push(Coords { x: x, y: y + 1});
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

    pub fn change_state(&mut self) {
        match self.state {
            State::Alive => self.state = State::Dead,
            State::Dead => self.state = State::Alive
        };
    }
}

#[derive(Debug)]
pub struct Map {
    cells: Vec<Rc<RefCell<Cell>>>,
    size: Coords,
    generation: usize
}

impl Map {
    pub fn new(size: Coords) -> Map {
        Map {
            cells: Map::populate(&size),
            size,
            generation: 0
        }
    }

    pub fn populate(size: &Coords) -> Vec<Rc<RefCell<Cell>>> {

        let mut vec : Vec<Rc<RefCell<Cell>>> = Vec::new();

        for x in 0..size.x {
            for y in 0..size.y {
                let cell = Cell::new(State::Dead, Coords { x, y }, Cell::find_neighboors(Coords { x, y }, &size));
                vec.push(Rc::new(RefCell::new(cell)));
            }
        }

        vec
    }

    pub fn next_tick(&mut self) {
        self.generation += 1;

        for cell in &self.cells {
            let mut borrow = cell.borrow_mut();
            let mut alives = 0;

            for neighboor in &borrow.neighboors {
                let neighboor_cell = self.get_cell(&neighboor);
                let state = &neighboor_cell.borrow().state;

                match *state {
                    State::Alive => alives += 1,
                    State::Dead => ()
                }
            }
            borrow.neighboors_alive = alives;            
        }

        for cell in &self.cells {
            let mut borrow = cell.borrow_mut();
            let alives = borrow.neighboors_alive;

            let state = borrow.state.clone();

            match state {
                State::Alive => {
                    if alives < 2 || alives > 3 {
                        borrow.change_state();
                    }
                },
                State::Dead => {
                    if alives == 3 {
                        borrow.change_state();
                    }
                }
            }
        }

    }

    pub fn map(&self) {
        let x_max = self.size.x;
        let y_max = self.size.y;

        for x in 0..x_max {
            for y in 0..y_max {
                let cell = self.get_cell(&Coords {x: x, y: y});
                let state = &cell.borrow().state;

                match *state {
                    State::Alive => print!("▀"),
                    State::Dead => print!("╳")
                }
            }
            println!("");
        }
    }

    pub fn get_cell(&self, coord: &Coords) -> Rc<RefCell<Cell>> {
        let pos = coord.y + (coord.x * &self.size.y);

        Rc::clone(&self.cells[pos])
    }

    pub fn set(&self, coords: Vec<Coords>) {
        for coord in coords {
            let cell = self.get_cell(&coord);
            cell.borrow_mut().state = State::Alive;
        }
    }

    pub fn blinker() -> Vec<Coords> {
        let mut blinker : Vec<Coords> = Vec::new();

        blinker.push(Coords {x: 1, y: 2});
        blinker.push(Coords {x: 2, y: 2});
        blinker.push(Coords {x: 3, y: 2});
        
        blinker
    }

    pub fn glider() -> Vec<Coords> {
        let mut glider : Vec<Coords> = Vec::new();

        glider.push(Coords {x: 0, y: 0});
        glider.push(Coords {x: 0, y: 2});
        glider.push(Coords {x: 1, y: 1});
        glider.push(Coords {x: 1, y: 2});
        glider.push(Coords {x: 2, y: 1});

        glider
    }

    pub fn gosper_glider_gun() -> Vec<Coords> {
        let mut gosper_glider_gun : Vec<Coords> = Vec::new();

        gosper_glider_gun.push(Coords {x: 5, y: 1});
        gosper_glider_gun.push(Coords {x: 6, y: 1});
        gosper_glider_gun.push(Coords {x: 5, y: 2});
        gosper_glider_gun.push(Coords {x: 6, y: 2});
        gosper_glider_gun.push(Coords {x: 5, y: 11});
        gosper_glider_gun.push(Coords {x: 6, y: 11});
        gosper_glider_gun.push(Coords {x: 7, y: 11});
        gosper_glider_gun.push(Coords {x: 8, y: 12});
        gosper_glider_gun.push(Coords {x: 4, y: 12});
        gosper_glider_gun.push(Coords {x: 9, y: 13});
        gosper_glider_gun.push(Coords {x: 3, y: 13});        
        gosper_glider_gun.push(Coords {x: 9, y: 14});
        gosper_glider_gun.push(Coords {x: 3, y: 14});
        gosper_glider_gun.push(Coords {x: 6, y: 15});
        gosper_glider_gun.push(Coords {x: 4, y: 16});
        gosper_glider_gun.push(Coords {x: 8, y: 16});
        gosper_glider_gun.push(Coords {x: 5, y: 17});
        gosper_glider_gun.push(Coords {x: 6, y: 17});
        gosper_glider_gun.push(Coords {x: 7, y: 17});
        gosper_glider_gun.push(Coords {x: 6, y: 18});
        gosper_glider_gun.push(Coords {x: 3, y: 21});
        gosper_glider_gun.push(Coords {x: 4, y: 21});
        gosper_glider_gun.push(Coords {x: 5, y: 21});
        gosper_glider_gun.push(Coords {x: 3, y: 22});
        gosper_glider_gun.push(Coords {x: 4, y: 22});
        gosper_glider_gun.push(Coords {x: 5, y: 22});
        gosper_glider_gun.push(Coords {x: 2, y: 23});
        gosper_glider_gun.push(Coords {x: 6, y: 23});
        gosper_glider_gun.push(Coords {x: 1, y: 25});
        gosper_glider_gun.push(Coords {x: 2, y: 25});
        gosper_glider_gun.push(Coords {x: 6, y: 25});
        gosper_glider_gun.push(Coords {x: 7, y: 25});
        gosper_glider_gun.push(Coords {x: 3, y: 35});
        gosper_glider_gun.push(Coords {x: 4, y: 35});
        gosper_glider_gun.push(Coords {x: 3, y: 36});
        gosper_glider_gun.push(Coords {x: 4, y: 36});
        

        gosper_glider_gun
    }

}

fn main() {
    let mut map = Map::new(Coords {x: 45, y: 150});
    
    map.set(Map::blinker());

    loop {
        thread::sleep(Duration::from_millis(300));
        print!("{}[2J", 27 as char);
        map.map();
        map.next_tick();
    }
}
