use ggez::audio;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;
use ggez::mint::Point2;
use dynamic_matrix::{dynamic_matrix, DynamicMatrix};

use crate::entities::enums::DominoInHandState;
pub struct Assets {
    pub empty_cell: graphics::Image,
    pub skeleton: graphics::Image,
    pub grave: graphics::Image,
    pub dominos_images : DynamicMatrix<graphics::Image>,
    pub domino_deck: graphics::Image,
    pub heart_full: graphics::Image,
    pub heart_outlined: graphics::Image,
}
impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let mut empty_cell = graphics::Image::from_path(ctx, "/empty_cell.png").unwrap();
        let mut skeleton = graphics::Image::from_path(ctx, "/skeleton.png").unwrap();
        let mut grave = graphics::Image::from_path(ctx, "/grave.png").unwrap();
        let mut dominos_images :DynamicMatrix<graphics::Image> =  DynamicMatrix::new_with_cols(7);
        let mut domino_deck = graphics::Image::from_path(ctx, "/domino_deck.png").unwrap();
        let mut heart_full = graphics::Image::from_path(ctx, "/heart_full.png").unwrap();
        let mut heart_outlined = graphics::Image::from_path(ctx, "/heart_outlined.png").unwrap();

       
        //println!("{} {}",dominos_images.cols(), dominos_images.rows());
        for i in 0.. 7 {
            let mut vec = Vec::new();
            for j in 0..7 {
                if i<=j {
                let path = format!("/domino_{}_{}.png",i,j);
                let image = graphics::Image::from_path(ctx, path).unwrap();
                vec.push(image);
                }
                else {
                let path = format!("/empty_cell.png");
                let image = graphics::Image::from_path(ctx, path).unwrap();
                vec.push(image);
                }
            }
            //println!("{:?}",vec.len());
            
            dominos_images.push_row(vec).unwrap();
        }
        // println!(
        //     "{} {} {} ",
        //     empty_cell.height(),
        //     skeleton.height(),
        //     grave.height()
        // );
        Ok(Assets {
            empty_cell: empty_cell,
            skeleton: skeleton,
            grave: grave,
            dominos_images : dominos_images,
            domino_deck : domino_deck,
            heart_full: heart_full,
            heart_outlined: heart_outlined,
        })
    }
}
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct DominoOnTable {
    pub points: (u16, u16),
    pub pinned_row: u16,
    pub pinned_col: u16,
    pub rotation: f32,
}
impl DominoOnTable {
    pub fn new(points: (u16,u16),pinned_row : u16,pinned_col: u16,rotation: f32) -> GameResult<DominoOnTable>{
        Ok(DominoOnTable {
            points,pinned_row,pinned_col,rotation
        })
    }
}
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct DominoInHand {
    pub points: (u16, u16),
    pub position: Point2<f32>,
    pub rotation: f32,
    pub state: DominoInHandState,
}
impl DominoInHand {
    pub fn new(points: (u16,u16),position : Point2<f32>,rotation: f32,state : DominoInHandState) -> GameResult<DominoInHand>{
        Ok(DominoInHand {
            points,position,rotation,state
        })
    }
}