# Levels 11-20 Plan: Advanced Navigation & Editing

## New Mechanics

### 1. Inline Find (`f`, `F`, `t`, `T`, `;`, `,`)
*   **Concept**: Precision horizontal jumping.
*   **Commands**:
    *   `f{char}`: Move forward to the next occurrence of `{char}` on the current line.
    *   `F{char}`: Move backward to the previous occurrence of `{char}`.
    *   `t{char}`: Move forward to the character *before* `{char}`.
    *   `T{char}`: Move backward to the character *after* `{char}`.
    *   `;`: Repeat the last `f`/`F`/`t`/`T` movement.
    *   `,`: Repeat the last movement in the opposite direction.
*   **Game Application**: Jumping between "islands" on a single line, stopping exactly before a hazard (using `t`).

### 2. Search (`/`, `?`, `n`, `N`)
*   **Concept**: Global teleportation.
*   **Commands**:
    *   `/{text}<Enter>`: Search forward for `{text}`.
    *   `?{text}<Enter>`: Search backward for `{text}`.
    *   `n`: Jump to next match.
    *   `N`: Jump to previous match.
*   **Game Application**: Teleporting across the map to specific unique beacons (e.g., jumping to a `@` symbol across a void).

### 3. Editing (`x`)
*   **Concept**: Removing obstacles.
*   **Commands**:
    *   `x`: Delete the character under the cursor.
*   **Game Application**: "Mining". Some walls (represented by `X` or `%`) are breakable. The player must move to them and press `x` to turn them into floor tiles, opening a path.

---

## Level Designs

### Level 11: The Sniper (`f`)
*   **Theme**: Long distance horizontal jumps.
*   **Layout**: A single row of islands separated by water. Each island has a unique letter.
*   **Goal**: Use `f` to jump from island to island.

### Level 12: The Brake (`t`)
*   **Theme**: Stopping before danger.
*   **Layout**: You are on a platform. The target is across a gap, but the landing spot is a single tile before a wall of fire/water.
*   **Goal**: Use `t` + [Wall Char] to land adjacent to the wall without hitting it (or overshooting).

### Level 13: Rhythm (`f` + `;`)
*   **Theme**: Repetitive movement.
*   **Layout**: A series of identical stepping stones (e.g., `*   *   *   *`).
*   **Goal**: `f*` to hit the first one, then `;` `;` `;` to hop across the rest quickly.

### Level 14: The Beacon (`/`)
*   **Theme**: Teleportation.
*   **Layout**: The player is trapped in a box. The goal is in another box far away.
*   **Goal**: The goal box contains a unique symbol (e.g., `Q`). Type `/Q` to warp there.

### Level 15: The Sequence (`n`)
*   **Theme**: Multi-step search.
*   **Layout**: A maze of islands. The path is marked by a sequence of `@` symbols.
*   **Goal**: `/ @` to start, then `n` `n` `n` to follow the trail of crumbs to the exit.

### Level 16: Breakout (`x`)
*   **Theme**: Destruction.
*   **Layout**: The player is surrounded by breakable walls (`X`).
*   **Goal**: Use `x` to break a specific wall to escape the cell.

### Level 17: The Miner (`x`)
*   **Theme**: Tunneling.
*   **Layout**: A dense field of `X`s with hidden hazards underneath or around.
*   **Goal**: Carefully `x` your way through the blockage to reach the other side.

### Level 18: Search & Destroy (`/` + `x`)
*   **Theme**: Combo.
*   **Layout**: Multiple sealed rooms. One contains the key/goal.
*   **Goal**: Search `/X` to jump to a room's door, `x` to break in, check for goal, repeat.

### Level 19: The Backtrack (`?` + `N`)
*   **Theme**: Reverse search.
*   **Layout**: A spiral or winding path where forward search might skip the target.
*   **Goal**: Use backward search mechanics to navigate.

### Level 20: The Vim Master
*   **Theme**: Final Exam for 11-20.
*   **Layout**: A complex level requiring `f` for precision, `/` for travel, and `x` for access.
