use ggez::{GameResult, graphics::{self, MeshBuilder, DrawMode, Rect, Mesh, DrawParam, FillOptions}, mint::{Point2, Vector2}, glam};

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
          Rect { x: 0.0, y: 744.0, w: 1000.0, h: 256.0 }, 
          graphics::Color { r: 1.0, g: 1.0, b: 0.6, a: 1.0 }).unwrap(), 
          DrawParam::default());

        for i in self.hand.iter() {
            if !i.visible {
                continue;
            }
            //println!("{} {}",assets.dominos_images.cols(), assets.dominos_images.rows());
            let draw_params = graphics::DrawParam::default()
            .dest(Point2 { x: i.position.x, y: i.position.y })
            .scale(Vector2 { x: 0.90, y: 0.90 });
            canvas.draw(&*assets.dominos_images.get((i.points.0 as usize,i.points.1 as usize)).unwrap(),draw_params);
        }
    }
}