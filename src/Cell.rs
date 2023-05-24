/*
 * Filename: c:\Users\Adrien\Documents\code\game-of-life\src\Cell.rs
 * Path: c:\Users\Adrien\Documents\code\game-of-life\src
 * Created Date: Tuesday, May 23rd 2023, 11:12:17 pm
 * Author: Adrien Keller
 * 
 * Copyright (c) 2023 Your Company
 */

pub struct Cell {
    state: bool,
    pos_x: u16,
    pos_y: u16,
    neighborScore : u8,
}

impl Cell {
    pub fn new() -> Cell{
        Cell {
            state: false,
            pos_x: 0,
            pos_y: 0,
            neighborScore : 0,
        }
    }

    pub fn update() {
        unimplemented!("Update cell state");
    }

    //WARN: Is it actually useful ?
    pub fn get_state() -> bool {
        unimplemented!("Returns cell state");
    }

    pub fn set_score(score: u8) {
        unimplemented!("Sets cell score");
    }

}