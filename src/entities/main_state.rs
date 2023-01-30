use ggez::conf::Conf;
use ggez::{Context, GameResult, event, graphics};
use ggez::mint::Point2;

use crate::assets::Assets;
use crate::entities::board::Board;
use crate::entities::hand::Hand;
use crate::entities::top_panel::TopPanel;

pub struct MainState {
    assets: Assets,
    game_board: Board,
    player_hand: Hand,
    top_panel: TopPanel,
    screen_width: f32,
    screen_heigth: f32,
}

impl MainState {
    pub fn new(ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height; 
       // println!("{} {}",screen_height, screen_width);
        Ok(MainState {
            assets: Assets::new(ctx)?,
            game_board: Board::new(Point2 { x: 180.0 as f32, y: 100.0 }).unwrap(),
            player_hand: Hand::new().unwrap(),
            top_panel: TopPanel::new().unwrap(),
            screen_width: screen_width,
            screen_heigth: screen_height,
        })
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while ctx.time.check_update_time(DESIRED_FPS) {
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) ->  GameResult<()>  {
        let red = graphics::Color::from_rgb(75, 0, 130);
        let mut canvas = graphics::Canvas::from_frame(ctx, red);

        self.game_board.draw(&mut canvas, &self.assets);
        
        canvas.finish(ctx)?;
        Ok(())
    }
}
