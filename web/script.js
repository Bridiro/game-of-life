import init, { GameOfLife } from "./pkg/game_of_life.js";

const CANVAS_ID = "gameCanvas";
let gameOfLife = null;
let isPlaying = false;
let animationId = null;
let speed = 100;

async function run() {
    await init();
    
    const canvas = document.getElementById(CANVAS_ID);
    const gridWidth = parseInt(document.getElementById("gridWidth").value);
    const gridHeight = parseInt(document.getElementById("gridHeight").value);
    
    canvas.width = 800;
    canvas.height = 600;
    
    try {
        gameOfLife = new GameOfLife(CANVAS_ID, gridWidth, gridHeight);
        console.log("Game of Life created successfully");
        
        gameOfLife.render();
    } catch (error) {
        console.error("Failed to initialize Game of Life:", error);
    }
}

function gameLoop() {
    if (!gameOfLife || !isPlaying) return;
    
    try {
        gameOfLife.step();
        gameOfLife.render();
    } catch (error) {
        console.error("Error in game loop:", error);
        stop();
    }
    
    setTimeout(() => {
        if (isPlaying) {
            animationId = requestAnimationFrame(gameLoop);
        }
    }, speed);
}

function play() {
    if (!gameOfLife) return;
    
    isPlaying = true;
    document.getElementById("playPause").textContent = "Pause";
    gameLoop();
}

function pause() {
    isPlaying = false;
    document.getElementById("playPause").textContent = "Play";
    if (animationId) {
        cancelAnimationFrame(animationId);
    }
}

function stop() {
    pause();
}

function step() {
    if (!gameOfLife) return;
    
    try {
        gameOfLife.step();
        gameOfLife.render();
    } catch (error) {
        console.error("Error stepping:", error);
    }
}

function randomize() {
    if (!gameOfLife) return;
    
    try {
        gameOfLife.randomize();
        
        gameOfLife.render();
    } catch (error) {
        console.error("Error randomizing:", error);
    }
}

function clear() {
    if (!gameOfLife) return;
    
    try {
        gameOfLife.clear();
        gameOfLife.render();
    } catch (error) {
        console.error("Error clearing:", error);
    }
}

function loadGlider() {
    if (!gameOfLife) return;
    
    try {
        gameOfLife.load_glider();
        gameOfLife.render();
    } catch (error) {
        console.error("Error loading glider:", error);
    }
}

function loadOscillator() {
    if (!gameOfLife) return;
    
    try {
        gameOfLife.load_oscillator();
        gameOfLife.render();
    } catch (error) {
        console.error("Error loading oscillator:", error);
    }
}

function loadBeacon() {
    if (!gameOfLife) return;
    
    try {
        gameOfLife.load_beacon();
        gameOfLife.render();
    } catch (error) {
        console.error("Error loading beacon:", error);
    }
}

function loadToad() {
    if (!gameOfLife) return;
    
    try {
        gameOfLife.load_toad();
        gameOfLife.render();
    } catch (error) {
        console.error("Error loading toad:", error);
    }
}

function loadSpaceship() {
    if (!gameOfLife) return;
    
    try {
        gameOfLife.load_spaceship();
        gameOfLife.render();
    } catch (error) {
        console.error("Error loading spaceship:", error);
    }
}

function loadPulsar() {
    if (!gameOfLife) return;
    
    try {
        gameOfLife.load_pulsar();
        gameOfLife.render();
    } catch (error) {
        console.error("Error loading pulsar:", error);
    }
}

function loadGliderGun() {
    if (!gameOfLife) return;
    
    try {
        gameOfLife.load_glider_gun();
        gameOfLife.render();
    } catch (error) {
        console.error("Error loading glider gun:", error);
    }
}

function resize() {
    if (!gameOfLife) return;
    
    const gridWidth = parseInt(document.getElementById("gridWidth").value);
    const gridHeight = parseInt(document.getElementById("gridHeight").value);
    
    try {
        gameOfLife.resize(gridWidth, gridHeight);
        gameOfLife.render();
    } catch (error) {
        console.error("Error resizing:", error);
    }
}

document.addEventListener("DOMContentLoaded", () => {
    run();
    
    document.getElementById("playPause").addEventListener("click", () => {
        if (isPlaying) {
            pause();
        } else {
            play();
        }
    });
    
    document.getElementById("step").addEventListener("click", step);
    document.getElementById("randomize").addEventListener("click", randomize);
    document.getElementById("clear").addEventListener("click", clear);
    document.getElementById("resize").addEventListener("click", resize);
    
    document.getElementById("glider").addEventListener("click", loadGlider);
    document.getElementById("blinker").addEventListener("click", loadOscillator);
    document.getElementById("beacon").addEventListener("click", loadBeacon);
    document.getElementById("toad").addEventListener("click", loadToad);
    document.getElementById("spaceship").addEventListener("click", loadSpaceship);
    document.getElementById("pulsar").addEventListener("click", loadPulsar);
    document.getElementById("gliderGun").addEventListener("click", loadGliderGun);
    
    const speedSlider = document.getElementById("speed");
    const speedValue = document.getElementById("speedValue");
    
    speedSlider.addEventListener("input", (e) => {
        speed = parseInt(e.target.value);
        speedValue.textContent = speed;
    });
    
    const canvas = document.getElementById(CANVAS_ID);
    canvas.addEventListener("click", (e) => {
        if (!gameOfLife) return;
        
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        
        const gridWidth = parseInt(document.getElementById("gridWidth").value);
        const gridHeight = parseInt(document.getElementById("gridHeight").value);
        
        const gridX = Math.floor((x / rect.width) * gridWidth);
        const gridY = Math.floor(((rect.height - y) / rect.height) * gridHeight);
        
        try {
            gameOfLife.add_cells_in_area(gridX, gridY, 1);
            gameOfLife.render();
        } catch (error) {
            console.error("Error adding cells:", error);
        }
    });
    
    document.addEventListener("keydown", (e) => {
        switch(e.key) {
            case " ":
                e.preventDefault();
                if (isPlaying) {
                    pause();
                } else {
                    play();
                }
                break;
            case "s":
                e.preventDefault();
                step();
                break;
            case "r":
                e.preventDefault();
                randomize();
                break;
            case "c":
                e.preventDefault();
                clear();
                break;
        }
    });
});

window.addEventListener("beforeunload", () => {
    if (animationId) {
        cancelAnimationFrame(animationId);
    }
});
