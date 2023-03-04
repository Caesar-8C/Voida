use crate::world::celestials::Celestials;
use crate::{Celestial, Vec3};

pub fn new_solar() -> Celestials {
    let sun_name = "Sun".to_string();
    let sun_mass = 1.989110_f64 * 10_f64.powi(30);
    let sun_pos = Vec3::default();
    let sun_vel = Vec3::default();

    let earth_name = "Earth".to_string();
    let earth_mass = 5.972_f64 * 10_f64.powi(24);
    let earth_pos = &sun_pos + Vec3 { x: 1.521_f64 * 10_f64.powi(11), y: 0., z: 0., };
    let earth_vel = &sun_vel + Vec3 { x: 0., y: 29290., z: 0., };

    let moon_name = "Moon".to_string();
    let moon_mass = 7.34767309_f64 * 10_f64.powi(22);
    let moon_pos = &earth_pos
        + Vec3 {
        x: 4.037634453_f64 * 10_f64.powi(8),
        y: 0.,
        z: 3.63901118372_f64 * 10_f64.powi(7),
    };
    let moon_vel = &earth_vel + Vec3 { x: 0., y: 970., z: 0. };

    let sun = Celestial::new(
        sun_name,
        sun_mass,
        sun_pos,
        sun_vel,
    );

    let earth = Celestial::new(
        earth_name,
        earth_mass,
        earth_pos,
        earth_vel,
    );

    let moon = Celestial::new(
        moon_name,
        moon_mass,
        moon_pos,
        moon_vel,
    );

    let mut celestials = Celestials::new();
    celestials.add(sun);
    celestials.add(earth);
    celestials.add(moon);

    celestials
}
