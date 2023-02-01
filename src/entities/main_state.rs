use std::vec;

use ggez::conf::Conf;
use ggez::{Context, GameResult, event, graphics};
use ggez::mint::Point2;
use rand::seq::SliceRandom;

use crate::assets::{Assets, DominoInHand};
use crate::entities::board::Board;
use crate::entities::hand::Hand;
use crate::entities::top_panel::TopPanel;

use rand::{thread_rng, Rng};
pub struct MainState {
    assets: Assets,
    all_dominos: Vec<DominoInHand>,
    pub game_board: Board,
    pub player_hand: Hand,
    top_panel: TopPanel,
    screen_width: f32,
    screen_heigth: f32,
}

impl MainState {
    pub fn new(ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;

        let hand_panel_offset_from_start = 116.0 as f32; 
        let hand_panel_y_start = 744.0 as f32;
        let card_width = 128.0 as f32;


        let all_dominos = all();
        let mut player_hand:Vec<DominoInHand> = Vec::new();
        for i in 0..6 {
            let mut dominInHand = all_dominos[i].clone();
            dominInHand.position = Point2 { 
                x: hand_panel_offset_from_start + (i as f32 * card_width) as f32,
                y: hand_panel_y_start
            };
            dominInHand.visible=true;
            player_hand.push(dominInHand);
        }
        let mut second_row_x= hand_panel_offset_from_start  + card_width;
        let mut second_row_y = hand_panel_y_start + card_width;
        for i in 0..4 {
            let mut dominInHand = DominoInHand::new((0,0), Point2 { 
                 x: second_row_x + (i as f32)*card_width, y: second_row_y },
                 0.0,
                 false).unwrap();
            player_hand.push(dominInHand);
        }

        

        Ok(MainState {
            assets: Assets::new(ctx)?,
            all_dominos: all_dominos.clone(),


            game_board: Board::new(Point2 { x: 180.0 as f32, y: 100.0 }).unwrap(),

            player_hand: Hand::new(player_hand.clone()).unwrap(),
           
           
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
        let purple = graphics::Color::from_rgb(75, 0, 130);
        let mut canvas = graphics::Canvas::from_frame(ctx, purple);

        self.game_board.draw(&mut canvas, ctx,&self.assets);
        self.player_hand.draw(&mut canvas,ctx, &self.assets);

        canvas.finish(ctx)?;
        Ok(())
    }
}
pub fn all() -> Vec<DominoInHand>
{
  let mut vec= vec![
    DominoInHand::new((0,0), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((0,1), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((0,2), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((0,3), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((0,4), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((0,5), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((0,6), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((1,1), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((1,2), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((1,3), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((1,4), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((1,5), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((1,6), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((2,2), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((2,3), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((2,4), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((2,5), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((2,6), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((3,3), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((3,4), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((3,5), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((3,6), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((4,4), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((4,5), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((4,6), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((5,5), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((5,6), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
    DominoInHand::new((6,6), Point2 { x: 0.0, y: 0.0 } , 0.0, false).unwrap(),
  ];
  vec.shuffle(&mut thread_rng());
  vec
}