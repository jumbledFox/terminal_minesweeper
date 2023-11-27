// Todo: rethink whatever this shitty file is

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor, SetBackgroundColor, Print, Stylize, StyledContent, Attribute, SetAttribute},
    execute,
    cursor, ExecutableCommand,
};

use std::io::stdout;

use crate::game::board::{Board, Tile, Position};

pub fn draw_board(board: &Board) -> std::io::Result<()> {
    let mut lines = String::new();
    for y in 0..board.height {
        // Define empty string for a line
        let mut l: String = String::new();
        // For each character, add the tile to the string
        for x in 0..board.width {
            // Get the tile
            let tile = board.get_tile(x, y);
            // And turn it into a string
            let mut tile_str: StyledContent<String> = match tile {
                &Tile::Blank                =>        " - ".to_owned(),
                &Tile::Unopened             =>        " - ".to_owned(),
                &Tile::Flag                 =>        " F ".to_owned(),
                &Tile::Numbered(number) => format!(" {} ", number),
            }.stylize();
            // Get the style info of the tile
            let style_info = get_tile_style(tile);
            if board.bombs.contains(&Position {x: x, y: y}) {
                //tile_str = " ! ".to_owned().stylize();
                //style_info = StyleInfo {fg: Color::Rgb{ r: 229, g: 113, b: 78 }, bg: Color::Reset, reversed: true };
            }
            // If the tile is meant to be reversed, reverse it
            if style_info.reversed {
                tile_str = tile_str.reverse();
            }
            // Push that string to the line string
            l.push_str(&format!("{}", tile_str.with(style_info.fg).on(style_info.bg)));
        }
        lines.push_str(&l);
        lines.push_str("\n\r");
    }
    // Generate the titlebar
    // This is really really ugly.. TODO: Make better :c
    let mut title_bar: String = format!("{}", "jumbledFox's Minesweeper\n\r".to_owned().stylize().with(Color::Rgb {r: 239, g: 125, b: 87}));
    title_bar.push_str(&format!("{}", "Mines: "));
    let mines_left = match board.flag_count >= board.bomb_count as usize {
        false => board.bomb_count as usize - board.flag_count,
        _ => 0,
    };
    title_bar.push_str(&format!("{}", &format!("{:0>2}", mines_left.to_string()).stylize().red()));
    title_bar.push_str(&format!("{}", "    Time: "));
    title_bar.push_str(&format!("{}", &format!("{:0>3}", "0").stylize().red()));
    // Draw the screen
    execute!(
        stdout(),
        cursor::MoveTo(0, 0),
        Print(title_bar),
        cursor::MoveTo(0, 2),
        Print(lines),
    )?;
    // Draw cursor
    let cursor_col = get_tile_style(&board.get_tile(board.selected_cell.x, board.selected_cell.y));

    stdout().execute(ResetColor)?;
    if cursor_col.reversed {
        stdout().execute(SetAttribute(Attribute::Reverse))?;
    }
    execute!(
        stdout(),
        SetForegroundColor(cursor_col.fg),
        SetBackgroundColor(cursor_col.bg),
        cursor::MoveTo(board.selected_cell.x * 3,     board.selected_cell.y + 2),
        Print("["),
        cursor::MoveTo(board.selected_cell.x * 3 + 2, board.selected_cell.y + 2),
        Print("]"),
        ResetColor,
        cursor::MoveTo(0, board.height+2),
    )?;
    
    Ok(())
}

struct StyleInfo {
    fg: Color,
    bg: Color,
    reversed: bool,
}

// Gets the color pair of a specified tile
fn get_tile_style(tile: &Tile) -> StyleInfo {
    match tile {
        Tile::Numbered(num) => match num {
            1 => StyleInfo { fg: Color::Rgb { r: 112, g: 181, b: 238 }, bg: Color::Reset, reversed: false }, 
            2 => StyleInfo { fg: Color::Rgb { r: 181, g: 238, b: 112 }, bg: Color::Reset, reversed: false }, 
            3 => StyleInfo { fg: Color::Rgb { r: 238, g: 112, b: 181 }, bg: Color::Reset, reversed: false }, 
            4 => StyleInfo { fg: Color::Rgb { r: 181, g: 112, b: 238 }, bg: Color::Reset, reversed: false }, 
            5 => StyleInfo { fg: Color::Rgb { r: 238, g: 181, b: 112 }, bg: Color::Reset, reversed: false },
            6 => StyleInfo { fg: Color::Rgb { r: 255, g: 236, b: 188 }, bg: Color::Reset, reversed: false },
            7 => StyleInfo { fg: Color::Rgb { r: 234, g: 186, b: 255 }, bg: Color::Reset, reversed: false }, 
            _ => StyleInfo { fg: Color::Rgb { r: 112, g: 238, b: 181 }, bg: Color::Reset, reversed: false }, 
        }
        Tile::Blank    => StyleInfo { fg: Color::Rgb { r: 170, g: 170, b: 170 }, bg: Color::Reset, reversed: false },
        Tile::Unopened => StyleInfo { fg: Color::Rgb { r: 170, g: 170, b: 170 }, bg: Color::Reset, reversed: true  },
        Tile::Flag     => StyleInfo { fg: Color::Rgb { r: 224, g: 83,  b: 160 }, bg: Color::Reset, reversed: true  },
    }
}