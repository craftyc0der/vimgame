use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Level {
    pub id: String,
    pub name: String,
    pub description: String,
    pub layout: Vec<String>,
    pub start_pos: Position,
    pub target_pos: Position,
    pub allowed_keys: Vec<String>,
    #[serde(default)]
    pub trained_commands: Vec<String>,
    #[serde(default)]
    pub tutorial_text: String,
    pub par_time: f32,
    pub par_keystrokes: u32,
}

impl Level {
    pub fn width(&self) -> usize {
        self.layout.iter().map(|row| row.len()).max().unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.layout.len()
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        if y >= self.height() {
            return true;
        }
        let row = &self.layout[y];
        if x >= row.len() {
            return true;
        }
        row.chars().nth(x).unwrap_or('.') == '#'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_dimensions() {
        let level = Level {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test".to_string(),
            layout: vec!["...".to_string(), "...".to_string()],
            start_pos: Position { x: 0, y: 0 },
            target_pos: Position { x: 2, y: 1 },
            allowed_keys: vec![],
            trained_commands: vec![],
            tutorial_text: String::new(),
            par_time: 10.0,
            par_keystrokes: 5,
        };
        assert_eq!(level.width(), 3);
        assert_eq!(level.height(), 2);
    }
}
