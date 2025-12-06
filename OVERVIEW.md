# FontView

## Vision
A lightweight font character viewer for exploring Unicode characters rendered in the DejaVu Sans font.

## Purpose
- Browse all Unicode characters available in DejaVu Sans
- Quick navigation to common Unicode blocks (Latin, Greek, Cyrillic, Symbols, etc.)
- View character details on hover (codepoint, decimal value, enlarged preview)

## Technology Stack
- **Language**: Rust
- **GUI Framework**: egui / eframe
- **Font**: DejaVu Sans (loaded from system fonts)

## User Interface
- Character grid with adjustable size and density
- Slider controls for character size and characters per row
- Quick-jump buttons for common Unicode blocks
- Hover tooltips with character details
