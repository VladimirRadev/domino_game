use std::{f32::consts::PI, fmt::format};

use crate::assets::{Assets, DominoOnTable};
use crate::entities::enums::BoardCell;
use ggez::glam::{self, Vec2};
use ggez::{
    graphics::{self, Canvas},
    mint::{Point2, Vector2},
    Context, GameResult,
};
pub struct Board {
    pub board: [[BoardCell; 8]; 8],
    pub starting_point: Point2<f32>,
    pub dominos: Vec<DominoOnTable>,
}
impl Board {
    pub fn new(starting_point: Point2<f32>) -> GameResult<Board> {
        let mut board = [[BoardCell::None; 8]; 8];
        board[0][1] = BoardCell::Skeleton {
            health: 9,
            row: 0,
            col: 0,
        };
        board[3][7] = BoardCell::Skeleton {
            health: 2,
            row: 0,
            col: 0,
        };
        board[5][4] = BoardCell::Grave;

        Ok(Board {
            board: board,
            starting_point: starting_point,
            dominos: Vec::new(),
        })
    }

    pub fn draw(
        &mut self,
        canvas: &mut graphics::Canvas,
        ctx: &mut ggez::Context,
        assets: &Assets,
    ) {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                let x = self.starting_point.x + (j as f32) * 80.0;
                let y = self.starting_point.y + (i as f32) * 80.0;
                let draw_params = graphics::DrawParam::default()
                    .dest(glam::Vec2::new(x,y))
                    .scale(Vector2 { x: 0.90, y: 0.90 })
                    .offset(glam::Vec2 {
                        x: x / 1000.0,
                        y: y / 1000.0,
                    });
                match self.board[i][j] {
                    BoardCell::Skeleton { health, row, col } => {
                        canvas.draw(&assets.skeleton, draw_params);

                        let x_health = x + 10.0;
                        let y_health = y - 10.0;
                        let draw_params_skelet = graphics::DrawParam::default()
                            .dest(glam::Vec2::new(x_health,y_health))
                            .scale(Vector2 { x: 0.97, y: 0.70 })
                            .offset(glam::Vec2 {
                                x: x / 1000.0,
                                y: y / 1000.0,
                            });

                        let path = format!("/health_{}.png", health);
                        let image = graphics::Image::from_path(ctx, path).unwrap();
                        canvas.draw(&image, draw_params_skelet);
                    }
                    BoardCell::Grave => {
                        canvas.draw(&assets.grave, draw_params);
                    }
                    BoardCell::None => {
                        canvas.draw(&assets.empty_cell, draw_params);
                    }
                }
            }
        }

        self.draw_dominos_on_table(canvas, ctx);
    }
    fn draw_dominos_on_table(&mut self, canvas: &mut graphics::Canvas, ctx: &mut ggez::Context) {
        for i in &self.dominos {
            match i.rotation % 4.0 {
                0.0 => {
                    let x = self.starting_point.x + (i.pinned_col as f32) * 80.0;
                    let y = self.starting_point.y + (i.pinned_row as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.90, y: 0.90 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        })
                        .rotation((0.0 as f32) * PI / 2.0);
                    let path = format!("/domino_{}.png", i.points.0);
                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);


                    let x = self.starting_point.x + (i.pinned_col as f32) * 80.0;
                    let y = self.starting_point.y + ((i.pinned_row + 1) as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.90, y: 0.90 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        })
                        .rotation((0.0 as f32) * PI / 2.0);
                    let path = format!("/domino_{}.png", i.points.1);
                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);
                    continue;
                }
                1.0 => {
                    
                    let x = self.starting_point.x + (i.pinned_col as f32) * 80.0;
                    let y = self.starting_point.y + (i.pinned_row as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.90, y: 0.90 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        })
                        .rotation((0.0 as f32) * PI / 2.0);
                    let path = format!("/domino_{}.png", i.points.0);
                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);


                    let x = self.starting_point.x + ((i.pinned_col - 1) as f32) * 80.0;
                    let y = self.starting_point.y + (i.pinned_row as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.90, y: 0.90 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        })
                        .rotation((0.0 as f32) * PI / 2.0);
                    let path = format!("/domino_{}.png", i.points.1);
                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);
                    continue;
                }
                2.0 => {
                    let x = self.starting_point.x + (i.pinned_col as f32) * 80.0;
                    let y = self.starting_point.y + (i.pinned_row as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.90, y: 0.90 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        })
                        .rotation((0.0 as f32) * PI / 2.0);
                    let path = format!("/domino_{}.png", i.points.0);
                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);


                    let x = self.starting_point.x + (i.pinned_col as f32) * 80.0;
                    let y = self.starting_point.y + ((i.pinned_row-1) as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.90, y: 0.90 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        })
                        .rotation((0.0 as f32) * PI / 2.0);
                    let path = format!("/domino_{}.png", i.points.1);
                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);
                    continue;
                }
                3.0 => {
                    let x = self.starting_point.x + (i.pinned_col as f32) * 80.0;
                    let y = self.starting_point.y + (i.pinned_row as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.90, y: 0.90 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        })
                        .rotation((0.0 as f32) * PI / 2.0);
                    let path = format!("/domino_{}.png", i.points.0);
                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);


                    let x = self.starting_point.x + ((i.pinned_col + 1) as f32) * 80.0;
                    let y = self.starting_point.y + (i.pinned_row as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.90, y: 0.90 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        })
                        .rotation((0.0 as f32) * PI / 2.0);
                    let path = format!("/domino_{}.png", i.points.1);
                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);
                    continue;
                }
                _ => (),
            };
        }
    }
}


