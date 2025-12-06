use crate::input::{InputHandler, VimCommand};
use crate::level::{Level, Position};
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub enum InputMode {
    Normal,
    WaitingForChar(VimCommand), // Stores the pending command (e.g. StartFindNext)
    CommandLine(String, VimCommand), // Stores current input and type (StartSearchForward/Backward)
}

pub enum GameStatus {
    Playing,
    LevelComplete,
    GameOver,
}

pub struct GameState {
    pub player_pos: Position,
    pub current_level: Level,
    pub status: GameStatus,
    pub time_elapsed: f32,
    pub keystrokes: u32,
    pub score: i32,
    pub show_help: bool,
    pub trained_commands: Vec<VimCommand>,
    pub penalties: u32,
    pub replay_queue: VecDeque<VimCommand>,
    pub replay_timer: f32,
    pub is_auto_playing: bool,
    pub last_auto_command: Option<VimCommand>,
    pub level_complete_timer: f32,
    pub input_mode: InputMode,
    pub last_find_command: Option<VimCommand>,
    pub last_search_query: Option<String>,
    pub search_direction_forward: bool,
}

impl GameState {
    pub fn new(level: Level) -> Self {
        let trained_commands = level
            .trained_commands
            .iter()
            .map(|s| InputHandler::from_string(s))
            .collect();

        Self {
            player_pos: level.start_pos,
            current_level: level,
            status: GameStatus::Playing,
            time_elapsed: 0.0,
            keystrokes: 0,
            score: 0,
            show_help: false,
            trained_commands,
            penalties: 0,
            replay_queue: VecDeque::new(),
            replay_timer: 0.0,
            is_auto_playing: false,
            last_auto_command: None,
            level_complete_timer: 0.0,
            input_mode: InputMode::Normal,
            last_find_command: None,
            last_search_query: None,
            search_direction_forward: true,
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn start_auto_play(&mut self, commands: Vec<VimCommand>) {
        self.replay_queue = VecDeque::from(commands);
        self.is_auto_playing = true;
        self.replay_timer = 0.0;
        self.last_auto_command = None;
    }

    pub fn update(&mut self, dt: f32) {
        match self.status {
            GameStatus::Playing => {
                self.time_elapsed += dt;

                if self.is_auto_playing {
                    self.replay_timer += dt;
                    if self.replay_timer >= 0.8 {
                        // Move every 0.8 seconds
                        self.replay_timer = 0.0;
                        if let Some(cmd) = self.replay_queue.pop_front() {
                            self.last_auto_command = Some(cmd.clone());
                            self.handle_command(cmd);
                        } else {
                            self.is_auto_playing = false;
                            self.last_auto_command = None;
                        }
                    }
                }
            }
            GameStatus::LevelComplete => {
                self.level_complete_timer += dt;
            }
            _ => {}
        }
    }

    pub fn handle_command(&mut self, command: VimCommand) {
        if let GameStatus::Playing = self.status {
            match &command {
                VimCommand::MoveLeft => self.move_player(-1, 0),
                VimCommand::MoveRight => self.move_player(1, 0),
                VimCommand::MoveUp => self.move_player(0, -1),
                VimCommand::MoveDown => self.move_player(0, 1),
                VimCommand::MoveWordForward => self.move_word_forward(),
                VimCommand::MoveWordBack => self.move_word_back(),
                VimCommand::MoveWordEnd => self.move_word_end(),
                VimCommand::MoveLineStart => self.move_line_start(),
                VimCommand::MoveLineEnd => self.move_line_end(),
                VimCommand::MoveScreenTop => self.move_screen_top(),
                VimCommand::MoveScreenMiddle => self.move_screen_middle(),
                VimCommand::MoveScreenBottom => self.move_screen_bottom(),
                VimCommand::MoveParagraphForward => self.move_paragraph_forward(),
                VimCommand::MoveParagraphBack => self.move_paragraph_back(),
                VimCommand::StartFindNext
                | VimCommand::StartFindPrev
                | VimCommand::StartTillNext
                | VimCommand::StartTillPrev => {
                    self.input_mode = InputMode::WaitingForChar(command.clone());
                }
                VimCommand::StartSearchForward | VimCommand::StartSearchBackward => {
                    self.input_mode = InputMode::CommandLine(String::new(), command.clone());
                }
                VimCommand::FindNextChar(c) => self.find_char_forward(*c),
                VimCommand::FindPrevChar(c) => self.find_char_backward(*c),
                VimCommand::TillNextChar(c) => self.till_char_forward(*c),
                VimCommand::TillPrevChar(c) => self.till_char_backward(*c),
                VimCommand::RepeatFind => self.repeat_find(false),
                VimCommand::RepeatFindReverse => self.repeat_find(true),
                VimCommand::SearchForward(s) => self.search_forward(s),
                VimCommand::SearchBackward(s) => self.search_backward(s),
                VimCommand::NextMatch => self.next_match(),
                VimCommand::PrevMatch => self.prev_match(),
                VimCommand::DeleteChar => self.delete_char(),
                _ => {} // Implement other commands later
            }

            if command != VimCommand::None
                && !matches!(
                    command,
                    VimCommand::StartFindNext
                        | VimCommand::StartFindPrev
                        | VimCommand::StartTillNext
                        | VimCommand::StartTillPrev
                        | VimCommand::StartSearchForward
                        | VimCommand::StartSearchBackward
                )
            {
                self.keystrokes += 1;

                // Check penalty
                // Only penalize movement commands, not Escape/Insert etc if we had them
                // For now, all handled commands are movement.
                // If trained_commands is empty, no penalties (allow all).
                if !self.trained_commands.is_empty() && !self.trained_commands.contains(&command) {
                    self.penalties += 1;
                }
            }

            self.check_win_condition();
        }
    }

    pub fn handle_char_input(&mut self, c: char) {
        match &self.input_mode {
            InputMode::WaitingForChar(cmd) => {
                let next_cmd = match cmd {
                    VimCommand::StartFindNext => VimCommand::FindNextChar(c),
                    VimCommand::StartFindPrev => VimCommand::FindPrevChar(c),
                    VimCommand::StartTillNext => VimCommand::TillNextChar(c),
                    VimCommand::StartTillPrev => VimCommand::TillPrevChar(c),
                    _ => VimCommand::None,
                };
                self.input_mode = InputMode::Normal;
                if next_cmd != VimCommand::None {
                    self.last_find_command = Some(next_cmd.clone());
                    self.handle_command(next_cmd);
                }
            }
            InputMode::CommandLine(current_text, cmd_type) => {
                // This is handled in handle_command_line_input usually, but if we get char here:
                let mut new_text = current_text.clone();
                new_text.push(c);
                self.input_mode = InputMode::CommandLine(new_text, cmd_type.clone());
            }
            _ => {}
        }
    }

    pub fn handle_special_key(&mut self, key: macroquad::input::KeyCode) {
        if let InputMode::CommandLine(ref mut text, ref cmd_type) = self.input_mode.clone() {
            match key {
                macroquad::input::KeyCode::Enter => {
                    let final_cmd = match cmd_type {
                        VimCommand::StartSearchForward => VimCommand::SearchForward(text.clone()),
                        VimCommand::StartSearchBackward => VimCommand::SearchBackward(text.clone()),
                        _ => VimCommand::None,
                    };
                    self.input_mode = InputMode::Normal;
                    if final_cmd != VimCommand::None {
                        self.last_search_query = Some(text.clone());
                        self.search_direction_forward =
                            matches!(cmd_type, VimCommand::StartSearchForward);
                        self.handle_command(final_cmd);
                    }
                }
                macroquad::input::KeyCode::Backspace => {
                    let mut new_text = text.clone();
                    new_text.pop();
                    self.input_mode = InputMode::CommandLine(new_text, cmd_type.clone());
                }
                macroquad::input::KeyCode::Escape => {
                    self.input_mode = InputMode::Normal;
                }
                _ => {}
            }
        }
    }

    fn find_char_forward(&mut self, target: char) {
        let y = self.player_pos.y;
        let start_x = self.player_pos.x + 1;
        let width = self.current_level.width();

        for x in start_x..width {
            if self.get_char_at(x, y) == target {
                if !self.current_level.is_wall(x, y) {
                    self.player_pos.x = x;
                    if self.get_char_at(x, y) == '~' {
                        self.status = GameStatus::GameOver;
                    }
                }
                return;
            }
        }
    }

    fn find_char_backward(&mut self, target: char) {
        let y = self.player_pos.y;
        let start_x = self.player_pos.x;
        if start_x == 0 {
            return;
        }

        for x in (0..start_x).rev() {
            if self.get_char_at(x, y) == target {
                if !self.current_level.is_wall(x, y) {
                    self.player_pos.x = x;
                    if self.get_char_at(x, y) == '~' {
                        self.status = GameStatus::GameOver;
                    }
                }
                return;
            }
        }
    }

    fn till_char_forward(&mut self, target: char) {
        let y = self.player_pos.y;
        let start_x = self.player_pos.x + 1;
        let width = self.current_level.width();

        for x in start_x..width {
            if self.get_char_at(x, y) == target {
                let target_x = x - 1;
                if target_x > self.player_pos.x && !self.current_level.is_wall(target_x, y) {
                    self.player_pos.x = target_x;
                    if self.get_char_at(target_x, y) == '~' {
                        self.status = GameStatus::GameOver;
                    }
                }
                return;
            }
        }
    }

    fn till_char_backward(&mut self, target: char) {
        let y = self.player_pos.y;
        let start_x = self.player_pos.x;
        if start_x == 0 {
            return;
        }

        for x in (0..start_x).rev() {
            if self.get_char_at(x, y) == target {
                let target_x = x + 1;
                if target_x < self.player_pos.x && !self.current_level.is_wall(target_x, y) {
                    self.player_pos.x = target_x;
                    if self.get_char_at(target_x, y) == '~' {
                        self.status = GameStatus::GameOver;
                    }
                }
                return;
            }
        }
    }

    fn repeat_find(&mut self, reverse: bool) {
        if let Some(cmd) = self.last_find_command.clone() {
            let cmd_to_run = if reverse {
                match cmd {
                    VimCommand::FindNextChar(c) => VimCommand::FindPrevChar(c),
                    VimCommand::FindPrevChar(c) => VimCommand::FindNextChar(c),
                    VimCommand::TillNextChar(c) => VimCommand::TillPrevChar(c),
                    VimCommand::TillPrevChar(c) => VimCommand::TillNextChar(c),
                    _ => return,
                }
            } else {
                cmd
            };
            self.handle_command(cmd_to_run);
        }
    }

    fn search_forward(&mut self, query: &str) {
        // Simple linear search from current pos
        // Flatten coordinates? Or just loop y then x
        let width = self.current_level.width();
        let height = self.current_level.height();

        let mut x = self.player_pos.x;
        let mut y = self.player_pos.y;

        // Start from next char
        loop {
            x += 1;
            if x >= width {
                x = 0;
                y += 1;
            }
            if y >= height {
                y = 0; // Wrap around? Vim wraps.
            }

            if x == self.player_pos.x && y == self.player_pos.y {
                break; // Full loop
            }

            // Check match
            if self.check_match_at(x, y, query) {
                if !self.current_level.is_wall(x, y) {
                    self.player_pos.x = x;
                    self.player_pos.y = y;
                    if self.get_char_at(x, y) == '~' {
                        self.status = GameStatus::GameOver;
                    }
                }
                return;
            }
        }
    }

    fn search_backward(&mut self, query: &str) {
        let width = self.current_level.width();
        let height = self.current_level.height();

        let mut x = self.player_pos.x;
        let mut y = self.player_pos.y;

        loop {
            if x == 0 {
                x = width - 1;
                if y == 0 {
                    y = height - 1;
                } else {
                    y -= 1;
                }
            } else {
                x -= 1;
            }

            if x == self.player_pos.x && y == self.player_pos.y {
                break;
            }

            if self.check_match_at(x, y, query) {
                if !self.current_level.is_wall(x, y) {
                    self.player_pos.x = x;
                    self.player_pos.y = y;
                    if self.get_char_at(x, y) == '~' {
                        self.status = GameStatus::GameOver;
                    }
                }
                return;
            }
        }
    }

    fn check_match_at(&self, x: usize, y: usize, query: &str) -> bool {
        // Check if query matches starting at x, y
        // Only horizontal matches for now
        let width = self.current_level.width();
        if x + query.len() > width {
            return false;
        }

        for (i, c) in query.chars().enumerate() {
            if self.get_char_at(x + i, y) != c {
                return false;
            }
        }
        true
    }

    fn next_match(&mut self) {
        if let Some(query) = self.last_search_query.clone() {
            if self.search_direction_forward {
                self.search_forward(&query);
            } else {
                self.search_backward(&query);
            }
        }
    }

    fn prev_match(&mut self) {
        if let Some(query) = self.last_search_query.clone() {
            if self.search_direction_forward {
                self.search_backward(&query);
            } else {
                self.search_forward(&query);
            }
        }
    }

    fn delete_char(&mut self) {
        let x = self.player_pos.x;
        let y = self.player_pos.y;

        // Can only delete if it's a wall or something?
        // Plan says: "Some walls (represented by X or %) are breakable."
        // But player is ON the tile. Player cannot be on a wall.
        // So player must be on top of 'X' or '%' which acts as floor but blocks path?
        // Or maybe 'x' deletes the character UNDER the cursor.
        // If the character is 'X', it becomes '.' (floor).

        // Wait, if 'X' is a wall, player cannot be on it.
        // Maybe 'x' deletes the character to the RIGHT? No, vim 'x' deletes under cursor.
        // So 'X' must be a walkable tile that acts as an obstacle for something else?
        // OR, maybe the level design has 'X' as a "breakable wall" that you can stand next to and break?
        // But vim 'x' is "delete character under cursor".

        // Let's assume 'X' is a walkable tile that represents "rubble" or "weak floor" that you need to clear?
        // OR, maybe we change the mechanic: 'x' deletes the character under cursor.
        // If you are standing on 'X', and you press 'x', it turns to '.'.
        // Maybe 'X' has high friction or penalty?

        // Re-reading plan: "The player must move to them and press x to turn them into floor tiles, opening a path."
        // This implies 'X' is walkable.
        // Maybe 'X' prevents you from moving PAST it? No.
        // Maybe 'X' is just a visual obstacle that you need to clear to "solve" the level?
        // Or maybe 'X' is surrounded by void, and clearing it spawns a bridge?

        // Let's just implement: 'x' replaces current char with '.' if it is not already '.' or 'S' or 'E'.
        // And maybe specific chars like 'X' trigger something.

        // For now: 'x' turns current tile to '.'

        if y < self.current_level.layout.len() {
            let row = &mut self.current_level.layout[y];
            if x < row.len() {
                let mut chars: Vec<char> = row.chars().collect();
                if chars[x] != 'S' && chars[x] != 'E' {
                    chars[x] = '.';
                    self.current_level.layout[y] = chars.into_iter().collect();
                }
            }
        }
    }

    fn move_player(&mut self, dx: i32, dy: i32) {
        let new_x = self.player_pos.x as i32 + dx;
        let new_y = self.player_pos.y as i32 + dy;

        if new_x >= 0 && new_y >= 0 {
            let x = new_x as usize;
            let y = new_y as usize;

            if !self.current_level.is_wall(x, y) {
                self.player_pos.x = x;
                self.player_pos.y = y;

                // Apply terrain cost
                let cost = self.get_terrain_cost(x, y);
                self.time_elapsed += cost;

                // Check for hazard (Void/Ravine)
                if self.get_char_at(x, y) == '~' {
                    self.status = GameStatus::GameOver;
                }
            }
        }
    }

    fn get_terrain_cost(&self, x: usize, y: usize) -> f32 {
        match self.get_char_at(x, y) {
            'X' => 5.0, // High cost for rubble/mud
            _ => 0.0,
        }
    }

    fn get_char_at(&self, x: usize, y: usize) -> char {
        if y >= self.current_level.height() || x >= self.current_level.width() {
            return ' ';
        }
        self.current_level.layout[y].chars().nth(x).unwrap_or(' ')
    }

    fn is_word_char(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn move_word_forward(&mut self) {
        let mut x = self.player_pos.x;
        let mut y = self.player_pos.y;
        let width = self.current_level.width();
        let height = self.current_level.height();

        // 1. If we are on a word, skip to the end of it
        // But VIM 'w' logic is: move to start of next word.
        // If we are on a word, we consume it.
        // If we are on whitespace, we consume it.

        // Simple state machine approach:
        // State 0: Consume current word (if on one)
        // State 1: Consume whitespace
        // State 2: Stop at start of next word

        let mut passed_current_word = false;

        // Loop to find next position
        loop {
            x += 1;
            if x >= width {
                x = 0;
                y += 1;
                if y >= height {
                    // End of document
                    return;
                }
            }

            let c = self.get_char_at(x, y);
            // println!("Checking ({}, {}): '{}', passed_word: {}", x, y, c, passed_current_word);

            if self.current_level.is_wall(x, y) {
                continue; // Treat walls as invisible/skip? Or stop? Let's skip.
            }

            if self.is_word_char(c) {
                if passed_current_word {
                    // Found start of next word
                    self.player_pos.x = x;
                    self.player_pos.y = y;
                    return;
                }
            } else {
                // We hit non-word char (whitespace or punctuation)
                passed_current_word = true;
            }
        }
    }

    fn move_word_back(&mut self) {
        let mut x = self.player_pos.x;
        let mut y = self.player_pos.y;
        let width = self.current_level.width();

        // Helper to move back one step
        let move_back_one = |x: &mut usize, y: &mut usize| -> bool {
            if *x > 0 {
                *x -= 1;
                true
            } else if *y > 0 {
                *y -= 1;
                *x = width - 1;
                true
            } else {
                false
            }
        };

        // 1. Move back one step initially
        if !move_back_one(&mut x, &mut y) {
            return;
        }

        // 2. Skip any non-word characters (whitespace/punctuation) going backwards
        while !self.is_word_char(self.get_char_at(x, y)) {
            if !move_back_one(&mut x, &mut y) {
                self.player_pos.x = x;
                self.player_pos.y = y;
                return;
            }
        }

        // 3. We are now on the last character of a word (or middle).
        // Move back until we find the start of this word.

        loop {
            // Check previous position
            let mut prev_x = x;
            let mut prev_y = y;

            if !move_back_one(&mut prev_x, &mut prev_y) {
                // We reached start of file, so current pos (x,y) is start of word
                break;
            }

            let c = self.get_char_at(prev_x, prev_y);

            if !self.is_word_char(c) {
                // Previous char is not a word char, so current pos (x,y) is start of word
                break;
            }

            // Move to previous
            x = prev_x;
            y = prev_y;
        }

        self.player_pos.x = x;
        self.player_pos.y = y;
    }

    fn move_word_end(&mut self) {
        let mut x = self.player_pos.x;
        let mut y = self.player_pos.y;
        let width = self.current_level.width();
        let height = self.current_level.height();

        // Helper to move forward one step
        let move_fwd_one = |x: &mut usize, y: &mut usize| -> bool {
            *x += 1;
            if *x >= width {
                *x = 0;
                *y += 1;
            }
            if *y >= height {
                return false;
            }
            true
        };

        // 1. Move forward one step initially
        if !move_fwd_one(&mut x, &mut y) {
            return;
        }

        // 2. Skip any non-word characters (whitespace) going forward
        while !self.is_word_char(self.get_char_at(x, y)) {
            if !move_fwd_one(&mut x, &mut y) {
                return;
            }
        }

        // 3. We are now on a word char. Move forward until the NEXT char is non-word.
        loop {
            let mut next_x = x;
            let mut next_y = y;

            if !move_fwd_one(&mut next_x, &mut next_y) {
                // End of file, so current is end of word
                break;
            }

            if !self.is_word_char(self.get_char_at(next_x, next_y)) {
                // Next char is not a word char, so current is end of word
                break;
            }

            x = next_x;
            y = next_y;
        }

        self.player_pos.x = x;
        self.player_pos.y = y;
    }

    fn move_line_start(&mut self) {
        let y = self.player_pos.y;
        // Find first non-wall from left
        for x in 0..self.current_level.width() {
            if !self.current_level.is_wall(x, y) {
                self.player_pos.x = x;
                // Check hazard
                if self.get_char_at(x, y) == '~' {
                    self.status = GameStatus::GameOver;
                }
                return;
            }
        }
    }

    fn move_line_end(&mut self) {
        let y = self.player_pos.y;
        // Find first non-wall from right
        for x in (0..self.current_level.width()).rev() {
            if !self.current_level.is_wall(x, y) {
                self.player_pos.x = x;
                // Check hazard
                if self.get_char_at(x, y) == '~' {
                    self.status = GameStatus::GameOver;
                }
                return;
            }
        }
    }

    fn move_screen_top(&mut self) {
        let x = self.player_pos.x;
        // Find first non-wall from top
        for y in 0..self.current_level.height() {
            if !self.current_level.is_wall(x, y) {
                self.player_pos.y = y;
                // Check hazard
                if self.get_char_at(x, y) == '~' {
                    self.status = GameStatus::GameOver;
                }
                return;
            }
        }
    }

    fn move_screen_middle(&mut self) {
        let x = self.player_pos.x;
        let mid_y = self.current_level.height() / 2;

        let mut target_y = None;

        // Try mid, then search out
        if !self.current_level.is_wall(x, mid_y) {
            target_y = Some(mid_y);
        } else {
            // Search up/down for closest valid
            for offset in 1..self.current_level.height() {
                if mid_y >= offset && !self.current_level.is_wall(x, mid_y - offset) {
                    target_y = Some(mid_y - offset);
                    break;
                }
                if mid_y + offset < self.current_level.height()
                    && !self.current_level.is_wall(x, mid_y + offset)
                {
                    target_y = Some(mid_y + offset);
                    break;
                }
            }
        }

        if let Some(y) = target_y {
            self.player_pos.y = y;
            if self.get_char_at(x, y) == '~' {
                self.status = GameStatus::GameOver;
            }
        }
    }

    fn move_screen_bottom(&mut self) {
        let x = self.player_pos.x;
        // Find first non-wall from bottom
        for y in (0..self.current_level.height()).rev() {
            if !self.current_level.is_wall(x, y) {
                self.player_pos.y = y;
                // Check hazard
                if self.get_char_at(x, y) == '~' {
                    self.status = GameStatus::GameOver;
                }
                return;
            }
        }
    }

    fn move_paragraph_forward(&mut self) {
        let x = self.player_pos.x;
        let mut y = self.player_pos.y + 1;

        while y < self.current_level.height() {
            if self.is_line_empty(y) {
                if !self.current_level.is_wall(x, y) {
                    self.player_pos.y = y;
                    if self.get_char_at(x, y) == '~' {
                        self.status = GameStatus::GameOver;
                    }
                    return;
                }
            }
            y += 1;
        }
        // If no empty line found, go to end
        let last_y = self.current_level.height() - 1;
        if !self.current_level.is_wall(x, last_y) {
            self.player_pos.y = last_y;
            if self.get_char_at(x, last_y) == '~' {
                self.status = GameStatus::GameOver;
            }
        }
    }

    fn move_paragraph_back(&mut self) {
        let x = self.player_pos.x;
        if self.player_pos.y == 0 {
            return;
        }
        let mut y = self.player_pos.y - 1;

        loop {
            if self.is_line_empty(y) {
                if !self.current_level.is_wall(x, y) {
                    self.player_pos.y = y;
                    if self.get_char_at(x, y) == '~' {
                        self.status = GameStatus::GameOver;
                    }
                    return;
                }
            }
            if y == 0 {
                break;
            }
            y -= 1;
        }
    }

    fn is_line_empty(&self, y: usize) -> bool {
        if y >= self.current_level.height() {
            return false;
        }
        // Check if line contains any alphanumeric chars or hazards
        // We ignore walls (#) and floor (.) so that a line with just walls/floor is considered "empty"
        // We also ignore 'S' and 'E' markers so they don't prevent jumping to start/end lines
        for c in self.current_level.layout[y].chars() {
            if (c.is_alphanumeric() && c != 'S' && c != 'E') || c == '~' {
                return false;
            }
        }
        true
    }

    fn check_win_condition(&mut self) {
        if self.player_pos == self.current_level.target_pos {
            self.status = GameStatus::LevelComplete;
            self.calculate_score();
        }
    }

    fn calculate_score(&mut self) {
        let base_score = 1000;
        let time_penalty = (self.time_elapsed - self.current_level.par_time).max(0.0) * 10.0;
        let keystroke_penalty =
            (self.keystrokes as i32 - self.current_level.par_keystrokes as i32).max(0) * 50;
        let penalty_score = self.penalties as i32 * 100;

        self.score = base_score - time_penalty as i32 - keystroke_penalty - penalty_score;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_level() -> Level {
        Level {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test".to_string(),
            layout: vec!["...".to_string(), "...".to_string(), "...".to_string()],
            start_pos: Position { x: 0, y: 0 },
            target_pos: Position { x: 2, y: 2 },
            allowed_keys: vec![
                "h".to_string(),
                "j".to_string(),
                "k".to_string(),
                "l".to_string(),
            ],
            trained_commands: vec![],
            tutorial_text: String::new(),
            par_time: 10.0,
            par_keystrokes: 4,
        }
    }

    #[test]
    fn test_movement() {
        let level = create_test_level();
        let mut game = GameState::new(level);

        game.handle_command(VimCommand::MoveRight);
        assert_eq!(game.player_pos.x, 1);
        assert_eq!(game.player_pos.y, 0);

        game.handle_command(VimCommand::MoveDown);
        assert_eq!(game.player_pos.x, 1);
        assert_eq!(game.player_pos.y, 1);
    }

    #[test]
    fn test_win_condition() {
        let level = create_test_level();
        let mut game = GameState::new(level);

        // Move to target (2, 2)
        game.handle_command(VimCommand::MoveRight); // 1, 0
        game.handle_command(VimCommand::MoveRight); // 2, 0
        game.handle_command(VimCommand::MoveDown); // 2, 1
        game.handle_command(VimCommand::MoveDown); // 2, 2

        if let GameStatus::LevelComplete = game.status {
            assert!(true);
        } else {
            assert!(false, "Game should be complete");
        }
    }

    #[test]
    fn test_word_movement() {
        let level = Level {
            id: "word_test".to_string(),
            name: "Word Test".to_string(),
            description: "Test".to_string(),
            layout: vec![
                "a bc def".to_string(), // 01234567
            ],
            start_pos: Position { x: 0, y: 0 },
            target_pos: Position { x: 0, y: 1 }, // Unreachable in this 1-line level
            allowed_keys: vec![],
            trained_commands: vec![],
            tutorial_text: String::new(),
            par_time: 10.0,
            par_keystrokes: 5,
        };
        let mut game = GameState::new(level);

        // Start at 'a' (0,0)

        // 'w' -> 'b' (2,0)
        game.handle_command(VimCommand::MoveWordForward);
        assert_eq!(game.player_pos.x, 2);

        // 'w' -> 'd' (5,0)
        game.handle_command(VimCommand::MoveWordForward);
        assert_eq!(game.player_pos.x, 5);

        // 'e' -> 'f' (7,0)
        game.handle_command(VimCommand::MoveWordEnd);
        assert_eq!(game.player_pos.x, 7);

        // 'b' -> 'd' (5,0)
        game.handle_command(VimCommand::MoveWordBack);
        assert_eq!(game.player_pos.x, 5);

        // 'b' -> 'b' (2,0)
        game.handle_command(VimCommand::MoveWordBack);
        assert_eq!(game.player_pos.x, 2);
    }

    #[test]
    fn test_paragraph_movement_level_08() {
        let level = Level {
            id: "level_08_test".to_string(),
            name: "Void Leaping Test".to_string(),
            description: "Test".to_string(),
            layout: vec![
                "####################".to_string(), // 0
                "#S aaaa            #".to_string(), // 1 (Text)
                "#..................#".to_string(), // 2 (Empty)
                "~~~~~~~~~~~~~~~~~~~~".to_string(), // 3 (Water)
                "~~~~~~~~~~~~~~~~~~~~".to_string(), // 4 (Water)
                "#  bbbb            #".to_string(), // 5 (Text)
                "#..................#".to_string(), // 6 (Empty)
                "~~~~~~~~~~~~~~~~~~~~".to_string(), // 7 (Water)
                "#                 E#".to_string(), // 8 (Empty - Goal)
                "####################".to_string(), // 9
            ],
            start_pos: Position { x: 1, y: 1 },
            target_pos: Position { x: 18, y: 8 },
            allowed_keys: vec![],
            trained_commands: vec![],
            tutorial_text: String::new(),
            par_time: 10.0,
            par_keystrokes: 3,
        };
        let mut game = GameState::new(level);

        // Start at (1, 1)

        // 1. Jump to first empty line (Line 2)
        game.handle_command(VimCommand::MoveParagraphForward);
        assert_eq!(game.player_pos.y, 2);

        // 2. Jump to next empty line (Line 6)
        // Should skip lines 3, 4 (water) and 5 (text)
        game.handle_command(VimCommand::MoveParagraphForward);
        assert_eq!(game.player_pos.y, 6);

        // 3. Jump to next empty line (Line 8)
        // Should skip line 7 (water)
        game.handle_command(VimCommand::MoveParagraphForward);
        assert_eq!(game.player_pos.y, 8);
    }

    #[test]
    fn test_find_char_movement() {
        let level = Level {
            id: "find_test".to_string(),
            name: "Find Test".to_string(),
            description: "Test".to_string(),
            layout: vec![
                "a b c d".to_string(), // 0123456
            ],
            start_pos: Position { x: 0, y: 0 },
            target_pos: Position { x: 6, y: 0 },
            allowed_keys: vec![],
            trained_commands: vec![],
            tutorial_text: String::new(),
            par_time: 10.0,
            par_keystrokes: 5,
        };
        let mut game = GameState::new(level);

        // Start at 'a' (0,0)

        // f c -> (4,0)
        game.handle_command(VimCommand::StartFindNext);
        if let InputMode::WaitingForChar(_) = game.input_mode {
            game.handle_char_input('c');
        }
        assert_eq!(game.player_pos.x, 4);

        // ; -> repeats 'f c'. No next 'c', so stay.
        game.handle_command(VimCommand::RepeatFind);
        assert_eq!(game.player_pos.x, 4);

        // F a -> (0,0)
        game.handle_command(VimCommand::StartFindPrev);
        game.handle_char_input('a');
        assert_eq!(game.player_pos.x, 0);
    }
}
