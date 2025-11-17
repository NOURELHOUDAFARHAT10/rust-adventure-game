use ggez::{Context, GameResult, graphics};
use ggez::graphics::{Canvas, DrawParam, Color, Image, Mesh};
use ggez::mint::Point2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemType {
    Coin,
    Potion,
    WeaponUpgrade,
    QuestArtifact,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub position: Point2<f32>,
    pub item_type: ItemType,
    pub is_collected: bool,
    sprite: Option<Image>,
}

impl Item {
    pub fn new(ctx: &mut Context, x: f32, y: f32, item_type: ItemType) -> Self {
        let sprite_path = match item_type {
            ItemType::Coin => "/coin.png",
            ItemType::Potion => "/potion.png",
            ItemType::WeaponUpgrade => "/weapon.png",
            ItemType::QuestArtifact => "/artifact.png",
        };
        
        let sprite = Image::from_path(ctx, sprite_path).ok();

        Item {
            position: Point2 { x, y },
            item_type,
            is_collected: false,
            sprite,
        }
    }

    pub fn new_quest(ctx: &mut Context, x: f32, y: f32) -> Self {
        let sprite = Image::from_path(ctx, "/artifact.png").ok();

        Item {
            position: Point2 { x, y },
            item_type: ItemType::QuestArtifact,
            is_collected: false,
            sprite,
        }
    }

    pub fn collect(&mut self) -> ItemType {
        self.is_collected = true;
        self.item_type
    }


    pub fn is_colliding_with_player(&self, player_pos: Point2<f32>) -> bool {
        if self.is_collected {
            return false;
        }

        let dx = player_pos.x - self.position.x;
        let dy = player_pos.y - self.position.y;
        let distance = (dx * dx + dy * dy).sqrt();

  
        distance < 25.0
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult<()> {
        if self.is_collected {
            return Ok(());
        }

        if let Some(sprite) = &self.sprite {
            let time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f32();
            
         
            match self.item_type {
                ItemType::QuestArtifact => {
              
                    let pulse = 1.0 + (time * 4.0).sin() * 0.2;
                    let glow = ((time * 3.0).sin() * 0.3 + 0.7) as f32;
                    
                    canvas.draw(
                        sprite,
                        DrawParam::default()
                            .dest(self.position)
                            .offset([0.5, 0.5])
                            .scale([pulse, pulse])
                            .color(Color::from_rgba(255, 255, 255, (glow * 255.0) as u8)),
                    );
                }
                _ => {
               
                    let pulse = 1.0 + (time * 3.0).sin() * 0.1;
                    canvas.draw(
                        sprite,
                        DrawParam::default()
                            .dest(self.position)
                            .offset([0.5, 0.5])
                            .scale([pulse, pulse])
                            .color(Color::WHITE),
                    );
                }
            }
        } else {
       
            let (color, size) = match self.item_type {
                ItemType::Coin => (Color::YELLOW, 8.0),
                ItemType::Potion => (Color::from_rgb(255, 0, 255), 10.0),
                ItemType::WeaponUpgrade => (Color::from_rgb(200, 200, 0), 12.0),
                ItemType::QuestArtifact => (Color::from_rgb(255, 215, 0), 15.0), // Artefact doré
            };

            let shape = Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Point2 { x: 0.0, y: 0.0 },
                size,
                0.1,
                color,
            )?;

            canvas.draw(&shape, DrawParam::default().dest(self.position));
        }

        Ok(())
    }
}