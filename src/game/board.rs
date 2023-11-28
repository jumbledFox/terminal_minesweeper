use std::cmp::min;
use rand::prelude::*;

// Represents a position on the board
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Tile {
    Unopened,
    Blank,
    Numbered(u8),
    Flag,
}

#[derive(PartialEq)]
pub enum ExitType {
    Win,
    Lose,
}
pub struct Board {
    pub width: u16,
    pub height: u16,
    pub tiles: Vec<Tile>,
    pub bomb_count: u16,
    pub bombs: Vec<Position>,
    pub selected_cell: Position,
    pub goes: usize,
    pub flag_count: usize,
    pub timer: u64,
    pub exit: Option<ExitType>,
}

impl Board {
    // Creates a new board
    pub fn new(width: u16, height: u16, bomb_count: u16) -> Board {
        // Initialise tiles as a bunch of unopened tiles
        let mut tiles = vec![Tile::Unopened; width as usize*height as usize];
        // Set the selected cell to be in the middle of the grid
        let selected_cell = Position { x: width/2, y: height/2 };

        Board { width: width, height: height, tiles: tiles, bombs: Vec::new(), bomb_count: bomb_count, goes: 0,
            selected_cell: selected_cell, flag_count: 0, timer: 0, exit: None }
    }

    // This function populates the bomb vector, making sure no bombs are generated in a 3x3 area around the selected cell
    pub fn populate_bombs(&mut self) {
        // Work out the maximum number of bombs that can fit on the board, with a hard limit of 2^16
        let max_bombs = min(self.width*self.height, u16::MAX);
        // Generate a vector of all of the possible positions for a bomb (in a flattened form)
        let mut possible_positions: Vec<u16> = (0..max_bombs).collect();
        // Shuffle it
        let mut rng = rand::thread_rng();
        possible_positions.shuffle(&mut rng);

        let mut pos_index = 0;
        'each_bomb: for _ in 0..self.bomb_count {
            // Add a bomb to the list, making sure it's not in a 3x3 area around the selected cell
            'generate_position: loop {
                // If there are no positions left, quit!!!
                if pos_index == possible_positions.len() { return; }
                // Get the position
                let possible_pos = possible_positions[pos_index];
                let x = possible_pos % self.width;
                let y = possible_pos / self.width;
                // Add 1 to the index
                pos_index += 1;
                // If it's not in a 3x3 area around the selected cell, add it and move on to the next bomb !!!
                if !self.in_3x3(x, y, self.selected_cell.x, self.selected_cell.y) {
                    self.bombs.push(Position { x, y });
                    continue 'each_bomb;
                }
            }
        }
    }

    // Moves where the selected cell is, making sure it stays within bounds
    pub fn move_selected_cell(&mut self, x: i32, y: i32) {
        let new_x = self.selected_cell.x as i32 + x;
        let new_y = self.selected_cell.y as i32 + y;

        if self.check_bounds(new_x, new_y) {
            self.selected_cell.x = new_x as u16;
            self.selected_cell.y = new_y as u16;
        }
    }

    // Gets the tile at a coordinate
    pub fn get_tile(&self, x: u16, y: u16) -> &Tile {
        &self.tiles[x as usize + y as usize * self.width as usize]
    }
    // Sets the tile at a coordinate
    pub fn set_tile(&mut self, x: u16, y: u16, tile: Tile) {
        self.tiles[x as usize + (y * self.width) as usize] = tile;
    }
    // Checks if a coordinate is inside of the board
    pub fn check_bounds(&self, x: i32, y: i32) -> bool {
        !(  x >= self.width as i32 ||
            x < 0 ||
            y >= self.height as i32 || 
            y < 0)
    }

    // Checks if x and y are in the 3x3 area around x2 and y2
    fn in_3x3(&self, x: u16, y: u16, x2: u16, y2: u16) -> bool {
        // i could do this mathematically but this is way easier
        for v in -1..=1 {
        for h in -1..=1 {
            // x and y position as i32s
            let x_i = x as i32 + h;
            let y_i = y as i32 + v;

            // Skip if we're checking somewhere that's out of bounds
            if !self.check_bounds(x_i, y_i) { continue; }
            // If the coordinate is in the area, return true!
            if x_i as u16 == x2 && y_i as u16 == y2 { return true }
        }
        }
        false
    }

    // Scans a 3x3 area, returns the loations of things inside it
    fn scan_3x3(&mut self, x: u16, y: u16) -> (u8, Vec<Position>) {
        let mut bomb_count = 0;
        let mut unopened_positions = Vec::new();

        // Loop through all neighbours in a 3x3 radius
        for v in -1..=1 {
        for h in -1..=1 {
            // If we're in the middle, skip it!!!
            if v == 0 && h == 0 { continue; }
            
            // x and y position as i32s
            let x_i = x as i32 + h;
            let y_i = y as i32 + v;

            // Skip if we're checking somewhere that's out of bounds
            if !self.check_bounds(x_i, y_i) { continue; }

            let pos = Position { x: x_i as u16, y: y_i as u16 };
            if self.bombs.contains(&pos) {
                bomb_count += 1;
            } else if self.get_tile(x_i as u16, y_i as u16) == &Tile::Unopened {
                unopened_positions.push(pos)      
            }
        }
        }

        (bomb_count, unopened_positions)
    }


    // Toggles a flag at the selected cell
    pub fn flag(&mut self) {
        let x = self.selected_cell.x;
        let y = self.selected_cell.y;
        match self.get_tile(x, y) {
            Tile::Unopened => { self.set_tile(x, y, Tile::Flag);     self.flag_count+=1 },
            Tile::Flag     => { self.set_tile(x, y, Tile::Unopened); self.flag_count-=1 },
            _ => {},
        }
    }

    // Digs at the selected cell
    pub fn dig(&mut self) {
        let x = self.selected_cell.x;
        let y = self.selected_cell.y;
        // You can only dig unopened cells
        if self.get_tile(x, y) != &Tile::Unopened { return; }
        // If it's the users first go, add bombs to the map
        if self.goes == 0 { self.populate_bombs(); }
        // Increase the go count
        self.goes += 1;

        // You dug a bomb!! Yeeooowwch!!
        if self.bombs.contains(&self.selected_cell) {
            // BANG!
            println!("BANG! You lose :c");
            self.exit = Some(ExitType::Lose);
            return;
        }
        // Start digging
        self.flood_dig(x, y);
        // Check if, once digging's complete, all uncovered tiles are mines
        if self.tiles.iter().filter(|&x| x == &Tile::Unopened || x == &Tile::Flag ).count() == self.bombs.len() {
            println!("You win!! Well done!");
            self.exit = Some(ExitType::Win);
        }
    }

    pub fn flood_dig(&mut self, x: u16, y: u16) {
        let scan_result = self.scan_3x3(x, y);
        let bomb_neighbours = scan_result.0;
        let tiles_to_dig = scan_result.1;

        // If the tile isn't next to a bomb, make it blank and dig all of the neighbours
        if bomb_neighbours == 0 {
            self.set_tile(x, y, Tile::Blank);

            //dig all the neighbours
            for tile in tiles_to_dig {
                self.flood_dig(tile.x, tile.y);
            }
        }
        // Otherwise, make it numbered
        else {
            self.set_tile(x, y, Tile::Numbered(bomb_neighbours as u8));
            return;
        }
    }
}