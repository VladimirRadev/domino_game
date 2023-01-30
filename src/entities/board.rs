use std::fmt::format;

use ggez::{Context, mint::{Point2, Vector2}, GameResult, graphics::{self, Canvas}};
use crate::assets::Assets;
use crate::entities::enums::BoardCell;
pub struct Board {
    pub board : [[BoardCell; 8]; 8],
    pub starting_point: Point2<f32>,
}
impl Board {
    pub fn new(starting_point: Point2<f32>) -> GameResult<Board> {
        let mut board = [[BoardCell::None; 8]; 8];
        board [0][1] =BoardCell::Skeleton { health: 9, row: 0, col: 0 };
        board [3][7] =BoardCell::Skeleton { health: 2, row: 0, col: 0 };
        board [5][4] = BoardCell::Grave;


        Ok(Board {
            board: board,
            starting_point: starting_point,
        })
    }

    pub fn draw(&mut self, canvas: &mut graphics::Canvas,ctx: &mut ggez::Context, assets: &Assets) {
        for i in 0..self.board.len()  {
            for j in 0.. self.board[i].len() {
                let  x = self.starting_point.x + (j  as f32)*80.0;
                let  y = self.starting_point.y + (i  as f32)*80.0;
                let draw_params = graphics::DrawParam::default().
                    dest(Point2{x,y}).
                    scale(Vector2 { x: 0.90 , y : 0.90}).
                    offset(Point2 {x: x/1000.0, y: y/1000.0});
                match self.board[i][j] {
                    BoardCell::Skeleton { health, row, col } => {
                        canvas.draw(&assets.skeleton, draw_params);

                        let  x_skelet = x + 10.0;
                        let  y_skelet = y - 10.0;
                        let draw_params_skelet = graphics::DrawParam::default().
                        dest(Point2{x: x_skelet, y: y_skelet}).
                        scale(Vector2 { x: 0.97 , y : 0.70}).
                        offset(Point2 {x: x_skelet/1000.0, y: y_skelet/1000.0});

                      
                        let path = format!("/health_{}.png",health);
                        let image= graphics::Image::from_path(ctx, path).unwrap();
                        canvas.draw(&image, draw_params_skelet);
                    }
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
