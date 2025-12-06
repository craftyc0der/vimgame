use macroquad::prelude::*;

mod game;
mod input;
mod level;
mod render;
mod solver;

use game::{GameState, GameStatus};
use input::InputHandler;
use level::Level;

use solver::Solver;

async fn load_level_from_file(path: &str) -> Level {
    let json_content = load_string(path).await.expect("Failed to read level file");
    serde_json::from_str(&json_content).expect("Failed to parse level JSON")
}

#[macroquad::main("VIM Game")]
async fn main() {
    let resources = render::Resources::new().await;

    let levels = vec![
        "assets/levels/level_01.json",
        "assets/levels/level_02.json",
        "assets/levels/level_03.json",
        "assets/levels/level_04.json",
        "assets/levels/level_05.json",
        "assets/levels/level_06.json",
        "assets/levels/level_07.json",
        "assets/levels/level_08.json",
        "assets/levels/level_09.json",
        "assets/levels/level_10.json",
    ];

    let mut current_level_index: Option<usize> = None;
    let mut game_state: Option<GameState> = None;
    let mut input_handler = InputHandler::new();

    let args: Vec<String> = std::env::args().collect();
    let solve_all_mode = args.contains(&"--solve-all".to_string());

    // Menu state
    let mut menu_selection = 0;
    let mut menu_scroll_y = 0.0;

    let cell_size = 40.0;

    if solve_all_mode {
        let level = load_level_from_file(levels[0]).await;
        let mut state = GameState::new(level);
        let solver = Solver::new(state.current_level.clone());
        if let Some(solution) = solver.solve() {
            state.start_auto_play(solution);
        }
        game_state = Some(state);
        current_level_index = Some(0);
    }

    loop {
        // Exit shortcut (Ctrl-C or Command-C)
        if (is_key_down(KeyCode::LeftControl)
            || is_key_down(KeyCode::RightControl)
            || is_key_down(KeyCode::LeftSuper)
            || is_key_down(KeyCode::RightSuper))
            && is_key_pressed(KeyCode::C)
        {
            break;
        }

        clear_background(BLACK);

        let mut should_exit_to_menu = false;
        let mut should_load_next_level = false;

        if let Some(ref mut state) = game_state {
            // GAME UPDATE

            // Update
            let dt = get_frame_time();
            state.update(dt);

            if solve_all_mode {
                if let GameStatus::LevelComplete = state.status {
                    if state.level_complete_timer > 2.0 {
                        should_load_next_level = true;
                    }
                }
            }

            // Input
            if let Some(key) = get_last_key_pressed() {
                if key == KeyCode::Escape {
                    should_exit_to_menu = true;
                } else if key == KeyCode::Slash
                    && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift))
                {
                    state.toggle_help();
                } else if key == KeyCode::P
                    && (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift))
                {
                    // Auto-play / Solve
                    let solver = Solver::new(state.current_level.clone());
                    if let Some(solution) = solver.solve() {
                        state.start_auto_play(solution);
                    }
                } else if let GameStatus::LevelComplete = state.status {
                    if key == KeyCode::Enter {
                        should_load_next_level = true;
                    }
                } else {
                    let shift = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
                    let command = input_handler.map_key(key, shift);
                    state.handle_command(command);
                }
            }
        }

        if should_exit_to_menu {
            game_state = None;
            current_level_index = None;
        }

        if should_load_next_level {
            if let Some(idx) = current_level_index {
                let next_idx = idx + 1;
                if next_idx < levels.len() {
                    let level = load_level_from_file(levels[next_idx]).await;
                    let mut new_state = GameState::new(level);

                    if solve_all_mode {
                        let solver = Solver::new(new_state.current_level.clone());
                        if let Some(solution) = solver.solve() {
                            new_state.start_auto_play(solution);
                        }
                    }

                    game_state = Some(new_state);
                    current_level_index = Some(next_idx);
                } else {
                    if solve_all_mode {
                        break;
                    }
                    // No more levels, return to menu
                    game_state = None;
                    current_level_index = None;
                }
            }
        }

        if let Some(ref state) = game_state {
            // GAME RENDER
            render::draw_game(state, &resources, cell_size);
        } else {
            // MENU RENDER & INPUT
            draw_text("VIM GAME", 100.0, 100.0, 60.0, GREEN);
            draw_text(
                "Select a Level (j/k to scroll, Enter to select):",
                100.0,
                180.0,
                30.0,
                WHITE,
            );

            // Menu Input
            if is_key_pressed(KeyCode::J) || is_key_pressed(KeyCode::Down) {
                if menu_selection < levels.len() - 1 {
                    menu_selection += 1;
                }
            }
            if is_key_pressed(KeyCode::K) || is_key_pressed(KeyCode::Up) {
                if menu_selection > 0 {
                    menu_selection -= 1;
                }
            }
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                let level = load_level_from_file(levels[menu_selection]).await;
                game_state = Some(GameState::new(level));
                current_level_index = Some(menu_selection);
            }

            // Scroll Logic
            let item_height = 40.0;
            let list_start_y = 240.0;
            let footer_height = 80.0;
            let list_height = screen_height() - list_start_y - footer_height;

            // Target scroll position to keep selection in view
            let selection_y = menu_selection as f32 * item_height;
            if selection_y < menu_scroll_y {
                menu_scroll_y = selection_y;
            } else if selection_y + item_height > menu_scroll_y + list_height {
                menu_scroll_y = selection_y + item_height - list_height;
            }

            // Draw List Background/Separator
            draw_line(
                100.0,
                list_start_y,
                screen_width() - 100.0,
                list_start_y,
                2.0,
                GRAY,
            );
            draw_line(
                100.0,
                list_start_y + list_height,
                screen_width() - 100.0,
                list_start_y + list_height,
                2.0,
                GRAY,
            );

            // Draw List with Clipping
            for (i, level_path) in levels.iter().enumerate() {
                let local_y = i as f32 * item_height - menu_scroll_y;
                // Add padding so text baseline is comfortably inside the row
                let screen_y = list_start_y + local_y + 30.0;

                // Only draw if fully visible within the list area
                // Text is drawn upwards from baseline (screen_y), approx 30px height
                if screen_y - 30.0 >= list_start_y && screen_y <= list_start_y + list_height {
                    let label = format!(
                        "{}. {}",
                        i + 1,
                        level_path
                            .replace("assets/levels/", "")
                            .replace(".json", "")
                    );

                    let color = if i == menu_selection {
                        // Draw cursor/highlight
                        draw_text(">", 80.0, screen_y, 30.0, GREEN);
                        WHITE
                    } else {
                        LIGHTGRAY
                    };

                    draw_text(&label, 120.0, screen_y, 30.0, color);
                }

                // Simple key press detection for 1-9 and 0
                let key_to_check = match i {
                    0 => KeyCode::Key1,
                    1 => KeyCode::Key2,
                    2 => KeyCode::Key3,
                    3 => KeyCode::Key4,
                    4 => KeyCode::Key5,
                    5 => KeyCode::Key6,
                    6 => KeyCode::Key7,
                    7 => KeyCode::Key8,
                    8 => KeyCode::Key9,
                    9 => KeyCode::Key0,
                    _ => KeyCode::Unknown,
                };

                if is_key_pressed(key_to_check) {
                    let level = load_level_from_file(level_path).await;
                    game_state = Some(GameState::new(level));
                    current_level_index = Some(i);
                    menu_selection = i; // Update selection to match
                }
            }

            draw_text(
                "Press 1-9, 0 to select",
                100.0,
                screen_height() - 50.0,
                20.0,
                GRAY,
            );
        }

        next_frame().await
    }
}
