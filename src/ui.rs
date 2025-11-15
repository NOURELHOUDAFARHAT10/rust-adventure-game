use ggez::{Context, GameResult, graphics};
use ggez::graphics::{Canvas, DrawParam, Color, Text, Mesh};
use ggez::mint::Point2;

pub struct UI;

impl UI {
    pub fn draw(
        ctx: &mut Context,
        canvas: &mut Canvas,
        player_health: i32,
        player_max_health: i32,
        player_coins: u32,
        player_weapon_level: u32,
        game_state: &GameState,
    ) -> GameResult<()> {
        let screen_width = ctx.gfx.drawable_size().0;

        let health_ratio = player_health as f32 / player_max_health as f32;
        let health_bar_width = 200.0;
        
        let health_background = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(20.0, 20.0, health_bar_width, 20.0),
            Color::from_rgb(100, 0, 0),
        )?;
        
        let health_fill = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(20.0, 20.0, health_bar_width * health_ratio, 20.0),
            Color::from_rgb(0, 200, 0),
        )?;

        canvas.draw(&health_background, DrawParam::default());
        canvas.draw(&health_fill, DrawParam::default());

       
        let health_text = Text::new(format!("Nour: {}/{}", player_health, player_max_health));
        canvas.draw(&health_text, DrawParam::default().dest(Point2 { x: 25.0, y: 22.0 }));

        let coins_text = Text::new(format!("Pièces: {}", player_coins));
        canvas.draw(&coins_text, DrawParam::default().dest(Point2 { x: 25.0, y: 50.0 }));

        let weapon_text = Text::new(format!("Arme: Niveau {}", player_weapon_level));
        canvas.draw(&weapon_text, DrawParam::default().dest(Point2 { x: 25.0, y: 75.0 }));

      
        let controls_text = Text::new("Contrôles: ZQSD - Déplacement, Espace - Attaque");
        canvas.draw(
            &controls_text,
            DrawParam::default().dest(Point2 { x: screen_width - 400.0, y: 20.0 }),
        );

     
        match game_state {
            GameState::Playing => {}
            GameState::GameOver => {
                let game_over_text = Text::new("GAME OVER - Appuyez sur R pour recommencer");
                canvas.draw(
                    &game_over_text,
                    DrawParam::default()
                        .dest(Point2 { x: screen_width / 2.0 - 150.0, y: 300.0 })
                        .color(Color::RED)
                        .scale([2.0, 2.0]),
                );
            }
            GameState::Victory => {
                let victory_text = Text::new("VICTOIRE ! - Appuyez sur R pour recommencer");
                canvas.draw(
                    &victory_text,
                    DrawParam::default()
                        .dest(Point2 { x: screen_width / 2.0 - 150.0, y: 300.0 })
                        .color(Color::GREEN)
                        .scale([2.0, 2.0]),
                );
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    Playing,
    GameOver,
    Victory,
}