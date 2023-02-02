use std::vec;

use ggez::conf::Conf;
use ggez::event::MouseButton;
use ggez::input::keyboard;
use ggez::{Context, GameResult, event, graphics};
use ggez::mint::Point2;
use rand::seq::SliceRandom;

use crate::assets::{Assets, DominoInHand, DominoOnTable};
use crate::entities::board::Board;
use crate::entities::enums::BoardCell;
use crate::entities::hand::Hand;
use crate::entities::top_panel::TopPanel;

use rand::{thread_rng, Rng};

use super::enums::{PlayerState, DominoInHandState};


const skeletonHealth:usize = 4;
const gravesCount: usize = 1;

pub struct MainState {
    assets: Assets,
    all_dominos: Vec<DominoInHand>,
    pub game_board: Board,
    pub player_hand: Hand,
    top_panel: TopPanel,
    player_state: PlayerState,
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
            dominInHand.state=DominoInHandState::Visible(true);
            player_hand.push(dominInHand);
        }
        let mut second_row_x= hand_panel_offset_from_start  + card_width;
        let mut second_row_y = hand_panel_y_start + card_width;
        for i in 0..4 {
            let mut dominInHand = DominoInHand::new((0,0), Point2 { 
                 x: second_row_x + (i as f32)*card_width, y: second_row_y },
                 0.0,
                 DominoInHandState::Visible( false)).unwrap();
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
            player_state: PlayerState::Active,
        })
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while ctx.time.check_update_time(DESIRED_FPS) {
           let seconds= 1.0 / DESIRED_FPS as f32;
           match &mut self.player_state {
            PlayerState::Active => {
                if ctx.mouse.button_pressed(MouseButton::Left) {
                   let mouse_position= ctx.mouse.position();

                   let res = self.player_hand.check_boundary_of_click(mouse_position);
                   if !res.0 {
                    continue;
                   }
                   self.player_state = PlayerState::Dragging { remember_x: self.player_hand.hand[res.1].position.x,
                     remember_y: self.player_hand.hand[res.1].position.y,
                     index_of_domino_in_hand: res.1 };
                     self.player_hand.hand[res.1].state= DominoInHandState::Moving;
                    //self.player_hand.update_domino_position(res.1, mouse_position,ctx,seconds);
                }
            },
            PlayerState::Dragging { remember_x, remember_y, index_of_domino_in_hand } => {
                if ctx.mouse.button_pressed(MouseButton::Left) {
                    let mouse_position = ctx.mouse.position();
                   // println!("{:?} {:?}",mouse_position,ctx.mouse.delta());
                    self.player_hand.update_domino_position(*index_of_domino_in_hand, mouse_position , ctx, seconds);
                }

                if ctx.mouse.button_just_released(MouseButton::Left) {
                    let mouse_position = ctx.mouse.position();
                    let res = Board::check_boundary_of_release(&self.game_board, mouse_position, &mut self.player_hand, index_of_domino_in_hand);
                    println!("{:?}",res);
                    if !res.0 {
                        let index = *index_of_domino_in_hand;
                        self.player_hand.update_domino_position(index, Point2 { x: *remember_x, y: *remember_y },ctx,seconds);
                        self.player_state= PlayerState::Active;
                        self.player_hand.hand[index].state= DominoInHandState::Visible(true);
                        self.player_hand.hand[index].rotation = 0.0;
                        continue;
                    }
                    //pinned
                    self.game_board.update_cell(res.1,BoardCell::Domino { point: self.player_hand.hand[*index_of_domino_in_hand].points.0 });
                    
                    //other
                    self.game_board.update_cell(res.2, BoardCell::Domino { point: self.player_hand.hand[*index_of_domino_in_hand].points.1 }); 

                    self.game_board.add_domino_on_table(DominoOnTable::new(
                        (self.player_hand.hand[*index_of_domino_in_hand].points.0,self.player_hand.hand[*index_of_domino_in_hand].points.1),
                        (res.1).0 as u16, 
                        (res.1).1 as u16,
                        self.player_hand.hand[*index_of_domino_in_hand].rotation).unwrap());

                    
                    self.player_hand.hand[*index_of_domino_in_hand].rotation=0.0;
                    self.player_hand.hand[*index_of_domino_in_hand].state=DominoInHandState::Visible(false);
                    self.player_state= PlayerState::Active;
                    
                    self.game_board.update_skeletons(res);
                    if self.game_board.all_skeletons_are_dead() {
                        println!("Pechelishhhh yeah");
                    }
                    //to do the levelwon, gamewon,gameloss etc and toppanel logic
                }
                
            },
        }
        }
        //println!("update");
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) ->  GameResult<()>  {
        let purple = graphics::Color::from_rgb(75, 0, 130);
        let mut canvas = graphics::Canvas::from_frame(ctx, purple);
        //println!("draw");

        self.game_board.draw(&mut canvas, ctx,&self.assets);
        self.player_hand.draw(&mut canvas,ctx, &self.assets);

        canvas.finish(ctx)?;
        Ok(())
    }
    fn key_down_event(
            &mut self,
            ctx: &mut Context,
            input: ggez::input::keyboard::KeyInput,
            _repeated: bool,
        ) -> GameResult<()> {
        match input.keycode {
            Some(keyboard::KeyCode::R) => {
                match self.player_state {
                    PlayerState::Active => (),
                    PlayerState::Dragging { remember_x, remember_y, index_of_domino_in_hand } => {
                        self.player_hand.hand[index_of_domino_in_hand].rotation += 1.0;
                        if self.player_hand.hand[index_of_domino_in_hand].rotation >=4.0 {
                            self.player_hand.hand[index_of_domino_in_hand].rotation = 0.0;
                        }
                    },
                };
            },
            Some(keyboard::KeyCode::Escape) => ctx.request_quit(),
            _ => ()
        };

        Ok(())
    }
}
pub fn all() -> Vec<DominoInHand>
{
  let mut vec= vec![
    DominoInHand::new((0,0), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((0,1), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((0,2), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((0,3), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((0,4), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((0,5), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((0,6), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((1,1), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((1,2), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((1,3), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((1,4), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((1,5), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((1,6), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((2,2), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((2,3), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((2,4), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((2,5), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((2,6), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((3,3), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((3,4), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((3,5), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((3,6), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((4,4), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((4,5), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((4,6), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((5,5), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((5,6), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
    DominoInHand::new((6,6), Point2 { x: 0.0, y: 0.0 } , 0.0,  DominoInHandState::Visible(false)).unwrap(),
  ];
  vec.shuffle(&mut thread_rng());
  vec
}