use std::path::PathBuf;
mod game;
mod player;
mod enemy;
mod items;
mod map;
mod ui;

use ggez::{ContextBuilder, event, conf};
use game::Game;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resource_dir = PathBuf::from("./resources");

    let window_setup = conf::WindowSetup::default()
        .title("Jeu d'Aventure");
    
    let window_mode = conf::WindowMode::default()
        .dimensions(800.0, 600.0);

    let (mut ctx, event_loop) = ContextBuilder::new("jeu_aventure", "VotreNom")
        .add_resource_path(resource_dir)
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()?;

    let game = Game::new(&mut ctx)?;
    
    println!(" Jeu lancé avec succès !");
    println!(" Mode WSL : Audio désactivé");
    println!(" Prêt à jouer !");
    
    event::run(ctx, event_loop, game);
}