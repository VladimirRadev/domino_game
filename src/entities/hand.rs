use std::f32::consts::PI;

use ggez::{GameResult, graphics::{self, MeshBuilder, DrawMode, Rect, Mesh, DrawParam, FillOptions}, mint::{Point2, Vector2}, glam};

use crate::assets::{Assets, DominoInHand};

use super::enums::DominoInHandState;

pub struct Hand{
    pub hand : Vec<DominoInHand>,
}
impl Hand {
    pub fn new(vec:Vec<DominoInHand>) -> GameResult<Hand> {
        Ok(Hand {
            hand: vec.clone(),
        })
    }
    pub fn update_domino_position(&mut self, index: usize, new_position: Point2<f32> , ctx:&mut ggez::Context, seconds: f32){
        let x = new_position.x ;
        let y= new_position.y ;
       // println!("{} {}",x,y);
        self.hand[index].position=Point2{x,y};
    }
    pub fn update_domino_rotation(&mut self, index: usize,new_rotation: f32 ) {
        self.hand[index].rotation=new_rotation;
    }
    pub fn draw(&self,canvas : &mut graphics::Canvas,ctx: &mut ggez::Context, assets: &Assets){
        canvas.draw(&Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), 
          Rect { x: 0.0, y: 744.0, w: 1000.0, h: 256.0 }, 
          graphics::Color { r: 1.0, g: 1.0, b: 0.6, a: 1.0 }).unwrap(), 
          DrawParam::default());

        for i in self.hand.iter() {
            if let DominoInHandState::Visible(false) = i.state {
                continue;
            }
            //println!("{} {}",assets.dominos_images.cols(), assets.dominos_images.rows());
            if let DominoInHandState::Visible(true) = i.state {
                let draw_params = graphics::DrawParam::default()
                .dest(Vector2 { x: i.position.x, y: i.position.y })
                .scale(Vector2 { x: 1.00, y: 1.00 });
                //.offset(Vector2{ x:i.position.x/1000.0 , y: i.position.y/1000.0});
                canvas.draw(&*assets.dominos_images.get((i.points.0 as usize,i.points.1 as usize)).unwrap(),draw_params);
            }
            if let DominoInHandState::Moving = i.state {
                let draw_params = graphics::DrawParam::default()
                .dest(Vector2 { x: i.position.x, y: i.position.y })
                .scale(Vector2 { x: 1.00, y: 1.00 })
                .offset(Point2{ x:i.position.x/1000.0 , y: i.position.y/1000.0})
                .rotation(i.rotation * PI/2.0);
                canvas.draw(&*assets.dominos_images.get((i.points.0 as usize,i.points.1 as usize)).unwrap(),draw_params);
            }
        }
    }

    pub fn check_boundary_of_click(&self,mouse : Point2<f32> ) -> (bool, usize){
        let mut index : usize = 0;
        for i in self.hand.iter() {
            if let DominoInHandState::Visible(false) = i.state{
                index+=1;
                continue;
            }
           
            if ((mouse.x >= i.position.x && mouse.x <= i.position.x + 128.0) && (mouse.y >= i.position.y && mouse.y <= i.position.y + 128.0)) {
               // println!("{:?} {}",mouse , index);
                return (true,index);
            }
            index+=1;

        }
        //every other case index = 10 , they are from [0:9]
        (false, index)
    }

    pub fn replace_domino(&mut self, new_domino: DominoInHand) {
        let mut index = 0;
        for i in &self.hand {
            if let DominoInHandState::Visible(false) = i.state {
                break;
            }
            index+=1;
        }
        //println!("{:?} {}",self.hand[index] , index);
        self.hand[index].points = new_domino.points;
        self.hand[index].rotation=0.0;
        self.hand[index].state= DominoInHandState::Visible(true);
    }

    pub fn empty(&self) -> bool {
        for i in &self.hand{
            if let DominoInHandState::Visible(true)=i.state{
                return false;
            }
        }
        true
    }
}