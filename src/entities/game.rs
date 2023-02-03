use ggez::GameResult;

use crate::entities::enums::GameStatus;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Game{
    pub skeletons: Vec<u16>,
    pub graves_count: u16,
    pub game_status: GameStatus 
}
impl Game {
    pub fn new(skeletons:Vec<u16>,graves_count: u16,game_status:GameStatus) -> GameResult<Game> {
        Ok(Game 
            { skeletons: skeletons.clone(), graves_count: graves_count, game_status: game_status }
        )
    }
}
