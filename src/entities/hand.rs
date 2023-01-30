use ggez::GameResult;

pub struct Hand{
    some: u32,
}
impl Hand {
    pub fn new() -> GameResult<Hand> {
        Ok(Hand {
            some: 0
        })
    }
}