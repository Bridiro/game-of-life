import init, { GameOfLife } from "./pkg/game_of_life.js";

const GRID_MIN_SIZE = 10;
const GRID_MAX_SIZE = 2000;
const BRUSH_CURSOR_SVG = "url('data:image/svg+xml;utf8,<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"20\" height=\"20\"><circle cx=\"10\" cy=\"10\" r=\"8\" fill=\"none\" stroke=\"%2300ff41\" stroke-width=\"2\"/></svg>') 10 10, crosshair";
const CANVAS_ID = "gameCanvas";
let gameOfLife = null;
let isPlaying = false;
let animationId = null;
let speed = 100;
let generation = 0;
let fpsCounter = 0;
let lastFpsTime = Date.now();

let drawMode = "single";
let isDrawing = false;
let lastDrawPos = null;
let brushSize = 3;

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
        setupEventListeners();
        gameOfLife.render();
        updateStats();
    } catch (error) {
        console.error("Failed to initialize Game of Life:", error);
    }
}

function setupEventListeners() {
    const canvas = document.getElementById(CANVAS_ID);
    
    canvas.addEventListener("mousedown", handleCanvasMouseDown);
    canvas.addEventListener("mousemove", handleCanvasMouseMove);
    canvas.addEventListener("mouseup", handleCanvasMouseUp);
    canvas.addEventListener("mouseleave", handleCanvasMouseUp);
    
    canvas.addEventListener("touchstart", handleCanvasTouchStart, { passive: false });
    canvas.addEventListener("touchmove", handleCanvasTouchMove, { passive: false });
    canvas.addEventListener("touchend", handleCanvasTouchEnd, { passive: false });
    
    document.getElementById("playPause").addEventListener("click", togglePlayPause);
    document.getElementById("step").addEventListener("click", step);
    document.getElementById("randomize").addEventListener("click", randomize);
    document.getElementById("clear").addEventListener("click", clear);
    
    document.querySelectorAll(".tool-btn").forEach(btn => {
        btn.addEventListener("click", (e) => {
            setDrawMode(e.target.dataset.mode);
        });
    });
    
    document.getElementById("applyGridSize").addEventListener("click", resizeGrid);
    
    document.getElementById("glider").addEventListener("click", loadGlider);
    document.getElementById("blinker").addEventListener("click", loadBlinker);
    document.getElementById("beacon").addEventListener("click", loadBeacon);
    document.getElementById("toad").addEventListener("click", loadToad);
    document.getElementById("spaceship").addEventListener("click", loadSpaceship);
    document.getElementById("pulsar").addEventListener("click", loadPulsar);
    document.getElementById("gliderGun").addEventListener("click", loadGliderGun);
    
    const speedSlider = document.getElementById("speed");
    const brushSlider = document.getElementById("brushSize");
    
    speedSlider.addEventListener("input", (e) => {
        speed = parseInt(e.target.value);
        document.getElementById("speedValue").textContent = speed;
    });
    
    brushSlider.addEventListener("input", (e) => {
        brushSize = parseInt(e.target.value);
        document.getElementById("brushSizeValue").textContent = brushSize;
    });
}

function getCanvasCoordinates(canvas, clientX, clientY) {
    const rect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / rect.width;
    const scaleY = canvas.height / rect.height;
    
    const x = Math.floor((clientX - rect.left) * scaleX);
    const y = Math.floor((clientY - rect.top) * scaleY);
    
    const gridWidth = parseInt(document.getElementById("gridWidth").value);
    const gridHeight = parseInt(document.getElementById("gridHeight").value);
    
    const cellX = Math.floor((x / canvas.width) * gridWidth);
    const cellY = gridHeight - 1 - Math.floor((y / canvas.height) * gridHeight);
    
    return { cellX, cellY };
}

function handleCanvasMouseDown(e) {
    if (!gameOfLife) return;
    
    const canvas = document.getElementById(CANVAS_ID);
    const { cellX, cellY } = getCanvasCoordinates(canvas, e.clientX, e.clientY);
    
    isDrawing = true;
    lastDrawPos = { x: cellX, y: cellY };
    
    if (drawMode === "single") {
        gameOfLife.toggle_cell(cellX, cellY);
        gameOfLife.render();
    } else if (drawMode === "brush") {
        drawBrush(cellX, cellY);
        gameOfLife.render();
    }
}

function handleCanvasMouseMove(e) {
    if (!gameOfLife || !isDrawing) return;
    
    const canvas = document.getElementById(CANVAS_ID);
    const { cellX, cellY } = getCanvasCoordinates(canvas, e.clientX, e.clientY);
    
    if (drawMode === "line" && lastDrawPos) {
        // Preview line (in real implementation, you'd want to show a preview)
    } else if (drawMode === "brush") {
        drawBrush(cellX, cellY);
        gameOfLife.render();
        lastDrawPos = { x: cellX, y: cellY };
    }
}

function handleCanvasMouseUp(e) {
    if (!gameOfLife || !isDrawing) return;
    
    const canvas = document.getElementById(CANVAS_ID);
    const { cellX, cellY } = getCanvasCoordinates(canvas, e.clientX, e.clientY);
    
    if (drawMode === "line" && lastDrawPos) {
        gameOfLife.draw_line(lastDrawPos.x, lastDrawPos.y, cellX, cellY);
        gameOfLife.render();
    }
    
    isDrawing = false;
    lastDrawPos = null;
}

function handleCanvasTouchStart(e) {
    e.preventDefault();
    const touch = e.touches[0];
    handleCanvasMouseDown({ clientX: touch.clientX, clientY: touch.clientY });
}

function handleCanvasTouchMove(e) {
    e.preventDefault();
    const touch = e.touches[0];
    handleCanvasMouseMove({ clientX: touch.clientX, clientY: touch.clientY });
}

