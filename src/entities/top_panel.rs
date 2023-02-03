use ggez::{GameResult, graphics::{self, Mesh, DrawMode, FillOptions, Rect, DrawParam}, mint::{Point2, Vector2}, glam};
use crate::assets::Assets;
pub struct TopPanel {
    pub level : u16,
    pub lives: u16,
    pub game_record : (u16,u16),
}
const deck_image_x:usize = 900;
const deck_image_y:usize = 10;
impl TopPanel {
    pub fn new(level: u16, lives: u16, game_record: (u16,u16)) -> GameResult<TopPanel> {
        Ok(TopPanel {
            level,lives,game_record
        })
    }

    pub fn draw(&self,canvas : &mut graphics::Canvas,ctx: &mut ggez::Context, assets: &Assets) {
        canvas.draw(&Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), 
        Rect { x: 0.0, y: 0.0, w: 1000.0, h: 80.0 }, 
        graphics::Color { r: 255.0/255.0, g: 218.0/255.0, b: 185.0/255.0, a: 1.0 }).unwrap(), 
        DrawParam::default());

        //deck image ( square 64x64)
        let x = deck_image_x as f32;
        let y = deck_image_y as f32; 
        let draw_params = graphics::DrawParam::default()
        .dest(Point2{x,y});
        canvas.draw(&assets.domino_deck, draw_params);
    }

    pub fn check_boundary_of_deck(&self, mouse_position: Point2<f32>) -> bool {
        if (mouse_position.x >= deck_image_x as f32 && mouse_position.x <= deck_image_x as f32 + 64.0) 
            && (mouse_position.y >= deck_image_y as f32 && mouse_position.y <= deck_image_y as f32 + 64.0) {
                return true;
            }
        false
    }
    
}