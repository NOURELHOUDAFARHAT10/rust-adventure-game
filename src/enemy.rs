use ggez::{Context, GameResult, graphics};
use ggez::graphics::{Canvas, DrawParam, Color, Image, Mesh};
use ggez::mint::Point2;

#[derive(Debug, Clone)]
pub struct Enemy {
    pub position: Point2<f32>,
    pub health: i32,
    pub max_health: i32,
    pub enemy_type: EnemyType,
    pub speed: f32,
    pub attack_timer: f32,
    pub is_alive: bool,
    sprite: Option<Image>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyType {
    Goblin,
    Orc,
    Dragon,
}

impl Enemy {
    pub fn new(ctx: &mut Context, x: f32, y: f32, enemy_type: EnemyType) -> Self {
        let (health, speed) = match enemy_type {
            EnemyType::Goblin => (30, 80.0),
            EnemyType::Orc => (60, 50.0),
            EnemyType::Dragon => (120, 40.0),
        };

        let sprite_path = match enemy_type {
            EnemyType::Goblin => "/goblin.png",
            EnemyType::Orc => "/orc.png",
            EnemyType::Dragon => "/dragon.png",
        };
        
        let sprite = Image::from_path(ctx, sprite_path).ok();

        Enemy {
            position: Point2 { x, y },
            health,
            max_health: health,
            enemy_type,
            speed,
            attack_timer: 0.0,
            is_alive: true,
            sprite,
        }
    }

    pub fn update(&mut self, dt: f32, player_pos: Point2<f32>) {
        if !self.is_alive {
            return;
        }

        let dx = player_pos.x - self.position.x;
        let dy = player_pos.y - self.position.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 10.0 {
            let direction_x = dx / distance;
            let direction_y = dy / distance;

            self.position.x += direction_x * self.speed * dt;
            self.position.y += direction_y * self.speed * dt;
        }

        if self.attack_timer > 0.0 {
            self.attack_timer -= dt;
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
        if self.health <= 0 {
            self.is_alive = false;
        }
    }

    pub fn can_attack(&self) -> bool {
        self.attack_timer <= 0.0 && self.is_alive
    }

    pub fn attack(&mut self) -> i32 {
        self.attack_timer = 1.5;
        
        match self.enemy_type {
            EnemyType::Goblin => 5,
            EnemyType::Orc => 10,
            EnemyType::Dragon => 20,
        }
    }

    pub fn is_in_attack_range(&self, player_pos: Point2<f32>) -> bool {
        let dx = player_pos.x - self.position.x;
        let dy = player_pos.y - self.position.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        distance < 30.0
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult<()> {
        if !self.is_alive {
            return Ok(());
        }

        if let Some(sprite) = &self.sprite {
            canvas.draw(
                sprite,
                DrawParam::default()
                    .dest(self.position)
                    .offset([0.5, 0.5])
                    .color(Color::WHITE),
            );
        } else {
           
            let color = match self.enemy_type {
                EnemyType::Goblin => Color::from_rgb(0, 150, 0),
                EnemyType::Orc => Color::from_rgb(150, 75, 0),
                EnemyType::Dragon => Color::from_rgb(150, 0, 0),
            };

            let circle = Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Point2 { x: 0.0, y: 0.0 },
                12.0,
                0.1,
                color,
            )?;

            canvas.draw(&circle, DrawParam::default().dest(self.position));
        }

    
        let health_ratio = self.health as f32 / self.max_health as f32;
        let health_bar = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(-15.0, -25.0, 30.0 * health_ratio, 3.0),
            Color::from_rgb(0, 200, 0),
        )?;

        canvas.draw(&health_bar, DrawParam::default().dest(self.position));

        Ok(())
    }
}