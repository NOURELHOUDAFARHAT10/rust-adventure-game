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
        quest_progress: (u32, u32),
        game_time: f32,
        game_state: &GameState,
    ) -> GameResult<()> {
        let (screen_width, _screen_height) = ctx.gfx.drawable_size();

        let health_ratio = if player_max_health > 0 {
            (player_health as f32) / (player_max_health as f32)
        } else {
            0.0
        }
        .clamp(0.0, 1.0);

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

        let hp_text = Text::new(format!("Nour: {}/{}", player_health, player_max_health));
        canvas.draw(&hp_text, DrawParam::default().dest(Point2 { x: 25.0, y: 22.0 }));

        let coins_text = Text::new(format!("Pièces: {}", player_coins));
        canvas.draw(&coins_text, DrawParam::default().dest(Point2 { x: 25.0, y: 46.0 }));

        let weapon_text = Text::new(format!("Arme: Niveau {}", player_weapon_level));
        canvas.draw(&weapon_text, DrawParam::default().dest(Point2 { x: 25.0, y: 68.0 }));

        // Quête et temps
        let quest_text = Text::new(format!("Artefacts: {}/{}", quest_progress.0, quest_progress.1));
        canvas.draw(&quest_text, DrawParam::default().dest(Point2 { x: 25.0, y: 92.0 }));

        let time_text = Text::new(format!("Temps: {:.1}s", game_time));
        canvas.draw(&time_text, DrawParam::default().dest(Point2 { x: 25.0, y: 110.0 }));

        // Contrôles
        let controls_text = Text::new("Contrôles: ZQSD - Déplacement, Espace - Attaque");
        let ctrl_x = (screen_width - 400.0).max(10.0);
        canvas.draw(&controls_text, DrawParam::default().dest(Point2 { x: ctrl_x, y: 20.0 }));

        // Etats de jeu
        match game_state {
            GameState::Playing => {}
            GameState::GameOver => {
                let go = Text::new("GAME OVER - Appuyez sur R pour recommencer");
                canvas.draw(&go, DrawParam::default().dest(Point2 { x: 200.0, y: 220.0 }));
            }
            GameState::Victory => {
                let v = Text::new("VICTOIRE ! - Appuyez sur R pour recommencer");
                canvas.draw(&v, DrawParam::default().dest(Point2 { x: 200.0, y: 220.0 }));
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