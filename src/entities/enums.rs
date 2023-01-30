#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum BoardCell {
    Skeleton { health: i16, row: u16, col: u16},
    Grave,
    None,
}
