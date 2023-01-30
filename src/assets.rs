use ggez::audio;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;
use ggez::mint::Point2;
pub struct Assets {
    pub empty_cell: graphics::Image,
    pub skeleton: graphics::Image,
    pub grave: graphics::Image,
}
impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let mut empty_cell = graphics::Image::from_path(ctx, "/empty_cell.png").unwrap();
        let mut skeleton = graphics::Image::from_path(ctx, "/skeleton.png").unwrap();
        let mut grave = graphics::Image::from_path(ctx, "/grave.png").unwrap();

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
        })
    }
}
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct DominoOnTable {
    pub points: (u16, u16),
    pub pinned_row: u16,
    pub pinned_col: u16,
    pub rotation: i16,
}
impl DominoOnTable {
    pub fn new(points: (u16,u16),pinned_row : u16,pinned_col: u16,rotation: i16) -> GameResult<DominoOnTable>{
        Ok(DominoOnTable {
            points,pinned_row,pinned_col,rotation
        })
    }
}
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct DominoInHand {
    pub points: (u16, u16),
    pub position: Point2<f32>,
    pub rotation: i16,
}
impl DominoInHand {
    pub fn new(points: (u16,u16),position : Point2<f32>,rotation: i16) -> GameResult<DominoInHand>{
        Ok(DominoInHand {
            points,position,rotation
        })
    }
}