function handleCanvasTouchEnd(e) {
    e.preventDefault();
    if (e.changedTouches.length > 0) {
        const touch = e.changedTouches[0];
        handleCanvasMouseUp({ clientX: touch.clientX, clientY: touch.clientY });
    }
}

function drawBrush(centerX, centerY) {
    const radius = Math.floor(brushSize / 2);
    
    for (let dx = -radius; dx <= radius; dx++) {
        for (let dy = -radius; dy <= radius; dy++) {
            if (dx * dx + dy * dy <= radius * radius) {
                const x = centerX + dx;
                const y = centerY + dy;
                if (x >= 0 && y >= 0) {
                    gameOfLife.set_cell(x, y, 255);
                }
            }
        }
    }
}

function setDrawMode(mode) {
    drawMode = mode;
    document.getElementById("currentDrawMode").textContent = mode.toUpperCase();
    
    document.querySelectorAll(".tool-btn").forEach(btn => {
        btn.classList.remove("active");
    });
    document.querySelector(`[data-mode="${mode}"]`).classList.add("active");
    
    const canvas = document.getElementById(CANVAS_ID);
    if (mode === "brush") {
        canvas.style.cursor = BRUSH_CURSOR_SVG;
    } else {
        canvas.style.cursor = "crosshair";
    }
}

function gameLoop() {
    if (!gameOfLife || !isPlaying) return;
    
    try {
        gameOfLife.step();
        gameOfLife.render();
        generation++;
        updateStats();
        
        fpsCounter++;
        const now = Date.now();
        if (now - lastFpsTime >= 1000) {
            document.getElementById("fpsCounter").textContent = fpsCounter;
            fpsCounter = 0;
            lastFpsTime = now;
        }
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

function updateStats() {
    document.getElementById("generationCounter").textContent = generation;
}

function togglePlayPause() {
    if (isPlaying) {
        stop();
    } else {
        play();
    }
}

function play() {
    if (!gameOfLife) return;
    
    isPlaying = true;
    document.getElementById("playPause").innerHTML = "⏸ PAUSE";
    document.getElementById("playPause").classList.add("active");
    gameLoop();
}

function stop() {
    isPlaying = false;
    document.getElementById("playPause").innerHTML = "▶ PLAY";
    document.getElementById("playPause").classList.remove("active");
    
    if (animationId) {
        cancelAnimationFrame(animationId);
        animationId = null;
    }
}

function step() {
    if (!gameOfLife || isPlaying) return;
    
    try {
        gameOfLife.step();
        gameOfLife.render();
        generation++;
        updateStats();
    } catch (error) {
        console.error("Error in step:", error);
    }
}

function randomize() {
    if (!gameOfLife) return;
    
    try {
        gameOfLife.randomize();
        gameOfLife.render();
        generation = 0;
        updateStats();
    } catch (error) {
        console.error("Error in randomize:", error);
    }
}

function clear() {
    if (!gameOfLife) return;
    
    try {
        gameOfLife.clear();
        gameOfLife.render();
        generation = 0;
        updateStats();
    } catch (error) {
        console.error("Error in clear:", error);
    }
}

function resizeGrid() {
    const gridWidth = parseInt(document.getElementById("gridWidth").value);
    const gridHeight = parseInt(document.getElementById("gridHeight").value);
    
    if (!gameOfLife || gridWidth < GRID_MIN_SIZE || gridHeight < GRID_MIN_SIZE || gridWidth > GRID_MAX_SIZE || gridHeight > GRID_MAX_SIZE) {
        alert("Grid dimensions must be between 10 and 2000\n\nNote: Very large grids (>1000) may impact performance depending on your hardware.");
        return;
    }
    
    try {
        gameOfLife.resize(gridWidth, gridHeight);
        gameOfLife.render();
        generation = 0;
        updateStats();
        console.log(`Grid resized to ${gridWidth}x${gridHeight}`);
    } catch (error) {
        console.error("Error resizing grid:", error);
    }
}

function loadGlider() {
    if (!gameOfLife) return;
    try {
        gameOfLife.load_glider();
        gameOfLife.render();
        generation = 0;
        updateStats();
    } catch (error) {
        console.error("Error loading glider:", error);
    }
}

function loadBlinker() {
    if (!gameOfLife) return;
    try {
        gameOfLife.load_oscillator();
        gameOfLife.render();
        generation = 0;
        updateStats();
    } catch (error) {
        console.error("Error loading blinker:", error);
    }
}

function loadBeacon() {
    if (!gameOfLife) return;
    try {
        gameOfLife.load_beacon();
        gameOfLife.render();
        generation = 0;
        updateStats();
    } catch (error) {
        console.error("Error loading beacon:", error);
    }
}

function loadToad() {
    if (!gameOfLife) return;
    try {
        gameOfLife.load_toad();
        gameOfLife.render();
        generation = 0;
        updateStats();
    } catch (error) {
        console.error("Error loading toad:", error);
    }
}

function loadSpaceship() {
    if (!gameOfLife) return;
    try {
        gameOfLife.load_spaceship();
        gameOfLife.render();
        generation = 0;
        updateStats();
    } catch (error) {
        console.error("Error loading spaceship:", error);
    }
}

function loadPulsar() {
    if (!gameOfLife) return;
    try {
        gameOfLife.load_pulsar();
        gameOfLife.render();
        generation = 0;
        updateStats();
    } catch (error) {
        console.error("Error loading pulsar:", error);
    }
}

function loadGliderGun() {
    if (!gameOfLife) return;
    try {
        gameOfLife.load_glider_gun();
        gameOfLife.render();
        generation = 0;
        updateStats();
    } catch (error) {
        console.error("Error loading glider gun:", error);
    }
}

run();
