#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

const MAX_PINS: u16 = 10;
const MAX_FRAMES: usize = 10;

pub struct BowlingGame {
    frames: Vec<Frame>,
}

#[derive(Debug, PartialEq, Eq)]
struct Frame {
    throws: Vec<Throw>,
    last_frame: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct Throw {
    knocked_pins: u16,
}

impl Frame {
    fn new(last_frame: bool) -> Self {
        Self {
            throws: Vec::with_capacity(if last_frame { 3 } else { 2 }),
            last_frame,
        }
    }

    fn knocked_pins(&self) -> u16 {
        self.throws.iter().map(|t| t.knocked_pins).sum()
    }

    fn bonus_pins(&self, throws: usize) -> u16 {
        self.throws
            .iter()
            .take(throws)
            .map(|t| t.knocked_pins)
            .sum()
    }

    fn is_open(&self) -> bool {
        match self.last_frame {
            true => {
                (self.throws.len() < 2)
                    || (self.throws.len() == 2 && self.knocked_pins() >= MAX_PINS)
            }
            false => self.throws.len() < 2 && self.knocked_pins() != MAX_PINS,
        }
    }

    fn is_strike(&self) -> bool {
        self.throws.len() == 1 && self.knocked_pins() == MAX_PINS
    }

    fn is_spare(&self) -> bool {
        self.throws.len() == 2 && self.knocked_pins() == MAX_PINS
    }

    fn is_valid_roll(&self, pins: u16) -> bool {
        if pins > MAX_PINS {
            return false;
        }

        if !self.is_open() {
            return false;
        }

        if !self.last_frame {
            let pins_remaining = MAX_PINS - self.knocked_pins();
            if pins > pins_remaining {
                return false;
            }
        } else {
            if self.throws.len() == 2
                && self.knocked_pins() > MAX_PINS
                && self.knocked_pins() < MAX_PINS * 2
                && pins > MAX_PINS - self.throws[1].knocked_pins
            {
                return false;
            }
        }

        true
    }

    fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if !self.is_valid_roll(pins) {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.throws.push(Throw { knocked_pins: pins });
        Ok(())
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        let mut frames = Vec::with_capacity(MAX_FRAMES);
        frames.push(Frame::new(false));
        Self { frames }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_complete() {
            return Err(Error::GameComplete);
        }

        if !self.current_frame().is_open() {
            match self.frames.len() {
                n if n == MAX_FRAMES - 1 => self.frames.push(Frame::new(true)),
                _ => self.frames.push(Frame::new(false)),
            }
        }

        self.current_frame().roll(pins)?;
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_complete() {
            return None;
        }

        let mut score: u16 = 0;
        for (i, frame) in self.frames.iter().enumerate() {
            score += frame.knocked_pins();
            if frame.is_spare() {
                score += self.calculate_frame_bonus(i + 1, 1);
            } else if frame.is_strike() {
                score += self.calculate_frame_bonus(i + 1, 2);
                score += self.calculate_frame_bonus(i + 2, 1);
            }
        }

        Some(score)
    }

    pub fn is_complete(&self) -> bool {
        self.frames.len() == MAX_FRAMES && !self.frames.last().unwrap().is_open()
    }

    fn current_frame(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    fn calculate_frame_bonus(&self, frame_index: usize, bonus_throws: usize) -> u16 {
        self.frames
            .get(frame_index)
            .unwrap_or(&Frame::new(false))
            .bonus_pins(bonus_throws)
    }
}
