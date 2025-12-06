# FontView - DejaVu Sans Font Character Viewer

## Overview
A simple egui Rust application that displays all characters in the DejaVu Sans font with navigation controls.

## Architecture
- Single-file egui application using eframe
- Loads DejaVu Sans font from system path (`/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf`)
- Grid-based character display with configurable size and layout

## Key Features
- Character grid display with hover tooltips showing Unicode codepoint
- Adjustable character size (12-72px)
- Configurable characters per row (8-32)
- Quick navigation to common Unicode blocks
- Previous/Next page navigation

## Commands
```bash
cargo run    # Run the application
cargo build --release  # Build optimized release
```

## Technical Details
- Uses egui 0.29 / eframe 0.29
- Default window size: 800x600
- Displays 16 rows of characters at a time
- Supports full Unicode range (U+0000 to U+10FFFF)
