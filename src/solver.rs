use crate::game::{GameState, GameStatus};
use crate::input::VimCommand;
use crate::level::{Level, Position};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    x: usize,
    y: usize,
    layout: Vec<String>,
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    state: State,
    path: Vec<VimCommand>,
    cost: usize,      // g(n)
    heuristic: usize, // h(n)
}

// Priority is based on f(n) = cost + heuristic.
// BinaryHeap is max-heap, so we need to reverse the ordering to get min-heap behavior.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_f = self.cost + self.heuristic;
        let other_f = other.cost + other.heuristic;
        other_f.cmp(&self_f) // Reverse for min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Solver {
    level: Level,
    interesting_words: Vec<String>,
}

impl Solver {
    pub fn new(level: Level) -> Self {
        let interesting_words = Self::extract_words(&level.layout);
        Self {
            level,
            interesting_words,
        }
    }

    fn extract_words(layout: &[String]) -> Vec<String> {
        let mut words = HashSet::new();
        for line in layout {
            let mut current_word = String::new();
            for c in line.chars() {
                if c.is_alphanumeric() || c == '_' {
                    current_word.push(c);
                } else {
                    if !current_word.is_empty() {
                        words.insert(current_word.clone());
                        current_word.clear();
                    }
                }
            }
            if !current_word.is_empty() {
                words.insert(current_word);
            }
        }
        // Filter out "terrain" words (e.g. "XXXXX")
        words.into_iter()
            .filter(|w| !w.chars().all(|c| c == 'X' || c == '.' || c == '#'))
            .collect()
    }

    pub fn solve(&self) -> Option<Vec<VimCommand>> {
        let start_state = State {
            x: self.level.start_pos.x,
            y: self.level.start_pos.y,
            layout: self.level.layout.clone(),
        };

        let start_node = Node {
            state: start_state.clone(),
            path: vec![],
            cost: 0,
            heuristic: self.heuristic(&start_state),
        };

        let mut queue = BinaryHeap::new();
        queue.push(start_node);

        let mut visited = HashSet::new();
        visited.insert(start_state);

        let max_nodes = 500_000; // Safety break
        let mut nodes_explored = 0;

        while let Some(node) = queue.pop() {
            nodes_explored += 1;
            if nodes_explored > max_nodes {
                println!("Solver hit max_nodes limit: {}", max_nodes);
                break;
            }

            if node.state.x == self.level.target_pos.x && node.state.y == self.level.target_pos.y {
                return Some(node.path);
            }

            // Generate commands
            let mut commands = vec![
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

            // Add DeleteChar if applicable
            let current_char = node
                .state
                .layout
                .get(node.state.y)
                .and_then(|row| row.chars().nth(node.state.x))
                .unwrap_or(' ');
            if current_char != '.'
                && current_char != ' '
                && current_char != 'S'
                && current_char != 'E'
            {
                commands.push(VimCommand::DeleteChar);
            }

            // Add Find/Till commands
            if let Some(row) = node.state.layout.get(node.state.y) {
                let mut unique_chars = HashSet::new();
                for c in row.chars() {
                    if c != ' ' && c != '#' {
                        unique_chars.insert(c);
                    }
                }
                for c in unique_chars {
                    commands.push(VimCommand::FindNextChar(c));
                    commands.push(VimCommand::FindPrevChar(c));
                    commands.push(VimCommand::TillNextChar(c));
                    commands.push(VimCommand::TillPrevChar(c));
                }
            }

            // Add Search commands
            for word in &self.interesting_words {
                commands.push(VimCommand::SearchForward(word.clone()));
            }

            for cmd in commands {
                // Simulate
                let mut temp_level = self.level.clone();
                temp_level.layout = node.state.layout.clone();

                let mut temp_game = GameState::new(temp_level);
                temp_game.player_pos = Position {
                    x: node.state.x,
                    y: node.state.y,
                };

                temp_game.handle_command(cmd.clone());

                if let GameStatus::GameOver = temp_game.status {
                    continue; // Died
                }

                let new_state = State {
                    x: temp_game.player_pos.x,
                    y: temp_game.player_pos.y,
                    layout: temp_game.current_level.layout.clone(),
                };

                if !visited.contains(&new_state) {
                    visited.insert(new_state.clone());
                    let mut new_path = node.path.clone();
                    new_path.push(cmd);

                    let new_node = Node {
                        state: new_state.clone(),
                        path: new_path,
                        cost: node.cost + 1,
                        heuristic: self.heuristic(&new_state),
                    };
                    queue.push(new_node);
                }
            }
        }

        None
    }

    fn heuristic(&self, state: &State) -> usize {
        let dx = (state.x as isize - self.level.target_pos.x as isize).abs();
        let dy = (state.y as isize - self.level.target_pos.y as isize).abs();
        (dx + dy) as usize
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

    #[test]
    fn test_solve_with_find() {
        let level = Level {
            id: "find_test".to_string(),
            name: "Find Test".to_string(),
            description: "Test".to_string(),
            layout: vec!["S a E b".to_string()],
            start_pos: Position { x: 0, y: 0 },
            target_pos: Position { x: 4, y: 0 },
            allowed_keys: vec![],
            trained_commands: vec![],
            tutorial_text: String::new(),
            par_time: 10.0,
            par_keystrokes: 2,
        };

        let solver = Solver::new(level);
        let solution = solver.solve();

        assert!(solution.is_some());
        let path = solution.unwrap();
        // Check if path contains FindNextChar('E')
        let has_find = path.iter().any(|cmd| matches!(cmd, VimCommand::FindNextChar('E')));
        assert!(has_find, "Solution should use FindNextChar('E')");
    }

    #[test]
    fn test_solve_with_search() {
        let level = Level {
            id: "search_test".to_string(),
            name: "Search Test".to_string(),
            description: "Test".to_string(),
            layout: vec![
                "S ... target ...".to_string(),
                "... E ...".to_string()
            ],
            start_pos: Position { x: 0, y: 0 },
            target_pos: Position { x: 4, y: 1 }, // E is at 4 on line 1
            allowed_keys: vec![],
            trained_commands: vec![],
            tutorial_text: String::new(),
            par_time: 10.0,
            par_keystrokes: 2,
        };

        let solver = Solver::new(level);
        let solution = solver.solve();

        assert!(solution.is_some());
        let path = solution.unwrap();
        // Check if path contains SearchForward
        let has_search = path.iter().any(|cmd| matches!(cmd, VimCommand::SearchForward(_)));
        assert!(has_search, "Solution should use SearchForward");
    }
}
