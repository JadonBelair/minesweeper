use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

// size of cell in pixels
const CELL_SIZE: u16 = 40;

// number of cell rows and coloumns
const ROWS: usize = 16;
const COLS: usize = 16;

// the total number of mines
const MINES: u16 = 40;

// offset for the panel
const OFFSET: i32 = 100;

// macroquad window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: "Minesweeper".to_owned(),
        window_height: (CELL_SIZE * ROWS as u16) as i32 + OFFSET,
        window_width: (CELL_SIZE * COLS as u16).into(),
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
}

// resets all the needed variables to their default state and generates a new level
fn setup(board: &mut [[i8; COLS]; ROWS], cover: &mut [[bool; COLS]; ROWS], flags: &mut Vec<(usize, usize)>, win: &mut bool, lose: &mut bool) {
    *board = [[0; COLS]; ROWS];
    *cover = [[true; COLS]; ROWS];
    *flags = Vec::new();

    *win = false;
    *lose = false;

    generate_board(board);
}

fn generate_board(board: &mut [[i8; COLS]; ROWS]) {
    let mut mine_count = 0;

    // keeps trying to add mines until the set amount is reached
    while mine_count < MINES {
        // generates a random position for the mine
        let x = rand::gen_range(0, COLS);
        let y = rand::gen_range(0, ROWS);

        // checks if that spot is already a mine,
        // and if it is, tries again
        if board[y][x] == -1 {
            continue;
        }

        // sets the spot to a mine and updates the mine count
        board[y][x] = -1;
        mine_count += 1;

        // updates the surrounding cells mine counts
        for i in 0..=2 {
            for j in 0..=2 {

                // skips the just set mine
                if i == 1 && j == 1 {
                    continue;
                }

                let x_index = x as isize + (i as isize - 1);
                let y_index = y as isize + (j as isize - 1);

                // ensures that the index is valid
                if x_index < 0 || x_index >= COLS as isize {
                    continue;
                }

                if y_index < 0 || y_index >= ROWS as isize {
                    continue;
                }

                // only increases the mine count if the cell isnt a mine
                if board[y_index as usize][x_index as usize] != -1 {
                    board[y_index as usize][x_index as usize] += 1;
                }
            }
        }
    }

}

// draws both the info panel and minefield
fn draw(font: Font, num_flagged: usize, board: &[[i8; COLS]; ROWS], 
        cover: &[[bool; COLS]; ROWS], flags: &Vec<(usize, usize)>, win: &bool, lose: &bool) {
    draw_panel(font, num_flagged);
    draw_board(board, cover, flags);
    
    // displays the appropriate text if the player won or lost
    if *lose || *win {
        draw_text(format!("You {}!", if *win {"Win"} else {"Lose"}).as_str(),
            screen_width() / 2. - 20., 70., 60., BLACK);
    }
}

fn draw_panel(font: Font, num_flagged: usize) {
    // draws the black box for the number of bombs to be written on top of
    draw_rectangle(5., 5., 195., 90., BLACK);

    let bombs_left = MINES as i16 - num_flagged as i16;

    // checks if there are more flags than bombs and adds the "-" to the text if so
    let text = if bombs_left < 0 {
        format!("-{:0>2}", bombs_left.abs() - (bombs_left.abs() / 100 * 100))
    } else {
        format!("{:0>3}", bombs_left)
    };

    // draws the numberof supposed bombs left
    draw_text_ex("888", 5., 90., TextParams {
        font,
        font_size: 80,
        color: Color::new(0.9, 0.16, 0.22, 0.3),
        ..Default::default()});

    draw_text_ex(text.as_str(), 5., 90., TextParams {
        font,
        font_size: 80,
        color: RED,
        ..Default::default()});
}

fn draw_board(board: &[[i8; COLS]; ROWS], cover: &[[bool; COLS]; ROWS], flags: &Vec<(usize, usize)>) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {

            // decides what color to draw the current cell as
            let color = if flags.contains(&(j, i)) {
                // red if it is flagged as a bomb
                RED
            } else if cover[i][j] {
                // light grey if it is still covered up
                Color::from_rgba(220, 220, 220, 255)
            } else {
                // dark grey if it has been uncovered
                Color::from_rgba(192, 192, 192, 255)
            };

            // draws the cell
            draw_rectangle(j as f32 * CELL_SIZE as f32, 
                i as f32 * CELL_SIZE as f32 + OFFSET as f32,
                CELL_SIZE.into(), CELL_SIZE.into(), color);

            // draws the cell outline
            draw_rectangle_lines(j as f32 * CELL_SIZE as f32, 
                i as f32 * CELL_SIZE as f32 + OFFSET as f32,
                CELL_SIZE.into(), CELL_SIZE.into(), 2., BLACK);
            
            // will only draw the number of surrounding bombs if the cell is uncovered
            if !cover[i][j] {

                // matches the number of surrounding bombs to a specific color for the text
                let color = match board[i][j] {
                    1 => Color::from_rgba(0, 0, 255, 255),
                    2 => Color::from_rgba(1, 127, 1, 255),
                    3 => Color::from_rgba(255, 0, 0, 255),
                    4 => Color::from_rgba(1, 0, 128, 255),
                    5 => Color::from_rgba(129, 1, 2, 255),
                    6 => Color::from_rgba(0, 128, 129, 255),
                    // draws 7, 8 and a mine as black
                    _ => BLACK
                };

                // checks if the current cell is a bomb or has no surrounding bombs
                let text = if board[i][j] == -1 {
                    String::from("*")
                } else if board[i][j] == 0{
                    String::from(" ")
                } else {
                    // if it isnt, draw the number
                    format!("{}", board[i][j])
                };
                
                draw_text(text.as_str(), 
                    (j as u16 * CELL_SIZE + (CELL_SIZE / 3)) as f32,
                    (i as u16 * CELL_SIZE + (CELL_SIZE / 4 * 3)) as f32 + OFFSET as f32,
                    CELL_SIZE.into(), color);
            }
        }
    }
}

