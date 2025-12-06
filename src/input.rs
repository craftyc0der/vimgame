use macroquad::input::KeyCode;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VimCommand {
    MoveLeft,
    MoveDown,
    MoveUp,
    MoveRight,
    MoveWordForward,
    MoveWordBack,
    MoveWordEnd,
    MoveLineStart,
    MoveLineEnd,
    MoveScreenTop,
    MoveScreenMiddle,
    MoveScreenBottom,
    MoveParagraphForward,
    MoveParagraphBack,
    InsertMode,
    Escape,
    None,
}

pub struct InputHandler {
    // State for multi-key commands could go here
}

impl InputHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn map_key(&mut self, key: KeyCode, shift: bool) -> VimCommand {
        match key {
            KeyCode::H => {
                if shift {
                    VimCommand::MoveScreenTop
                } else {
                    VimCommand::MoveLeft
                }
            }
            KeyCode::J => VimCommand::MoveDown,
            KeyCode::K => VimCommand::MoveUp,
            KeyCode::L => {
                if shift {
                    VimCommand::MoveScreenBottom
                } else {
                    VimCommand::MoveRight
                }
            }
            KeyCode::M => {
                if shift {
                    VimCommand::MoveScreenMiddle
                } else {
                    VimCommand::None
                }
            }
            KeyCode::W => VimCommand::MoveWordForward,
            KeyCode::B => VimCommand::MoveWordBack,
            KeyCode::E => VimCommand::MoveWordEnd,
            KeyCode::Key0 => VimCommand::MoveLineStart,
            KeyCode::Key4 => {
                if shift {
                    VimCommand::MoveLineEnd
                } else {
                    VimCommand::None
                }
            } // $ is Shift+4
            KeyCode::RightBracket => {
                if shift {
                    VimCommand::MoveParagraphForward
                } else {
                    VimCommand::None
                }
            } // } is Shift+]
            KeyCode::LeftBracket => {
                if shift {
                    VimCommand::MoveParagraphBack
                } else {
                    VimCommand::None
                }
            } // { is Shift+[
            KeyCode::I => VimCommand::InsertMode,
            KeyCode::Escape => VimCommand::Escape,
            _ => VimCommand::None,
        }
    }

    // Helper to convert char to command if we process chars instead of KeyCodes
    pub fn map_char(&mut self, c: char) -> VimCommand {
        match c {
            'h' => VimCommand::MoveLeft,
            'j' => VimCommand::MoveDown,
            'k' => VimCommand::MoveUp,
            'l' => VimCommand::MoveRight,
            'w' => VimCommand::MoveWordForward,
            'b' => VimCommand::MoveWordBack,
            'e' => VimCommand::MoveWordEnd,
            'i' => VimCommand::InsertMode,
            // Escape is usually not a char in this context, handled by KeyCode
            _ => VimCommand::None,
        }
    }

    pub fn from_string(s: &str) -> VimCommand {
        match s {
            "h" => VimCommand::MoveLeft,
            "j" => VimCommand::MoveDown,
            "k" => VimCommand::MoveUp,
            "l" => VimCommand::MoveRight,
            "w" => VimCommand::MoveWordForward,
            "b" => VimCommand::MoveWordBack,
            "e" => VimCommand::MoveWordEnd,
            "0" => VimCommand::MoveLineStart,
            "$" => VimCommand::MoveLineEnd,
            "H" => VimCommand::MoveScreenTop,
            "M" => VimCommand::MoveScreenMiddle,
            "L" => VimCommand::MoveScreenBottom,
            "}" => VimCommand::MoveParagraphForward,
            "{" => VimCommand::MoveParagraphBack,
            _ => VimCommand::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_movement_mapping() {
        let mut handler = InputHandler::new();
        assert_eq!(handler.map_key(KeyCode::H, false), VimCommand::MoveLeft);
        assert_eq!(handler.map_key(KeyCode::J, false), VimCommand::MoveDown);
        assert_eq!(handler.map_key(KeyCode::K, false), VimCommand::MoveUp);
        assert_eq!(handler.map_key(KeyCode::L, false), VimCommand::MoveRight);
    }
}
