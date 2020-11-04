use std::fmt;
use std::str::FromStr;

use enigo::KeyboardControllable;



mod keys;
use keys::MacrobotKey;

mod errors;
pub use errors::ParseError;



#[derive(Clone, Eq, PartialEq, Debug)]
pub struct KeyCommandGroup {
    commands: Vec<KeyCommand>
}

impl KeyCommandGroup
{
    pub fn execute(&self, enigo: &mut enigo::Enigo) {
        for command in self.commands.iter() {
            command.execute(enigo);
        }
    }

    pub fn to_command_string(&self) -> String {
        self.commands.iter()
            .map(KeyCommand::to_command_string)
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl FromStr for KeyCommandGroup
{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command_results: Result<Vec<KeyCommand>, ParseError> = s.split(' ')
            .map(str::parse::<KeyCommand>)
            .collect();
        Ok(KeyCommandGroup{ commands: command_results? })
    }
}
impl fmt::Display for KeyCommandGroup
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KeyCommandGroup({})", self.to_command_string())
    }
}



#[derive(Clone, Eq, PartialEq, Debug)]
pub struct KeyCommand {
    keys: Vec<MacrobotKey>
}

impl KeyCommand
{
    pub fn execute(&self, enigo: &mut enigo::Enigo) {
        for key in self.keys.iter() {
            enigo.key_down((*key).into());
        }
        for key in self.keys.iter() {
            enigo.key_up((*key).into());
        }
    }
    pub fn to_command_string(&self) -> String {
        self.keys.iter()
            .map(MacrobotKey::to_key_string)
            .collect::<Vec<String>>()
            .join("+")
    }
}

impl FromStr for KeyCommand
{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key_results: Result<Vec<MacrobotKey>, _> = s.split('+')
            .map(str::parse::<MacrobotKey>)
            .collect();
        Ok(KeyCommand{ keys: key_results? })
    }
}
impl fmt::Display for KeyCommand
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "KeyCommand({})", self.to_command_string())
    }
}