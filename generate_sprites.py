from PIL import Image, ImageDraw
import os

# Créer le dossier resources s'il n'existe pas
os.makedirs('resources', exist_ok=True)

def create_player(size=64):
    """Créer le sprite du joueur (héros vert)"""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Corps (cercle vert)
    draw.ellipse([8, 8, size-8, size-8], fill=(0, 200, 0, 255))
    
    # Bordure noire
    draw.ellipse([8, 8, size-8, size-8], outline=(0, 0, 0, 255), width=3)
    
    # Yeux
    draw.ellipse([18, 20, 26, 28], fill=(255, 255, 255, 255))
    draw.ellipse([38, 20, 46, 28], fill=(255, 255, 255, 255))
    draw.ellipse([20, 22, 24, 26], fill=(0, 0, 0, 255))
    draw.ellipse([40, 22, 44, 26], fill=(0, 0, 0, 255))
    
    # Sourire
    draw.arc([18, 25, 46, 45], 0, 180, fill=(0, 0, 0, 255), width=2)
    
    img.save('resources/player.png')
    print("✅ player.png créé")

def create_goblin(size=64):
    """Créer le sprite du goblin (petit, vert foncé)"""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Corps (cercle vert foncé)
    draw.ellipse([12, 12, size-12, size-12], fill=(0, 100, 0, 255))
    
    # Bordure
    draw.ellipse([12, 12, size-12, size-12], outline=(0, 50, 0, 255), width=2)
    
    # Yeux méchants (rouges)
    draw.ellipse([20, 22, 26, 28], fill=(200, 0, 0, 255))
    draw.ellipse([38, 22, 44, 28], fill=(200, 0, 0, 255))
    
    # Dents pointues
    draw.polygon([25, 35, 28, 40, 22, 40], fill=(255, 255, 255, 255))
    draw.polygon([35, 35, 38, 40, 32, 40], fill=(255, 255, 255, 255))
    
    img.save('resources/goblin.png')
    print("✅ goblin.png créé")

def create_orc(size=64):
    """Créer le sprite de l'orc (moyen, marron)"""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Corps (cercle marron/orange)
    draw.ellipse([8, 8, size-8, size-8], fill=(150, 75, 0, 255))
    
    # Bordure
    draw.ellipse([8, 8, size-8, size-8], outline=(100, 50, 0, 255), width=3)
    
    # Yeux jaunes féroces
    draw.ellipse([18, 20, 26, 28], fill=(255, 200, 0, 255))
    draw.ellipse([38, 20, 46, 28], fill=(255, 200, 0, 255))
    draw.ellipse([20, 22, 24, 26], fill=(0, 0, 0, 255))
    draw.ellipse([40, 22, 44, 26], fill=(0, 0, 0, 255))
    
    # Cicatrice
    draw.line([22, 18, 28, 14], fill=(200, 0, 0, 255), width=2)
    
    # Bouche méchante
    draw.arc([18, 30, 46, 45], 180, 360, fill=(0, 0, 0, 255), width=3)
    
    img.save('resources/orc.png')
    print("✅ orc.png créé")

def create_dragon(size=64):
    """Créer le sprite du dragon (grand, rouge)"""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Corps (cercle rouge)
    draw.ellipse([4, 4, size-4, size-4], fill=(200, 0, 0, 255))
    
    # Bordure épaisse
    draw.ellipse([4, 4, size-4, size-4], outline=(150, 0, 0, 255), width=4)
    
    # Yeux de feu (jaune-orange)
    draw.ellipse([16, 18, 26, 30], fill=(255, 150, 0, 255))
    draw.ellipse([38, 18, 48, 30], fill=(255, 150, 0, 255))
    draw.ellipse([19, 21, 23, 27], fill=(255, 0, 0, 255))
    draw.ellipse([41, 21, 45, 27], fill=(255, 0, 0, 255))
    
    # Cornes
    draw.polygon([16, 8, 20, 4, 20, 12], fill=(100, 0, 0, 255))
    draw.polygon([44, 4, 48, 8, 44, 12], fill=(100, 0, 0, 255))
    
    # Flammes de la bouche
    draw.ellipse([26, 36, 32, 42], fill=(255, 200, 0, 255))
    draw.ellipse([32, 36, 38, 42], fill=(255, 100, 0, 255))
    
    img.save('resources/dragon.png')
    print("✅ dragon.png créé")

def create_coin(size=32):
    """Créer le sprite de la pièce (or)"""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Pièce dorée
    draw.ellipse([4, 4, size-4, size-4], fill=(255, 215, 0, 255))
    draw.ellipse([4, 4, size-4, size-4], outline=(200, 150, 0, 255), width=2)
    
    # Symbole $ ou €
    draw.text((10, 6), "$", fill=(200, 150, 0, 255))
    
    img.save('resources/coin.png')
    print("✅ coin.png créé")

def create_potion(size=32):
    """Créer le sprite de la potion (flacon magenta)"""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Flacon (rectangle avec haut)
    draw.rectangle([10, 12, 22, 28], fill=(200, 0, 200, 255))
    draw.rectangle([10, 12, 22, 28], outline=(150, 0, 150, 255), width=2)
    
    # Goulot
    draw.rectangle([13, 8, 19, 12], fill=(150, 0, 150, 255))
    
    # Bouchon
    draw.rectangle([12, 6, 20, 8], fill=(100, 50, 0, 255))
    
    # Liquide brillant
    draw.ellipse([12, 16, 20, 24], fill=(255, 100, 255, 255))
    
    img.save('resources/potion.png')
    print("✅ potion.png créé")

def create_weapon(size=32):
    """Créer le sprite de l'amélioration d'arme (épée)"""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Lame (gris métallique)
    draw.polygon([16, 4, 20, 20, 16, 22, 12, 20], fill=(200, 200, 220, 255))
    draw.polygon([16, 4, 20, 20, 16, 22, 12, 20], outline=(150, 150, 170, 255), width=2)
    
    # Garde
    draw.rectangle([10, 20, 22, 23], fill=(150, 100, 0, 255))
    
    # Poignée
    draw.rectangle([14, 23, 18, 28], fill=(100, 50, 0, 255))
    
    # Pommeau
    draw.ellipse([13, 27, 19, 30], fill=(200, 150, 0, 255))
    
    # Éclat magique
    draw.line([16, 8, 16, 16], fill=(255, 255, 100, 255), width=2)
    
    img.save('resources/weapon.png')
    print("✅ weapon.png créé")

# Générer tous les sprites
print("🎨 Génération des sprites du jeu...")
print("=" * 40)

create_player(64)
create_goblin(64)
create_orc(64)
create_dragon(64)
create_coin(32)
create_potion(32)
create_weapon(32)

print("=" * 40)
print("🎉 Tous les sprites ont été créés avec succès !")
print("📁 Emplacement : ./resources/")
print("\nSprites créés :")
print("  👤 player.png   - Héros (vert)")
print("  👺 goblin.png   - Goblin (vert foncé, petit)")
print("  👹 orc.png      - Orc (marron, moyen)")
print("  🐉 dragon.png   - Dragon (rouge, grand)")
print("  💰 coin.png     - Pièce d'or")
print("  🧪 potion.png   - Potion de soin")
print("  ⚔️  weapon.png   - Amélioration d'arme")
