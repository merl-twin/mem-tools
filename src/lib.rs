
struct MemoryMonitor {
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
        let data = MemoryMonitor::rusage()?;       
        Ok(MemoryMonitor{
            data: data,
            tm: std::time::Instant::now(),
        })
    }
    pub fn memory(&self) -> i64 {
        let mem: i64 = self.data.ru_maxrss * 1024;
        mem
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
        println!("MAXRSS: {}",MemoryMonitor::rusage().unwrap().ru_maxrss);
        panic!();
    }

    #[test]
    fn test_vec() {
        let mut v: Vec<u64> = Vec::with_capacity(1024);
        for _ in 0 .. 1024*1024 {
            v.push(0);
        }
        println!("MAXRSS: {}",MemoryMonitor::rusage().unwrap().ru_maxrss);
        panic!();
    }

}


