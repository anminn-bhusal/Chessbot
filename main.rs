use macroquad::prelude::*;
use shakmaty::{Chess, Position, Square, Role, EnPassantMode}; 
use shakmaty::fen::Fen;

use macroquad::color::Color as RenderColor;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Chess Client (Python AI Backend)".to_owned(),
        window_width: 640,
        window_height: 640,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut chess_game = Chess::default();
    let square_size = 640.0 / 8.0;
    let mut selected_square: Option<Square> = None;
    
    let http_client = reqwest::blocking::Client::new();

    loop {
        clear_background(DARKGRAY);

        if chess_game.turn() == shakmaty::Color::Black && !chess_game.is_game_over() {
            let fen_string = Fen::from_position(chess_game.clone(), EnPassantMode::Legal).to_string();
            let payload = serde_json::json!({ "fen": fen_string });

            if let Ok(res) = http_client.post("http://127.0.0.1:5000/predict").json(&payload).send() {
                if let Ok(json_response) = res.json::<serde_json::Value>() {
                    if let Some(move_str) = json_response["move"].as_str() {
                        if let Ok(ai_move) = move_str.parse::<shakmaty::uci::Uci>() {
                            if let Ok(valid_move) = ai_move.to_move(&chess_game) {
                                chess_game = chess_game.play(&valid_move).unwrap();
                            }
                        }
                    }
                }
            }
        }

        if chess_game.turn() == shakmaty::Color::White && is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            
            let file_idx = (mouse_x / square_size).floor() as i32;
            let rank_idx = 7 - (mouse_y / square_size).floor() as i32; 

            if (0..8).contains(&file_idx) && (0..8).contains(&rank_idx) {
                let clicked_square = Square::new((rank_idx * 8 + file_idx) as u32);

                match selected_square {
                    Some(from_sq) => {
                        let legal_moves = chess_game.legal_moves();
                        let intended_move = legal_moves.iter().find(|m| {
                            m.from() == Some(from_sq) && m.to() == clicked_square
                        });

                        if let Some(valid_move) = intended_move {
                            chess_game = chess_game.play(valid_move).unwrap();
                            selected_square = None; 
                        } else {
                            let current_board = chess_game.board();
                            if current_board.piece_at(clicked_square).map(|p| p.color) == Some(chess_game.turn()) {
                                selected_square = Some(clicked_square);
                            } else {
                                selected_square = None; 
                            }
                        }
                    }
                    None => {
                        let current_board = chess_game.board();
                        if let Some(piece) = current_board.piece_at(clicked_square) {
                            if piece.color == chess_game.turn() {
                                selected_square = Some(clicked_square);
                            }
                        }
                    }
                }
            }
        }

        let rendering_board = chess_game.board();

        for rank in 0..8 {
            for file in 0..8 {
                let x = file as f32 * square_size;
                let y = (7 - rank) as f32 * square_size;
                let current_square = Square::new((rank * 8 + file) as u32);

                let mut square_color = if (file + rank) % 2 == 0 { 
                    RenderColor::from_rgba(238, 238, 210, 255) 
                } else { 
                    RenderColor::from_rgba(118, 150, 86, 255)  
                };

                if selected_square == Some(current_square) {
                    square_color = RenderColor::from_rgba(130, 170, 255, 255); 
                }

                draw_rectangle(x, y, square_size, square_size, square_color);

                if let Some(piece) = rendering_board.piece_at(current_square) {
                    let text_color = match piece.color {
                        shakmaty::Color::White => WHITE,
                        shakmaty::Color::Black => BLACK,
                    };

                    let letter = match piece.role {
                        Role::Pawn => "P",
                        Role::Knight => "N",
                        Role::Bishop => "B",
                        Role::Rook => "R",
                        Role::Queen => "Q",
                        Role::King => "K",
                    };

                    draw_text(
                        letter, 
                        x + (square_size * 0.35), 
                        y + (square_size * 0.65), 
                        40.0, 
                        text_color
                    );
                }
            }
        }

        if chess_game.is_checkmate() {
            draw_text("CHECKMATE!", 20.0, 40.0, 30.0, RED);
        } else if chess_game.is_stalemate() {
            draw_text("STALEMATE!", 20.0, 40.0, 30.0, ORANGE);
        }

        next_frame().await
    }
}