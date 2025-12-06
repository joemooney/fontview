# FontView

## Vision
A lightweight font character viewer for exploring Unicode characters rendered in the DejaVu Sans font, with the ability to save and annotate favorite characters.

## Purpose
- Browse all Unicode characters available in DejaVu Sans
- Quick navigation to common Unicode blocks (Latin, Greek, Cyrillic, Symbols, etc.)
- View character details on hover (codepoint, name, enlarged preview)
- Save favorite characters with custom notes
- Persistent favorites across sessions

## Technology Stack
- **Language**: Rust
- **GUI Framework**: egui / eframe
- **Font**: DejaVu Sans (loaded from system fonts)
- **Persistence**: JSON via serde

## User Interface
- **Browse Tab**: Character grid with adjustable size and density
- **Favorites Tab**: List of saved characters with notes and delete option
- Slider controls for character size and characters per row
- Quick-jump buttons for common Unicode blocks
- Hover tooltips with character details
- Click-to-favorite interaction (gold highlighting for favorites)
