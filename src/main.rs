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

    let mut turn = false;

    loop {
        clear_background(WHITE);

        draw_edges();

        draw_board();

        selected_square = set_new_selected_square(selected_square, turn, &board);

        selected_piece = get_piece_in_square(coordinate_to_grid_square(selected_square), &board);

        if let Some(piece) = selected_piece {
            possible_moves = piece.possible_moves();
            selected_square = move_piece(
                selected_square,
                piece,
                &possible_moves,
                &mut board,
                &mut turn,
            );
        } else {
            possible_moves.clear();
        }

        draw_selected_square(selected_square);

        draw_possible_moves(selected_square, &possible_moves, &board);

        draw_pieces(&board).await;

        draw_turn_string(turn);

        next_frame().await
    }
}

fn move_piece(
    selected_square: (f32, f32),
    selected_piece: Piece,
    possible_moves: &Vec<(i32, i32)>,
    board: &mut Board,
    turn: &mut bool,
) -> (f32, f32) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_grid_pos_x = snap_to_grid(mouse_position().0);
        let mouse_grid_pos_y = snap_to_grid(mouse_position().1);

        let clicked_square = coordinate_to_grid_square((mouse_grid_pos_x, mouse_grid_pos_y));
        let current_square = coordinate_to_grid_square(selected_square);

        for mv in possible_moves {
            let possible_new_coord = (
                (current_square.0 as i32 + mv.0) as usize,
                (current_square.1 as i32 - mv.1) as usize,
            );

            if possible_new_coord.0 < 8
                && possible_new_coord.1 < 8
                && possible_new_coord.0 == clicked_square.0
                && possible_new_coord.1 == clicked_square.1
                && !check_if_square_is_blocked(possible_new_coord, &board, current_square)
            {
                board.set(current_square.0, current_square.1, None);
                board.set(
                    possible_new_coord.0,
                    possible_new_coord.1,
                    Some(selected_piece),
                );
                *turn = !*turn;
                return (-1.0, -1.0);
            }
        }
    }

    selected_square
}

fn get_piece_in_square(coordinates: (usize, usize), board: &Board) -> Option<Piece> {
    return board.get(coordinates.0, coordinates.1);
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

fn draw_possible_moves(selected_square: (f32, f32), moves: &Vec<(i32, i32)>, board: &Board) {
    let block_size = screen_width() / 9.0;

    if selected_square != (-1.0, -1.0) {
        for mv in moves {
            let coordinate = (
                selected_square.0 + mv.0 as f32 * block_size,
                selected_square.1 - mv.1 as f32 * block_size,
            );

            if *mv != (0, 0)
                && coordinate.0 > 0.0
                && coordinate.0 < screen_width() - block_size
                && coordinate.1 > 0.0
                && coordinate.1 < screen_height() - block_size
                && !check_if_square_is_blocked(
                    coordinate_to_grid_square(coordinate),
                    &board,
                    coordinate_to_grid_square(selected_square),
                )
            {
                draw_rectangle(coordinate.0, coordinate.1, block_size, block_size, GREEN);
            }
        }
    }
}

fn check_if_square_is_blocked(
    target_coordinate: (usize, usize),
    board: &Board,
    start_coordinate: (usize, usize),
) -> bool {
    // Knights can jump over everything
    let piece = get_piece_in_square(start_coordinate, board);
    if let Some(piece) = piece {
        if piece.kind == PieceType::Knight {
            return false;
        }

        if piece.kind == PieceType::Pawn {
            let dx = target_coordinate.0 as i32 - start_coordinate.0 as i32;
            let dy = target_coordinate.1 as i32 - start_coordinate.1 as i32;

            if dy.abs() > 1 && (start_coordinate.1 != 6 && start_coordinate.1 != 1) {
                return true;
            }

            if dx == 0
                && board
                    .get(target_coordinate.0, target_coordinate.1)
                    .is_some()
            {
                return true;
            }

            if dx.abs() == 1 && dy.abs() == 1 {
                if board
                    .get(target_coordinate.0, target_coordinate.1)
                    .is_none()
                {
                    return true;
                }
                if let Some(target_piece) = board.get(target_coordinate.0, target_coordinate.1) {
                    return target_piece.color == piece.color;
                }
            }
        }
    }

    let dx = (target_coordinate.0 as i32 - start_coordinate.0 as i32).signum();
    let dy = (target_coordinate.1 as i32 - start_coordinate.1 as i32).signum();

    let mut current_x = start_coordinate.0 as i32 + dx;
    let mut current_y = start_coordinate.1 as i32 + dy;

    while (current_x as usize, current_y as usize) != target_coordinate {
        if board.get(current_x as usize, current_y as usize).is_some() {
            return true;
        }
        current_x += dx;
        current_y += dy;
    }

    if let Some(target_piece) = board.get(target_coordinate.0, target_coordinate.1) {
        if let Some(start_piece) = piece {
            return target_piece.color == start_piece.color;
        }
    }

    false
}

fn set_new_selected_square(selected_square: (f32, f32), turn: bool, board: &Board) -> (f32, f32) {
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
            let piece = get_piece_in_square(coordinate_to_grid_square(new_selected_square), board);
            if let Some(piece) = piece {
                if (!turn && piece.color == Player::Black) || (turn && piece.color == Player::White)
                {
                    return new_selected_square;
                }
            }
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

async fn draw_pieces(board: &Board) {
    let mut piece_index = 0;
    for piece in board.get_all_pieces() {
        let row = piece_index / 8;
        let col = piece_index % 8;
        let block_size = screen_width() / 9.0;
        let block_offset = screen_width() / 18.0 / 3.0;

        let piece_x = col as f32 * block_size + block_size / 2.0;
        let piece_y = row as f32 * block_size + block_size / 2.0;

        if let Some(piece) = piece {
            let color_string = format!("{:?}", piece.color).to_lowercase(); // "black"
            let piece_str = format!("{:?}", piece.kind).to_lowercase(); // "queen"

            let texture_path = format!("images/{}_{}.png", color_string, piece_str);

            let texture: Texture2D = load_texture(&texture_path).await.unwrap();

            draw_texture(
                &texture,
                piece_x + block_offset,
                piece_y + block_offset,
                WHITE,
            );
        }

        piece_index += 1;
    }
}

fn draw_turn_string(turn: bool) {
    let mut turn_string = "Blacks turn";

    if turn {
        turn_string = "Whites turn";
    }

    draw_text(&turn_string, 20.0, 30.0, 30.0, BLACK);
}

fn draw_board() {
    let block_size = screen_width() / 9.0;
    let block_offset = screen_width() / 18.0;

    for i in 0..8 {
        for j in 0..8 {
            let mut block_color: Color = BEIGE;

            if i % 2 == 1 && j % 2 == 0 {
                block_color = BROWN;
            } else if i % 2 == 0 && j % 2 == 1 {
                block_color = BROWN;
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
