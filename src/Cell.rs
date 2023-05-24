/*
 * Filename: c:\Users\Adrien\Documents\code\game-of-life\src\Cell.rs
 * Path: c:\Users\Adrien\Documents\code\game-of-life\src
 * Created Date: Tuesday, May 23rd 2023, 11:12:17 pm
 * Author: Adrien Keller
 * 
 * Copyright (c) 2023 Your Company
 */

// Max width and height will match a u8 use for now
pub struct Cell {
    state: bool,
    posX: u8,
    posY: u8,
    neighborScore : u8,
}

impl Cell {
    fn update() {
        unimplemented!("Update cell state");
    }

    //WARN: Is it actually useful ?
    fn get_state() -> bool {
        unimplemented!("Returns cell state");
    }

    fn set_score(score: u8) {
        unimplemented!("Sets cell score");
    }

}