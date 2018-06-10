#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec2(pub f64, pub f64);

impl Vec2 {
    pub fn dot(&self, rhs: Vec2) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    pub fn abs(&self) -> Vec2 {
        Vec2(self.0.abs(), self.1.abs())
    }

    pub fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }
}

use std::ops;

impl ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl ops::Div for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2(self.0 / rhs.0, self.1 / rhs.1)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec4(pub f64, pub f64, pub f64, pub f64);

impl Vec4 {
    pub fn dot(&self, rhs: Vec4) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2 + self.3 * rhs.3
    }

    pub fn abs(&self) -> Vec4 {
        Vec4(self.0.abs(), self.1.abs(), self.2.abs(), self.3.abs())
    }

    pub fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub fn to_argb(&self) -> u32 {
        let r = (self.0 * 255.0).max(0.0).min(255.0) as u32;
        let g = (self.1 * 255.0).max(0.0).min(255.0) as u32;
        let b = (self.2 * 255.0).max(0.0).min(255.0) as u32;
        let a = (self.3 * 255.0).max(0.0).min(255.0) as u32;
        a << 24 | r << 16 | g << 8 | b
    }
}

impl ops::Add for Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Vec4) -> Vec4 {
        Vec4(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl ops::Sub for Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: Vec4) -> Vec4 {
        Vec4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl ops::Mul for Vec4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
            self.3 * rhs.3,
        )
    }
}

impl ops::Div for Vec4 {
    type Output = Vec4;
    fn div(self, rhs: Vec4) -> Vec4 {
        Vec4(
            self.0 / rhs.0,
            self.1 / rhs.1,
            self.2 / rhs.2,
            self.3 / rhs.3,
        )
    }
}

impl ops::Add<f64> for Vec4 {
    type Output = Vec4;
    fn add(self, rhs: f64) -> Vec4 {
        Vec4(
            self.0 + rhs,
            self.1 + rhs,
            self.2 + rhs,
            self.3 + rhs,
        )
    }
}

impl ops::Sub<f64> for Vec4 {
    type Output = Vec4;
    fn sub(self, rhs: f64) -> Vec4 {
        Vec4(
            self.0 - rhs,
            self.1 - rhs,
            self.2 - rhs,
            self.3 - rhs,
        )
    }
}

impl ops::Mul<f64> for Vec4 {
    type Output = Vec4;
    fn mul(self, rhs: f64) -> Vec4 {
        Vec4(
            self.0 * rhs,
            self.1 * rhs,
            self.2 * rhs,
            self.3 * rhs,
        )
    }
}

impl ops::Div<f64> for Vec4 {
    type Output = Vec4;
    fn div(self, rhs: f64) -> Vec4 {
        Vec4(
            self.0 / rhs,
            self.1 / rhs,
            self.2 / rhs,
            self.3 / rhs,
        )
    }
}

#[derive(Clone)]
pub struct Texture<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Texture<T> {
    pub fn new(data: impl Into<Vec<T>>, width: usize, height: usize) -> Texture<T> {
        let data = data.into();
        assert_eq!(data.len(), width * height);
        Texture {
            data,
            width,
            height,
        }
    }

    pub fn sample(&self, uv: Vec2) -> T {
        let Vec2(mut x, mut y) = uv;
        if x.is_nan() || x.is_infinite() {
            x = 0.0;
        }
        if y.is_nan() || y.is_infinite() {
            y = 0.0;
        }
        let x = ((x % 1.0) + 1.0) % 1.0;
        let y = ((y % 1.0) + 1.0) % 1.0;
        assert!(x >= 0.0 && x < 1.0);
        assert!(y >= 0.0 && y < 1.0);
        let mut x = (x * self.width as f64) as usize;
        let mut y = (y * self.height as f64) as usize;
        // Could possibly happen for very big widths/heights
        if x >= self.width {
            x = self.width - 1;
        }
        if y >= self.height {
            y = self.height - 1;
        }
        self.data[x + y * self.width].clone()
    }
}
