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
    ratio: f32,
}

impl OneDropGenerator {
    pub fn generate(&mut self) -> Option<Rune> {
        let uniform = Uniform::new(0.0f32, 1.0f32);
        if uniform.sample(&mut self.rng) > self.drop_rate {
            return None;
        }
        let mut rune = Rune::El;
        while rune != Rune::Zod {
            if uniform.sample(&mut self.rng) > self.ratio {
                return Some(rune);
            } else {
                rune = rune.next().unwrap()
            }
        }
        Some(rune)
    }
}

#[test]
fn play() {
    let mut generator = OneDropGenerator {
        rng: StdRng::seed_from_u64(2151232),
        drop_rate: 1.0,
        ratio: 0.75,
    };
    for _ in 0..10000 {
        if let Some(rune) = generator.generate() {
            if rune >= Rune::Lem {
                println!("Here is rune: {:?}", rune)
            }
        }
    }
}
