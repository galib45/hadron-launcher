# Hadron Project (`GEMINI.md`)

This document provides a comprehensive overview of the Hadron project for future development and maintenance.

## 1. Project Overview

**Purpose:**
Hadron is a graphical user interface (GUI) application for Linux, designed to act as a simple "Wrapper to Proton." It allows users to add, manage, and launch games (likely Windows games) using Valve's Proton compatibility layer.

**Technology:**
- **Language:** Rust (2024 Edition)
- **GUI Framework:** [`iced`](https://github.com/iced-rs/iced), a cross-platform GUI library for Rust.
- **Data Serialization:** Game metadata and settings are persisted using the TOML format.

**Architecture:**
The application follows The Elm Architecture (Model-View-Update), which is idiomatic for `iced` applications.
- **`main.rs`**: The application entry point, responsible for setting up the main window and running the `iced` application.
- **`app.rs`**: The core of the application. It defines the main `App` struct, the possible pages (`Page` enum), and all possible user interactions (`Message` enum). It manages the application's state, including page navigation and data persistence.
- **`models.rs`**: Defines the primary data structures: `Game` (containing name, paths to the executable, cover art, and Wine prefix) and `Settings` (containing the path to Proton).
- **`app/AppData`**: A struct that holds the list of games and settings. It includes methods to load this data from and save it to `~/.local/share/hadron/data.toml`.
- **`pages/`**: A module containing sub-modules for each screen of the application (e.g., `home.rs`, `add_game.rs`, `settings.rs`). Each page module has its own `State`, `Message` enum for page-specific actions, and `view` function to render its UI.
- **`widgets/`**: Contains custom, reusable UI components that are used across different pages.

## 2. Building and Running

The project uses the standard Rust build tool, Cargo.

- **Build the project:**
  ```bash
  cargo build
  ```

- **Run the application:**
  ```bash
  cargo run
  ```

- **Run tests:**
  ```bash
  cargo test
  ```

## 3. Development Conventions

- **Modularity:** The codebase is well-structured into modules based on functionality (`pages`, `widgets`, `models`, `resources`, `utils`). This separation of concerns should be maintained.
- **State Management:** The global application state resides in `app.rs`. When adding or modifying features, consider if the state is global (belongs in `app::AppData`) or local to a specific page (belongs in the page's own `State` struct).
- **Messaging:** User interactions and other events are handled via the `Message` enums. Global messages that can be triggered from any page (like navigation) are in `app::Message`. Page-specific messages are defined within their respective modules and wrapped into the main `app::Message` enum.
- **UI and Styling:** The UI is built declaratively. Reusable components are abstracted into the `widgets` module. Styling is defined directly in the code using `iced`'s styling system.
- **Data Persistence:** All user data is stored in a single TOML file. Changes to the `Game` or `Settings` structs in `models.rs` must be compatible with the serialization/deserialization logic in `app.rs`.
