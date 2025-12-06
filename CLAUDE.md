# FontView - DejaVu Sans Font Character Viewer

## Overview
A simple egui Rust application that displays all characters in the DejaVu Sans font with navigation controls and favorites management.

## Architecture
- Single-file egui application using eframe
- Loads DejaVu Sans font from system path (`/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf`)
- Grid-based character display with configurable size and layout
- Tabbed interface (Browse / Favorites)
- JSON-based persistence for favorites (`~/.local/share/fontview/favorites.json`)

## Key Features
- Character grid display with hover tooltips showing Unicode codepoint
- Adjustable character size (12-72px)
- Configurable characters per row (8-32)
- Quick navigation to common Unicode blocks
- Previous/Next page navigation
- **Favorites system**: Click characters to add/remove from favorites
- **Notes**: Add custom notes to favorite characters
- **Persistence**: Favorites saved between sessions

## Commands
```bash
cargo run    # Run the application
cargo build --release  # Build optimized release
```

## Technical Details
- Uses egui 0.29 / eframe 0.29, serde, serde_json, dirs
- Default window size: 900x700
- Displays 16 rows of characters at a time
- Supports full Unicode range (U+0000 to U+10FFFF)
- Favorites stored in `~/.local/share/fontview/favorites.json`
