use ggez::{Context, GameResult, graphics};
use ggez::graphics::{Canvas, DrawParam, Color, Image, Mesh};
use ggez::mint::Point2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Player {
    pub position: Point2<f32>,
    pub velocity: Point2<f32>,
    pub health: i32,
    pub max_health: i32,
    pub coins: u32,
    pub direction: Direction,
    pub speed: f32,
    pub is_attacking: bool,
    pub attack_timer: f32,
    pub weapon_level: u32,
    sprite: Option<Image>,
}

impl Player {
    pub fn new(ctx: &mut Context, x: f32, y: f32) -> GameResult<Self> {
        let sprite = Image::from_path(ctx, "/player.png").ok();
        
        Ok(Player {
            position: Point2 { x, y },
            velocity: Point2 { x: 0.0, y: 0.0 },
            health: 100,
            max_health: 100,
            coins: 0,
            direction: Direction::Right,
            speed: 200.0,
            is_attacking: false,
            attack_timer: 0.0,
            weapon_level: 1,
            sprite,
        })
    }

    pub fn update(&mut self, dt: f32, map_width: f32, map_height: f32) {
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
        self.position.x = self.position.x.clamp(20.0, map_width - 20.0);
        self.position.y = self.position.y.clamp(20.0, map_height - 20.0);

        if self.is_attacking {
            self.attack_timer -= dt;
            if self.attack_timer <= 0.0 {
                self.is_attacking = false;
            }
        }
    }

    pub fn move_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                self.velocity.x = -self.speed;
                self.direction = Direction::Left;
            }
            Direction::Right => {
                self.velocity.x = self.speed;
                self.direction = Direction::Right;
            }
            Direction::Up => {
                self.velocity.y = -self.speed;
            }
            Direction::Down => {
                self.velocity.y = self.speed;
            }
        }
    }

    pub fn stop_movement(&mut self) {
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;
    }

    pub fn attack(&mut self) {
        if !self.is_attacking {
            self.is_attacking = true;
            self.attack_timer = 0.3;
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn add_coin(&mut self) {
        self.coins += 1;
    }

    pub fn upgrade_weapon(&mut self) {
        self.weapon_level += 1;
    }

    pub fn get_attack_damage(&self) -> i32 {
        match self.weapon_level {
            1 => 10,
            2 => 15,
            3 => 20,
            _ => 25,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult<()> {
        if let Some(sprite) = &self.sprite {
            let scale = if self.is_attacking { 1.2 } else { 1.0 };
            let color = if self.is_attacking { 
                Color::from_rgba(255, 255, 150, 255) 
            } else { 
                Color::WHITE 
            };
            
            canvas.draw(
                sprite,
                DrawParam::default()
                    .dest(self.position)
                    .offset([0.5, 0.5])
                    .scale([scale, scale])
                    .color(color),
            );
        } else {
          
            let color = if self.is_attacking { Color::YELLOW } else { Color::GREEN };
            
            let circle = Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Point2 { x: 0.0, y: 0.0 },
                15.0,
                0.1,
                color,
            )?;

            canvas.draw(&circle, DrawParam::default().dest(self.position));

           
            if self.is_attacking {
                let weapon_offset = match self.direction {
                    Direction::Right => Point2 { x: 25.0, y: 0.0 },
                    Direction::Left => Point2 { x: -25.0, y: 0.0 },
                    Direction::Up => Point2 { x: 0.0, y: -25.0 },
                    Direction::Down => Point2 { x: 0.0, y: 25.0 },
                };

                let weapon = Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(-5.0, -3.0, 10.0, 6.0),
                    Color::RED,
                )?;

                canvas.draw(
                    &weapon,
                    DrawParam::default()
                        .dest(Point2 {
                            x: self.position.x + weapon_offset.x,
                            y: self.position.y + weapon_offset.y,
                        })
                        .rotation(match self.direction {
                            Direction::Up => -std::f32::consts::FRAC_PI_2,
                            Direction::Down => std::f32::consts::FRAC_PI_2,
                            _ => 0.0,
                        }),
                );
            }
        }

        Ok(())
    }
}