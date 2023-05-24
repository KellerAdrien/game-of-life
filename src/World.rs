/*
 * Filename: c:\Users\Adrien\Documents\code\game-of-life\src\World.rs
 * Path: c:\Users\Adrien\Documents\code\game-of-life\src
 * Created Date: Tuesday, May 23rd 2023, 11:12:22 pm
 * Author: Adrien Keller
 * 
 * Copyright (c) 2023 Your Company
 */


// Max width and height will match a u8 use for now
pub struct World {
    world_height: u8,
    world_width: u8,
    map : Vec<u16>,
}

impl World {
    fn create_cell() {
        unimplemented!("Update cells state");
    }
    fn compute_alive_neighbors() -> int {
        unimplemented!("Computes number of alive neighbors and returns it to main function");
    }
    fn compute_world() {
        unimplemented!("Contains the two computing loops to update cells and update screen");

        /*
        Sur toutes les cells de la map :
            compter le nombre de voisin allumés
            mettre à jour le score de la cell
        Fin de boucle
        */

        /*
        Sur toutes les cells de la map :
             Update la map
        Fin de boucle
        */
    }
}