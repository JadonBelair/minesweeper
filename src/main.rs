use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets, UiContent};
use std::borrow::Cow;
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

#[derive(Clone, Copy)]
struct Cell {
    value: i8,
    covered: bool,
    flagged: bool
}

impl Cell {
    fn new() -> Self {
        Self {
            value: 0,
            covered: true,
            flagged: false
        }
    }

    // will only flag the cell if it is still covered
    pub fn flag(&mut self) {
        if self.covered {
            self.flagged = !self.flagged;
        }
    }
}

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
fn setup(board: &mut [[Cell; COLS]; ROWS], win: &mut bool, lose: &mut bool) {
    *board = [[Cell::new(); COLS]; ROWS];

    *win = false;
    *lose = false;

    generate_board(board);
}

// generates a random level on the specified board
fn generate_board(board: &mut [[Cell; COLS]; ROWS]) {
    let mut mine_count = 0;

    // keeps trying to add mines until the set amount is reached
    while mine_count < MINES {
        // generates a random position for the mine
        let x = rand::gen_range(0, COLS);
        let y = rand::gen_range(0, ROWS);

        // checks if that spot is already a mine,
        // and if it is, tries again
        if board[y][x].value == -1 {
            continue;
        }

        // sets the spot to a mine and updates the mine count
        board[y][x].value = -1;
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
                if board[y_index as usize][x_index as usize].value != -1 {
                    board[y_index as usize][x_index as usize].value += 1;
                }
            }
        }
    }

}

// draws both the info panel and minefield
fn draw(font: Font, board: &[[Cell; COLS]; ROWS], win: &bool, lose: &bool) {

    // clears the screen with a mid-grey color
    clear_background(Color::from_rgba(138, 138, 138, 255));
    
    let num_flagged = get_total_flagged(board);
    draw_panel(font, num_flagged);

    draw_board(board);
    
    // displays the appropriate text if the player won or lost
    if *lose || *win {
        draw_text(format!("You {}!", if *win {"Win"} else {"Lose"}).as_str(),
            screen_width() / 2. - 20., 70., 60., BLACK);
    }
}

// displays to the user how many bombs there are left, 
// assuming that all their flags are right
fn draw_panel(font: Font, num_flagged: i16) {
    // draws the black box for the number of bombs to be written on top of
    draw_rectangle(5., 5., 195., 90., BLACK);

    let bombs_left = MINES as i16 - num_flagged;

    // checks if there are more flags than bombs and adds the "-" to the text if so
    let text = if bombs_left < 0 {
        format!("-{:0>2}", bombs_left.abs() - (bombs_left.abs() / 100 * 100))
    } else {
        format!("{:0>3}", bombs_left)
    };

    // draws the dark red background text to show unused segments of text
    draw_text_ex("888", 5., 90., TextParams {
        font,
        font_size: 80,
        color: Color::new(0.9, 0.16, 0.22, 0.3),
        ..Default::default()});

    // draws the number of supposed bombs left
    draw_text_ex(text.as_str(), 5., 90., TextParams {
        font,
        font_size: 80,
        color: RED,
        ..Default::default()});
}

// draws the minefield to the screen
fn draw_board(board: &[[Cell; COLS]; ROWS]) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {

            // decides what color to draw the current cell as
            let color = if board[i][j].flagged {
                // red if it is flagged as a bomb
                RED
            } else if board[i][j].covered {
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
            if !board[i][j].covered {

                // matches the number of surrounding bombs to a specific color for the text
                let color = match board[i][j].value {
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
                let text = if board[i][j].value == -1 {
                    String::from("*")
                } else if board[i][j].value == 0{
                    String::from(" ")
                } else {
                    // if it isnt, draw the number
                    format!("{}", board[i][j].value)
                };
                
                draw_text(text.as_str(), 
                    (j as u16 * CELL_SIZE + (CELL_SIZE / 3)) as f32,
                    (i as u16 * CELL_SIZE + (CELL_SIZE / 4 * 3)) as f32 + OFFSET as f32,
                    CELL_SIZE.into(), color);
            }
        }
    }
}

// reveals the the cell at (x, y) to the player and 
// if the cell has no surrounding bombs any adjacent empty cels
fn reveal(board: &mut [[Cell; COLS]; ROWS], x: usize, y: usize) {
    // prevents the user from clicking  on a flagged cell
    if board[y][x].flagged {
        return;
    }
    
    // uncoveres the current cell
    board[y as usize][x as usize].covered = false;

    // does not preform flood fill on non-empty cells
    if board[y as usize][x as usize].value != 0 {
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
            if board[y_index as usize][x_index as usize].covered == true {
                reveal(board, x_index as usize, y_index as usize);
            }
        }
    }
}

// gets the total number of flagged cells
fn get_total_flagged(board: &[[Cell; COLS]; ROWS]) -> i16 {
    return board.iter().flat_map(|x| x.iter())
            .filter(|x| x.flagged == true).count() as i16;
}

// flags all mine positions so the player 
// can see their locations easier
fn flag_all_mines(board: &mut [[Cell; COLS]; ROWS]) {
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x].value == -1 {
                board[y][x].flagged = true;
            }
        }
    }
}

// checks to see if the number of still-covered cells is equal to
// the number of mines, and if true, means that the game has been won
fn win_check(board: &[[Cell; COLS]; ROWS]) -> bool{
    // this is a mess, might see if there is a nicer way of doing this later
    return board.iter()
        .flat_map(|a| a.iter())
        .filter(|x| x.covered == true)
        .count() as u16 == MINES;
}

// goes through every cell and if it is a bomb, uncoveres it
fn uncover_all_mines(board: &mut [[Cell; COLS]; ROWS]) {
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x].value == -1 {
                board[y][x].covered = false;
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

    let mut board: [[Cell; COLS]; ROWS] = [[Cell::new(); COLS]; ROWS];

    let mut win = false;
    let mut lose = false;

    // generates the initial minefield
    generate_board(&mut board);

    // main game loop
    loop {

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
                reveal(&mut board, grid_x, grid_y);

                // checks if the player clicked a mine
                if !board[grid_y][grid_x].flagged && board[grid_y][grid_x].value == -1 {
                    lose = true;
                    uncover_all_mines(&mut board);
                }

                // checks if the player has found all the mines
                if !lose {
                    win = win_check(&board);
                }

                // flags all the mines if the player has won
                if win {
                    flag_all_mines(&mut board);
                }

            } else if is_mouse_button_pressed(MouseButton::Right) {
                board[grid_y][grid_x].flag();
            }
        }

        let button = widgets::Button::new(UiContent::Label(Cow::Borrowed("Reset")))
                    .position(vec2(screen_width() - 95., 5.))
                    .size(vec2(90., 90.));

        // restarts tne game
        if is_key_pressed(KeyCode::Space) || button.ui(&mut *root_ui()) {
            setup(&mut board, &mut win, &mut lose);
        }

        // draws everything to the screen
        draw(font, &board, &win, &lose);

        // waits for frame to end before continuing
        next_frame().await
    }
}