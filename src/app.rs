use rand::Rng;
use std::{
    num::{NonZeroU8, ParseIntError},
    str::FromStr,
};
use vizia::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct RangeInt(NonZeroU8);

impl ToString for RangeInt {
    fn to_string(&self) -> String {
        let mut buf = String::new();
        itoap::write_to_string(&mut buf, self.0.get());
        buf
    }
}

impl FromStr for RangeInt {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<NonZeroU8>().map(Self)
    }
}

impl Data for RangeInt {
    fn same(&self, other: &Self) -> bool {
        self.0.same(&other.0)
    }
}

impl RangeInt {
    pub fn get(&self) -> u8 {
        self.0.get()
    }
}

#[derive(Lens)]
pub struct AppData {
    pub range_start: RangeInt,
    pub range_end: RangeInt,
    pub result: Option<RangeInt>,
    pub button_disabled: bool,
}

impl AppData {
    fn gen_rand(&mut self) {
        self.result = unsafe {
            Some(RangeInt(NonZeroU8::new_unchecked(
                rand::thread_rng().gen_range(self.range_start.get()..=self.range_end.get()),
            )))
        }
    }
}

impl Default for AppData {
    fn default() -> Self {
        unsafe {
            Self {
                range_start: RangeInt(NonZeroU8::new_unchecked(1)),
                range_end: RangeInt(NonZeroU8::new_unchecked(1)),
                result: None,
                button_disabled: false,
            }
        }
    }
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        let Some(app_event) = event.take() else { return; };

        match app_event {
            AppEvent::SetRangeStart(val) => {
                self.range_start = val;
                self.button_disabled = self.range_end.get() < self.range_start.get();
            }
            AppEvent::SetRangeEnd(val) => {
                self.range_end = val;
                self.button_disabled = self.range_end.get() < self.range_start.get();
            }
            AppEvent::Generate => {
                if !self.button_disabled {
                    self.gen_rand()
                }
            }
        }
    }
}

pub enum AppEvent {
    Generate,
    SetRangeStart(RangeInt),
    SetRangeEnd(RangeInt),
}
