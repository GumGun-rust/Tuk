#[derive(Debug)]
pub enum KeyCode {
    Letter(u8),
    Number(u8),
    CtrlKey(u8),
    AltKey(u8),
    Arrow(Arrow),
    AltCtrlKey(u8),
    SpecialKey(SpecialKey)
    //None,
}

#[derive(Debug)]
pub enum Arrow {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug)]
pub enum SpecialKey {
    BackSpace,
    Tab,
    Enter,
    Space,
    Escape,
    Debug,
}

pub enum EscapeCode {
    Esc,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    AltKey(u8),
    AltCtrlKey(u8),
    //SpecialKey(SpecialKey),
}

impl From<EscapeCode> for KeyCode {
    //type Error = ();
    fn from(value:EscapeCode) -> KeyCode {
        use EscapeCode::*;
        match value {
            Esc => KeyCode::SpecialKey(SpecialKey::Escape),
            ArrowUp => KeyCode::Arrow(Arrow::Up),
            ArrowDown => KeyCode::Arrow(Arrow::Down),
            ArrowLeft => KeyCode::Arrow(Arrow::Left),
            ArrowRight => KeyCode::Arrow(Arrow::Right),
            AltKey(number) => KeyCode::AltKey(number),
            AltCtrlKey(number) => KeyCode::AltCtrlKey(number),
            //SpecialKey(key) => KeyCode::SpecialKey(key),
        }
    }
}


/*
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
*/
