# Rust Chess Client with Python AI Backend

A simple project that connects a fast desktop GUI built in Rust to a Python Flask server running a PyTorch Deep Q-Network (DQN) evaluator. The Rust app handles user input, rendering, and logic, while the Python server dictates the opponent's moves.

## Project Structure
* **Rust Client:** Uses `macroquad` for basic window management/rendering and `shakmaty` for standard chess rule verification and FEN parsing.
* **Python Server:** A lightweight `Flask` API running a 3-layer `PyTorch` neural network that scores positions and applies basic reinforcement learning updates on the fly after every move it makes.

---

## How It Works
1. You play as **White** against the engine.
2. When it is **Black's** turn, the Rust client converts the board position into a FEN string and hits the Flask backend endpoint.
3. The AI loops through every legal move, simulates the resulting layout, feeds it into the neural network, and chooses the one with the best position score.
4. It updates its weights immediately via a backpropagation step (penalizing bad setups or heavily rewarding checkmates) and returns the chosen move string back to Rust.

---

## Getting Started

### 1. Fire up the Python AI Backend
Make sure you have your virtual environment active and dependencies installed (`torch`, `python-chess`, `flask`).

```bash
# Install required libraries
pip install torch flask python-chess

# Run the backend script
python server.py
```
The server will boot up locally on http://127.0.0.1:5000. Keep this terminal window open.

2. Launch the Rust Client
Open a second terminal window in your Rust project folder and execute:

```Bash
cargo run
```