fn reveal(board: &[[i8; COLS]; ROWS], cover: &mut [[bool; COLS]; ROWS],
        flags: &Vec<(usize, usize)>, x: usize, y: usize) {
    // prevents the user from clicking  on a flagged cell
    if flags.contains(&(x as usize, y as usize)) {
        return;
    }
    
    // uncoveres the current cell
    cover[y as usize][x as usize] = false;

    // does not preform flood fill on non-empty cells
    if board[y as usize][x as usize] != 0 {
        return
    }

    // preforms the flood fill algorithm 
    // in order to reveal all adjacent empty cells
    for i in 0..=2 {
        for j in 0..=2 {
            // this skips the current cell
            if i == 1 && j == 1 {
                continue;
            }

            let x_index = x as isize + (i as isize - 1);
            let y_index = y as isize + (j as isize - 1);

            // border checking

            if x_index < 0 || x_index >= COLS as isize {
                continue;
            }

            if y_index < 0 || y_index >= ROWS as isize {
                continue;
            }

            // will only continue on still-covered cells
            if cover[y_index as usize][x_index as usize] == true {
                reveal(board, cover, flags, x_index as usize, y_index as usize);
            }
        }
    }
}

fn flag(flags: &mut Vec<(usize, usize)>, cover: &[[bool; COLS]; ROWS], x: usize, y: usize) {
    // will only flag still-covered cells
    if cover[y][x] == true {
        // will flag the cell if it isn't already flagged
        if !flags.contains(&(x, y)) {
            flags.push((x, y));
        } else {
            // will unflag cell if it is flagged
            let pos = flags.iter().position(|&vals| vals == (x, y)).unwrap();
            flags.remove(pos);
        }
    }
}

// flags all mine positions so the player 
// can see their locations easier
fn flag_all_mines(flags: &mut Vec<(usize, usize)>, board: &[[i8; COLS]; ROWS]) {
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == -1 && !flags.contains(&(x, y)) {
                flags.push((x, y));
            }
        }
    }
}

fn win_check(cover: &[[bool; COLS]; ROWS]) -> bool{
    // checks to see if the number of still-covered cells is equal to
    // the number of mines, and if true, means that the game has been won
    return cover.iter()
        .flat_map(|a| a.iter())
        .filter(|x| **x == true)
        .collect::<Vec<_>>().len() as u16 == MINES;
}

fn uncover_all_mines(board: &[[i8; COLS]; ROWS], cover: &mut [[bool; COLS]; ROWS]) {
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == -1 {
                cover[y][x] = false;
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    // seeds the rnadom number generator using the current time
    rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64() as u64);
    
    // loads in the font for the mines display
    let font = load_ttf_font("./DSEG7Classic-Bold.ttf").await.unwrap();

    let mut board: [[i8; COLS]; ROWS] = [[0; COLS]; ROWS];
    let mut cover: [[bool; COLS]; ROWS] = [[true; COLS]; ROWS];
    let mut flags: Vec<(usize, usize)> = Vec::new();

    let mut win = false;
    let mut lose = false;

    // generates the initial minefield
    generate_board(&mut board);

    // main game loop
    loop {
        // clears the screen with a mid-grey color
        clear_background(Color::from_rgba(138, 138, 138, 255));

        // gets the mouse position
        let (mouse_x, mouse_y) = mouse_position();

        // converts mouse position into a position on the minefield
        let (mut grid_x, mut grid_y) = ((mouse_x / CELL_SIZE as f32).floor() as usize,
            ((mouse_y - OFFSET as f32) / CELL_SIZE as f32).floor() as usize);
        
        // it is possible to be 1 cell over when on the bottom and right edges,
        // this prevents that from crashing the game
        if grid_x >= COLS {
            grid_x = grid_x - (grid_x - (COLS - 1));
        }

        if grid_y >= ROWS {
            grid_y = grid_y - (grid_y - (ROWS - 1));
        }

        // will only check what spot the user clicked
        // if the mouse is in the playing field
        if mouse_y > OFFSET as f32 && !win && !lose{
            if is_mouse_button_pressed(MouseButton::Left) {
                // reveals the clicked-on cell
                reveal(&board, &mut cover, &flags, grid_x, grid_y);

                // checks if the player clicked a mine
                if !flags.contains(&(grid_x, grid_y)) && board[grid_y][grid_x] == -1 {
                    lose = true;
                    uncover_all_mines(&board, &mut cover);
                }

                // checks if the player has found all the mines
                if !lose {
                    win = win_check(&cover);
                }

                // flags all the mines if the player has won
                if win {
                    flag_all_mines(&mut flags, &board);
                }

            } else if is_mouse_button_pressed(MouseButton::Right) {
                flag(&mut flags, &cover, grid_x, grid_y);
            }
        }

        // restarts tne game
        if is_key_pressed(KeyCode::Space) {
            setup(&mut board, &mut cover, &mut flags, &mut win, &mut lose);
        }

        // draws everything to the screen
        draw(font, flags.len(), &board, &cover, &flags, &win, &lose);

        // waits for frame to end before continuing
        next_frame().await
    }
}