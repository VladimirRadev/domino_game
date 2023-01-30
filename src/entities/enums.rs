#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum BoardCell {
    Domino {point : u16} ,
    Skeleton { health: i16},
    Grave,
    None,
}
