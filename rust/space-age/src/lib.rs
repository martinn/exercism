#[derive(Debug)]
pub struct Duration {
    value: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { value: s as f64 }
    }
}

pub trait Planet {
    const EARTH_YEAR_IN_SECONDS: f64 = 31557600 as f64;
    const EARTH_ORBITAL_YEAR: f64;

    fn years_during(d: &Duration) -> f64 {
        d.value / Self::EARTH_YEAR_IN_SECONDS / Self::EARTH_ORBITAL_YEAR
    }
}

macro_rules! add_planet {
    ($planet_name:ident, $earth_orbital_year:expr) => {
        pub struct $planet_name;

        impl Planet for $planet_name {
            const EARTH_ORBITAL_YEAR: f64 = $earth_orbital_year;
        }
    };
}

add_planet!(Mercury, 0.2408467);
add_planet!(Venus, 0.61519726);
add_planet!(Earth, 1 as f64);
add_planet!(Mars, 1.8808158);
add_planet!(Jupiter, 11.862615);
add_planet!(Saturn, 29.447498);
add_planet!(Uranus, 84.016846);
add_planet!(Neptune, 164.79132);
