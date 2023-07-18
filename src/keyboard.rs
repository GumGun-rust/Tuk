#[derive(Debug)]
pub enum KeyCode {
    Letter(u8),
    Arrow(Arrow),
    //None,
}

#[derive(Debug)]
pub enum Arrow {
    Left,
    Up,
    Right,
    Down,
}

pub enum EscapeCode {
    Esc,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

impl From<EscapeCode> for KeyCode {
    //type Error = ();
    fn from(value:EscapeCode) -> KeyCode {
        use EscapeCode::*;
        match value {
            Esc => KeyCode::Letter(27),
            ArrowUp => KeyCode::Arrow(Arrow::Up),
            ArrowDown => KeyCode::Arrow(Arrow::Down),
            ArrowLeft => KeyCode::Arrow(Arrow::Left),
            ArrowRight => KeyCode::Arrow(Arrow::Right),
        }
    }
}

impl TryFrom<u8> for Arrow {
    type Error = ();
    fn try_from(value:u8) -> Result<Arrow, Self::Error> {
        match value {
            b'j' => Ok(Arrow::Down),
            b'k' => Ok(Arrow::Up),
            _ => {
                Err(())
            }
        }
    }
}

