# Hring Launcher

Hring is an experimental orbital app launcher for Linux. It uses a radial, donut-style interface to help you organize and launch applications quickly. Designed for tiling window managers and minimal desktop environments.

## 🚀 Features
- **Orbital UI:** Applications are arranged in a radial "donut" layout for quick, keyboard-driven access.
- **Async Filtering:** Search results are processed in a background thread to prevent UI freezing.
- **Keyboard-Focused:** Primary navigation is handled via custom keybinds.
- **Rust Powered:** Fast, memory-safe, and low-resource usage.

## 📸 Preview
![Hring Screenshot 1](assets/screenshot_1.png)
![Hring Screenshot 2](assets/screenshot_2.png)
![Hring Screenshot 3](assets/screenshot_3.png)

*Note: The wallpaper shown in the screenshot is for illustrative purposes only and is not included with the software.*

**Since Hring is semi-transparent by default, your desktop wallpaper will blend perfectly with the UI, making every setup unique.**

## 📦 Installation
### Prerequisites
- [Rust](https://www.rust-lang.org/) (latest stable)
- Linux with X11 or Wayland

### Build from source
```bash
git clone https://github.com/Xhelgi/hring
cd hring
cargo build --release
# The binary will be available at target/release/hring
```

## ⚙️ Configuration

Hring looks for its configuration file at ~/.config/hring/config.json.
You can find an example configuration in [examples/config.json](examples/config.json).

Copy it to your config directory:
```Bash
mkdir -p ~/.config/hring
cp examples/config.json ~/.config/hring/config.json
```

## 📐 Capacity Guidelines
Hring is optimized for standard 1920x1080 resolution (1.0x - 1.25x scaling). The recommended limits are:
- **4 groups:** up to 4 applications per group.
- **3 groups:** up to 6 applications per group (recommended).
- **2 groups:** up to 9 applications per group.

*Note: You can adjust the layout by modifying `theme.rs` (font sizes, padding, and offsets) to fit more items.*

## ⌨️ Keybinds
Available keys: `q`, `w`, `e`, `a`, `s`, `d`, `z`, `x`, `c`, `1`, `2`, `3`, `4`, `5`.
Keybinds are scoped: you can use the same key for a group trigger and an application launch without conflict.

**Recommended configurations:**
- **Layout A:** Groups mapped to numbers (1, 2, 3), apps mapped to letters (q, w, e, a, s).
- **Layout B:** Groups mapped to letters (q, w, e), apps mapped to numbers (1, 2, 3, 4, 5).

## 🎨 Design & Customization
All visual parameters (colors, line strokes, padding) are currently defined in `theme.rs`.
*Planned: Move these settings to a persistent JSON configuration file.*
The default theme features a clean, semi-transparent monochrome aesthetic with green accents.

## 🛠 Built With
- `Rust`
- `egui` - Immediate mode GUI
- `serde` - JSON configuration

## Known Issues & Limitations
- **Experimental Stage:** The configuration file format may change in future versions.
- **Icon Support:** Currently, application icons are not displayed (only names and binds).
- **Groups-Name Support** Groups
- **Wayland Input:** In some Wayland compositors, the window focus might behave differently compared to X11.
- **System Paths:** Currently, it scans `/usr/share/applications` and `~/.local/share/applications`. If your system stores apps elsewhere, they might not appear.
- **Group Labels:** Currently, group names are not rendered in the UI; only their assigned keybinds are visible.
- **Keybind Conflicts:** There is no validation for duplicate keybinds. Assigning the same key to multiple apps or groups may cause unexpected behavior.
- **Scaling:** UI elements may overflow if too many groups or applications are added. A dynamic layout engine (recalculating spacing and visibility) is planned for future versions.

## Feedback
Since this is an early-stage project, feedback is highly appreciated! Feel free to open an issue if you encounter bugs or have suggestions for new features.

## 📜 License
Distributed under the GPL-3.0 License. See LICENSE for more information.