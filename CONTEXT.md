# Cheese Manager - Project Context & Guidelines

## 1. Project Overview
**Name:** Cheese Manager
**Type:** Graphical File Manager
**Target Audience:** Linux Power Users (NixOS, Arch, Ubuntu)
**Goal:** Build a blazing-fast, highly aesthetic, and fully functional file manager using modern native Linux toolkits, dropping the traditional boring white/gray themes for a highly customized "Rat/Sewer" Art Direction.

## 2. Tech Stack
*   **Language:** Rust (Strict, fast, memory-safe)
*   **GUI Toolkit:** GTK4 (via the `gtk` / `gtk-rs` crate)
*   **Styling:** GTK CSS Provider (for writing actual CSS to style native widgets)
*   **Environment:** Nix Flakes (for reproducible development)
*   **Typography:** Hack Nerd Font

## 3. Art Direction: The "Rat" Theme 🐀
The aesthetic is dark, underground, and heavily driven by "Sewer" vibes. The UI should feel raw but polished, with glowing purple elements piercing through deep, dark grays.

### Color Palette
*   **Background / Deep Layers:** `#1a1a24` *(Deep Sewer Void)*
*   **Surface / Panels / Inputs:** `#252535` *(Elevated Concrete/Stone)*
*   **Borders / Separators:** `#51516a` *(Dimmed Steel)*
*   **Primary Accent (Hover / Focus / Active):** `#9b59b6` *(Toxic Neon Purple)*
*   **Secondary Accent (Gradients / Selected):** `#7b4aa3` *(Deep Rat Purple)*
*   **Text (Primary):** `#EAEAEA` *(Bright White/Gray)*
*   **Text (Muted / Subtitles):** `#A0A0B0` *(Dusty Gray)*

### UI/UX Rules
1.  **Shadows:** Use subtle purple drop-shadows `box-shadow: 0 0 10px rgba(155, 89, 182, 0.3);` on active elements or popups to simulate a neon glow in a dark sewer.
2.  **Borders:** GTK widgets should have slightly rounded corners (`border-radius: 6px` or `8px`).
3.  **Icons:** Replace standard GTK folder icons with Nerd Font icons (e.g., 󰉋 for folders, 🐀 for root/home if applicable).
4.  **Transitions:** Smooth color transitions on hover states (`transition: all 0.2s ease`).

## 4. Development Guidelines
*When writing code for this project, you should always:*
1.  Use **GTK4** methods (avoid GTK3 deprecated paradigms).
2.  Implement a custom `GtkCssProvider` at the application startup to inject the Rat Theme CSS.
3.  Structure the Rust code cleanly: separate logic (file system traversal) from UI (GTK window generation).
4.  Handle errors gracefully without crashing the GUI.

## 5. Startup Guide (For Contributors)
1. Run `nix develop` to enter the GTK4/Rust workspace.
2. Run `cargo run` to build and test the application window.
