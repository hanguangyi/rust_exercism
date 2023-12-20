// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration{
    s : u64,
}
// const EARTH_SECS: f64 = 31557600.0;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration{s: s}
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.s as f64 / Self::period()
    }
    fn period()->f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn period()->f64 {
        7600543.81992 
    }
}
impl Planet for Venus {
    fn period()->f64 {
        19414149.052176    
    }
}
impl Planet for Earth {
    fn period()->f64 {
        31557600.0
    }
}
impl Planet for Mars {
    fn period()->f64 {
        59354032.69008    
    }
}
impl Planet for Jupiter {
    fn period()->f64 {
        374355659.124
    }
}
impl Planet for Saturn {
    fn period()->f64 {
        929292362.8848   
    }
}
impl Planet for Uranus {
    fn period()->f64 {
        2651370019.3296   
    }
}
impl Planet for Neptune {
    fn period()->f64 {
        5200418560.032
    }
}
