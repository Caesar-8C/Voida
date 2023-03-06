use crate::world::celestials::Celestials;
use crate::{Celestial, Vec3};
use crate::world::spaceship::Spaceship;

pub fn sun() -> Celestial {
    let sun_name = "Sun".to_string();
    let sun_mass = 1.989110_f64 * 10_f64.powi(30);
    let sun_pos = Vec3::default();
    let sun_vel = Vec3::default();
    let sun_rad = 6.9634_f64 * 10_f64.powi(8);

    Celestial::new(
        sun_name,
        sun_mass,
        sun_pos,
        sun_vel,
        sun_rad,
    )
}

pub fn earth() -> Celestial {
    let sun = sun();

    let earth_name = "Earth".to_string();
    let earth_mass = 5.972_f64 * 10_f64.powi(24);
    let earth_pos = &sun.pos() + Vec3 { x: 1.521_f64 * 10_f64.powi(11), y: 0., z: 0., };
    let earth_vel = &sun.vel() + Vec3 { x: 0., y: 29290., z: 0., };
    let earth_rad = 6.371_f64 * 10_f64.powi(6);

    Celestial::new(
        earth_name,
        earth_mass,
        earth_pos,
        earth_vel,
        earth_rad,
    )
}

pub fn moon() -> Celestial {
    let earth = earth();

    let moon_name = "Moon".to_string();
    let moon_mass = 7.34767309_f64 * 10_f64.powi(22);
    let moon_pos = &earth.pos()
        + Vec3 {
        x: 4.037634453_f64 * 10_f64.powi(8),
        y: 0.,
        z: 3.63901118372_f64 * 10_f64.powi(7),
    };
    let moon_vel = &earth.vel() + Vec3 { x: 0., y: 970., z: 0. };
    let moon_rad = 1.740_f64 * 10_f64.powi(6);

    Celestial::new(
        moon_name,
        moon_mass,
        moon_pos,
        moon_vel,
        moon_rad,
    )
}

pub fn new_solar() -> Celestials {
    let mut celestials = Celestials::new();
    celestials.add(sun());
    celestials.add(earth());
    celestials.add(moon());

    celestials
}

pub fn iss() -> Spaceship {
    let earth = earth();

    let iss_name = "ISS".to_string();
    let iss_mass = 4.19725_f64 * 10_f64.powi(5);
    let iss_pos = &earth.pos() + Vec3 { x: 422_000.+earth.rad(), y: 0., z: 0. };
    let iss_vel = &earth.vel() + Vec3 { x: 0., y: 7660., z: 0. };

    Spaceship::new(
        iss_name,
        iss_mass,
        iss_pos,
        iss_vel,
    )
}
