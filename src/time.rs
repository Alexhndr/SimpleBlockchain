use std::result::Result;
use std::fmt::{Display, Formatter, Error};
use std::time::SystemTime;
use chrono::{DateTime, offset::Utc};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct Time {
    system_time: SystemTime,
}

impl Time {
    #[allow(dead_code)]
    pub fn new(system_time: SystemTime) -> Time {
        Time {
            system_time,
        }
    }
    
    pub fn now() -> Time {
        Time {
            system_time: SystemTime::now(),
        }
    }
    
    #[allow(dead_code)]
    pub fn system_time(&self) -> SystemTime {
        self.system_time
    }
}

impl Display for Time {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let date_time: DateTime<Utc> = self.system_time.into();
        write!(fmt, "{}", date_time.format("%Y.%m.%d %H:%M:%S.%f"))
    }
}
