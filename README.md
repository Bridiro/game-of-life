# Conway's Game of Life - WebAssembly

A high-performance implementation of Conway's Game of Life using Rust, WebAssembly, and WebGL for GPU-accelerated rendering.

## Features

- **GPU-accelerated**: Uses WebGL compute shaders for high-performance simulation
- **Interactive**: Click to toggle cells, resize grid dynamically
- **Wraparound**: Cells that exit one side re-enter from the other
- **Responsive**: Works on desktop and mobile browsers
- **Pattern Library**: Pre-built patterns including gliders, oscillators, and spaceships
- **Real-time**: Adjustable simulation speed from 10ms to 1000ms per generation

## Patterns Included

- **Glider**: A simple spaceship that travels diagonally
- **Blinker**: A basic oscillator with period 2
- **Beacon**: A period-2 oscillator
- **Toad**: Another period-2 oscillator
- **Lightweight Spaceship**: Travels horizontally
- **Pulsar**: A large period-3 oscillator
- **Gosper Glider Gun**: Produces gliders indefinitely

## Live Demo

Visit the live demo: [https://bridiro.github.io/game-of-life/](https://bridiro.github.io/game-of-life/)

## Local Development

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Building

```bash
# Clone the repository
git clone https://github.com/Bridiro/game-of-life.git
cd game-of-life

# Build the WebAssembly module
wasm-pack build --target web

# Serve the web directory
cd web
python3 -m http.server 8000
# Or use any static file server

# Open http://localhost:8000 in your browser
```

## Architecture

The project is structured as a modular Rust codebase:

- **`lib.rs`**: Main game struct and WebAssembly bindings
- **`webgl.rs`**: WebGL context initialization and utilities
- **`shaders.rs`**: WebGL shader programs for compute and rendering
- **`patterns.rs`**: Conway's Game of Life pattern definitions
- **`texture.rs`**: Texture management for GPU state storage

The simulation runs entirely on the GPU using WebGL fragment shaders, allowing for high-performance computation even with large grids.

## How It Works

1. **State Storage**: The game state is stored in WebGL textures (RGBA, where R channel represents cell state)
2. **Compute Shader**: A fragment shader implements Conway's rules, reading from current state and writing to next state
3. **Render Shader**: A separate shader renders the current state to the canvas with visual styling
4. **Double Buffering**: Two textures are swapped each generation for efficient GPU computation
5. **Wraparound**: The shader uses `fract()` for seamless edge wrapping

## Controls

- **Play/Pause**: Start/stop the simulation
- **Step**: Advance one generation manually
- **Clear**: Reset the grid to empty
- **Randomize**: Fill grid with random cells
- **Resize**: Change grid dimensions
- **Speed**: Adjust simulation speed (10-1000ms per generation)
- **Patterns**: Load pre-defined Conway's Game of Life patterns
- **Click**: Toggle individual cells

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for bugs and feature requests.
