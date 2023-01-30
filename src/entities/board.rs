use ggez::{Context, mint::{Point2, Vector2}, GameResult, graphics::{self, Canvas}};
use crate::assets::Assets;
use crate::entities::enums::BoardCell;
pub struct Board {
    board : [[BoardCell; 8]; 8],
    starting_point: Point2<f32>,
}
impl Board {
    pub fn new(starting_point: Point2<f32>) -> GameResult<Board> {
        let board = [[BoardCell::None; 8]; 8];
        Ok(Board {
            board: board,
            starting_point: starting_point,
        })
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas, assets: &Assets) {
        for i in 0..self.board.len()  {
            for j in 0.. self.board[i].len() {
                let  x = self.starting_point.x + (j  as f32)*80.0;
                let  y = self.starting_point.y + (i  as f32)*80.0;
                let draw_params = graphics::DrawParam::default().
                    dest(Point2{x,y}).
                    scale(Vector2 { x: 1.0 , y : 1.0}).
                    offset(Point2 {x: x/1000.0, y: y/1000.0});
                match  &self.board[i][j] {
                    BoardCell::Domino { point } => todo!(),
                    BoardCell::Skeleton { health } => todo!(),
                    BoardCell::Grave => {
                        canvas.draw(&assets.grave,draw_params);
                    },
                    BoardCell::None => {
                        canvas.draw(&assets.empty_cell, draw_params);
                    },
                }
            }
        }

        
    }
}
