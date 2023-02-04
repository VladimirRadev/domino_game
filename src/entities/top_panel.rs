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

    pub fn draw(&self,canvas : &mut graphics::Canvas,ctx: &mut ggez::Context, assets: &Assets,level_to_reach: &usize) {
        canvas.draw(&Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), 
        Rect { x: 0.0, y: 0.0, w: 1000.0, h: 80.0 }, 
        graphics::Color { r: 255.0/255.0, g: 218.0/255.0, b: 185.0/255.0, a: 1.0 }).unwrap(), 
        DrawParam::default());
        
        
        // level

        let mut text = graphics::Text::new(format!("LEVEL {}/{}",self.level,*level_to_reach));
        text.set_font("dejavu_font");
        text.set_scale(graphics::PxScale::from(60.0));
        
        let draw_params= graphics::DrawParam::default().
        dest(Point2 { x: 10.0, y: 20.0})
        .offset(glam::Vec2 {
             x: 10.0 / 1000.0,
             y: 20.0 / 1000.0,})
        .color(graphics::Color { r: 20.0/255.0, g: 218.0/255.0, b: 185.0/255.0, a: 1.0 });
        canvas.draw(&text,draw_params );
        
        // hearts x= 360 , y =30  , square :48 x 48
        let x = 360.0;
        let y = 30.0;
        
        for i in 0..self.lives {
            let draw_params= graphics::DrawParam::default().
        dest(Point2 { x: x + (i as f32)*48.0, y})
        .offset(Point2 {
             x: (x + (i as f32)*48.0) / 1000.0,
             y: y / 1000.0,})
             .scale(Point2{x:1.5, y:1.5});

        canvas.draw(&assets.heart_full, draw_params)

        }
        for i in self.lives..4 {
            let draw_params= graphics::DrawParam::default().
            dest(Point2 { x: x + (i as f32)*48.0, y})
            .offset(Point2 {
                 x: (x + (i as f32)*48.0) / 1000.0,
                 y: y / 1000.0,})
                 .scale(Point2{x:1.5, y:1.5});
    
            canvas.draw(&assets.heart_outlined, draw_params)
    
        }

        //game record
          let mut text = graphics::Text::new(format!("W: {} / L: {}",self.game_record.0,self.game_record.1));
        text.set_font("dejavu_font");
        text.set_scale(graphics::PxScale::from(60.0));
        
        let x = 780.0;
        let y = 20.0;

        let draw_params= graphics::DrawParam::default().
        dest(Point2 { x,y})
        .offset(glam::Vec2 {
             x: x / 1000.0,
             y: y / 1000.0,})
        .color(graphics::Color { r: 20.0/255.0, g: 218.0/255.0, b: 185.0/255.0, a: 1.0 });
        canvas.draw(&text,draw_params );
  

        
        //deck image ( square 64x64)
        let x = deck_image_x as f32;
        let y = deck_image_y as f32; 
        let draw_params = graphics::DrawParam::default()
        .dest(Point2{x,y});
        canvas.draw(&assets.domino_deck, draw_params);
    }

    pub fn check_boundary_of_deck(&self, mouse_position: Point2<f32>) -> bool {
       // println!("{:?}",mouse_position);
        if (mouse_position.x >= deck_image_x as f32 && mouse_position.x <= deck_image_x as f32 + 64.0) 
            && (mouse_position.y >= deck_image_y as f32 && mouse_position.y <= deck_image_y as f32 + 64.0) {
                return true;
            }
        false
    }
    
}