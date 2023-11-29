use rand::{
    distributions::{Distribution, Uniform},
    rngs::StdRng,
    Rng, RngCore, SeedableRng,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rune {
    El,
    Eld,
    Tir,
    Nef,
    Eth,
    Ith,
    Tal,
    Ral,
    Ort,
    Thul,
    Amn,
    Sol,
    Shael,
    Dol,
    Hel,
    Io,
    Lum,
    Ko,
    Fal,
    Lem,
    Pul,
    Um,
    Mal,
    Ist,
    Gul,
    Vex,
    Ohm,
    Lo,
    Sur,
    Ber,
    Jah,
    Cham,
    Zod,
}

impl std::ops::Deref for Rune {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl Rune {
    pub fn next(self) -> Option<Self> {
        match self {
            Rune::Zod => None,
            _ => Some(unsafe { std::mem::transmute(std::mem::transmute::<_, u8>(self) + 1) }),
        }
    }
}

pub struct OneDropGenerator {
    rng: StdRng,
    drop_rate: f32,
    treasure_rate_upgrade_chance: f32,
}

impl OneDropGenerator {
    pub fn new(seed: u64, drop_rate: f32, treasure_rate_upgrade_chance: f32) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
            drop_rate,
            treasure_rate_upgrade_chance,
        }
    }

    pub fn generate(&mut self) -> Option<Rune> {
        let uniform_0_1 = Uniform::new(0.0f32, 1.0f32);
        if uniform_0_1.sample(&mut self.rng) > self.drop_rate {
            return None;
        }
        let mut rune_max = Rune::Ral;
        while rune_max != Rune::Zod {
            if uniform_0_1.sample(&mut self.rng) > self.treasure_rate_upgrade_chance {
                return Some(rune_max);
            } else {
                rune_max = rune_max.next().unwrap()
            }
        }
        let uniform = &Uniform::new(*Rune::El, *rune_max + 1);
        let rune = uniform
            .sample(&mut self.rng)
            .min(uniform.sample(&mut self.rng));
        Some(unsafe { std::mem::transmute(rune) })
    }
}

#[test]
fn play() {
    let mut generator = OneDropGenerator {
        rng: StdRng::seed_from_u64(2151232),
        drop_rate: 1.0,
        treasure_rate_upgrade_chance: 0.75,
    };
    for _ in 0..10000 {
        if let Some(rune) = generator.generate() {
            if rune >= Rune::Lem {
                println!("Here is rune: {:?}", rune)
            }
        }
    }
}
