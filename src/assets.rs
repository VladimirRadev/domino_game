use ggez::audio;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;
pub struct Assets {
    pub empty_cell: graphics:: Image,
    pub skeleton: graphics::Image,
    pub grave: graphics::Image,
}
impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let mut empty_cell = graphics::Image::from_path(ctx,"/empty_cell.png").unwrap();
        let mut  skeleton = graphics::Image::from_path(ctx,"/skeleton.png").unwrap();
        let mut grave = graphics::Image::from_path(ctx,"/grave.png").unwrap();

        println!("{} {} {}",empty_cell.height(),skeleton.height(),grave.height());
        Ok(Assets { 
            empty_cell: empty_cell,
            skeleton: skeleton,
            grave: grave,
        })
    }
}
