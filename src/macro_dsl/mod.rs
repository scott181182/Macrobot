use std::convert::TryFrom;
use enigo::KeyboardControllable;

mod keys;
mod errors;
pub use errors::ParseError;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct KeyCommand {
    modifiers: Vec<keys::ModifierKeys>,
    key: keys::Key
}

pub fn parse(command: &str) -> Result<KeyCommand, ParseError> {
    let mut cmd_iter: Vec<&str> = command.split('+').collect();
    // Parsing the key (removes it from the vector)
    let key = cmd_iter.pop().ok_or(ParseError::Unexpected)?;
    if key.len() != 1 {
        return Err(ParseError::InvalidKeyLength(key.to_owned()))
    }
    let key = key.chars().next().ok_or(ParseError::Unexpected)?;
    let key = keys::Key(key);
    // Parsing the key modifiers
    let mut modifiers = Vec::new();
    for modifier_str in cmd_iter {
        let modifier = keys::ModifierKeys::try_from(modifier_str)?;
        modifiers.push(modifier);
    }
    Ok(KeyCommand{ modifiers, key })
}

pub fn execute(command: &KeyCommand, enigo: &mut enigo::Enigo) {
    for modifier in command.modifiers.iter() {
        enigo.key_down((*modifier).into());
    }
    enigo.key_click(command.key.into());
    for modifier in command.modifiers.iter() {
        enigo.key_up((*modifier).into());
    }
}