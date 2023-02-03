use ggez::GameResult;

pub struct TopPanel {
    pub level : u16,
    pub lives: u16,
    pub game_record : (u16,u16),
}
impl TopPanel {
    pub fn new(level: u16, lives: u16, game_record: (u16,u16)) -> GameResult<TopPanel> {
        Ok(TopPanel {
            level,lives,game_record
        })
    }

    pub fn update() {
        todo!();
    }
    pub fn draw(&self) {
        
    }
    
}