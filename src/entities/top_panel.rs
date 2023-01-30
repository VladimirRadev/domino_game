use ggez::GameResult;

pub struct TopPanel {
    some: u32
}
impl TopPanel {
    pub fn new() -> GameResult<TopPanel> {
        Ok(TopPanel {
            some: 0
        })
    }
}