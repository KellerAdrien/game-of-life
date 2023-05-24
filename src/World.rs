/*
 * Filename: c:\Users\Adrien\Documents\code\game-of-life\src\World.rs
 * Path: c:\Users\Adrien\Documents\code\game-of-life\src
 * Created Date: Tuesday, May 23rd 2023, 11:12:22 pm
 * Author: Adrien Keller
 * 
 * Copyright (c) 2023 Your Company
 */


pub struct World {
    world_height: u16,
    world_width: u16,
    map : Vec<u16>,
}

impl World {

    pub fn new(grid_size: u16) -> World {
        World {
            world_height : grid_size,
            world_width : grid_size,
            map : Vec::with_capacity((grid_size * grid_size).into()),
        }
    }

    pub fn create_grid() {
        unimplemented!("Creates grid");
    }
    pub fn compute_alive_neighbors() -> u16 {
        unimplemented!("Computes number of alive neighbors and returns it to main function");
    }
    pub fn compute_world() {
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