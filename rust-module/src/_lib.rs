#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

extern crate rand;
extern crate bit_vec;

use bit_vec::BitVec;

use std::thread;
use rand::distributions::{IndependentSample, Range};


#[derive(PartialEq, Debug, Clone)]
#[wasm_bindgen]
pub enum State {
    Alive,
    Dead
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Coords {
    x: usize,
    y: usize
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Cell {
    state: State,
    position: usize,
    neighboors: Vec<usize>,
    neighboors_alive: i32
}

#[wasm_bindgen]
impl Cell {
    pub fn new(state: State, position: usize, neighboors: Vec<usize>) -> Cell {
        Cell {
            state,
            position,
            neighboors,
            neighboors_alive: 0
        }
    }

    pub fn change_state(&mut self) {
        match self.state {
            State::Alive => self.state = State::Dead,
            State::Dead => self.state = State::Alive
        };
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Map {
    alives: Vec<usize>,
    bit_map: BitVec,
    visible_size: Coords,
    true_size: Coords,
    offset: Coords,
    generation: usize
}

#[wasm_bindgen]
impl Map {
    pub fn new(visible_size: Coords) -> Map {
        let true_size = Coords {x: visible_size.x * 4, y: visible_size.y * 4};
        Map {
            alives: Vec::new(),
            bit_map: BitVec::from_elem(true_size.x * true_size.y, false),
            offset: Coords { x: true_size.x/2, y: true_size.y/2 },
            true_size,
            visible_size,
            generation: 0
        }
    }


    pub fn next_tick(&mut self) {
        self.generation += 1;

        let mut next_gen : Vec<usize> = Vec::new();
        let mut kill : Vec<usize> = Vec::new();

        for cell_bit_pos in &self.alives {
            let cell_coord = self.get_coords(*cell_bit_pos);
            let mut cell = Cell::new(State::Alive, 
                *cell_bit_pos, 
                self.find_neighboors(*cell_bit_pos));

            for neighboor in cell.neighboors {
                if self.is_alive(neighboor) {
                    cell.neighboors_alive += 1;
                } else {
                    let mut neighboor_cell = Cell::new(State::Dead, 
                        neighboor, 
                        self.find_neighboors(neighboor));

                    let alives = self.count_neighboors_alive(&mut neighboor_cell);

                    if alives == 3 {
                        let already;
                        match &next_gen.iter().position(|next_gen_cell| next_gen_cell == &neighboor) {
                            Some(_) => already = true,
                            None => already = false
                        };
                        if !already {
                            next_gen.push(neighboor);                            
                        }
                    }
                }
            }

            if cell.neighboors_alive == 2 || cell.neighboors_alive == 3 {
                next_gen.push(self.get_pos(&Coords {x: cell_coord.x, y: cell_coord.y}));
            } else {
                kill.push(self.get_pos(&cell_coord));
            }

        }

        for pos in &next_gen {
            self.bit_map.set(*pos, true);
        }

        for to_kill in kill {
            self.bit_map.set(to_kill, false);
        }

        self.alives = next_gen;

    }

        pub fn find_neighboors(&self, pos: usize) -> Vec<usize> {
        
        let coord = self.get_coords(pos);
        let x = coord.x;
        let y = coord.y;

        let mut neighboors : Vec<usize> = Vec::new();

        if y == 0 {
            if x == 0 {
                neighboors.push(self.get_pos(&Coords { x: x, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y}));
            } else if x == self.true_size.x - 1 {
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y + 1}));
            } else {
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y}));
            }
        } else if y == self.true_size.y - 1 {
            if x == 0 {
                neighboors.push(self.get_pos(&Coords { x: x, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y}));
            } else if x == self.true_size.x - 1 {
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y - 1}));
            } else {
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y}));
            }
        } else {
            if x == 0 {
                neighboors.push(self.get_pos(&Coords { x: x, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y + 1}));
            } else if x == self.true_size.x - 1 {
                neighboors.push(self.get_pos(&Coords { x: x, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y + 1}));
            } else {
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x - 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y + 1}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y}));
                neighboors.push(self.get_pos(&Coords { x: x + 1, y: y - 1}));
                neighboors.push(self.get_pos(&Coords { x: x, y: y - 1}));
            }
        }

        neighboors
    }

    pub fn count_neighboors_alive(&self, cell: &mut Cell) -> i32 {
        for neighboor in &cell.neighboors {
            if self.is_alive(*neighboor) {
                cell.neighboors_alive += 1;
            }
        }
        cell.neighboors_alive
    }

    pub fn map(&self) {
        let x_offset = self.offset.x;
        let y_offset = self.offset.y;
        let x_max = self.visible_size.x + x_offset;
        let y_max = self.visible_size.y + y_offset;

        for x in x_offset..x_max {
            for y in y_offset..y_max {
                if self.is_alive(self.get_pos(&Coords {x, y})) {
                    print!("0");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    pub fn get_pos(&self, coord: &Coords) -> usize {
        coord.y + (coord.x * self.true_size.y)
    }

    pub fn get_coords(&self, pos: usize) -> Coords {
        Coords {
            x: pos / self.true_size.y,
            y: pos % self.true_size.y
        }
    }

    pub fn is_alive(&self, pos: usize) -> bool {
        self.bit_map[pos]
    }

    pub fn set_alive(&mut self, pos_vec: Vec<usize>) {
        for pos in pos_vec {
            if !self.is_alive(pos) {
                self.alives.push(pos);
                self.bit_map.set(pos, true);
            }
        }
    }
    
    pub fn set_random(&mut self) {

        let max_max;

        if self.true_size.x * self.true_size.y / 2 > 10000 {
            max_max = 10000;
        } else {
            max_max = self.true_size.x * self.true_size.y / 2;
        }

        let max_range = Range::new(0, max_max);
        let mut rng = rand::thread_rng();
        let max = max_range.ind_sample(&mut rng) as usize;

        let mut coords : Vec<usize> = Vec::new();

        for _ in 0..max {
            let range_x = Range::new(0usize, self.true_size.x);
            let range_y = Range::new(0usize, self.true_size.y);

            let x = range_x.ind_sample(&mut rng);
            let y = range_y.ind_sample(&mut rng);

            coords.push(self.get_pos(&Coords {x, y}));
        }

        self.set_alive(coords);
    }

    pub fn blinker(&self) -> Vec<usize> {
        let mut blinker : Vec<usize> = Vec::new();

        blinker.push(self.get_pos(&Coords {x: 1, y: 2}));
        blinker.push(self.get_pos(&Coords {x: 2, y: 2}));
        blinker.push(self.get_pos(&Coords {x: 3, y: 2}));
        
        blinker
    }

    pub fn glider(&self) -> Vec<usize> {
        let mut glider : Vec<usize> = Vec::new();

        glider.push(self.get_pos(&Coords {x: 0 + self.offset.x, y: 0 + self.offset.y}));
        glider.push(self.get_pos(&Coords {x: 0 + self.offset.x, y: 2 + self.offset.y}));
        glider.push(self.get_pos(&Coords {x: 1 + self.offset.x, y: 1 + self.offset.y}));
        glider.push(self.get_pos(&Coords {x: 1 + self.offset.x, y: 2 + self.offset.y}));
        glider.push(self.get_pos(&Coords {x: 2 + self.offset.x, y: 1 + self.offset.y}));

        glider
    }

    pub fn gosper_glider_gun(&self) -> Vec<usize> {
        let mut gosper_glider_gun : Vec<usize> = Vec::new();

        gosper_glider_gun.push(self.get_pos(&Coords {x: 5, y: 1}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 6, y: 1}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 5, y: 2}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 6, y: 2}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 5, y: 11}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 6, y: 11}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 7, y: 11}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 8, y: 12}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 4, y: 12}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 9, y: 13}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 3, y: 13}));        
        gosper_glider_gun.push(self.get_pos(&Coords {x: 9, y: 14}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 3, y: 14}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 6, y: 15}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 4, y: 16}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 8, y: 16}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 5, y: 17}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 6, y: 17}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 7, y: 17}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 6, y: 18}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 3, y: 21}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 4, y: 21}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 5, y: 21}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 3, y: 22}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 4, y: 22}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 5, y: 22}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 2, y: 23}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 6, y: 23}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 1, y: 25}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 2, y: 25}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 6, y: 25}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 7, y: 25}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 3, y: 35}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 4, y: 35}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 3, y: 36}));
        gosper_glider_gun.push(self.get_pos(&Coords {x: 4, y: 36}));
        

        gosper_glider_gun
    }

}