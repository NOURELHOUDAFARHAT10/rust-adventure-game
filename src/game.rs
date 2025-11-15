use ggez::{Context, GameResult, event, graphics, input::keyboard::{KeyCode, KeyInput}};
use ggez::graphics::Canvas;
use ggez::mint::Point2;
use rand::Rng;

use crate::player::{Player, Direction};
use crate::enemy::{Enemy, EnemyType};
use crate::items::{Item, ItemType};
use crate::map::Map;
use crate::ui::{UI, GameState};

pub struct Game {
    player: Player,
    enemies: Vec<Enemy>,
    items: Vec<Item>,
    map: Map,
    game_state: GameState,
    spawn_timer: f32,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let map_width = 800.0;
        let map_height = 600.0;

        let mut game = Game {
            player: Player::new(ctx, 400.0, 300.0)?, 
            enemies: Vec::new(),
            items: Vec::new(),
            map: Map::new(map_width, map_height),
            game_state: GameState::Playing,
            spawn_timer: 2.0, 
        };

        game.spawn_initial_items(ctx)?;
        
        Ok(game)
    }

    fn spawn_initial_items(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut rng = rand::thread_rng();
        
        for _ in 0..15 {
            let item_type = match rng.gen_range(0..3) {
                0 => ItemType::Coin,
                1 => ItemType::Potion,
                _ => ItemType::WeaponUpgrade,
            };
            
            let position = Point2 {
                x: rng.gen_range(100.0..700.0),
                y: rng.gen_range(100.0..500.0),
            };
            
            self.items.push(Item::new(ctx, position.x, position.y, item_type));
        }
        
        Ok(())
    }

    fn spawn_enemy(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut rng = rand::thread_rng();
        
        let enemy_type = match rng.gen_range(0..10) {
            0..=5 => EnemyType::Goblin,
            6..=8 => EnemyType::Orc,
            _ => EnemyType::Dragon,
        };

        let position = Point2 {
            x: rng.gen_range(100.0..700.0),
            y: rng.gen_range(100.0..500.0),
        };

        self.enemies.push(Enemy::new(ctx, position.x, position.y, enemy_type));
        Ok(())
    }

    fn check_collisions(&mut self) {
        for item in &mut self.items {
            if !item.is_collected && item.is_colliding_with_player(self.player.position) {
                match item.collect() {
                    ItemType::Coin => {
                        self.player.add_coin();
                        println!(" Pièce collectée ! Total: {}", self.player.coins);
                    }
                    ItemType::Potion => {
                        self.player.heal(25);
                        println!(" Potion utilisée ! PV: {}", self.player.health);
                    }
                    ItemType::WeaponUpgrade => {
                        self.player.upgrade_weapon();
                        println!(" Arme améliorée ! Niveau: {}", self.player.weapon_level);
                    }
                }
            }
        }

        self.items.retain(|item| !item.is_collected);

        for enemy in &mut self.enemies {
            if enemy.is_alive && enemy.is_in_attack_range(self.player.position) && enemy.can_attack() {
                let damage = enemy.attack();
                self.player.take_damage(damage);
                println!(" Touché ! PV restants: {}", self.player.health);
                
                if !self.player.is_alive() {
                    self.game_state = GameState::GameOver;
                    println!(" GAME OVER!");
                }
            }
        }

        if self.player.is_attacking {
            for enemy in &mut self.enemies {
                if enemy.is_alive {
                    let dx = enemy.position.x - self.player.position.x;
                    let dy = enemy.position.y - self.player.position.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance < 50.0 { 
                        let damage = self.player.get_attack_damage();
                        enemy.take_damage(damage);
                        println!(" Ennemi touché ! Dégâts: {}", damage);
                    }
                }
            }
        }

        self.enemies.retain(|enemy| enemy.is_alive);
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.game_state != GameState::Playing {
            return Ok(());
        }

        let dt = ctx.time.delta().as_secs_f32();

        self.player.update(dt, self.map.width, self.map.height);

        for enemy in &mut self.enemies {
            enemy.update(dt, self.player.position);
        }

        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 && self.enemies.len() < 5 {
            self.spawn_enemy(ctx)?;
            self.spawn_timer = 3.0;
            println!(" Nouvel ennemi spawné ! Total: {}", self.enemies.len());
        }

        self.check_collisions();

        if self.player.coins >= 20 {
            self.game_state = GameState::Victory;
            println!(" VICTOIRE !");
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::BLACK);

        self.map.draw(ctx, &mut canvas)?;

        for item in &self.items {
            item.draw(ctx, &mut canvas)?;
        }

        for enemy in &self.enemies {
            enemy.draw(ctx, &mut canvas)?;
        }

        self.player.draw(ctx, &mut canvas)?;

        UI::draw(
            ctx,
            &mut canvas,
            self.player.health,
            self.player.max_health,
            self.player.coins,
            self.player.weapon_level,
            &self.game_state,
        )?;

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult<()> {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Z | KeyCode::Up => {
                    self.player.move_direction(Direction::Up);
                    println!("⬆ Haut");
                }
                KeyCode::S | KeyCode::Down => {
                    self.player.move_direction(Direction::Down);
                    println!("⬇ Bas");
                }
                KeyCode::Q | KeyCode::Left => {
                    self.player.move_direction(Direction::Left);
                    println!("⬅ Gauche");
                }
                KeyCode::D | KeyCode::Right => {
                    self.player.move_direction(Direction::Right);
                    println!("Droite");
                }
                KeyCode::Space => {
                    self.player.attack();
                    println!(" Attaque !");
                }
                KeyCode::R if self.game_state != GameState::Playing => {
                    println!(" Redémarrage...");
                    *self = Game::new(ctx)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> GameResult<()> {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Z | KeyCode::S | KeyCode::Q | KeyCode::D |
                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                    self.player.stop_movement();
                }
                _ => {}
            }
        }
        Ok(())
    }
}