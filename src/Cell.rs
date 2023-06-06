/*
 * Filename: c:\Users\Adrien\Documents\code\game-of-life\src\Cell.rs
 * Path: c:\Users\Adrien\Documents\code\game-of-life\src
 * Created Date: Tuesday, May 23rd 2023, 11:12:17 pm
 * Author: Adrien Keller
 * 
 * Copyright (c) 2023 Your Company
 */


#[derive(Debug)]
pub struct Cell {
    state: bool,
    pos_x: u16,
    pos_y: u16,
    neighbor_score : u8,
}

impl Cell {
    pub fn new(x: u16, y: u16) -> Cell{
        Cell {
            state: false,
            pos_x: x,
            pos_y: y,
            neighbor_score : 0,
        }
    }

    pub fn update() {
        unimplemented!("Update cell state");
    }

    pub fn get_state() -> bool {
        unimplemented!("Returns cell state");
    }

    pub fn set_score(score: u8) {
        unimplemented!("Sets cell score");
    }

}