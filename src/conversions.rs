use std::collections::HashMap;
use crate::errors::ErrVariants;

#[derive(Debug)]
pub struct ValueConverter {
    forward_map: HashMap<char, u32>,
    reverse_map: HashMap<u32, char>,
}

impl ValueConverter {
    pub fn new() -> Self {
        Self {
            forward_map: HashMap::from([
                ('0', 0),
                ('1', 1),
                ('2', 2),
                ('3', 3),
                ('4', 4),
                ('5', 5),
                ('6', 6),
                ('7', 7),
                ('8', 8),
                ('9', 9),
                ('A', 10),
                ('B', 11),
                ('C', 12),
                ('D', 13),
                ('E', 14),
                ('F', 15),
                ('G', 16),
                ('H', 17),
                ('I', 18),
                ('J', 19),
                ('K', 20),
                ('L', 21),
                ('M', 22),
                ('N', 23),
                ('O', 24),
                ('P', 25),
                ('Q', 26),
                ('R', 27),
                ('S', 28),
                ('T', 29),
                ('U', 30),
                ('V', 31),
                ('W', 32),
                ('X', 33),
                ('Y', 34),
                ('Z', 35)
            ]),
            reverse_map: HashMap::from([
                (0, '0'),
                (1, '1'),
                (2, '2'),
                (3, '3'),
                (4, '4'),
                (5, '5'),
                (6, '6'),
                (7, '7'),
                (8, '8'),
                (9, '9'),
                (10, 'A'),
                (11, 'B'),
                (12, 'C'),
                (13, 'D'),
                (14, 'E'),
                (15, 'F'),
                (16, 'G'),
                (17, 'H'),
                (18, 'I'),
                (19, 'J'),
                (20, 'K'),
                (21, 'L'),
                (22, 'M'),
                (23, 'N'),
                (24, 'O'),
                (25, 'P'),
                (26, 'Q'),
                (27, 'R'),
                (28, 'S'),
                (29, 'T'),
                (30, 'U'),
                (31, 'V'),
                (32, 'W'),
                (33, 'X'),
                (34, 'Y'),
                (35, 'Z')
            ]),
        }
    }

    pub fn from_char(&self, c: &char) -> Result<u32, ErrVariants> {
        if let Some(num) = self.forward_map.get(&c) {
            return Ok(*num) 
        }
        return Err(ErrVariants::InvalidCharacter("Invalid character".to_string()));
    }

    pub fn from_num(&self, num: &u32) -> Result<char, ErrVariants> {
        if let Some(character) = self.reverse_map.get(&num) {
            return Ok(*character) 
        }
        return Err(ErrVariants::InvalidCharacter("Invalid character".to_string()));
    }

}