use std::cmp::min;
use rand::prelude::*;

#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum Tile {
    Unopened,
    Blank,
    Numbered(u8),
    Flag,
}

pub struct Board {
    pub width: u16,
    pub height: u16,
    pub tiles: Vec<Tile>,
    pub bombs: Vec<[u16; 2]>,
}

impl Board {
    pub fn new(width: u16, height: u16, bomb_amount: u16) -> Board {
        // Work out the maximum number of bombs that can fit on the board, with a hard limit of 2^16
        let max_bombs = min(width*height, u16::MAX)-1;
        // Generate a vector of all of the possible positions for a bomb (in a flattened form)
        let mut possible_positions: Vec<u16> = (0..max_bombs).collect();
        // Shuffle it
        let mut rng = rand::thread_rng();
        possible_positions.shuffle(&mut rng);
        // And now loop through each one, unflatten it, and add it to the bombs vector
        let mut bombs: Vec<[u16; 2]> = Vec::new();
        for position in &possible_positions[0..bomb_amount as usize] {
            let x = position % width;
            let y = position / width;
            bombs.push([x, y]);
        }
        println!("{:?}", bombs);

        // Tiles
        let mut tiles = vec![Tile::Unopened; (width*height) as usize];
        tiles[0] = Tile::Flag;
        tiles[1] = Tile::Blank;
        for i in 1..=8 {
            tiles[i+1] = Tile::Numbered(i as u8);
        }
        Board { width: width, height: height, tiles: tiles, bombs: bombs }
    }
    pub fn get_tile(&self, x: u16, y: u16) -> &Tile {
        &self.tiles[x as usize + (y * self.width) as usize]
    }
}