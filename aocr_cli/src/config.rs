use std::{env::current_dir, fs, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StateFile {
    pub current_year: Option<u16>,
    pub current_day: Option<u8>,
    initialized_years: Vec<u16>,
    initialized_days: Vec<(u16, u8)>,
}

impl StateFile {
    pub fn load() -> Result<Self> {
        let path = Self::state_path()?;
        let file = fs::read_to_string(&path);
        match file {
            Ok(contents) => toml::from_str(&contents).map_err(Into::into),
            Err(_) => {
                let state = Self::default();
                state.save()?;
                Ok(state)
            }
        }
    }

    fn save(&self) -> Result<()> {
        let path = Self::state_path()?;
        let contents = toml::to_string(self)?;
        fs::write(path, contents)?;
        Ok(())
    }

    pub fn set_current_year(&mut self, year: u16) -> Result<()> {
        self.current_year = Some(year);
        if !self.initialized_years.contains(&year) {
            self.initialized_years.push(year);
        }
        self.initialized_years.sort();

        self.save()
    }

    pub fn set_current_day(&mut self, day: u8, year: u16) -> Result<()> {
        self.current_day = Some(day);
        self.current_year = Some(year);
        if !self.initialized_years.contains(&year) {
            self.initialized_years.push(year);
        }
        if !self.initialized_days.contains(&(year, day)) {
            self.initialized_days.push((year, day));
        }
        self.initialized_years.sort();
        self.initialized_days.sort();
        self.save()
    }

    fn state_path() -> Result<PathBuf> {
        current_dir()
            .map(|dir| dir.join(".aocr.toml"))
            .map_err(Into::into)
    }

    pub fn has_initialized_year(&self, year: &u16) -> bool {
        self.initialized_years.contains(year)
    }

    pub fn has_initialized_day(&self, year: &u16, day: &u8) -> bool {
        self.initialized_days.contains(&(*year, *day))
    }
}
