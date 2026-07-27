pub struct Board {
    width: usize,
    height: usize,
    zone: Vec<Vec<Cell>>,
}

pub struct Cell {
    value: Option<u32>,
    
}