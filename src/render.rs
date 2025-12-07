use crate::game::{GameState, GameStatus, InputMode};
use crate::input::VimCommand;
use macroquad::prelude::*;

pub struct Resources {
    pub player_texture: Option<Texture2D>,
    pub wall_texture: Option<Texture2D>,
    pub floor_texture: Option<Texture2D>,
    pub water_texture: Option<Texture2D>,
    pub goal_texture: Option<Texture2D>,
    pub font: Option<Font>,
}

impl Resources {
    pub async fn new() -> Self {
        let load_texture_safe = |path: &str| {
            let path = path.to_string();
            async move { load_texture(&path).await.ok() }
        };

        Self {
            player_texture: load_texture_safe("assets/textures/player.png").await,
            wall_texture: load_texture_safe("assets/textures/wall.png").await,
            floor_texture: load_texture_safe("assets/textures/floor.png").await,
            water_texture: load_texture_safe("assets/textures/water.png").await,
            goal_texture: load_texture_safe("assets/textures/goal.png").await,
            font: None,
        }
    }
}

pub fn draw_game(state: &GameState, resources: &Resources, cell_size: f32) {
    let level = &state.current_level;

    // Draw Level
    for (y, row) in level.layout.iter().enumerate() {
        for (x, char) in row.chars().enumerate() {
            let screen_x = x as f32 * cell_size;
            let screen_y = y as f32 * cell_size + 50.0; // Offset for UI

            match char {
                '#' => draw_wall(screen_x, screen_y, cell_size, resources),
                '~' => draw_water(screen_x, screen_y, cell_size, resources, state.time_elapsed),
                '.' | 'S' | 'E' => draw_floor(screen_x, screen_y, cell_size, resources),
                _ => draw_floor(screen_x, screen_y, cell_size, resources), // Default to floor for text
            }

            // Draw content on top of floor
            match char {
                'S' => draw_text_centered("S", screen_x, screen_y, cell_size, GREEN),
                'E' => draw_goal(screen_x, screen_y, cell_size, resources),
                '.' | '#' | '~' => {} // Already drawn base
                c => {
                    // Check if it's a word char
                    if c.is_alphanumeric() || c == '_' {
                        // Determine neighbors to draw connected platform
                        let is_left_word = if x > 0 {
                            let left_char = row.chars().nth(x - 1).unwrap_or(' ');
                            left_char.is_alphanumeric() || left_char == '_'
                        } else {
                            false
                        };

                        let is_right_word = if x < row.len() - 1 {
                            let right_char = row.chars().nth(x + 1).unwrap_or(' ');
                            right_char.is_alphanumeric() || right_char == '_'
                        } else {
                            false
                        };

                        draw_platform(screen_x, screen_y, cell_size, is_left_word, is_right_word);

                        // Draw the character on top of the platform
                        draw_text_centered(&c.to_string(), screen_x, screen_y, cell_size, BLACK);
                    } else {
                        // Unknown char, just draw text
                        draw_text_centered(
                            &c.to_string(),
                            screen_x,
                            screen_y,
                            cell_size,
                            LIGHTGRAY,
                        );
                    }
                }
            }
        }
    }

    // Draw Player
    let player_x = state.player_pos.x as f32 * cell_size;
    let player_y = state.player_pos.y as f32 * cell_size + 50.0;
    draw_player(player_x, player_y, cell_size, resources, state.time_elapsed);

    // Draw UI
    draw_ui(state);
}

fn draw_platform(x: f32, y: f32, size: f32, left: bool, right: bool) {
    // Base platform color
    let color = Color::new(0.0, 0.8, 0.8, 1.0); // Cyan/Neon
    let border_color = WHITE;
    let padding = 4.0;

    // Draw main body
    draw_rectangle(x, y + padding, size, size - padding * 2.0, color);

    // Draw borders based on connectivity
    if !left {
        // Left edge (Start of word)
        draw_rectangle(x, y + padding, 4.0, size - padding * 2.0, border_color);
    }
    if !right {
        // Right edge (End of word)
        draw_rectangle(
            x + size - 4.0,
            y + padding,
            4.0,
            size - padding * 2.0,
            border_color,
        );
    }

    // Top/Bottom lines
    draw_line(x, y + padding, x + size, y + padding, 2.0, border_color);
    draw_line(
        x,
        y + size - padding,
        x + size,
        y + size - padding,
        2.0,
        border_color,
    );
}

