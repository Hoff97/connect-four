# Connect Four WebAssembly Game

A web-based Connect Four game where you can play against an AI opponent. Built with Rust compiled to WebAssembly for high-performance AI calculations.

## 🎮 Play Online

The game is automatically deployed to GitHub Pages: **[Play Connect Four](https://Hoff97.github.io/connect-four)**

## Features

- 🎮 Play Connect Four against AI
- 🤖 Multiple AI difficulty levels (Easy to Expert)
- 🎨 Beautiful, responsive web interface
- ⚡ Fast AI calculations powered by WebAssembly
- 🔄 Undo functionality
- 📱 Mobile-friendly design
- 🎯 Choose to play as first or second player
- 🧠 AI evaluation display showing position assessment

## Prerequisites

- Rust (latest stable version)
- wasm-pack (will be installed automatically by build script)
- A web server to serve the files (Python, Node.js, or any static server)

## Quick Start

1. **Build the WebAssembly module:**
   ```bash
   ./build.sh
   ```

2. **Start a local server:**
   ```bash
   # Using Python (recommended)
   python3 -m http.server 8000
   
   # Or using Node.js
   npx serve .
   
   # Or using npm (if you have package.json)
   npm run serve
   ```

3. **Open your browser:**
   Navigate to `http://localhost:8000` and open `index.html`

## How to Play

1. **You are Red pieces, AI is Blue pieces**
2. **Click on a column** to drop your piece
3. **Get 4 in a row** (horizontal, vertical, or diagonal) to win
4. **Choose AI difficulty** from the dropdown menu
5. **Use "New Game"** to restart or **"Undo Move"** to take back your last move

## AI Difficulty Levels

- **Easy (Depth 3)**: Quick moves, good for beginners
- **Medium (Depth 5)**: Balanced gameplay, default setting
- **Hard (Depth 7)**: Challenging opponent, longer think time
- **Expert (Depth 9)**: Very strong play, may take several seconds per move

## Technical Details

### Architecture
- **Frontend**: HTML5, CSS3, JavaScript (ES6 modules)
- **Backend**: Rust compiled to WebAssembly
- **AI Algorithm**: Minimax with alpha-beta pruning
- **Heuristic**: Advanced position evaluation with threat detection

### AI Implementation
The AI uses a sophisticated minimax algorithm with:
- Alpha-beta pruning for efficiency
- Position-based heuristics
- Threat detection (lines of 3 that can become 4)
- Dynamic evaluation based on board control

### WebAssembly Interface
The Rust code is compiled to WebAssembly and provides:
- Game state management
- Move validation
- AI move calculation
- Game result detection

## Development

### Project Structure
```
rs-four/
├── src/
│   ├── lib.rs          # WebAssembly bindings
│   ├── four.rs         # Game logic
│   ├── minmax.rs       # AI implementation
│   └── main.rs         # CLI version (optional)
├── pkg/                # Generated WebAssembly files
├── index.html          # Web interface
├── build.sh            # Build script
├── Cargo.toml          # Rust dependencies
└── package.json        # npm scripts
```

### Building from Source

1. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone and build:**
   ```bash
   git clone <your-repo>
   cd rs-four
   ./build.sh
   ```

3. **Serve locally:**
   ```bash
   python3 -m http.server 8000
   ```

### Customization

#### Modifying AI Difficulty
Edit the difficulty options in `index.html`:
```javascript
<select id="difficulty">
    <option value="3">Easy (Depth 3)</option>
    <option value="5" selected>Medium (Depth 5)</option>
    <option value="7">Hard (Depth 7)</option>
    <option value="9">Expert (Depth 9)</option>
</select>
```

#### Adjusting AI Heuristics
Modify the scoring in `src/four.rs`:
```rust
// Column position scores
let column_scores = [0.0005, 0.002, 0.004, 0.005, 0.004, 0.002, 0.0005];

// Line of 3 scores
let score = if extension_sides == 2 { 0.08 } else { 0.05 };
```

#### Styling Changes
All visual styling is in the `<style>` section of `index.html`. The design uses:
- CSS Grid for board layout
- CSS Gradients for modern look
- Flexbox for responsive design
- CSS Transitions for smooth animations

## Deployment

### Automatic Deployment (GitHub Pages)

The project includes a GitHub Actions workflow that automatically:
1. Builds the WebAssembly module
2. Deploys to GitHub Pages on every push to the main branch

To enable GitHub Pages for your fork:
1. Go to your repository settings
2. Navigate to "Pages" section
3. Set source to "GitHub Actions"
4. Push to main branch to trigger deployment

### Manual Deployment

To deploy to any static hosting service:

1. **Build the project:**
   ```bash
   wasm-pack build --target web --out-dir pkg
   ```

2. **Upload all files** to your hosting service:
   - `index.html`
   - `pkg/` directory (contains WebAssembly files)
   - `.nojekyll` file (for GitHub Pages)

3. **Ensure proper MIME types:**
   - `.wasm` files should be served as `application/wasm`
   - Most modern hosting services handle this automatically

## Troubleshooting

### Common Issues

1. **"wasm-pack not found"**
   - The build script will install it automatically
   - Or install manually: `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`

2. **"Module not found" errors**
   - Ensure you're serving the files from a web server
   - Files must be served over HTTP, not opened directly in browser

3. **CORS errors**
   - Use a proper web server (Python, Node.js, etc.)
   - Don't open `index.html` directly in browser

4. **AI moves slowly**
   - Reduce AI difficulty level
   - WebAssembly runs much faster than JavaScript but still takes time for deep searches

### Performance Tips

- Use **Medium difficulty** for best balance of speed and challenge
- **Expert difficulty** may take 5-10 seconds per move
- The AI gets faster as the game progresses and there are fewer possible moves

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## Acknowledgments

- Built with Rust and WebAssembly
- Uses minimax algorithm for AI
- Inspired by classic Connect Four gameplay
