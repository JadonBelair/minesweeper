use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

const CELL_SIZE: u16 = 40;
const ROWS: usize = 16;
const COLS: usize = 16;
const OFFSET: i32 = 100;
const MINES: u16 = 40;

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

fn setup(board: &mut [[i8; 16]; 16], cover: &mut [[bool; 16]; 16], flags: &mut Vec<(usize, usize)>) {
    *board = [[0; COLS]; ROWS];
    *cover = [[true; COLS]; ROWS];
    *flags = Vec::new();

    generate_board(board);
}

fn generate_board(board: &mut [[i8; 16]; 16]) {
    let mut mine_count = 0;

    while mine_count < MINES {
        let x = rand::gen_range(0, COLS);
        let y = rand::gen_range(0, ROWS);

        if board[y][x] == -1 {
            continue;
        }

        board[y][x] = -1;
        mine_count += 1;

        for i in 0..=2 {
            for j in 0..=2 {

                if i == 1 && j == 1 {
                    continue;
                }

                let x_index = x as isize + (i as isize - 1);
                let y_index = y as isize + (j as isize - 1);

                if x_index < 0 || x_index >= COLS as isize {
                    continue;
                }

                if y_index < 0 || y_index >= ROWS as isize {
                    continue;
                }

                if board[y_index as usize][x_index as usize] != -1 {
                    board[y_index as usize][x_index as usize] += 1;
                }
            }
        }
    }

}

fn draw_panel(font: Font, num_flagged: usize) {
    draw_rectangle(5., 5., 130., 90., BLACK);

    let bombs_left = MINES - num_flagged as u16;

    draw_text_ex(format!("{:0>2}", bombs_left).as_str(), 5., 90., TextParams { font, font_size: 80, color: RED, ..Default::default() });
}

fn draw_board(board: &[[i8; COLS]; ROWS], cover: &[[bool; COLS]; ROWS], flags: &Vec<(usize, usize)>) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {

            let color = if flags.contains(&(j, i)) {
                RED
            } else if cover[i][j] {
                Color::from_rgba(220, 220, 220, 255)
            } else {
                Color::from_rgba(192, 192, 192, 255)
            };

            draw_rectangle(j as f32 * CELL_SIZE as f32, i as f32 * CELL_SIZE as f32 + OFFSET as f32,
                CELL_SIZE.into(), CELL_SIZE.into(), color);

            draw_rectangle_lines(j as f32 * CELL_SIZE as f32, i as f32 * CELL_SIZE as f32 + OFFSET as f32,
                CELL_SIZE.into(), CELL_SIZE.into(), 2., BLACK);

            if !cover[i][j] {

                let color = match board[i][j] {
                    1 => Color::from_rgba(0, 0, 255, 255),
                    2 => Color::from_rgba(1, 127, 1, 255),
                    3 => Color::from_rgba(255, 0, 0, 255),
                    4 => Color::from_rgba(1, 0, 128, 255),
                    5 => Color::from_rgba(129, 1, 2, 255),
                    6 => Color::from_rgba(0, 128, 129, 255),
                    7 => BLACK,
                    _ => BLACK
                };

                let text = if board[i][j] == -1 {
                    String::from("*")
                } else if board[i][j] == 0{
                    String::from(" ")
                } else {
                    format!("{}", board[i][j])
                };
                
                draw_text(text.as_str(), j as f32 * CELL_SIZE as f32 + (CELL_SIZE / 3) as f32, 
                    i as f32 * CELL_SIZE as f32 + (CELL_SIZE / 4 * 3) as f32 + OFFSET as f32, CELL_SIZE.into(), color);
            }
        }
    }
}

fn reveal(board: &[[i8; COLS]; ROWS], cover: &mut [[bool; COLS]; ROWS], flags: &Vec<(usize, usize)>, x: usize, y: usize) {
    if flags.contains(&(x as usize, y as usize)) {
        return;
    }
    
    cover[y as usize][x as usize] = false;

    if board[y as usize][x as usize] != 0 {
        return
    }

    for i in 0..=2 {
        for j in 0..=2 {
            if i == 1 && j == 1 {
                continue;
            }

            let x_index = x as isize + (i as isize - 1);
            let y_index = y as isize + (j as isize - 1);

            if x_index < 0 || x_index >= COLS as isize {
                continue;
            }

            if y_index < 0 || y_index >= ROWS as isize {
                continue;
            }

            if cover[y_index as usize][x_index as usize] == true {
                reveal(board, cover, flags, x_index as usize, y_index as usize);
            }
        }
    }
}

fn flag(flags: &mut Vec<(usize, usize)>, cover: &[[bool; COLS]; ROWS], x: usize, y: usize) {
    if cover[y][x] == true {
        if !flags.contains(&(x, y)) {
            flags.push((x, y));
        } else {
            let pos = flags.iter().position(|&index| index == (x, y)).unwrap();
            flags.remove(pos);
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64() as u64);
    
    let font = load_ttf_font("./DSEG7Classic-Bold.ttf")
        .await
        .unwrap();

    let mut board: [[i8; COLS]; ROWS] = [[0; COLS]; ROWS];
    let mut cover: [[bool; COLS]; ROWS] = [[true; COLS]; ROWS];
    let mut flags: Vec<(usize, usize)> = Vec::new();

    setup(&mut board, &mut cover, &mut flags);

    loop {
        clear_background(Color::from_rgba(138, 138, 138, 255));

        let (mouse_x, mouse_y) = mouse_position();

        let (mut grid_x, mut grid_y) = ((mouse_x / CELL_SIZE as f32).floor() as usize, 
            ((mouse_y - OFFSET as f32) / CELL_SIZE as f32).floor() as usize);
        
        if grid_x >= COLS {
            grid_x = grid_x - (grid_x - (COLS - 1));
        }

        if grid_y >= ROWS {
            grid_y = grid_y - (grid_y - (COLS - 1));
        }

        if mouse_y > OFFSET as f32 {
            if is_mouse_button_pressed(MouseButton::Left) {
                reveal(&board, &mut cover, &flags, grid_x, grid_y);           
            } else if is_mouse_button_pressed(MouseButton::Right) {
                flag(&mut flags, &cover, grid_x, grid_y);
            }
        }

        draw_panel(font, flags.len());

        draw_board(&board, &cover, &flags);

        next_frame().await
    }
}