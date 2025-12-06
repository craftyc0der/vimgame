# VIM Game Development Checklist

## Phase 1: Setup and Design

- [x] Initialize Rust project
- [x] Create Game Design Document (levels, mechanics, JSON schema)
- [x] Set up project structure (modules for engine, levels, input)
- [x] Create Makefile for cross-compilation (Native + WASM)

## Phase 2: Core Engine

- [x] Implement JSON Level Parser
- [x] Implement Input Handler (VIM state machine)
- [x] Implement Game Loop (Update/Render)
- [x] Implement Scoring System (Time + Keystrokes)

## Phase 3: Level Implementation

- [x] Define Level 1: Basic Navigation (h, j, k, l)
- [x] Define Level 2: Word Movement (w, b, e)
- [x] Define Level 3: The Long Jump (w) - Large gaps requiring 'w' to cross.
- [x] Define Level 4: Precision Landing (e) - Landing on specific end-of-word tiles to avoid hazards.
- [x] Define Level 5: The Rewind (b) - Backtracking quickly through a hazard course.
- [x] Define Level 6: Line Mastery (0, $) - Long horizontal levels requiring instant jumps to start/end of line.
- [x] Define Level 7: Screen Jumps (H, M, L) - Vertical teleportation to Top, Middle, Bottom of screen.
- [x] Define Level 8: Void Leaping (}) - Jumping over blocks of empty lines (paragraphs).
- [x] Define Level 9: The Complex - Maze requiring all movement types.
- [x] Define Level 10: Grandmaster - The ultimate test.
- [x] Create JSON files for levels

## Phase 4: Graphics and UI

- [x] Set up Terminal UI (using `ratatui` or `crossterm`) or 2D Engine (using `bevy` or `macroquad`)?
  - Decision: Using `macroquad` for 2D graphics support.
- [x] Implement Sprite Rendering (Basic shapes and text)
- [x] Implement UI (Score, Instructions)
- [x] Implement Help System (Overlay with `?`)
- [x] Implement Penalty System UI (Red warning text)

## Phase 5: Testing

- [x] Unit tests for Input Handler
- [x] Unit tests for Scoring
- [x] Integration tests for level loading
- [ ] Automated Level Playthrough tests (Bot/Solver) - Started in `src/solver.rs`

## Phase 6: Polish

- [ ] Add sound effects (optional)
- [x] Refine scoring algorithm (Implemented Penalties for non-trained keys)
- [ ] Add more levels (Target: 100)
- [x] Add Level Selector / Menu

