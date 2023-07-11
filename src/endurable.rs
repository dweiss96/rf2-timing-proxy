use std::time::Duration;

pub trait Endurable {
    fn nanosecond(&self) -> Duration;
    fn nanoseconds(&self) -> Duration;
    fn microsecond(&self) -> Duration;
    fn microseconds(&self) -> Duration;
    fn millisecond(&self) -> Duration;
    fn milliseconds(&self) -> Duration;
    fn second(&self) -> Duration;
    fn seconds(&self) -> Duration;
    fn minute(&self) -> Duration;
    fn minutes(&self) -> Duration;
    fn hour(&self) -> Duration;
    fn hours(&self) -> Duration;
    fn day(&self) -> Duration;
    fn days(&self) -> Duration;
}

impl Endurable for u64 {
    fn nanosecond(&self) -> Duration {
        Duration::from_nanos(self.clone())
    }
    fn nanoseconds(&self) -> Duration {
        Duration::from_nanos(self.clone())
    }
    fn microsecond(&self) -> Duration {
        Duration::from_micros(self.clone())
    }
    fn microseconds(&self) -> Duration {
        Duration::from_micros(self.clone())
    }
    fn millisecond(&self) -> Duration {
        Duration::from_millis(self.clone())
    }
    fn milliseconds(&self) -> Duration {
        Duration::from_millis(self.clone())
    }
    fn second(&self) -> Duration {
        Duration::from_secs(self.clone())
    }
    fn seconds(&self) -> Duration {
        Duration::from_secs(self.clone())
    }
    fn minute(&self) -> Duration {
        Duration::from_secs(self.clone() * 60)
    }
    fn minutes(&self) -> Duration {
        Duration::from_secs(self.clone() * 60)
    }
    fn hour(&self) -> Duration {
        Duration::from_secs(self.clone() * 3600)
    }
    fn hours(&self) -> Duration {
        Duration::from_secs(self.clone() * 3600)
    }
    fn day(&self) -> Duration {
        Duration::from_secs(self.clone() * 86400)
    }
    fn days(&self) -> Duration {
        Duration::from_secs(self.clone() * 86400)
    }
}

impl Endurable for u8 {
    fn nanosecond(&self) -> Duration {
        Duration::from_nanos(u64::from(self.clone()))
    }
    fn nanoseconds(&self) -> Duration {
        Duration::from_nanos(u64::from(self.clone()))
    }
    fn microsecond(&self) -> Duration {
        Duration::from_micros(u64::from(self.clone()))
    }
    fn microseconds(&self) -> Duration {
        Duration::from_micros(u64::from(self.clone()))
    }
    fn millisecond(&self) -> Duration {
        Duration::from_millis(u64::from(self.clone()))
    }
    fn milliseconds(&self) -> Duration {
        Duration::from_millis(u64::from(self.clone()))
    }
    fn second(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()))
    }
    fn seconds(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()))
    }
    fn minute(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 60)
    }
    fn minutes(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 60)
    }
    fn hour(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 3600)
    }
    fn hours(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 3600)
    }
    fn day(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 86400)
    }
    fn days(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 86400)
    }
}
impl Endurable for u16 {
    fn nanosecond(&self) -> Duration {
        Duration::from_nanos(u64::from(self.clone()))
    }
    fn nanoseconds(&self) -> Duration {
        Duration::from_nanos(u64::from(self.clone()))
    }
    fn microsecond(&self) -> Duration {
        Duration::from_micros(u64::from(self.clone()))
    }
    fn microseconds(&self) -> Duration {
        Duration::from_micros(u64::from(self.clone()))
    }
    fn millisecond(&self) -> Duration {
        Duration::from_millis(u64::from(self.clone()))
    }
    fn milliseconds(&self) -> Duration {
        Duration::from_millis(u64::from(self.clone()))
    }
    fn second(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()))
    }
    fn seconds(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()))
    }
    fn minute(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 60)
    }
    fn minutes(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 60)
    }
    fn hour(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 3600)
    }
    fn hours(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 3600)
    }
    fn day(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 86400)
    }
    fn days(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 86400)
    }
}
impl Endurable for u32 {
    fn nanosecond(&self) -> Duration {
        Duration::from_nanos(u64::from(self.clone()))
    }
    fn nanoseconds(&self) -> Duration {
        Duration::from_nanos(u64::from(self.clone()))
    }
    fn microsecond(&self) -> Duration {
        Duration::from_micros(u64::from(self.clone()))
    }
    fn microseconds(&self) -> Duration {
        Duration::from_micros(u64::from(self.clone()))
    }
    fn millisecond(&self) -> Duration {
        Duration::from_millis(u64::from(self.clone()))
    }
    fn milliseconds(&self) -> Duration {
        Duration::from_millis(u64::from(self.clone()))
    }
    fn second(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()))
    }
    fn seconds(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()))
    }
    fn minute(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 60)
    }
    fn minutes(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 60)
    }
    fn hour(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 3600)
    }
    fn hours(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 3600)
    }
    fn day(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 86400)
    }
    fn days(&self) -> Duration {
        Duration::from_secs(u64::from(self.clone()) * 86400)
    }
}
