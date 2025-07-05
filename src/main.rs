mod chess;
use chess::{Board, Piece, PieceType, Player};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "chuss".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut board: Board = Board::start_position();

    let mut selected_square: (f32, f32) = (-1.0, -1.0);
    let mut selected_piece: Option<Piece> = None;
    let mut possible_moves: Vec<(i32, i32)> = Vec::new();

    loop {
        clear_background(LIGHTGRAY);

        draw_edges();

        draw_board();

        selected_square = set_new_selected_square(selected_square);

        selected_piece = get_selected_piece(selected_square, &board);

        if let Some(piece) = selected_piece {
            possible_moves = piece.possible_moves();
            selected_square = move_piece(selected_square, piece, &possible_moves, &mut board);
        } else {
            possible_moves.clear();
        }

        draw_selected_square(selected_square);

        draw_possible_moves(selected_square, &possible_moves);

        draw_pieces(&board);

        next_frame().await
    }
}

fn move_piece(
    selected_square: (f32, f32),
    selected_piece: Piece,
    possible_moves: &Vec<(i32, i32)>,
    board: &mut Board,
) -> (f32, f32) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_grid_pos_x = snap_to_grid(mouse_position().0);
        let mouse_grid_pos_y = snap_to_grid(mouse_position().1);

        let clicked_square = coordinate_to_grid_square((mouse_grid_pos_x, mouse_grid_pos_y));
        let current_square = coordinate_to_grid_square(selected_square);

        for mv in possible_moves {
            let possible_new_coord = (
                (current_square.0 as i32 - mv.0) as usize,
                (current_square.1 as i32 - mv.1) as usize,
            );

            if possible_new_coord.0 == clicked_square.0 && possible_new_coord.1 == clicked_square.1
            {
                board.set(current_square.0, current_square.1, None);
                board.set(
                    possible_new_coord.0,
                    possible_new_coord.1,
                    Some(selected_piece),
                );
                return (-1.0, -1.0);
            }
        }
    }

    selected_square
}

fn get_selected_piece(coordinates: (f32, f32), board: &Board) -> Option<Piece> {
    if coordinates.0 >= 0.0 && coordinates.1 >= 0.0 {
        let formatted_coordinates: (usize, usize) = coordinate_to_grid_square(coordinates);

        return board.get(formatted_coordinates.0, formatted_coordinates.1);
    } else {
        return None;
    }
}

fn coordinate_to_grid_square(coordinates: (f32, f32)) -> (usize, usize) {
    let block_offset: f32 = screen_width() / 18.0;
    let block_size: f32 = screen_width() / 9.0;

    let adjusted_coordinates: (f32, f32) = (
        ((coordinates.0 - block_offset) / block_size),
        ((coordinates.1 - block_offset) / block_size),
    );

    let formatted_coordinates: (usize, usize) = (
        adjusted_coordinates.0.round() as usize,
        adjusted_coordinates.1.round() as usize,
    );

    return formatted_coordinates;
}

fn draw_possible_moves(selected_square: (f32, f32), moves: &Vec<(i32, i32)>) {
    let block_size = screen_width() / 9.0;

    if selected_square != (-1.0, -1.0) {
        for mv in moves {
            if *mv != (0, 0) {
                let coordinate = (
                    selected_square.0 + mv.0 as f32 * block_size,
                    selected_square.1 - mv.1 as f32 * block_size,
                );

                draw_rectangle(coordinate.0, coordinate.1, block_size, block_size, GREEN);
            }
        }
    }
}

fn set_new_selected_square(selected_square: (f32, f32)) -> (f32, f32) {
    if selected_square.0 == -10.0 {
        return (-1.0, -1.0);
    } else if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_grid_pos_x = mouse_position().0;
        let mouse_grid_pos_y = mouse_position().1;

        let new_selected_square = (
            snap_to_grid(mouse_grid_pos_x),
            snap_to_grid(mouse_grid_pos_y),
        );

        let block_size = screen_width() / 9.0;

        if new_selected_square == selected_square {
            return (-1.0, -1.0);
        } else if selected_square == (-1.0, -1.0)
            && new_selected_square.0 > 0.0
            && new_selected_square.1 > 0.0
            && new_selected_square.0 < screen_width() - block_size
            && new_selected_square.0 < screen_height() - block_size
        {
            return new_selected_square;
        }
    }

    return selected_square;
}

fn draw_selected_square(selected_square: (f32, f32)) {
    let block_size = screen_width() / 9.0;

    if selected_square != (-1.0, -1.0) {
        draw_rectangle(
            selected_square.0,
            selected_square.1,
            block_size,
            block_size,
            RED,
        );
    }
}

fn snap_to_grid(value: f32) -> f32 {
    let block_size = screen_width() / 9.0;
    let block_offset = screen_width() / 18.0;

    let index = ((value - block_offset) / block_size).floor();
    index * block_size + block_offset
}

fn draw_pieces(board: &Board) {
    let mut piece_index = 0;
    for piece in board.get_all_pieces() {
        let row = piece_index / 8;
        let col = piece_index % 8;
        let block_size = screen_width() / 9.0;
        let block_offset = screen_width() / 9.0 / 2.0;
        let piece_x = col as f32 * block_size + 5.0 + block_size / 2.0;
        let piece_y = row as f32 * block_size + block_size / 2.0;

        if let Some(piece) = piece {
            let color_string = format!("{:?}", piece.color);
            let piece_str = format!("{:?}", piece.kind);
            draw_text(
                &color_string,
                piece_x,
                piece_y + block_offset,
                30.0,
                DARKGRAY,
            );
            draw_text(
                &piece_str,
                piece_x,
                piece_y + block_offset * 1.5,
                30.0,
                DARKGRAY,
            );
        }

        piece_index += 1;
    }
}

fn draw_board() {
    let block_size = screen_width() / 9.0;
    let block_offset = screen_width() / 18.0;

    for i in 0..8 {
        for j in 0..8 {
            let mut block_color: Color = WHITE;

            if i % 2 == 1 && j % 2 == 0 {
                block_color = BLACK;
            } else if i % 2 == 0 && j % 2 == 1 {
                block_color = BLACK;
            }

            draw_rectangle(
                i as f32 * block_size + block_offset,
                j as f32 * block_size + block_offset,
                block_size,
                block_size,
                block_color,
            );
        }
    }
}

fn draw_edges() {
    let characters: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let block_offset = screen_width() / 18.0;

    for n in numbers {
        draw_text(
            &n.to_string(),
            block_offset / 2.0,
            n as f32 * screen_height() / 9.0 - screen_height() / 9.0 / 2.0 + block_offset,
            30.0,
            BLACK,
        );
    }

    for c in 0..characters.len() {
        draw_text(
            &characters[c].to_string(),
            c as f32 * screen_width() / 9.0 + screen_width() / 9.0 / 2.0 + block_offset,
            screen_height() - block_offset / 2.0,
            30.0,
            BLACK,
        );
    }
}
