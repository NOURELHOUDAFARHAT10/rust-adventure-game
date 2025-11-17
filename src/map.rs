use ggez::{Context, GameResult, graphics};
use ggez::graphics::{Canvas, DrawParam, Color, Image, Mesh};
use ggez::mint::Point2;

pub struct Map {
    pub width: f32,
    pub height: f32,
    tiles: Vec<Tile>,
    obstacles: Vec<Obstacle>,
    tileset: Option<Image>,
}

#[derive(Clone)]
struct Tile {
    position: Point2<f32>,
    tile_type: TileType,
}

#[derive(Clone, PartialEq)]
enum TileType {
    Grass,
    Path,
    Water,
    Stone,
}

struct Obstacle {
    position: Point2<f32>,
    width: f32,
    height: f32,
    obstacle_type: ObstacleType,
}

#[derive(PartialEq)]
enum ObstacleType {
    Tree,
    Rock,
    Bush,
}

impl Map {
    pub fn new(ctx: &mut Context, width: f32, height: f32) -> GameResult<Self> {
        let tileset = Image::from_path(ctx, "/tileset.png").ok();
        
        let mut tiles = Vec::new();
        let mut obstacles = Vec::new();
        
        let tile_size = 64.0;
        let cols = (width / tile_size).ceil() as usize;
        let rows = (height / tile_size).ceil() as usize;

        for x in 0..cols {
            for y in 0..rows {
                let position = Point2 {
                    x: x as f32 * tile_size + tile_size / 2.0,
                    y: y as f32 * tile_size + tile_size / 2.0,
                };

                let tile_type = if (x + y) % 4 == 0 {
                    TileType::Path
                } else if (x * y) % 7 == 0 {
                    TileType::Stone
                } else if (x + y * 2) % 5 == 0 {
                    TileType::Water
                } else {
                    TileType::Grass
                };

                tiles.push(Tile { position, tile_type });

                if (x + y) % 6 == 0 && x > 2 && x < cols - 2 && y > 2 && y < rows - 2 {
                    let obstacle_type = match (x + y) % 3 {
                        0 => ObstacleType::Tree,
                        1 => ObstacleType::Rock,
                        _ => ObstacleType::Bush,
                    };
                    
                    obstacles.push(Obstacle {
                        position,
                        width: 40.0,
                        height: 40.0,
                        obstacle_type,
                    });
                }
            }
        }

        Ok(Map {
            width,
            height,
            tiles,
            obstacles,
            tileset,
        })
    }

    pub fn is_position_valid(&self, position: Point2<f32>, radius: f32) -> bool {
    
        if position.x < radius || position.x > self.width - radius ||
           position.y < radius || position.y > self.height - radius {
            return false;
        }

       
        for obstacle in &self.obstacles {
            let dx = obstacle.position.x - position.x;
            let dy = obstacle.position.y - position.y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            if distance < (obstacle.width / 2.0 + radius) {
                return false;
            }
        }

        true
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult<()> {
        let tile_size = 64.0;

     
        for tile in &self.tiles {
            let color = match tile.tile_type {
                TileType::Grass => Color::from_rgb(100, 200, 100),
                TileType::Path => Color::from_rgb(200, 180, 100),
                TileType::Water => Color::from_rgb(100, 150, 255),
                TileType::Stone => Color::from_rgb(150, 150, 150),
            };

            let tile_mesh = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    tile.position.x - tile_size / 2.0,
                    tile.position.y - tile_size / 2.0,
                    tile_size,
                    tile_size,
                ),
                color,
            )?;
            canvas.draw(&tile_mesh, DrawParam::default());

          
            let border = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(1.0),
                graphics::Rect::new(
                    tile.position.x - tile_size / 2.0,
                    tile.position.y - tile_size / 2.0,
                    tile_size,
                    tile_size,
                ),
                Color::from_rgba(255, 255, 255, 50),
            )?;
            canvas.draw(&border, DrawParam::default());
        }

       
        for obstacle in &self.obstacles {
            let (color, shape) = match obstacle.obstacle_type {
                ObstacleType::Tree => (Color::from_rgb(50, 120, 50), "circle"),
                ObstacleType::Rock => (Color::from_rgb(120, 120, 120), "rectangle"),
                ObstacleType::Bush => (Color::from_rgb(80, 160, 80), "circle"),
            };

            let obstacle_mesh = if shape == "circle" {
                Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Point2 { x: 0.0, y: 0.0 },
                    obstacle.width / 2.0,
                    0.1,
                    color,
                )?
            } else {
                Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        -obstacle.width / 2.0,
                        -obstacle.height / 2.0,
                        obstacle.width,
                        obstacle.height,
                    ),
                    color,
                )?
            };

            canvas.draw(&obstacle_mesh, DrawParam::default().dest(obstacle.position));
        }

        Ok(())
    }
}