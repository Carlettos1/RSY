from PIL import Image
from os import mkdir

def name(idx):
    color = "white" if idx < 6 else "black"
    piece = idx % 6
    name = ""
    match piece:
        case 0: name = "king"
        case 1: name = "queen"
        case 2: name = "bishop"
        case 3: name = "knight"
        case 4: name = "rook"
        case 5: name = "pawn"
    return f"{color}_{name}"

mkdir("tiles/")
sheet = Image.open("sprites.png")
i = 0
l = 213
for y in range(2):
    for x in range(6):
        x0 = x*l
        y0 = y*l
        x1 = x0 + l
        y1 = y0 + l
        sheet.crop(( x0, y0, x1, y1 )).save(f"tiles/{name(i)}.png")
        i += 1