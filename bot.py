import chess
import torch
import torch.nn as nn
import torch.optim as optim
from flask import Flask, request, jsonify

app = Flask(__name__)

class ChessEvaluator(nn.Module):
    def __init__(self):
        super(ChessEvaluator, self).__init__()
        self.fc1 = nn.Linear(64, 128)
        self.fc2 = nn.Linear(128, 64)
        self.output = nn.Linear(64, 1)

    forward = lambda self, x: self.output(torch.relu(self.fc2(torch.relu(self.fc1(x)))))

model = ChessEvaluator()
optimizer = optim.Adam(model.parameters(), lr=0.005)

def board_to_tensor(board):
    squares = []
    for square in chess.SQUARES:
        piece = board.piece_at(square)
        if piece is None:
            squares.append(0)
        else:
            val = piece.piece_type
            squares.append(val if piece.color == chess.WHITE else -val)
    return torch.tensor(squares, dtype=torch.float32)

@app.route('/predict', methods=['POST'])
def predict():
    data = request.json
    fen = data.get("fen")
    board = chess.Board(fen)
    
    if board.is_game_over():
        return jsonify({"move": None, "status": "game_over"})

    legal_moves = list(board.legal_moves)
    best_move = None
    best_value = float('inf')  
    best_tensor = None

    for move in legal_moves:
        board.push(move)
        state_tensor = board_to_tensor(board)
        with torch.no_grad():
            value = model(state_tensor).item()
        board.pop()

        if value < best_value:
            best_value = value
            best_move = move
            best_tensor = state_tensor

    board.push(best_move)
    reward = -100 if board.is_checkmate() else 0 
    board.pop()

    target = torch.tensor([best_value + reward], dtype=torch.float32)
    current_prediction = model(best_tensor)
    
    loss = nn.MSELoss()(current_prediction, target)
    optimizer.zero_grad()
    loss.backward()
    optimizer.step()

    return jsonify({"move": best_move.uci()})

if __name__ == '__main__':
    app.run(port=5000)