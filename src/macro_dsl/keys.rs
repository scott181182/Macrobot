use std::fmt;
use std::str::FromStr;

use phf::phf_map;

use super::errors::ParseError;





#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MacrobotKey
{
    NamedKey(enigo::Key),
    CharKey(char)
}
impl MacrobotKey
{
    pub fn to_key_string(&self) -> String {
        match self {
            MacrobotKey::NamedKey(ref key) => {
                match key {
                    enigo::Key::Alt       => "alt".to_owned(),
                    enigo::Key::Backspace => "backspace".to_owned(),
                    enigo::Key::Control   => "ctrl".to_owned(),
                    enigo::Key::Return    => "enter".to_owned(),
                    enigo::Key::Escape    => "escape".to_owned(),
                    enigo::Key::Meta      => "meta".to_owned(),
                    enigo::Key::Shift     => "shift".to_owned(),
                    enigo::Key::Space     => "space".to_owned(),
                    enigo::Key::Tab       => "tab".to_owned(),
                    
                    // Lock Keys
                    enigo::Key::CapsLock => "capslock".to_owned(),
                    // Not supported by Enigo?
                    // "Key(numlock)"    => MacrobotKey::NamedKey(enigo::Key::???),
                    // "Key(scrolllock)" => MacrobotKey::NamedKey(enigo::Key::???),

                    // Extention Keys
                    enigo::Key::Delete   => "delete".to_owned(),
                    enigo::Key::End      => "end".to_owned(),
                    enigo::Key::Home     => "home".to_owned(),
                    // Not supported by Enigo?
                    // "Key(ins)"      => MacrobotKey::NamedKey(enigo::Key::???),
                    // "Key(insert)"   => MacrobotKey::NamedKey(enigo::Key::???),
                    enigo::Key::PageDown => "pagedown".to_owned(),
                    enigo::Key::PageUp   => "pageup".to_owned(),

                    // Arrow Keys
                    enigo::Key::UpArrow    => "up".to_owned(),
                    enigo::Key::DownArrow  => "down".to_owned(),
                    enigo::Key::LeftArrow  => "left".to_owned(),
                    enigo::Key::RightArrow => "right".to_owned(),

                    // function Keys
                    enigo::Key::F1  => "f1".to_owned(),
                    enigo::Key::F2  => "f2".to_owned(),
                    enigo::Key::F3  => "f3".to_owned(),
                    enigo::Key::F4  => "f4".to_owned(),
                    enigo::Key::F5  => "f5".to_owned(),
                    enigo::Key::F6  => "f6".to_owned(),
                    enigo::Key::F7  => "f7".to_owned(),
                    enigo::Key::F8  => "f8".to_owned(),
                    enigo::Key::F9  => "f9".to_owned(),
                    enigo::Key::F10 => "f10".to_owned(),
                    enigo::Key::F11 => "f11".to_owned(),
                    enigo::Key::F12 => "f12".to_owned(),

                    key => format!("unknown({:?})", key)
                }
            },
            MacrobotKey::CharKey(ref c) => format!("'{}'", c)
        }
    }
}
impl fmt::Display for MacrobotKey
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key({})", self.to_key_string())
    }
}


static NAMED_KEY_MAP: phf::Map<&'static str, MacrobotKey> = phf_map!{
    // Named Keys
    "alt"       => MacrobotKey::NamedKey(enigo::Key::Alt),
    "backspace" => MacrobotKey::NamedKey(enigo::Key::Backspace),
    "ctrl"      => MacrobotKey::NamedKey(enigo::Key::Control),
    "enter"     => MacrobotKey::NamedKey(enigo::Key::Return),
    "return"    => MacrobotKey::NamedKey(enigo::Key::Return),
    "slap"      => MacrobotKey::NamedKey(enigo::Key::Return),
    "esc"       => MacrobotKey::NamedKey(enigo::Key::Escape),
    "escape"    => MacrobotKey::NamedKey(enigo::Key::Escape),
    "meta"      => MacrobotKey::NamedKey(enigo::Key::Meta),
    "shift"     => MacrobotKey::NamedKey(enigo::Key::Shift),
    "space"     => MacrobotKey::NamedKey(enigo::Key::Space),
    "tab"       => MacrobotKey::NamedKey(enigo::Key::Tab),
    
    // Lock Keys
    "capslock"   => MacrobotKey::NamedKey(enigo::Key::CapsLock),
    // Not supported by Enigo?
    // "numlock"    => MacrobotKey::NamedKey(enigo::Key::???),
    // "scrolllock" => MacrobotKey::NamedKey(enigo::Key::???),

    // Extention Keys
    "del"      => MacrobotKey::NamedKey(enigo::Key::Delete),
    "delete"   => MacrobotKey::NamedKey(enigo::Key::Delete),
    "end"      => MacrobotKey::NamedKey(enigo::Key::End),
    "home"     => MacrobotKey::NamedKey(enigo::Key::Home),
    // Not supported by Enigo?
    // "ins"      => MacrobotKey::NamedKey(enigo::Key::???),
    // "insert"   => MacrobotKey::NamedKey(enigo::Key::???),
    "pagedown" => MacrobotKey::NamedKey(enigo::Key::PageDown),
    "pageup"   => MacrobotKey::NamedKey(enigo::Key::PageUp),
    "pgdown"   => MacrobotKey::NamedKey(enigo::Key::PageDown),
    "pgup"     => MacrobotKey::NamedKey(enigo::Key::PageUp),

    // Arrow Keys
    "up"    => MacrobotKey::NamedKey(enigo::Key::UpArrow),
    "down"  => MacrobotKey::NamedKey(enigo::Key::DownArrow),
    "left"  => MacrobotKey::NamedKey(enigo::Key::LeftArrow),
    "right" => MacrobotKey::NamedKey(enigo::Key::RightArrow),

    // function Keys
    "f1"  => MacrobotKey::NamedKey(enigo::Key::F1),
    "f2"  => MacrobotKey::NamedKey(enigo::Key::F2),
    "f3"  => MacrobotKey::NamedKey(enigo::Key::F3),
    "f4"  => MacrobotKey::NamedKey(enigo::Key::F4),
    "f5"  => MacrobotKey::NamedKey(enigo::Key::F5),
    "f6"  => MacrobotKey::NamedKey(enigo::Key::F6),
    "f7"  => MacrobotKey::NamedKey(enigo::Key::F7),
    "f8"  => MacrobotKey::NamedKey(enigo::Key::F8),
    "f9"  => MacrobotKey::NamedKey(enigo::Key::F9),
    "f10" => MacrobotKey::NamedKey(enigo::Key::F10),
    "f11" => MacrobotKey::NamedKey(enigo::Key::F11),
    "f12" => MacrobotKey::NamedKey(enigo::Key::F12),
};

impl FromStr for MacrobotKey
{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if NAMED_KEY_MAP.contains_key(s) {
            return Ok(*NAMED_KEY_MAP.get(s).unwrap());
        } else if s.len() == 1 {
            // TODO: add check for non-shifted characters
            let c = s.chars().next().unwrap();
            return Ok(MacrobotKey::CharKey(c));
        }

        Err(ParseError::UnknownKey(s.into()))
    }
}

impl From<MacrobotKey> for enigo::Key
{
    fn from(key: MacrobotKey) -> enigo::Key {
        match key {
            MacrobotKey::NamedKey(enigo_key) => enigo_key,
            MacrobotKey::CharKey(c) => enigo::Key::Layout(c)
        }
    }
}