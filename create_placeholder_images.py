from PIL import Image, ImageDraw
import os

# Créer le dossier resources s'il n'existe pas
os.makedirs('resources/images', exist_ok=True)

# Player sprite sheet (3x4 frames)
player_img = Image.new('RGBA', (384, 192), (0, 0, 0, 0))
draw = ImageDraw.Draw(player_img)
for row in range(4):
    for col in range(3):
        x = col * 128
        y = row * 48
        # Différentes couleurs pour différentes animations
        colors = [(0, 255, 0), (0, 200, 0), (255, 255, 0), (255, 200, 0)]
        draw.ellipse([x+20, y+10, x+108, y+38], fill=colors[row])
        # Bras et jambes pour faire un personnage
        draw.rectangle([x+40, y+38, x+48, y+46], fill=colors[row])  # jambe gauche
        draw.rectangle([x+80, y+38, x+88, y+46], fill=colors[row])  # jambe droite
player_img.save('resources/images/player_sheet.png')

# Enemy sprite sheet
enemy_img = Image.new('RGBA', (384, 192), (0, 0, 0, 0))
draw = ImageDraw.Draw(enemy_img)
for row in range(4):
    for col in range(3):
        x = col * 128
        y = row * 48
        colors = [(255, 0, 0), (200, 0, 0), (255, 100, 0), (200, 50, 0)]
        draw.rectangle([x+30, y+10, x+98, y+38], fill=colors[row])  # Corps carré
        # Yeux
        draw.ellipse([x+45, y+15, x+55, y+25], fill=(255, 255, 255))
        draw.ellipse([x+73, y+15, x+83, y+25], fill=(255, 255, 255))
enemy_img.save('resources/images/enemy_sheet.png')

# Tileset pour la map
tileset = Image.new('RGBA', (256, 256), (0, 0, 0, 0))
draw = ImageDraw.Draw(tileset)
# Différents types de tuiles
draw.rectangle([0, 0, 64, 64], fill=(100, 80, 60))    # Terre
draw.rectangle([64, 0, 128, 64], fill=(80, 100, 60))  # Herbe
draw.rectangle([128, 0, 192, 64], fill=(60, 80, 100)) # Eau
draw.rectangle([192, 0, 256, 64], fill=(90, 90, 90))  # Pierre

draw.rectangle([0, 64, 64, 128], fill=(120, 100, 80)) # Sable
draw.rectangle([64, 64, 128, 128], fill=(70, 90, 50)) # Forêt
draw.rectangle([128, 64, 192, 128], fill=(50, 70, 90)) # Eau profonde
draw.rectangle([192, 64, 256, 128], fill=(110, 110, 110)) # Mur

# Ajouter des motifs
for i in range(4):
    for j in range(4):
        x = i * 64 + 10
        y = j * 64 + 10
        if (i + j) % 2 == 0:
            draw.rectangle([x, y, x+8, y+8], fill=(255, 255, 255, 100))
tileset.save('resources/images/tileset.png')

# Coin
coin_img = Image.new('RGBA', (32, 32), (0, 0, 0, 0))
draw = ImageDraw.Draw(coin_img)
draw.ellipse([4, 4, 28, 28], fill=(255, 215, 0))
draw.ellipse([8, 8, 24, 24], fill=(255, 255, 0))
draw.text((12, 10), "$", fill=(0, 0, 0))
coin_img.save('resources/images/coin.png')

# Potion
potion_img = Image.new('RGBA', (32, 32), (0, 0, 0, 0))
draw = ImageDraw.Draw(potion_img)
draw.ellipse([8, 4, 24, 12], fill=(255, 0, 255))  # Bouchon
draw.rectangle([12, 12, 20, 28], fill=(200, 0, 200))  # Corps
draw.ellipse([12, 26, 20, 30], fill=(180, 0, 180))  # Base
potion_img.save('resources/images/potion.png')

# Arme
weapon_img = Image.new('RGBA', (32, 16), (0, 0, 0, 0))
draw = ImageDraw.Draw(weapon_img)
draw.rectangle([4, 6, 28, 10], fill=(200, 200, 200))  # Lame
draw.rectangle([0, 4, 8, 12], fill=(150, 100, 50))    # Poignée
weapon_img.save('resources/images/weapon.png')

# Background
bg_img = Image.new('RGBA', (800, 600), (40, 40, 80))
draw = ImageDraw.Draw(bg_img)
# Ajouter des étoiles/points pour le background
for i in range(50):
    x = (i * 37) % 780 + 10
    y = (i * 23) % 580 + 10
    draw.ellipse([x, y, x+4, y+4], fill=(255, 255, 255, 100))
bg_img.save('resources/images/background.png')

print("✅ Toutes les images placeholder ont été créées !")