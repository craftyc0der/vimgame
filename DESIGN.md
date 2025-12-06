# VIM Game Design Document

## Overview
A game to teach VIM keystrokes through increasingly complex levels.

## Core Mechanics
- **Input**: VIM commands (h, j, k, l, w, b, e, i, esc, x, etc.)
- **Goal**: Navigate to a target location or perform a specific edit.
- **Scoring**: Efficiency based.
  - `Score = MaxScore - (Time * TimeFactor) - (Keystrokes * KeystrokeFactor)`

## Level Structure (JSON)
Each level is a JSON file containing:
- `id`: Unique identifier.
- `name`: Display name.
- `description`: Instructions for the player.
- `layout`: Grid or text content.
- `target`: Goal condition (e.g., position, final text state).
- `allowed_keys`: List of keys enabled for this level (to scaffold learning).
- `par_time`: Expected time.
- `par_keystrokes`: Expected keystroke count.

### Example JSON
```json
{
  "id": "level_01",
  "name": "Basic Movement",
  "description": "Use h, j, k, l to reach the green flag.",
  "layout": [
    "..........",
    "...S......",
    "..........",
    "......E..."
  ],
  "start_pos": {"x": 3, "y": 1},
  "target_pos": {"x": 6, "y": 3},
  "allowed_keys": ["h", "j", "k", "l"],
  "par_time": 10.0,
  "par_keystrokes": 5
}
```

## Tech Stack
- **Language**: Rust
- **Graphics**: `macroquad` (Simple 2D game library)
- **Serialization**: `serde`, `serde_json`

## Modules
1.  `main.rs`: Entry point, game loop.
2.  `level.rs`: Level loading and state.
3.  `input.rs`: VIM command parsing.
4.  `render.rs`: Drawing logic.
5.  `score.rs`: Scoring logic.
