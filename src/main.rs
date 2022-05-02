use macroquad::prelude::*;

const CELL_SIZE: u16 = 40;
const ROWS: usize = 16;
const COLS: usize = 16;
const MINES: u8 = 40;

fn window_conf() -> Conf {
    Conf {
        window_title: "Minesweeper".to_owned(),
        window_height: (CELL_SIZE * ROWS as u16).into(),
        window_width: (CELL_SIZE * COLS as u16).into(),
        window_resizable: false,
        fullscreen: false,
        ..Default::default()
    }
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

fn draw_board(board: &[[i8; COLS]; ROWS], cover: &[[bool; COLS]; ROWS], flags: &Vec<(usize, usize)>) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {

            let color = if flags.contains(&(j, i)) {
                RED
            } else {
                GRAY
            };

            draw_rectangle(j as f32 * CELL_SIZE as f32, i as f32 * CELL_SIZE as f32,
                CELL_SIZE.into(), CELL_SIZE.into(), color);

            draw_rectangle_lines(j as f32 * CELL_SIZE as f32, i as f32 * CELL_SIZE as f32,
                CELL_SIZE.into(), CELL_SIZE.into(), 2., BLACK);

            if !cover[i][j] {

                let text = if board[i][j] == -1 {
                    String::from("*")
                } else {
                    format!("{}", board[i][j])
                };
                
                draw_text(text.as_str(), j as f32 * CELL_SIZE as f32 + (CELL_SIZE / 3) as f32, 
                    i as f32 * CELL_SIZE as f32 + (CELL_SIZE / 4 * 3) as f32, CELL_SIZE.into(), BLACK);
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

    let mut board: [[i8; COLS]; ROWS] = [[0; COLS]; ROWS];
    let mut cover: [[bool; COLS]; ROWS] = [[true; COLS]; ROWS];
    let mut flags: Vec<(usize, usize)> = Vec::new();

    generate_board(&mut board);

    loop {
        clear_background(Color::from_rgba(138, 138, 138, 255));

        let (mut mouse_x, mut mouse_y) = ((mouse_position().0 / CELL_SIZE as f32).floor() as usize, 
            (mouse_position().1 / CELL_SIZE as f32).floor() as usize);
        
        mouse_x = mouse_x - (mouse_x - 15);
        mouse_y = mouse_y - (mouse_y - 15);

        if is_mouse_button_pressed(MouseButton::Left) {
            reveal(&board, &mut cover, &flags, mouse_x, mouse_y);           
        } else if is_mouse_button_pressed(MouseButton::Right) {
            flag(&mut flags, &cover, mouse_x, mouse_y);
        }

        draw_board(&board, &cover, &flags);

        next_frame().await
    }
}