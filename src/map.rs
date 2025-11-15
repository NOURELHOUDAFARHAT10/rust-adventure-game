use ggez::{Context, GameResult, graphics};
use ggez::graphics::{Canvas, DrawParam, Color, Mesh};
use ggez::mint::Point2;

pub struct Map {
    pub width: f32,
    pub height: f32,
    obstacles: Vec<Obstacle>,
}

struct Obstacle {
    position: Point2<f32>,
    width: f32,
    height: f32,
}

impl Map {
    pub fn new(width: f32, height: f32) -> Self {
        let mut obstacles = Vec::new();

       
        for i in 0..8 {
            for j in 0..6 {
                if (i + j) % 3 == 0 { 
                    obstacles.push(Obstacle {
                        position: Point2 {
                            x: (i as f32 * 100.0) + 50.0,
                            y: (j as f32 * 80.0) + 50.0,
                        },
                        width: 60.0,
                        height: 60.0,
                    });
                }
            }
        }

        Map {
            width,
            height,
            obstacles,
        }
    }

    pub fn is_position_valid(&self, position: Point2<f32>, radius: f32) -> bool {
       
        if position.x < radius || position.x > self.width - radius ||
           position.y < radius || position.y > self.height - radius {
            return false;
        }

        for obstacle in &self.obstacles {
            let obstacle_left = obstacle.position.x - obstacle.width / 2.0;
            let obstacle_right = obstacle.position.x + obstacle.width / 2.0;
            let obstacle_top = obstacle.position.y - obstacle.height / 2.0;
            let obstacle_bottom = obstacle.position.y + obstacle.height / 2.0;

            let player_left = position.x - radius;
            let player_right = position.x + radius;
            let player_top = position.y - radius;
            let player_bottom = position.y + radius;

            if player_right > obstacle_left && player_left < obstacle_right &&
               player_bottom > obstacle_top && player_top < obstacle_bottom {
                return false;
            }
        }

        true
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult<()> {
    
        let background = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, self.width, self.height),
            Color::from_rgb(40, 40, 80),
        )?;
        canvas.draw(&background, DrawParam::default());

       
        for obstacle in &self.obstacles {
            let obstacle_mesh = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    obstacle.position.x - obstacle.width / 2.0,
                    obstacle.position.y - obstacle.height / 2.0,
                    obstacle.width,
                    obstacle.height,
                ),
                Color::from_rgb(100, 70, 40),
            )?;
            canvas.draw(&obstacle_mesh, DrawParam::default());
        }

        Ok(())
    }
}