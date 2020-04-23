
#[derive(Debug,Clone,Copy)]
enum Unit {
    Unknown,
    Tera,
    Giga,
    Mega,
    Kilo,
    Byte,
}
impl Into<&'static str> for Unit {
    fn into(self) -> &'static str {
        match self {
            Unit::Unknown => "??",
            Unit::Tera => "TB",
            Unit::Giga => "GB",
            Unit::Mega => "MB",
            Unit::Kilo => "kB",
            Unit::Byte => "B",
        }
    }
}
impl Unit {
    fn as_u64(self) -> u64 {
        match self {
            Unit::Unknown => 1,
            Unit::Tera => 1024*1024*1024*1024,
            Unit::Giga => 1024*1024*1024,
            Unit::Mega => 1024*1024,
            Unit::Kilo => 1024,
            Unit::Byte => 1,
        }
    }
    fn as_f64(self) -> f64 {
        self.as_u64() as f64
    }
    fn inc(self) -> Option<Unit> {
        match self {
            Unit::Unknown => None,
            Unit::Tera => None,
            Unit::Giga => Some(Unit::Tera),
            Unit::Mega => Some(Unit::Giga),
            Unit::Kilo => Some(Unit::Mega),
            Unit::Byte => Some(Unit::Kilo),
        }
    }
}

struct MemoryMonitor {
    unit: Unit,
    data: libc::rusage,
    tm: std::time::Instant,
}
impl MemoryMonitor {
    pub fn rusage() -> Result<libc::rusage,i32> {
        let mut usage = libc::rusage {
            ru_utime: libc::timeval { tv_sec: 0, tv_usec: 0 },
            ru_stime: libc::timeval { tv_sec: 0, tv_usec: 0 },
            ru_maxrss: 0,
            ru_ixrss: 0,
            ru_idrss: 0,
            ru_isrss: 0,
            ru_minflt: 0,
            ru_majflt: 0,
            ru_nswap: 0,
            ru_inblock: 0,
            ru_oublock: 0,
            ru_msgsnd: 0,
            ru_msgrcv: 0,
            ru_nsignals: 0,
            ru_nvcsw: 0,
            ru_nivcsw: 0,
        };
        match unsafe { libc::getrusage(libc::RUSAGE_SELF,&mut usage) } {
            0 => Ok(usage),
            c @ _ => Err(c),
        }
    }
    pub fn new() -> Result<MemoryMonitor,i32> {
        let unit = match cfg!(linux) {
            true => Unit::Kilo,
            false => match cfg!(macos) {
                true => Unit::Byte,
                false => Unit::Unknown,
            },
        };
        let data = MemoryMonitor::rusage()?;       
        Ok(MemoryMonitor{
            unit: unit,
            data: data,
            tm: std::time::Instant::now(),
        })
    }
    pub fn memory(&self) -> i64 {
        let mem: i64 = self.data.ru_maxrss * (self.unit.as_u64() as i64);
        mem
    }
    pub fn hmem(&self) -> String {
        let mut unit = Unit::Byte;
        let mut mem: f64 = (self.data.ru_maxrss as f64) * self.unit.as_f64();
        while mem > 600.0 {
            match unit.inc() {
                Some(un) => {
                    mem /= 1024.0;
                    unit = un;
                },
                None => break,
            }
        }
        let un: &'static str = unit.into();
        format!("{:0.3} {}",mem,un)
    }
    pub fn refresh(&mut self) -> bool {
        match MemoryMonitor::rusage() {
            Ok(ru) => {
                self.data = ru;
                self.tm = std::time::Instant::now();
                true
            },
            Err(_) => false,
        } 
    }
    pub fn sec_since_last_update(&self) -> u64 {
        self.tm.elapsed().as_secs()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty() {
        let mem = MemoryMonitor::new().unwrap();
        println!("MAXRSS: {}",mem.hmem());
        panic!();
    }

    #[test]
    fn test_vec() {
        let mut v: Vec<u64> = Vec::with_capacity(1024);
        for _ in 0 .. 1024*1024 {
            v.push(0);
        }
        let mem = MemoryMonitor::new().unwrap();
        println!("MAXRSS: {}",mem.hmem());
        panic!();
    }

}