fn draw_wall(x: f32, y: f32, size: f32, res: &Resources) {
    if let Some(tex) = &res.wall_texture {
        draw_texture_ex(
            tex,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    } else {
        // Procedural Brick Wall
        draw_rectangle(x, y, size, size, Color::new(0.7, 0.3, 0.3, 1.0)); // Reddish base

        let brick_h = size / 4.0;
        let brick_w = size / 2.0;

        draw_line(x, y + brick_h, x + size, y + brick_h, 2.0, BLACK);
        draw_line(
            x,
            y + brick_h * 2.0,
            x + size,
            y + brick_h * 2.0,
            2.0,
            BLACK,
        );
        draw_line(
            x,
            y + brick_h * 3.0,
            x + size,
            y + brick_h * 3.0,
            2.0,
            BLACK,
        );

        // Vertical lines (staggered)
        draw_line(x + brick_w, y, x + brick_w, y + brick_h, 2.0, BLACK);
        draw_line(
            x + brick_w * 0.5,
            y + brick_h,
            x + brick_w * 0.5,
            y + brick_h * 2.0,
            2.0,
            BLACK,
        );
        draw_line(
            x + brick_w * 1.5,
            y + brick_h,
            x + brick_w * 1.5,
            y + brick_h * 2.0,
            2.0,
            BLACK,
        );
        draw_line(
            x + brick_w,
            y + brick_h * 2.0,
            x + brick_w,
            y + brick_h * 3.0,
            2.0,
            BLACK,
        );
    }
}

fn draw_water(x: f32, y: f32, size: f32, res: &Resources, time: f32) {
    if let Some(tex) = &res.water_texture {
        draw_texture_ex(
            tex,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    } else {
        draw_rectangle(x, y, size, size, BLUE);

        // Animated waves
        let offset = (time * 2.0).sin() * 5.0;
        let y_wave = y + size / 2.0 + offset;
        draw_line(x + 5.0, y_wave, x + size - 5.0, y_wave, 2.0, SKYBLUE);

        let offset2 = (time * 3.0 + 1.0).sin() * 3.0;
        let y_wave2 = y + size / 4.0 + offset2;
        draw_line(x + 10.0, y_wave2, x + size - 10.0, y_wave2, 1.0, WHITE);
    }
}

fn draw_floor(x: f32, y: f32, size: f32, res: &Resources) {
    if let Some(tex) = &res.floor_texture {
        draw_texture_ex(
            tex,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    } else {
        draw_rectangle(x, y, size, size, Color::new(0.2, 0.2, 0.2, 1.0)); // Dark gray
        draw_rectangle_lines(x, y, size, size, 1.0, Color::new(0.3, 0.3, 0.3, 1.0));
    }
}

fn draw_player(x: f32, y: f32, size: f32, res: &Resources, time: f32) {
    if let Some(tex) = &res.player_texture {
        draw_texture_ex(
            tex,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    } else {
        // Procedural Player (Robot/Cursor)
        let padding = size * 0.1;
        let inner_size = size - padding * 2.0;

        // Body
        draw_rectangle(x + padding, y + padding, inner_size, inner_size, GREEN);

        // Eyes (Blinking)
        if (time * 0.5).sin() > -0.9 {
            draw_rectangle(
                x + size * 0.3,
                y + size * 0.3,
                size * 0.15,
                size * 0.15,
                BLACK,
            );
            draw_rectangle(
                x + size * 0.55,
                y + size * 0.3,
                size * 0.15,
                size * 0.15,
                BLACK,
            );
        } else {
            draw_line(
                x + size * 0.3,
                y + size * 0.35,
                x + size * 0.45,
                y + size * 0.35,
                2.0,
                BLACK,
            );
            draw_line(
                x + size * 0.55,
                y + size * 0.35,
                x + size * 0.7,
                y + size * 0.35,
                2.0,
                BLACK,
            );
        }

        // Cursor underline (VIM style)
        let alpha = (time * 5.0).sin().abs();
        draw_rectangle(
            x,
            y + size - 5.0,
            size,
            5.0,
            Color::new(0.0, 1.0, 0.0, alpha),
        );
    }
}

fn draw_goal(x: f32, y: f32, size: f32, res: &Resources) {
    if let Some(tex) = &res.goal_texture {
        draw_texture_ex(
            tex,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    } else {
        draw_text_centered("E", x, y, size, RED);
        // Flag pole
        draw_line(
            x + size * 0.8,
            y + size * 0.2,
            x + size * 0.8,
            y + size * 0.9,
            2.0,
            WHITE,
        );
        // Flag
        draw_triangle(
            vec2(x + size * 0.8, y + size * 0.2),
            vec2(x + size * 0.8, y + size * 0.5),
            vec2(x + size * 0.4, y + size * 0.35),
            RED,
        );
    }
}

fn draw_text_centered(text: &str, x: f32, y: f32, size: f32, color: Color) {
    let font_size = size * 0.6;
    let text_dims = measure_text(text, None, font_size as u16, 1.0);
    let text_x = x + (size - text_dims.width) / 2.0;
    let text_y = y + (size + text_dims.height) / 2.0 - text_dims.offset_y / 2.0; // Approximate vertical centering

    draw_text(text, text_x, text_y, font_size, color);
}

fn draw_ui(state: &GameState) {
    draw_text(
        &format!("Level: {}", state.current_level.name),
        10.0,
        30.0,
        30.0,
        WHITE,
    );
    draw_text(
        &format!("Time: {:.1}", state.time_elapsed),
        300.0,
        30.0,
        30.0,
        WHITE,
    );
    draw_text(
        &format!("Keystrokes: {}", state.keystrokes),
        500.0,
        30.0,
        30.0,
        WHITE,
    );
    if state.penalties > 0 {
        draw_text(
            &format!("Penalties: {}", state.penalties),
            750.0,
            30.0,
            30.0,
            RED,
        );
    }
    if state.is_auto_playing {
        let box_width = 400.0;
        let box_height = 100.0;
        let box_x = screen_width() / 2.0 - box_width / 2.0;
        let box_y = 60.0;

        draw_rectangle(
            box_x,
            box_y,
            box_width,
            box_height,
            Color::new(0.0, 0.0, 0.0, 0.8),
        );
        draw_rectangle_lines(box_x, box_y, box_width, box_height, 2.0, YELLOW);

        draw_text("AUTO-PLAYING...", box_x + 20.0, box_y + 30.0, 30.0, YELLOW);
        if let Some(cmd) = &state.last_auto_command {
            let cmd_text = cmd.to_display_string();
            let dims = measure_text(&cmd_text, None, 30, 1.0);
            draw_text(
                &cmd_text,
                screen_width() / 2.0 - dims.width / 2.0,
                box_y + 70.0,
                30.0,
                GREEN,
            );
        }
    }

    // Draw Input Mode Status
    match &state.input_mode {
        InputMode::WaitingForChar(cmd) => {
            let cmd_name = match cmd {
                VimCommand::StartFindNext => "f",
                VimCommand::StartFindPrev => "F",
                VimCommand::StartTillNext => "t",
                VimCommand::StartTillPrev => "T",
                _ => "?",
            };
            draw_text(
                &format!("Waiting for char: {}", cmd_name),
                10.0,
                screen_height() - 40.0,
                30.0,
                YELLOW,
            );
        }
        InputMode::CommandLine(text, cmd_type) => {
            let prefix = match cmd_type {
                VimCommand::StartSearchForward => "/",
                VimCommand::StartSearchBackward => "?",
                _ => ":",
            };
            let display_text = format!("{}{}", prefix, text);
            draw_text(&display_text, 10.0, screen_height() - 40.0, 30.0, YELLOW);
            // Draw cursor
            let dims = measure_text(&display_text, None, 30, 1.0);
            if (get_time() * 2.0) as i32 % 2 == 0 {
                draw_rectangle(
                    10.0 + dims.width,
                    screen_height() - 60.0,
                    10.0,
                    30.0,
                    YELLOW,
                );
            }
        }
        _ => {}
    }

    draw_text(
        "Press ESC to Menu | F1 for Help | Shift+P to Solve",
        10.0,
        screen_height() - 10.0,
        20.0,
        GRAY,
    );

    if state.show_help {
        draw_help_overlay(state);
    }

    if let GameStatus::LevelComplete = state.status {
        draw_overlay("LEVEL COMPLETE!", &format!("Score: {}", state.score), GOLD);
    } else if let GameStatus::GameOver = state.status {
        draw_overlay("GAME OVER", "You fell into the void!", RED);
    }
}

fn draw_help_overlay(state: &GameState) {
    draw_rectangle(
        50.0,
        100.0,
        screen_width() - 100.0,
        screen_height() - 200.0,
        Color::new(0.0, 0.0, 0.0, 0.95),
    );
    draw_rectangle_lines(
        50.0,
        100.0,
        screen_width() - 100.0,
        screen_height() - 200.0,
        2.0,
        GREEN,
    );

    let title = "LEVEL HELP";
    let title_dims = measure_text(title, None, 50, 1.0);
    draw_text(
        title,
        screen_width() / 2.0 - title_dims.width / 2.0,
        160.0,
        50.0,
        GREEN,
    );

    // Draw tutorial text
    let text = if state.current_level.tutorial_text.is_empty() {
        "No specific tutorial for this level."
    } else {
        &state.current_level.tutorial_text
    };

    // Simple word wrap or just draw lines
    let lines: Vec<&str> = text.split('\n').collect();
    for (i, line) in lines.iter().enumerate() {
        draw_text(line, 80.0, 220.0 + i as f32 * 30.0, 30.0, WHITE);
    }

    // Draw trained commands
    if !state.current_level.trained_commands.is_empty() {
        let start_y = 220.0 + lines.len() as f32 * 30.0 + 40.0;
        draw_text("Trained Commands:", 80.0, start_y, 30.0, YELLOW);
        let cmds = state.current_level.trained_commands.join(", ");
        draw_text(&cmds, 80.0, start_y + 40.0, 30.0, WHITE);
        draw_text(
            "Using other commands will be penalized!",
            80.0,
            start_y + 80.0,
            25.0,
            RED,
        );
    }

    draw_text(
        "Press F1 to close",
        screen_width() / 2.0 - 100.0,
        screen_height() - 130.0,
        20.0,
        GRAY,
    );
}

fn draw_overlay(title: &str, subtitle: &str, color: Color) {
    draw_rectangle(
        50.0,
        200.0,
        screen_width() - 100.0,
        300.0,
        Color::new(0.0, 0.0, 0.0, 0.9),
    );
    draw_rectangle_lines(50.0, 200.0, screen_width() - 100.0, 300.0, 2.0, color);

    let title_dims = measure_text(title, None, 60, 1.0);
    draw_text(
        title,
        screen_width() / 2.0 - title_dims.width / 2.0,
        300.0,
        60.0,
        color,
    );

    let sub_dims = measure_text(subtitle, None, 40, 1.0);
    draw_text(
        subtitle,
        screen_width() / 2.0 - sub_dims.width / 2.0,
        380.0,
        40.0,
        WHITE,
    );

    let help = if title == "LEVEL COMPLETE!" {
        "Press ENTER for Next Level, ESC to Menu"
    } else {
        "Press ESC to return"
    };
    let help_dims = measure_text(help, None, 30, 1.0);
    draw_text(
        help,
        screen_width() / 2.0 - help_dims.width / 2.0,
        450.0,
        30.0,
        GRAY,
    );
}
