# Vidan TUI Video Player 🎥🖥️ [in RUST 🦀]

Welcome to **Vidan**, a **terminal-based video player** (TUI) designed for **Wayland**! This is a **beta version**, and it's built with a lot of help from AI as I (a complete beginner) learn to code! 🧑‍💻🤖

## 🚀 Features (Coming Soon)

- Browse **playlists** and **videos** in a sleek two-panel layout 📂
- Play videos directly in the terminal using **MPV** 🎬
- Keyboard navigation: easily browse using arrow keys and play with Enter ⬆️⬇️↩️

## 🔮 Planned Features

- **Favorites**: Mark your favorite videos and playlists ❤️
- **Remember Last Session**: Auto-load your last playlist and video 🧠
- **Thumbnail Previews**: View video thumbnails before playing 🖼️

## 📁 Required Directory Structure

`vidan-tui-video-player` expects your video files to be organized like this:

```
~/Videos/Collection/
├── Playlist1/
│   ├── video1.mp4
│   └── video2.mp4
├── Playlist2/
│   └── video3.mkv
```

Make sure you create the `~/Videos/Collection/` directory and add your playlists (folders) and video files inside. This structure is currently **hardcoded** in the beta version.


## 📥 Installation

There are multiple ways to install and run **Vidan**!

### 1. **Install from Source**

To install `vidan`, you need **Rust** and **MPV**.

#### a. Install Rust

If you don't have Rust installed, you can easily install it via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### b. Clone the Repository

Clone the repository from GitHub:

```bash
git clone https://github.com/your-username/vidan-tui-video-player.git
cd vidan-tui-video-player
```

#### c. Build the Project

Compile the app:

```bash
cargo build --release
```

#### d. Run the App

Run the app in your terminal:

```bash
./target/release/vidan-tui-video-player
```

### 2. **Download Beta Release**

If you're looking for a pre-built **beta version** and don't want to build it yourself, you can download it from the **releases section** on GitHub.

- Go to the [Releases page](https://github.com/your-username/vidan-tui-video-player/releases)
- Download the latest **beta** release suitable for your system.
- Extract the downloaded file and run the app:

```bash
./vidan-tui-video-player
```
-✅ Just make sure it has execute permission (chmod +x vidan-tui-video-player) once downloaded.

### 3. **Install via Package Manager (Coming Soon)**

I’m also planning to release **Vidan** through package managers like **Homebrew** or **AUR** (for Arch Linux users) in the future! Stay tuned for updates. 📦

## 🐛 Beta Version

This is a **beta release** and may have bugs 🐞. As a beginner, I'm constantly learning and improving. Your feedback is welcome and will help shape the future of the app! 💬

## 💬 Feedback & Contributions

Feel free to open issues or contribute! I'm still learning, and your input will be valuable as I continue to develop this app. 🙏

## 📜 License

This project is licensed under the MIT License - it's free.
Feel free to fork, modify, and distribute it — just like Linux, it's open to everyone!

---

T``
