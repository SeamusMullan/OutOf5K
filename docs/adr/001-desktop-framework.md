# ADR-001: Tauri as Desktop Framework

## Status

Accepted

## Date

2024-01-15

## Context

OutOf5K requires a desktop application framework to deliver a native experience for CS2 players reviewing their gameplay. The application needs to:

1. Access the local file system (demo files can be 100MB+)
2. Run computationally intensive tasks (demo parsing)
3. Provide a modern, responsive UI
4. Be distributable as a standalone application
5. Keep bundle size reasonable for easy distribution
6. Support Windows, macOS, and Linux

The main contenders in the desktop application space are:
- **Electron**: The industry standard, used by VS Code, Discord, Slack
- **Tauri**: Newer Rust-based alternative using system webviews
- **Qt/PyQt**: Traditional native GUI framework
- **Flutter Desktop**: Google's cross-platform framework

## Decision

We will use **Tauri v2** as our desktop framework.

## Consequences

### Positive

1. **Small Bundle Size**: Tauri apps are typically 10-20MB vs Electron's 150-200MB. This is significant for distribution and updates.

2. **Better Performance**: Uses system webview instead of bundling Chromium. Lower memory footprint and faster startup.

3. **Security by Default**: Tauri's permission system is more restrictive by default, requiring explicit capability grants.

4. **Rust Backend**: The Rust backend provides excellent performance for file operations and can interface well with our SQLite database.

5. **Active Development**: Tauri v2 (recently released) brings significant improvements including mobile support (future consideration).

6. **Web Tech Frontend**: We can use familiar web technologies (Svelte, TypeScript) for the UI while getting native performance.

### Negative

1. **Younger Ecosystem**: Smaller community than Electron, fewer resources and tutorials.

2. **System Webview Variance**: Behavior may differ slightly across platforms due to different webview implementations (WebKit on macOS, WebView2 on Windows).

3. **Rust Learning Curve**: While our heavy lifting is in Python, the Tauri backend requires Rust knowledge.

4. **Less Battle-Tested**: Fewer production applications compared to Electron, potentially more undiscovered edge cases.

### Neutral

1. **Python Integration**: Both Tauri and Electron would require subprocess communication for Python. No difference here.

2. **DevTools**: Tauri supports Chrome DevTools for debugging, similar to Electron.

## Alternatives Considered

### Alternative 1: Electron

The most mature option with the largest ecosystem.

**Why Rejected:**
- Bundle size is a significant concern for distribution (150MB+ minimum)
- Higher memory usage could be problematic when users have CS2 running simultaneously
- Overkill for our needs; we don't need Node.js in the main process

### Alternative 2: Qt/PyQt

Would allow a fully Python stack.

**Why Rejected:**
- Dated look and feel by default, requiring significant styling effort
- Smaller ecosystem for modern UI patterns
- More difficult to achieve the rich visualizations we need (heatmaps, interactive maps)
- Harder to find developers familiar with Qt

### Alternative 3: Flutter Desktop

Cross-platform with good performance.

**Why Rejected:**
- Dart is an unfamiliar language, adding learning curve
- Desktop support is less mature than mobile
- Fewer UI libraries for data visualization compared to web ecosystem
- Would still need subprocess for Python integration

## References

- [Tauri vs Electron Comparison](https://tauri.app/v1/guides/getting-started/prerequisites)
- [Tauri v2 Release Notes](https://tauri.app/blog/tauri-2-0-0/)
- [Electron Memory Usage Discussion](https://www.electronjs.org/docs/latest/tutorial/performance)
