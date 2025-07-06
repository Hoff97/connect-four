# Checkers Web App

This is a web-based checkers game where you can play against an AI opponent. The game is built using Rust and WebAssembly for high performance.

## How to Access

1. **Game Hub**: Visit `game-hub.html` to choose between Connect Four and Checkers
2. **Direct Access**: Visit `checkers.html` to play checkers directly

## Game Features

### Checkers Rules
- **Movement**: Pieces move diagonally on dark squares only
- **Regular Pieces**: Move forward diagonally one square
- **Queens**: Move diagonally in any direction (forward/backward)
- **Captures**: Jump over opponent pieces diagonally (exactly 2 squares)
- **Multiple Captures**: Chain multiple captures in a single move
- **Mandatory Captures**: If a capture is possible, you must take it
- **Promotion**: Pieces become queens when reaching the opposite end

### Game Controls
- **Select Piece**: Click on a highlighted piece to select it
- **Move**: Click on a highlighted destination square to move
- **Deselect**: Click elsewhere to deselect a piece

### Player Options
- **Color Choice**: Play as Red (first) or Black (second)
- **AI Difficulty**: Choose from 5 difficulty levels (Depth 2-6)

### Game Statistics
- **Piece Count**: Real-time count of remaining pieces
- **Move Counter**: Tracks moves without captures (50-move rule)
- **AI Evaluation**: Shows AI's assessment of the position

### Visual Indicators
- **Golden Glow**: Selectable pieces
- **Red Glow**: Currently selected piece
- **Green Glow**: Possible move destinations
- **Crown Symbol**: Queen pieces

## Technical Details

### Implementation
- **Backend**: Rust with minimax algorithm and alpha-beta pruning
- **Frontend**: JavaScript with WebAssembly
- **Action Format**: Multiple captures represented as move sequences
- **Board Representation**: 8x8 grid with piece type tracking

### Performance
- **WebAssembly**: Compiled Rust code for fast AI computation
- **Responsive Design**: Works on desktop and mobile devices
- **Smooth Animations**: CSS transitions for better UX

## Difficulty Levels

- **Easy (Depth 2)**: Good for beginners
- **Medium (Depth 3)**: Moderate challenge
- **Hard (Depth 4)**: Requires strategic thinking
- **Expert (Depth 5)**: Very challenging
- **Master (Depth 6)**: Extremely difficult

Higher depths mean the AI looks further ahead, making it more strategic but taking longer to think.

## Game End Conditions

The game ends when:
1. **Victory**: One player captures all opponent pieces
2. **Draw**: 50 moves without any captures
3. **Stalemate**: A player has no legal moves available

## Tips for Playing

1. **Control the Center**: Central squares offer more movement options
2. **Advance Carefully**: Don't rush pieces forward without support
3. **Create Chains**: Set up multiple capture opportunities
4. **Protect Your Back Rank**: Prevent easy promotions
5. **Think Ahead**: Consider AI's counter-moves before making yours

Enjoy playing checkers against the AI! 🏁
