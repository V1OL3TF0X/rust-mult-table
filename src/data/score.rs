use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Score {
    pub(crate) tries: u16,
    pub(crate) correct: u16,
    pub(crate) percentage: Option<u16>,
}
pub const MAX_PERCENT: u16 = 10_000;
impl Score {
    pub fn get_percentage(&self) -> Option<u16> {
        self.percentage
    }

    fn calc_percentage(correct: u16, tries: u16) -> Option<u16> {
        if tries != 0 {
            Some(correct * MAX_PERCENT / tries)
        } else {
            None
        }
    }

    pub fn update(&mut self, is_correct: bool) {
        if is_correct {
            self.correct += 1;
        }
        self.tries += 1;
        self.percentage = Self::calc_percentage(self.correct, self.tries);
    }

    pub fn new(tries: u16, correct: u16) -> Self {
        Self {
            tries,
            correct,
            percentage: Self::calc_percentage(correct, tries),
        }
    }
}

impl From<&Sdto> for Score {
    fn from(value: &Sdto) -> Self {
        Self {
            percentage: Self::calc_percentage(value.1, value.0),
            tries: value.0,
            correct: value.1,
        }
    }
}

impl From<Sdto> for Score {
    fn from(value: Sdto) -> Self {
        (&value).into()
    }
}

impl From<&Score> for Sdto {
    fn from(val: &Score) -> Self {
        Sdto(val.tries, val.correct)
    }
}

impl<'de> Deserialize<'de> for Score {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = Sdto::deserialize(deserializer)?;
        Ok(s.into())
    }
}

impl Serialize for Score {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Sdto::serialize(&self.into(), serializer)
    }
}

impl Default for Score {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.get_percentage(), other.get_percentage()) {
            (None, None) => std::cmp::Ordering::Equal,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), None) => std::cmp::Ordering::Greater,
            (Some(p1), Some(p2)) => p1.cmp(&p2).then_with(|| self.tries.cmp(&other.tries)),
        }
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("correct: {}\ntotal tries: {}", self.correct, self.tries);
        if let Some(p) = self.percentage {
            s = format!("{s}\n percent correct: {}%", p as f32 / 100.0);
        }
        write!(f, "{s}")
    }
}

#[derive(Serialize, Deserialize)]
struct Sdto(u16, u16);
