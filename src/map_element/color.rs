#[derive(PartialEq, Clone, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Pink,
    Custom([f32; 4]),
}

impl Default for Color {
    fn default() -> Self {
        Color::Red
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        match self {
            Color::Red => [1.0, 0.36, 0.2, 1.0],
            Color::Green => [0.0, 0.6, 0.2, 1.0],
            Color::Blue => [0.3, 0.47, 1.0, 1.0],
            Color::Yellow => [1.0, 0.83, 0.2, 1.0],
            Color::Orange => [1.0, 0.6, 0.2, 1.0],
            Color::Pink => [0.8, 0.0, 0.8, 1.0],
            Color::Custom(color) => color,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into() {
        let red = [1.0, 0.36, 0.2, 1.0];
        let green = [0.0, 0.6, 0.2, 1.0];
        let blue = [0.3, 0.47, 1.0, 1.0];
        let yellow = [1.0, 0.83, 0.2, 1.0];
        let orange = [1.0, 0.6, 0.2, 1.0];
        let pink = [0.8, 0.0, 0.8, 1.0];
        let custom = [0.1, 0.2, 0.3, 0.4];

        assert_eq!(Into::<[f32; 4]>::into(Color::Red), red);
        assert_eq!(Into::<[f32; 4]>::into(Color::Green), green);
        assert_eq!(Into::<[f32; 4]>::into(Color::Blue), blue);
        assert_eq!(Into::<[f32; 4]>::into(Color::Yellow), yellow);
        assert_eq!(Into::<[f32; 4]>::into(Color::Orange), orange);
        assert_eq!(Into::<[f32; 4]>::into(Color::Pink), pink);
        assert_eq!(Into::<[f32; 4]>::into(Color::Custom(custom)), custom);
    }
}