/* fn draw_dominos_on_table(&mut self, canvas: &mut graphics::Canvas, ctx: &mut ggez::Context) {
        for i in &self.dominos {
            match i.rotation % 4.0 {
                0.0 => {
                    let x = self.starting_point.x + (i.pinned_col as f32) * 80.0;
                    let y = self.starting_point.y + (i.pinned_row as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.90, y: 0.90 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0 - 0.15,
                        })
                        .rotation((0.0 as f32) * PI / 2.0);
                    let path = format!("/domino_{}_{}.png", i.points.0, i.points.1);
                    let mut image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);
                    continue;
                }
                1.0 => {
                    
                    let x = self.starting_point.x + (i.pinned_col as f32) * 80.0;
                    let y = self.starting_point.y + (i.pinned_row as f32) * 80.0 ;
                    //println!("{} {}", x, y);
                    let draw_params = graphics::DrawParam::default().
                        scale(Vector2 { x: 0.625, y: 1.25 }).
                        dest(glam::Vec2::new(x,y)).
                        rotation((1.0 as f32) * PI / 2.0).
                        dest(glam::Vec2::new(x,y - 80.0)).
                        offset(glam::Vec2::new((x-80.0)/1000.0,y/1000.0));
                        
                    let path = format!("/domino_{}_{}.png", i.points.0, i.points.1);
                    let mut image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);
                }
                2.0 => {
                    let x = self.starting_point.x + (i.pinned_col as f32) * 80.0;
                    let y = self.starting_point.y + (i.pinned_row as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.90, y: 0.90 })
                        .offset(Point2 {
                            x: x / 1000.0 + 0.12,
                            y: y / 1000.0,
                        })
                        .rotation((2.0 as f32) * PI / 2.0);
                    let path = format!("/domino_{}_{}.png", i.points.0, i.points.1);
                    let mut image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);
                }
                3.0 => {
                    let x = self.starting_point.x + (i.pinned_col as f32) * 80.0;
                    let y = self.starting_point.y + (i.pinned_row as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.90, y: 0.90 })
                        .offset(Point2 {
                            x: x / 1000.0 + 0.15,
                            y: y / 1000.0 - 0.10,
                        })
                        .rotation((3.0 as f32) * PI / 2.0);
                        let path = format!("/domino_{}_{}.png", i.points.0, i.points.1);
                        let mut image = graphics::Image::from_path(ctx, path).unwrap();
                        canvas.draw(&image, draw_params);
                }
                _ => (),
            };
        }
    }*/