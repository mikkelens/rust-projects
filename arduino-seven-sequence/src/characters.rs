use ufmt::derive::uDebug;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum DecimalDigit {
    Zero = 0b0111111,
    One = 0b0000110,
    Two = 0b1011011,
    Three = 0b1001111,
    Four = 0b1100110,
    Five = 0b1101101,
    Six = 0b1111101,
    Seven = 0b0000111,
    Eight = 0b1111111,
    Nine = 0b1101111,
}

#[allow(unused)]
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum HexDigit {
    Zero = DecimalDigit::Zero as u8,
    One = DecimalDigit::One as u8,
    Two = DecimalDigit::Two as u8,
    Three = DecimalDigit::Three as u8,
    Four = DecimalDigit::Four as u8,
    Five = DecimalDigit::Five as u8,
    Six = DecimalDigit::Six as u8,
    Seven = DecimalDigit::Seven as u8,
    Eight = DecimalDigit::Eight as u8,
    Nine = DecimalDigit::Nine as u8,
    Ten = 0b1110111,      // A
    Eleven = 0b1111100,   // B
    Twelve = 0b0111001,   // C
    Thirteen = 0b1011110, // D
    Fourteen = 0b1111001, // E
    Fifteen = 0b1110001,  // F
}
impl From<DecimalDigit> for HexDigit {
    fn from(digit: DecimalDigit) -> Self {
        match digit {
            DecimalDigit::Zero => HexDigit::Zero,
            DecimalDigit::One => HexDigit::One,
            DecimalDigit::Two => HexDigit::Two,
            DecimalDigit::Three => HexDigit::Three,
            DecimalDigit::Four => HexDigit::Four,
            DecimalDigit::Five => HexDigit::Five,
            DecimalDigit::Six => HexDigit::Six,
            DecimalDigit::Seven => HexDigit::Seven,
            DecimalDigit::Eight => HexDigit::Eight,
            DecimalDigit::Nine => HexDigit::Nine,
        }
    }
}

#[derive(Debug)]
pub struct DecimalDigitParseErr(pub u8);
impl TryFrom<u8> for DecimalDigit {
    type Error = DecimalDigitParseErr;

    fn try_from(num: u8) -> Result<Self, Self::Error> {
        Ok(match num {
            0 => DecimalDigit::Zero,
            1 => DecimalDigit::One,
            2 => DecimalDigit::Two,
            3 => DecimalDigit::Three,
            4 => DecimalDigit::Four,
            5 => DecimalDigit::Five,
            6 => DecimalDigit::Six,
            7 => DecimalDigit::Seven,
            8 => DecimalDigit::Eight,
            9 => DecimalDigit::Nine,
            too_big => Err(DecimalDigitParseErr(too_big))?,
        })
    }
}

#[derive(Debug)]
pub struct LetterParseErr(pub char);
#[derive(uDebug, Debug, Copy, Clone)]
pub struct DisplayLetter(u8);
impl TryFrom<char> for DisplayLetter {
    type Error = LetterParseErr;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(DisplayLetter(match c {
            'A' => 0b1110111,
            'B' => 0b1111100,
            'C' => 0b0111001,
            'D' => 0b1011110,
            'E' => 0b1111001,
            'F' => 0b1110001,
            'G' => 0b0111101,
            'H' => 0b1110110,
            'I' => 0b0110000,
            'J' => 0b0011110,
            'K' => 0b1110101,
            'L' => 0b0111000,
            'M' => 0b0010101,
            'N' => 0b0110111,
            'O' => 0b0111111,
            'P' => 0b1110011,
            'Q' => 0b1101011,
            'R' => 0b0110011,
            'S' => 0b1101101,
            'T' => 0b1111000,
            'U' => 0b0111110,
            'V' => 0b0111110,
            'W' => 0b0101010,
            'X' => 0b1110110,
            'Y' => 0b1101110,
            'Z' => 0b1011011,
            'a' => 0b1011111,
            'b' => 0b1111100,
            'c' => 0b1011000,
            'd' => 0b1011110,
            'e' => 0b1111011,
            'f' => 0b1110001,
            'g' => 0b1101111,
            'h' => 0b1110100,
            'i' => 0b0010000,
            'j' => 0b0001100,
            'k' => 0b1110101,
            'l' => 0b0110000,
            'm' => 0b0010100,
            'n' => 0b1010100,
            'o' => 0b1011100,
            'p' => 0b1110011,
            'q' => 0b1100111,
            'r' => 0b1010000,
            's' => 0b1101101,
            't' => 0b1111000,
            'u' => 0b0011100,
            'v' => 0b0011100,
            'w' => 0b0010100,
            'x' => 0b1110110,
            'y' => 0b1101110,
            'z' => 0b1011011,
            other_letter => Err(LetterParseErr(other_letter))?,
        }))
    }
}
pub trait BitDisplayable {
    fn displayable_bits(&self) -> u8;
}
impl BitDisplayable for DecimalDigit {
    fn displayable_bits(&self) -> u8 {
        *self as u8
    }
}
impl BitDisplayable for HexDigit {
    fn displayable_bits(&self) -> u8 {
        *self as u8
    }
}
impl BitDisplayable for DisplayLetter {
    fn displayable_bits(&self) -> u8 {
        self.0
    }
}
