use std::convert::TryFrom;
use super::errors::ParseError;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ModifierKeys {
    Shift,
    Ctrl,
    Alt,
    Meta
}

impl TryFrom<&str> for ModifierKeys {
    type Error = ParseError;

    fn try_from(key: &str) -> Result<Self, ParseError> {
        let key_str = key.to_lowercase();
        match key_str.as_ref() {
            "shift" => Ok(Self::Shift),
            "ctrl"  => Ok(Self::Ctrl),
            "alt"   => Ok(Self::Alt),
            "meta"  => Ok(Self::Meta),
            _ => Err(ParseError::UnknownKey(key.to_owned()))
        }
    }
}

impl From<ModifierKeys> for enigo::Key {
    fn from(key: ModifierKeys) -> enigo::Key {
        match key {
            ModifierKeys::Shift => enigo::Key::Shift,
            ModifierKeys::Ctrl  => enigo::Key::Control,
            ModifierKeys::Alt   => enigo::Key::Alt,
            ModifierKeys::Meta  => enigo::Key::Meta
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Key(pub char);

impl From<Key> for enigo::Key {
    fn from(key: Key) -> enigo::Key {
        let Key(key_char) = key;
        enigo::Key::Layout(key_char)
    }
}