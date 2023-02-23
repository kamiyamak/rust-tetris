

// フィールドサイズ
const FIELD_WIDTH: usize = 12;
const FIELD_HEIGHT: usize = 22;

type FieldSize = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

struct Position {
    x: usize,
    y: usize,
}
