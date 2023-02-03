#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum BoardCell {
    Domino { point: u16},
    Skeleton { health: i16, row: u16, col: u16},
    Grave,
    None,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum GameStatus {
    LevelInProgress,
    LevelWon,
    GameLoss,
    GameWon
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PlayerState {
    Active,
    Dragging{remember_x: f32 , remember_y: f32, index_of_domino_in_hand: usize}
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum DominoInHandState {
    Visible(bool),
    Moving,
}