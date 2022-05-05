use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum Diff {
    /// The current System time is greater than the existing time
    HasPassed(Duration),

    /// The current System time is lesser than the existing time
    ToPass(Duration),
}

#[derive(Debug, Clone, Copy)]
pub struct SystemTimeX(u64);

impl SystemTimeX {
    pub fn now_duration() -> Duration {
        let t = SystemTime::now();

        t.duration_since(UNIX_EPOCH)
            .expect("[P00006] Some error occured with SystemTimeX duration conversion")
    }

    pub fn now_millis() -> u64 {
        let current_system_time = Self::now_duration();
        current_system_time.as_millis() as u64
    }

    pub fn now() -> Self {
        let now_ms = Self::now_millis();
        Self(now_ms as u64)
    }

    pub const fn from_millis(ms: u64) -> Self {
        Self(ms as u64)
    }

    pub const fn from_duration(d: Duration) -> Self {
        Self(d.as_millis() as u64)
    }

    pub fn as_millis(&self) -> u64 {
        self.0
    }

    pub fn as_duration(&self) -> Duration {
        let ms = self.as_millis();

        Duration::from_millis(ms)
    }

    pub fn since(&self) -> Diff {
        let now_ms = Self::now_millis();
        let existing = self.0;

        if existing > now_ms {
            let diff = existing - now_ms;

            return Diff::ToPass(Duration::from_millis(diff as u64));
        }

        let diff = now_ms - existing;
        Diff::HasPassed(Duration::from_millis(diff as u64))
    }

    pub fn has_passed(&self) -> Option<Duration> {
        if let Diff::HasPassed(d) = self.since() {
            return Some(d);
        }

        None
    }

    pub fn to_pass(&self) -> Option<Duration> {
        if let Diff::ToPass(d) = self.since() {
            return Some(d);
        }

        None
    }

    pub fn set_now(&mut self) -> Self {
        let now_ms = Self::now_millis();

        self.0 = now_ms;

        Self::from_millis(now_ms)
    }

    pub fn add_duration_to_now(&mut self, d: Duration) -> Self {
        let now_d = Self::now_duration() + d;

        self.0 = now_d.as_millis() as u64;

        Self::from_duration(now_d)
    }

    pub fn add_millis_to_now(&mut self, millis: u64) -> Self {
        let now_ms = Self::now_millis() + millis;

        self.0 = now_ms;

        Self::from_millis(now_ms)
    }

    pub fn add_duration(&mut self, d: Duration) -> Self {
        let next = self.as_duration() + d;

        self.0 = next.as_millis() as u64;

        Self::from_duration(next)
    }

    pub fn add_millis(&mut self, millis: u64) -> Self {
        let next = self.as_millis() + millis;

        self.0 = next;

        Self::from_millis(next)
    }
}
