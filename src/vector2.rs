use std::ops::Add;


#[derive(Debug, Copy, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}


impl<T: Add<Output = T>> Add for Vector2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
