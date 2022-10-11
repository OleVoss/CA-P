use bevy::math::Vec3;
use fraction::Fraction;
use rand::Rng;

pub fn generate_noise_universe(noise: f32, dimensions: Vec3) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let fraction = Fraction::from(noise);

    let universe_length =
        (dimensions.x.max(1.) * dimensions.y.max(1.) * dimensions.z.max(1.)) as usize;
    let mut universe = Vec::<u8>::new();

    (0..universe_length).for_each(|_| {
        if rng.gen_ratio(
            *fraction.numer().unwrap() as u32,
            *fraction.denom().unwrap() as u32,
        ) {
            universe.push(1);
        } else {
            universe.push(0);
        }
    });

    universe
}
