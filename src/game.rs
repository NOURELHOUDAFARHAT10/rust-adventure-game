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
    quest_items_collected: u32,
    total_quest_items: u32,
    boss_spawned: bool,
    game_time: f32,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let map_width = 800.0;
        let map_height = 600.0;

        let mut game = Game {
            player: Player::new(ctx, 400.0, 300.0)?,
            enemies: Vec::new(),
            items: Vec::new(),
            map: Map::new(ctx, map_width, map_height)?,
            game_state: GameState::Playing,
            spawn_timer: 2.0,
            quest_items_collected: 0,
            total_quest_items: 5, 
            boss_spawned: false,
            game_time: 0.0,
        };

        game.spawn_initial_items(ctx)?;
        game.spawn_quest_items(ctx)?;
        
        println!(" Nouvelle partie lancée !");
        println!(" Objectif: Collecter {} artefacts pour affronter le Dragon !", game.total_quest_items);
        
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
            
            let mut position;
            let mut attempts = 0;
            
            loop {
                position = Point2 {
                    x: rng.gen_range(100.0..700.0),
                    y: rng.gen_range(100.0..500.0),
                };
                
                if self.map.is_position_valid(position, 10.0) && attempts < 50 {
                    break;
                }
                attempts += 1;
                
                if attempts >= 50 {
                    position = Point2 { x: 400.0, y: 300.0 };
                    break;
                }
            }
            
            self.items.push(Item::new(ctx, position.x, position.y, item_type));
        }
        
        Ok(())
    }

    fn spawn_quest_items(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut rng = rand::thread_rng();
        
        for i in 0..self.total_quest_items {
            let mut position;
            let mut attempts = 0;
            
            loop {
                position = Point2 {
                    x: rng.gen_range(100.0..700.0),
                    y: rng.gen_range(100.0..500.0),
                };
                
                if self.map.is_position_valid(position, 15.0) && attempts < 50 {
                    break;
                }
                attempts += 1;
                
                if attempts >= 50 {
                
                    let angle = (i as f32 / self.total_quest_items as f32) * 2.0 * std::f32::consts::PI;
                    position = Point2 {
                        x: 400.0 + angle.cos() * 200.0,
                        y: 300.0 + angle.sin() * 150.0,
                    };
                    break;
                }
            }
            
         
            self.items.push(Item::new_quest(ctx, position.x, position.y));
        }
        
        println!(" {} artefacts cachés sur la carte !", self.total_quest_items);
        Ok(())
    }

    fn spawn_enemy(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut rng = rand::thread_rng();
        
        let enemy_type = match rng.gen_range(0..10) {
            0..=5 => EnemyType::Goblin,
            6..=8 => EnemyType::Orc,
            _ => EnemyType::Dragon,
        };

        let mut position;
        let mut attempts = 0;
        
        loop {
         
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let distance = rng.gen_range(150.0..300.0);
            position = Point2 {
                x: self.player.position.x + angle.cos() * distance,
                y: self.player.position.y + angle.sin() * distance,
            };
            
            if self.map.is_position_valid(position, 12.0) &&
               position.x >= 50.0 && position.x <= self.map.width - 50.0 &&
               position.y >= 50.0 && position.y <= self.map.height - 50.0 &&
               attempts < 30 {
                break;
            }
            attempts += 1;
            
            if attempts >= 30 {
               
                position = Point2 {
                    x: rng.gen_range(100.0..700.0),
                    y: rng.gen_range(100.0..500.0),
                };
                break;
            }
        }

        self.enemies.push(Enemy::new(ctx, position.x, position.y, enemy_type));
        
        if enemy_type == EnemyType::Dragon && !self.boss_spawned {
            println!(" Un dragon sauvage apparaît !");
        }
        
        Ok(())
    }

    fn spawn_boss(&mut self, ctx: &mut Context) -> GameResult<()> {
        if !self.boss_spawned {
         
            let boss_position = Point2 { x: 400.0, y: 100.0 };
            let boss = Enemy::new(ctx, boss_position.x, boss_position.y, EnemyType::Dragon);
            self.enemies.push(boss);
            self.boss_spawned = true;
            println!(" LE BOSS DRAGON APPARAÎT !");
            println!(" Affrontez-le pour gagner la partie !");
        }
        Ok(())
    }

    fn check_collisions(&mut self) {
 
        let mut new_quest_items = 0;
        
        for item in &mut self.items {
            if !item.is_collected && item.is_colliding_with_player(self.player.position) {
                match item.collect() {
                    ItemType::Coin => {
                        self.player.add_coin();
                        println!("Pièce collectée ! Total: {}", self.player.coins);
                    }
                    ItemType::Potion => {
                        let old_health = self.player.health;
                        self.player.heal(25);
                        println!(" Potion utilisée ! Nour: {} → {}", old_health, self.player.health);
                    }
                    ItemType::WeaponUpgrade => {
                        let old_level = self.player.weapon_level;
                        self.player.upgrade_weapon();
                        println!("  Arme améliorée ! Niveau: {} → {}", old_level, self.player.weapon_level);
                    }
                    ItemType::QuestArtifact => {
                        self.quest_items_collected += 1;
                        new_quest_items += 1;
                        let remaining = self.total_quest_items - self.quest_items_collected;
                        println!(" ARTEFACT COLLECTÉ ! ({}/{}) - Plus que {} restant(s)", 
                                self.quest_items_collected, self.total_quest_items, remaining);
                    }
                }
            }
        }

        if new_quest_items > 0 && self.quest_items_collected == self.total_quest_items {
            println!(" TOUS LES ARTEFACTS COLLECTÉS !");
        }

        self.items.retain(|item| !item.is_collected);

      
        for enemy in &mut self.enemies {
            if enemy.is_alive && enemy.is_in_attack_range(self.player.position) && enemy.can_attack() {
                let damage = enemy.attack();
                let old_health = self.player.health;
                self.player.take_damage(damage);
                
                let enemy_name = match enemy.enemy_type {
                    EnemyType::Goblin => "Goblin",
                    EnemyType::Orc => "Orc",
                    EnemyType::Dragon => "Dragon",
                };
                
                println!(" {} vous attaque ! -{} PV ({} → {})", 
                        enemy_name, damage, old_health, self.player.health);
                
                if !self.player.is_alive() {
                    self.game_state = GameState::GameOver;
                    println!(" GAME OVER! Le héros est tombé au combat...");
                }
            }
        }

        if self.player.is_attacking {
            let mut enemies_hit = 0;
            
            for enemy in &mut self.enemies {
                if enemy.is_alive {
                    let dx = enemy.position.x - self.player.position.x;
                    let dy = enemy.position.y - self.player.position.y;
                    let distance = (dx * dx + dy * dy).sqrt();

                    if distance < 50.0 { 
                        let damage = self.player.get_attack_damage();
                        let old_health = enemy.health;
                        enemy.take_damage(damage);
                        enemies_hit += 1;
                        
                        let enemy_name = match enemy.enemy_type {
                            EnemyType::Goblin => "Goblin",
                            EnemyType::Orc => "Orc",
                            EnemyType::Dragon => "Dragon",
                        };
                        
                        if !enemy.is_alive {
                            println!(" {} vaincu ! -{} nour", enemy_name, damage);
                        } else {
                            println!(" {} touché ! -{} nour ({} → {})", 
                                    enemy_name, damage, old_health, enemy.health);
                        }
                    }
                }
            }
            
            if enemies_hit > 0 {
                println!("Attaque réussie ! {} ennemi(s) touché(s)", enemies_hit);
            }
        }

    
        let enemies_before = self.enemies.len();
        self.enemies.retain(|enemy| enemy.is_alive);
        let enemies_after = self.enemies.len();
        
        if enemies_before != enemies_after {
            println!(" {} ennemi(s) nettoyé(s)", enemies_before - enemies_after);
        }
    }

    fn check_quest_progress(&mut self, ctx: &mut Context) -> GameResult<()> {
     
        if self.quest_items_collected >= self.total_quest_items && !self.boss_spawned {
            println!(" Tous les artefacts collectés ! Le boss final arrive...");
            self.spawn_boss(ctx)?;
        }
    
        if self.boss_spawned {
            let boss_alive = self.enemies.iter()
                .any(|e| e.is_alive && e.enemy_type == EnemyType::Dragon);
            
            if !boss_alive {
                self.game_state = GameState::Victory;
                println!("VICTOIRE ! Vous avez sauvé le royaume !");
                println!(" Temps total: {:.1} secondes", self.game_time);
                println!(" Pièces collectées: {}", self.player.coins);
            }
        }
        
        Ok(())
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.game_state != GameState::Playing {
            return Ok(());
        }

        let dt = ctx.time.delta().as_secs_f32();
        self.game_time += dt;
        self.player.update(dt, self.map.width, self.map.height);

        for enemy in &mut self.enemies {
            enemy.update(dt, self.player.position);
        }

        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 && self.enemies.len() < 8 {
            self.spawn_enemy(ctx)?;
            self.spawn_timer = 3.0;
        }

        self.check_collisions();

     
        self.check_quest_progress(ctx)?;

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
            (self.quest_items_collected, self.total_quest_items),
            self.game_time,
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
                }
                KeyCode::S | KeyCode::Down => {
                    self.player.move_direction(Direction::Down);
                }
                KeyCode::Q | KeyCode::Left => {
                    self.player.move_direction(Direction::Left);
                }
                KeyCode::D | KeyCode::Right => {
                    self.player.move_direction(Direction::Right);
                }
                KeyCode::Space => {
                    self.player.attack();
                }
                KeyCode::R if self.game_state != GameState::Playing => {
                    println!(" Redémarrage de la partie...");
                    *self = Game::new(ctx)?;
                }
                KeyCode::Escape => {
                    println!("Au revoir !");
                    ctx.request_quit();
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