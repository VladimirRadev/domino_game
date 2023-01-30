use ggez::{GameResult, graphics::{self, MeshBuilder, DrawMode, Rect, Mesh, DrawParam, FillOptions}, mint::{Point2, Vector2}};

use crate::assets::{Assets, DominoInHand};

pub struct Hand{
    pub hand : Vec<DominoInHand>,
}
impl Hand {
    pub fn new(vec:Vec<DominoInHand>) -> GameResult<Hand> {
        Ok(Hand {
            hand: vec.clone(),
        })
    }

    pub fn draw(&self,canvas : &mut graphics::Canvas,ctx: &mut ggez::Context, assets: &Assets){
        canvas.draw(&Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), 
          Rect { x: 0.0, y: 760.0, w: 1000.0, h: 240.0 }, 
          graphics::Color { r: 1.0, g: 1.0, b: 0.6, a: 1.0 }).unwrap(), 
          DrawParam::default());
    }
}