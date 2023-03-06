
#[derive(Debug)]
pub enum ErrVariants {
    InvalidCharacter(String),
    CharacterNotAbidingByBase(String),
    EmptyInput(String),
}

impl ErrVariants {
    pub fn message(&self) -> String {
        match &self {
            ErrVariants::InvalidCharacter(message) => message.to_owned(),
            ErrVariants::CharacterNotAbidingByBase(message) => message.to_owned(),
            ErrVariants::EmptyInput(message) => message.to_owned(),
        }
    }
}