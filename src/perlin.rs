use rand::rng;
use rand::seq::SliceRandom;

pub struct Perlin {
    perm: [u8; 512],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = rng();
        let mut p: Vec<u8> = (0..=255).collect();
        p.shuffle(&mut rng);

        let mut perm = [0u8; 512];
        for i in 0..512 {
            perm[i] = p[i & 255];
        }

        Self { perm }
    }

    fn fade(t: f64) -> f64 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    fn lerp(t: f64, a: f64, b: f64) -> f64 {
        a + t * (b - a)
    }

    fn grad(hash: u8, x: f64, y: f64, z: f64) -> f64 {
        let h = hash & 15;
        let u = if h < 8 { x } else { y };
        let v = if h < 4 {
            y
        } else if h == 12 || h == 14 {
            x
        } else {
            z
        };
        (if (h & 1) == 0 { u } else { -u }) + (if (h & 2) == 0 { v } else { -v })
    }

    pub fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        let xi = x.floor() as i32 & 255;
        let yi = y.floor() as i32 & 255;
        let zi = z.floor() as i32 & 255;

        let xf = x - x.floor();
        let yf = y - y.floor();
        let zf = z - z.floor();

        let u = Self::fade(xf);
        let v = Self::fade(yf);
        let w = Self::fade(zf);

        let p = &self.perm;

        let a = p[xi as usize] as usize + yi as usize;
        let aa = p[a] as usize + zi as usize;
        let ab = p[a + 1] as usize + zi as usize;
        let b = p[xi as usize + 1] as usize + yi as usize;
        let ba = p[b] as usize + zi as usize;
        let bb = p[b + 1] as usize + zi as usize;

        Self::lerp(
            w,
            Self::lerp(
                v,
                Self::lerp(
                    u,
                    Self::grad(p[aa], xf, yf, zf),
                    Self::grad(p[ba], xf - 1.0, yf, zf),
                ),
                Self::lerp(
                    u,
                    Self::grad(p[ab], xf, yf - 1.0, zf),
                    Self::grad(p[bb], xf - 1.0, yf - 1.0, zf),
                ),
            ),
            Self::lerp(
                v,
                Self::lerp(
                    u,
                    Self::grad(p[aa + 1], xf, yf, zf - 1.0),
                    Self::grad(p[ba + 1], xf - 1.0, yf, zf - 1.0),
                ),
                Self::lerp(
                    u,
                    Self::grad(p[ab + 1], xf, yf - 1.0, zf - 1.0),
                    Self::grad(p[bb + 1], xf - 1.0, yf - 1.0, zf - 1.0),
                ),
            ),
        )
    }
}
