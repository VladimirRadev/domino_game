use std::{f32::consts::PI, fmt::format};

use crate::assets::{Assets, DominoOnTable};
use crate::entities::enums::BoardCell;
use ggez::glam::{self, Vec2};
use ggez::{
    graphics::{self, Canvas},
    mint::{Point2, Vector2},
    Context, GameResult,
};
use rand::Rng;

use crate::entities::hand::Hand;
use crate::entities::game::Game;
pub struct Board {
    pub board: [[BoardCell; 8]; 8],
    pub starting_point: Point2<f32>,
    pub dominos: Vec<DominoOnTable>,
}
impl Board {
    pub fn new(starting_point: Point2<f32>, starting_domino: DominoOnTable, game: &Game) -> GameResult<Board> {
        let mut board = [[BoardCell::None; 8]; 8];
        board[3][3] = BoardCell::Domino { point: starting_domino.points.0 };
        board[3][4] = BoardCell::Domino { point: starting_domino.points.1 };
        for i in &game.skeletons {
            let mut rng = rand::thread_rng();
            loop {
                let x = rng.gen_range(0..=7);
                let y = rng.gen_range(0..=7);
                match &board[x][y] {
                    BoardCell::None => {
                    board[x][y]=BoardCell::Skeleton { health: *i as i16, row: x as u16, col: y as u16};
                    break;  
                    },
                    _ => continue
                };
            }
        }
        for i in 0..game.graves_count {
            let mut rng = rand::thread_rng();
            loop {
                let x = rng.gen_range(0..=7);
                let y = rng.gen_range(0..=7);
                match &board[x][y] {
                    BoardCell::None => {
                    board[x][y]=BoardCell::Grave;
                    break;  
                    },
                    _ => continue
                };
            }
        }
        Ok(Board {
            board: board,
            starting_point: starting_point,
            dominos: vec![starting_domino]
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
                    BoardCell::Domino { point } => (),
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
                        .scale(Vector2 { x: 0.87, y: 0.87 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        });
                    let path = format!("/domino_{}_rotation_{}.png", i.points.0,i.rotation as u16);

                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);


                    let x = self.starting_point.x + (i.pinned_col as f32) * 80.0;
                    let y = self.starting_point.y + ((i.pinned_row + 1) as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.87, y: 0.87 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        });
                    let path = format!("/domino_{}_rotation_{}.png", i.points.1,(i.rotation - 2.0).abs() as u16);
                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);
                    continue;
                }
                1.0 => {
                    
                    let x = self.starting_point.x + (i.pinned_col as f32) * 80.0;
                    let y = self.starting_point.y + (i.pinned_row as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.87, y: 0.87 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        });
                       
                    let path = format!("/domino_{}_rotation_{}.png", i.points.0,i.rotation as u16);

                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);


                    let x = self.starting_point.x + ((i.pinned_col - 1) as f32) * 80.0;
                    let y = self.starting_point.y + (i.pinned_row as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.87, y: 0.87 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        });
                        
                        let path = format!("/domino_{}_rotation_{}.png", i.points.1,(i.rotation + 2.0).abs() as u16);
                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);
                    continue;
                }
                2.0 => {
                    let x = self.starting_point.x + (i.pinned_col as f32) * 80.0;
                    let y = self.starting_point.y + (i.pinned_row as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.87, y: 0.87 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        });
                        let path = format!("/domino_{}_rotation_{}.png", i.points.0,i.rotation as u16);

                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);


                    let x = self.starting_point.x + (i.pinned_col as f32) * 80.0;
                    let y = self.starting_point.y + ((i.pinned_row-1) as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.87, y: 0.87 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        });
    
                    let path = format!("/domino_{}_rotation_{}.png", i.points.1,(i.rotation - 2.0).abs() as u16);
                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);
                    continue;
                }
                3.0 => {
                    let x = self.starting_point.x + (i.pinned_col as f32) * 80.0;
                    let y = self.starting_point.y + (i.pinned_row as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.87, y: 0.87 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        });
                    let path = format!("/domino_{}_rotation_{}.png", i.points.0,i.rotation as u16);
                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);


                    let x = self.starting_point.x + ((i.pinned_col + 1) as f32) * 80.0;
                    let y = self.starting_point.y + (i.pinned_row as f32) * 80.0;
                    let draw_params = graphics::DrawParam::default()
                        .dest(Point2 { x, y })
                        .scale(Vector2 { x: 0.87, y: 0.87 })
                        .offset(Point2 {
                            x: x / 1000.0,
                            y: y / 1000.0,
                        });
                    let path = format!("/domino_{}_rotation_{}.png", i.points.1,(i.rotation - 2.0).abs() as u16);
                    let image = graphics::Image::from_path(ctx, path).unwrap();
                    canvas.draw(&image, draw_params);
                    continue;
                }
                _ => (),
            };
        }
    }
   
    /// returns if it's  posible , pinned : (row,col) , other : (row, col) , 
    /// health_left of a skelet on pinned and other (i16 , i16) ( if skelet isn't on position .0 or .1  => value = 10  )
    pub fn check_boundary_of_release(&self, mouse_position: Point2<f32>, hand : &mut Hand , index_of_domino_in_hand: &usize) -> (bool, (usize,usize) , (usize , usize) , (i16,i16)) {
        
        // .0 = x ; .1 = y
        let mut possible_index_of_pinned = (-1,-1);
        let starting_x = 165.0;
        let starting_y = 93.0;
        for i in 0..self.board.len() {
            for j in 0 .. self.board[i].len() {
                let x = starting_x+ (j as f32) * 72.0;
                let y = starting_y + (i as f32) * 72.0;
               
                if (mouse_position.x >= x && mouse_position.x <= x + 72.0) && (mouse_position.y >= y && mouse_position.y <= y + 72.0){
                    possible_index_of_pinned = (i as i32 ,j as i32);
                    break;
                }
            }
        }
        
        if !((possible_index_of_pinned.0 >= 0 && possible_index_of_pinned.0 <= 7) && (possible_index_of_pinned.1 >= 0 && possible_index_of_pinned.1 <= 7)){
            return  (false,(0,0),(0,0),(10,10));
        }

        let k =  hand.hand[*index_of_domino_in_hand].rotation;
        if k == 0.0 {

            //edge case
           if possible_index_of_pinned.0 + 1 > 7 {
                return  (false,(0,0),(0,0),(10,10));
           }  
           let possible_index_of_pinned: Point2<usize> = Point2 { x: possible_index_of_pinned.0 as usize, y: possible_index_of_pinned.1 as usize };
           let possible_index_of_other: Point2<usize> = Point2 { x: possible_index_of_pinned.x + 1 as usize, y: possible_index_of_pinned.y as usize };
           
           //graves & dominos
           match (self.board[possible_index_of_pinned.x][possible_index_of_pinned.y], self.board[possible_index_of_other.x][possible_index_of_other.y] ) {
            (BoardCell::Grave , _) | (_ , BoardCell::Grave) => {
                 return (false,(0,0),(0,0),(10,10));
            },
            (BoardCell::Domino { point } , _ ) | (_ , BoardCell::Domino { point }) => {
                return (false,(0,0),(0,0),(10,10));
            }
            _ => ()
            };
          

            //skeletons  (pinned,other)
            let mut skeletonHealth : (i16,i16) = (10,10);
            match  self.board[possible_index_of_pinned.x][possible_index_of_pinned.y] {
                BoardCell::Skeleton { health, row, col } => {
                    skeletonHealth.0 = health - (hand.hand[*index_of_domino_in_hand].points.0 as i16);
                }
                _ => (),
            };
            match  self.board[possible_index_of_other.x][possible_index_of_other.y] {
                BoardCell::Skeleton { health, row, col } => {
                    skeletonHealth.1 = health - (hand.hand[*index_of_domino_in_hand].points.1 as i16);
                }
                _ => (),
            };

            
            let mut matches: Vec<bool> = Vec::new(); 
            let pinned_point = hand.hand[*index_of_domino_in_hand].points.0;
            let other_point = hand.hand[*index_of_domino_in_hand].points.1;
            
            //pinned
            if Board::check_indexes((possible_index_of_pinned.x ) as i32 , (possible_index_of_pinned.y as i32- 1) as i32)
            {
                
                match self.board[possible_index_of_pinned.x][possible_index_of_pinned.y - 1] {
                    BoardCell::Domino { point } => {
                        if ( point == pinned_point) {
                            matches.push(true);
                        }
                        else {
                            matches.push(false);
                        }
                    },
                    _ => (),
                }
            }
            if Board::check_indexes((possible_index_of_pinned.x as i32  - 1) as i32 , (possible_index_of_pinned.y) as i32)
            {
                match self.board[possible_index_of_pinned.x - 1 ][possible_index_of_pinned.y] {
                    BoardCell::Domino { point } => {
                        if ( point == pinned_point) {
                            matches.push(true);
                        }
                        else {
                            matches.push(false);
                        }
                    },
                    _ => (),
                }
            }
            if Board::check_indexes((possible_index_of_pinned.x ) as i32 , (possible_index_of_pinned.y + 1) as i32)
            {
                match self.board[possible_index_of_pinned.x][possible_index_of_pinned.y + 1] {
                    BoardCell::Domino { point } => {
                        if ( point == pinned_point) {
                            matches.push(true);
                        }
                        else {
                            matches.push(false);
                        }
                    },
                    _ => (),
                }
            }

            //other

            if Board::check_indexes((possible_index_of_other.x ) as i32 , (possible_index_of_other.y + 1) as i32)
            {
                match self.board[possible_index_of_other.x][possible_index_of_other.y + 1] {
                    BoardCell::Domino { point } => {
                        if ( point == other_point) {
                            matches.push(true);
                        }
                        else {
                            matches.push(false);
                        }
                    },
                    _ => (),
                }
            }

            if Board::check_indexes((possible_index_of_other.x + 1 ) as i32 , (possible_index_of_other.y) as i32)
            {
                match self.board[possible_index_of_other.x + 1][possible_index_of_other.y] {
                    BoardCell::Domino { point } => {
                        if ( point == other_point) {
                            matches.push(true);
                        }
                        else {
                            matches.push(false);
                        }
                    },
                    _ => (),
                }
            }

            if Board::check_indexes((possible_index_of_other.x ) as i32 , (possible_index_of_other.y as i32 - 1) as i32)
            {
                match self.board[possible_index_of_other.x][possible_index_of_other.y - 1] {
                    BoardCell::Domino { point } => {
                        if ( point == other_point) {
                            matches.push(true);
                        }
                        else {
                            matches.push(false);
                        }
                    },
                    _ => (),
                }
            }
           
            if matches.contains(&true) {
                //(bool, (usize,usize) , (usize , usize) , (i16,i16)) 
                
                return (true,(possible_index_of_pinned.x,possible_index_of_pinned.y),
                (possible_index_of_other.x,possible_index_of_other.y),skeletonHealth);
            }
            else {
                return (false,(0,0),(0,0),(10,10))
            }
            




            
        }

        if k == 1.0 {
              //edge case
           if possible_index_of_pinned.1 - 1 < 0 {
            return  (false,(0,0),(0,0),(10,10));
       }  
       let possible_index_of_pinned: Point2<usize> = Point2 { x: possible_index_of_pinned.0 as usize, y: possible_index_of_pinned.1 as usize };
       let possible_index_of_other: Point2<usize> = Point2 { x: possible_index_of_pinned.x as usize, y: possible_index_of_pinned.y - 1 as usize };
       
       //graves & dominos
       match (self.board[possible_index_of_pinned.x][possible_index_of_pinned.y], self.board[possible_index_of_other.x][possible_index_of_other.y] ) {
        (BoardCell::Grave , _) | (_ , BoardCell::Grave) => {
             return (false,(0,0),(0,0),(10,10));
        },
        (BoardCell::Domino { point } , _ ) | (_ , BoardCell::Domino { point }) => {
            return (false,(0,0),(0,0),(10,10));
        }
        _ => ()
        };

        //skeletons  (pinned,other)
        let mut skeletonHealth : (i16,i16) = (10,10);
        match  self.board[possible_index_of_pinned.x][possible_index_of_pinned.y] {
            BoardCell::Skeleton { health, row, col } => {
                skeletonHealth.0 = health - hand.hand[*index_of_domino_in_hand].points.0 as i16;
            }
            _ => (),
        };
        match  self.board[possible_index_of_other.x][possible_index_of_other.y] {
            BoardCell::Skeleton { health, row, col } => {
                skeletonHealth.1 = health - hand.hand[*index_of_domino_in_hand].points.1 as i16;
            }
            _ => (),
        };

        let mut matches: Vec<bool> = Vec::new(); 
        let pinned_point = hand.hand[*index_of_domino_in_hand].points.0;
        let other_point = hand.hand[*index_of_domino_in_hand].points.1;
        
        //pinned
        if Board::check_indexes((possible_index_of_pinned.x as i32 - 1 ) as i32 , (possible_index_of_pinned.y) as i32)
        {
            match self.board[possible_index_of_pinned.x - 1][possible_index_of_pinned.y] {
                BoardCell::Domino { point } => {
                    if ( point == pinned_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }
        if Board::check_indexes((possible_index_of_pinned.x) as i32 , (possible_index_of_pinned.y + 1) as i32)
        {
            match self.board[possible_index_of_pinned.x][possible_index_of_pinned.y + 1] {
                BoardCell::Domino { point } => {
                    if ( point == pinned_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }
        if Board::check_indexes((possible_index_of_pinned.x + 1 ) as i32 , (possible_index_of_pinned.y) as i32)
        {
            match self.board[possible_index_of_pinned.x + 1][possible_index_of_pinned.y] {
                BoardCell::Domino { point } => {
                    if ( point == pinned_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }

        //other

        if Board::check_indexes((possible_index_of_other.x + 1 ) as i32 , (possible_index_of_other.y ) as i32)
        {
            match self.board[possible_index_of_other.x+ 1][possible_index_of_other.y] {
                BoardCell::Domino { point } => {
                    if ( point == other_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }

        if Board::check_indexes((possible_index_of_other.x  ) as i32 , (possible_index_of_other.y as i32 - 1) as i32)
        {
            match self.board[possible_index_of_other.x][possible_index_of_other.y - 1] {
                BoardCell::Domino { point } => {
                    if ( point == other_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }

        if Board::check_indexes((possible_index_of_other.x as i32 - 1 ) as i32 , (possible_index_of_other.y) as i32)
        {
            match self.board[possible_index_of_other.x - 1][possible_index_of_other.y] {
                BoardCell::Domino { point } => {
                    if ( point == other_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }

        if matches.contains(&true) {
            //(bool, (usize,usize) , (usize , usize) , (i16,i16)) 
            return (true,(possible_index_of_pinned.x,possible_index_of_pinned.y),
            (possible_index_of_other.x,possible_index_of_other.y),skeletonHealth);
        }
        else {
            return (false,(0,0),(0,0),(10,10))
        }
   
        }
        if k==2.0 {
                 //edge case
           if possible_index_of_pinned.0 - 1 < 0 {
            return  (false,(0,0),(0,0),(10,10));
       }  
       let possible_index_of_pinned: Point2<usize> = Point2 { x: possible_index_of_pinned.0 as usize, y: possible_index_of_pinned.1 as usize };
       let possible_index_of_other: Point2<usize> = Point2 { x: possible_index_of_pinned.x - 1 as usize, y: possible_index_of_pinned.y as usize };
       
       //graves & dominos
       match (self.board[possible_index_of_pinned.x][possible_index_of_pinned.y], self.board[possible_index_of_other.x][possible_index_of_other.y] ) {
        (BoardCell::Grave , _) | (_ , BoardCell::Grave) => {
             return (false,(0,0),(0,0),(10,10));
        },
        (BoardCell::Domino { point } , _ ) | (_ , BoardCell::Domino { point }) => {
            return (false,(0,0),(0,0),(10,10));
        }
        _ => ()
        };

        //skeletons  (pinned,other)
        let mut skeletonHealth : (i16,i16) = (10,10);
        match  self.board[possible_index_of_pinned.x][possible_index_of_pinned.y] {
            BoardCell::Skeleton { health, row, col } => {
                skeletonHealth.0 = health - hand.hand[*index_of_domino_in_hand].points.0 as i16;
            }
            _ => (),
        };
        match  self.board[possible_index_of_other.x][possible_index_of_other.y] {
            BoardCell::Skeleton { health, row, col } => {
                skeletonHealth.1 = health - hand.hand[*index_of_domino_in_hand].points.1 as i16;
            }
            _ => (),
        };

        let mut matches: Vec<bool> = Vec::new(); 
        let pinned_point = hand.hand[*index_of_domino_in_hand].points.0;
        let other_point = hand.hand[*index_of_domino_in_hand].points.1;
        
        //pinned
        if Board::check_indexes((possible_index_of_pinned.x ) as i32 , (possible_index_of_pinned.y  as i32 - 1) as i32)
        {
            match self.board[possible_index_of_pinned.x ][possible_index_of_pinned.y - 1] {
                BoardCell::Domino { point } => {
                    if ( point == pinned_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }
        if Board::check_indexes((possible_index_of_pinned.x + 1) as i32 , (possible_index_of_pinned.y) as i32)
        {
            match self.board[possible_index_of_pinned.x + 1][possible_index_of_pinned.y ] {
                BoardCell::Domino { point } => {
                    if ( point == pinned_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }
        if Board::check_indexes((possible_index_of_pinned.x ) as i32 , (possible_index_of_pinned.y + 1) as i32)
        {
            match self.board[possible_index_of_pinned.x][possible_index_of_pinned.y + 1] {
                BoardCell::Domino { point } => {
                    if ( point == pinned_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }

        //other

        if Board::check_indexes((possible_index_of_other.x ) as i32 , (possible_index_of_other.y + 1) as i32)
        {
            match self.board[possible_index_of_other.x][possible_index_of_other.y + 1] {
                BoardCell::Domino { point } => {
                    if ( point == other_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }

        if Board::check_indexes((possible_index_of_other.x as i32 -1 ) as i32 , (possible_index_of_other.y) as i32)
        {
            match self.board[possible_index_of_other.x - 1][possible_index_of_other.y] {
                BoardCell::Domino { point } => {
                    if ( point == other_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }

        if Board::check_indexes((possible_index_of_other.x ) as i32 , (possible_index_of_other.y as i32 - 1) as i32)
        {
            match self.board[possible_index_of_other.x][possible_index_of_other.y - 1] {
                BoardCell::Domino { point } => {
                    if ( point == other_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }

        if matches.contains(&true) {
            //(bool, (usize,usize) , (usize , usize) , (i16,i16)) 
            return (true,(possible_index_of_pinned.x,possible_index_of_pinned.y),
            (possible_index_of_other.x,possible_index_of_other.y),skeletonHealth);
        }
        else {
            return (false,(0,0),(0,0),(10,10))
        }
   
        }
        if k==3.0 {
                      //edge case
           if possible_index_of_pinned.1 + 1 > 7 {
            return  (false,(0,0),(0,0),(10,10));
       }  
       let possible_index_of_pinned: Point2<usize> = Point2 { x: possible_index_of_pinned.0 as usize, y: possible_index_of_pinned.1 as usize };
       let possible_index_of_other: Point2<usize> = Point2 { x: possible_index_of_pinned.x as usize, y: possible_index_of_pinned.y + 1 as usize };
       
       //graves & dominos
       match (self.board[possible_index_of_pinned.x][possible_index_of_pinned.y], self.board[possible_index_of_other.x][possible_index_of_other.y] ) {
        (BoardCell::Grave , _) | (_ , BoardCell::Grave) => {
             return (false,(0,0),(0,0),(10,10));
        },
        (BoardCell::Domino { point } , _ ) | (_ , BoardCell::Domino { point }) => {
            return (false,(0,0),(0,0),(10,10));
        }
        _ => ()
        };

        //skeletons  (pinned,other)
        let mut skeletonHealth : (i16,i16) = (10,10);
        match  self.board[possible_index_of_pinned.x][possible_index_of_pinned.y] {
            BoardCell::Skeleton { health, row, col } => {
                skeletonHealth.0 = health - hand.hand[*index_of_domino_in_hand].points.0 as i16;
            }
            _ => (),
        };
        match  self.board[possible_index_of_other.x][possible_index_of_other.y] {
            BoardCell::Skeleton { health, row, col } => {
                skeletonHealth.1 = health - hand.hand[*index_of_domino_in_hand].points.1 as i16;
            }
            _ => (),
        };

        let mut matches: Vec<bool> = Vec::new(); 
        let pinned_point = hand.hand[*index_of_domino_in_hand].points.0;
        let other_point = hand.hand[*index_of_domino_in_hand].points.1;
        
        //pinned
        if Board::check_indexes((possible_index_of_pinned.x + 1) as i32 , (possible_index_of_pinned.y) as i32)
        {
            match self.board[possible_index_of_pinned.x + 1][possible_index_of_pinned.y ] {
                BoardCell::Domino { point } => {
                    if ( point == pinned_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }
        if Board::check_indexes((possible_index_of_pinned.x ) as i32 , (possible_index_of_pinned.y as i32 - 1) as i32)
        {
            match self.board[possible_index_of_pinned.x ][possible_index_of_pinned.y -1] {
                BoardCell::Domino { point } => {
                    if ( point == pinned_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }
        if Board::check_indexes((possible_index_of_pinned.x as i32 - 1 ) as i32 , (possible_index_of_pinned.y) as i32)
        {
            match self.board[possible_index_of_pinned.x - 1][possible_index_of_pinned.y ] {
                BoardCell::Domino { point } => {
                    if ( point == pinned_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }

        //other

        if Board::check_indexes((possible_index_of_other.x as i32 - 1 ) as i32 , (possible_index_of_other.y ) as i32)
        {
            match self.board[possible_index_of_other.x - 1][possible_index_of_other.y ] {
                BoardCell::Domino { point } => {
                    if ( point == other_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }

        if Board::check_indexes((possible_index_of_other.x) as i32 , (possible_index_of_other.y + 1) as i32)
        {
            match self.board[possible_index_of_other.x ][possible_index_of_other.y + 1] {
                BoardCell::Domino { point } => {
                    if ( point == other_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }

        if Board::check_indexes((possible_index_of_other.x + 1) as i32 , (possible_index_of_other.y) as i32)
        {
            match self.board[possible_index_of_other.x + 1][possible_index_of_other.y] {
                BoardCell::Domino { point } => {
                    if ( point == other_point) {
                        matches.push(true);
                    }
                    else {
                        matches.push(false);
                    }
                },
                _ => (),
            }
        }

        if matches.contains(&true) {
            //(bool, (usize,usize) , (usize , usize) , (i16,i16)) 
            return (true,(possible_index_of_pinned.x,possible_index_of_pinned.y),
            (possible_index_of_other.x,possible_index_of_other.y),skeletonHealth);
        }
        else {
            return (false,(0,0),(0,0),(10,10))
        }
   
        }


        (false,(0,0),(0,0),(10,10))
    }

    pub fn check_indexes(x: i32, y:i32) -> bool {
        if (x>=0 && x<=7 ) && (y >= 0 && y <=7) {
            return true;
        }
        false
    }

    pub fn update_cell(&mut self, cell: (usize, usize), new_board_cell:  BoardCell) {
            self.board[cell.0][cell.1] = new_board_cell;
    }

    pub fn add_domino_on_table(&mut self, new: DominoOnTable) {
        self.dominos.push(new);
    }
   
    pub fn update_skeletons(&mut self, res: (bool, (usize, usize), (usize, usize), (i16, i16)))  {
        //pinned
        let mut rng = rand::thread_rng();
        if (res.3).0  >= 1 && (res.3).0 <=9 {
            loop {
                let a = rng.gen_range(0..7);
                let b = rng.gen_range(0..7);
                match &mut self.board[a][b] {
                    BoardCell::None => {
                        self.board[a][b]=BoardCell::Skeleton { health: (res.3).0, row: a as u16, col: b as u16 };
                        break;
                    }
                    _ => (),
                }
            }
            
        }
        //other
        if (res.3).1 >= 1 && (res.3).1 <= 9 {
            loop {
                let a = rng.gen_range(0..7);
                let b = rng.gen_range(0..7);
                match &mut self.board[a][b] {
                    BoardCell::None => {
                        self.board[a][b]=BoardCell::Skeleton { health: (res.3).1, row: a as u16, col: b as u16 };
                        break;
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn all_skeletons_are_dead(&self) -> bool {
        let mut res : bool = true;
        for i in 0..self.board.len(){
            for j in 0.. self.board[i].len() {
                match self.board[i][j] {
                    BoardCell::Skeleton { health, row, col } => {
                        res=false;
                        return res;
                    },
                    _ => (),
                };
            }
        }
        res
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