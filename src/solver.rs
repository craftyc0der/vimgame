use crate::game::{GameState, GameStatus};
use crate::input::VimCommand;
use crate::level::{Level, Position};
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    x: usize,
    y: usize,
}

pub struct Solver {
    level: Level,
}

impl Solver {
    pub fn new(level: Level) -> Self {
        Self { level }
    }

    pub fn solve(&self) -> Option<Vec<VimCommand>> {
        let start_node = State {
            x: self.level.start_pos.x,
            y: self.level.start_pos.y,
        };

        let mut queue = VecDeque::new();
        queue.push_back((start_node.clone(), vec![]));

        let mut visited = HashSet::new();
        visited.insert(start_node);

        // Limit depth to prevent infinite loops in open areas or complex cycles
        let max_depth = 100;

        while let Some((current_state, path)) = queue.pop_front() {
            if path.len() > max_depth {
                continue;
            }

            // Check if we reached the target
            if current_state.x == self.level.target_pos.x
                && current_state.y == self.level.target_pos.y
            {
                return Some(path);
            }

            // Try all possible commands
            // We need a way to simulate commands without a full GameState if possible,
            // or we create a temporary GameState.
            // Creating a GameState is safer to ensure logic matches exactly.

            let commands = vec![
                VimCommand::MoveLeft,
                VimCommand::MoveRight,
                VimCommand::MoveUp,
                VimCommand::MoveDown,
                VimCommand::MoveWordForward,
                VimCommand::MoveWordBack,
                VimCommand::MoveWordEnd,
                VimCommand::MoveLineStart,
                VimCommand::MoveLineEnd,
                VimCommand::MoveScreenTop,
                VimCommand::MoveScreenMiddle,
                VimCommand::MoveScreenBottom,
                VimCommand::MoveParagraphForward,
                VimCommand::MoveParagraphBack,
            ];

            for cmd in commands {
                // Filter by allowed keys if strict?
                // The level has `allowed_keys`, we should probably respect that.
                // But mapping VimCommand back to string key is tricky without InputHandler reverse map.
                // For now, let's assume the solver has access to all "logic" commands
                // and we verify if they are allowed later, or we just try to solve it physically.

                // Simulate
                let mut temp_game = GameState::new(self.level.clone());
                temp_game.player_pos = Position {
                    x: current_state.x,
                    y: current_state.y,
                };
                temp_game.handle_command(cmd);

                if let GameStatus::GameOver = temp_game.status {
                    continue; // Died
                }

                let new_state = State {
                    x: temp_game.player_pos.x,
                    y: temp_game.player_pos.y,
                };

                if !visited.contains(&new_state) {
                    visited.insert(new_state.clone());
                    let mut new_path = path.clone();
                    new_path.push(cmd);
                    queue.push_back((new_state, new_path));
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_simple_level() {
        let level = Level {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test".to_string(),
            layout: vec!["S...E".to_string()],
            start_pos: Position { x: 0, y: 0 },
            target_pos: Position { x: 4, y: 0 },
            allowed_keys: vec![],
            trained_commands: vec![],
            tutorial_text: String::new(),
            par_time: 10.0,
            par_keystrokes: 5,
        };

        let solver = Solver::new(level);
        let solution = solver.solve();

        assert!(solution.is_some());
        let path = solution.unwrap();
        // Should be some moves to the right
        assert!(path.len() > 0);
    }
}
