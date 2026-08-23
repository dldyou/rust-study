use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    min: Vec2,
    max: Vec2,
}

impl Bounds {
    pub const fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    fn contains(&self, position: Vec2) -> bool {
        position.x >= self.min.x
            && position.x <= self.max.x
            && position.y >= self.min.y
            && position.y <= self.max.y
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bullet {
    position: Vec2,
    angle: f32,
    speed: f32,
}

impl Bullet {
    pub const fn new(position: Vec2, angle: f32, speed: f32) -> Self {
        Self {
            position,
            angle,
            speed,
        }
    }

    pub const fn position(&self) -> Vec2 {
        self.position
    }
}

pub trait Pattern {
    fn update(&mut self, dt: f32);
    fn emit(&mut self, origin: Vec2) -> Vec<Bullet>;
}

pub struct IntervalMultipleSpiral {
    angle: f32,
    angle_step: f32,
    speed: f32,
    shoot_count: i32,
    interval: i32,
    time: i32,
}

impl IntervalMultipleSpiral {
    pub fn new(
        start_angle: f32,
        angle_step: f32,
        speed: f32,
        shoot_count: i32,
        interval: i32,
    ) -> Self {
        Self {
            angle: start_angle,
            angle_step,
            speed,
            shoot_count,
            interval,
            time: 0,
        }
    }
}

impl Pattern for IntervalMultipleSpiral {
    fn update(&mut self, dt: f32) {
        self.angle = (self.angle + self.angle_step * dt).rem_euclid(360.0);
    }
    fn emit(&mut self, origin: Vec2) -> Vec<Bullet> {
        let mut bullets = Vec::<Bullet>::new();
        if self.time == 0 {
            for i in 0..self.shoot_count {
                let bullet = Bullet::new(
                    origin,
                    self.angle + (i as f32 / self.shoot_count as f32) * 360.0,
                    self.speed,
                );
                bullets.push(bullet);
            }
            self.time = (self.time + 1) % self.interval;
        }
        bullets
    }
}

pub struct Spawner {
    pattern: Box<dyn Pattern>,
    interval: f32,
    elapsed: f32,
    origin: Vec2,
}

impl Spawner {
    pub fn new(pattern: Box<dyn Pattern>, interval: f32, origin: Vec2) -> Self {
        assert!(interval > 0.0, "spawner interval must be greater than zero");

        Self {
            pattern,
            interval,
            elapsed: 0.0,
            origin,
        }
    }

    pub fn tick(&mut self, dt: f32, world: &mut World) {
        let mut remaining = dt.max(0.0);

        while remaining > 0.0 {
            let until_spawn = self.interval - self.elapsed;
            let step = remaining.min(until_spawn);

            self.pattern.update(step);
            self.elapsed += step;
            remaining -= step;

            if self.elapsed >= self.interval {
                self.elapsed = 0.0;

                for bullet in self.pattern.emit(self.origin) {
                    world.add_bullet(bullet);
                }
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct World {
    bullets: Vec<Bullet>,
}

fn velocity(angle: f32, speed: f32) -> Vec2 {
    let radians = angle.to_radians();

    Vec2 {
        x: radians.cos() * speed,
        y: -radians.sin() * speed,
    }
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_bullet(&mut self, bullet: Bullet) {
        self.bullets.push(bullet);
    }

    pub fn tick(&mut self, dt: f32) {
        for bullet in &mut self.bullets {
            bullet.position += velocity(bullet.angle, bullet.speed) * dt;
        }
    }

    pub fn cull(&mut self, bounds: Bounds) {
        self.bullets
            .retain(|bullet| bounds.contains(bullet.position));
    }

    pub fn bullets(&self) -> &[Bullet] {
        &self.bullets
    }
}

#[cfg(test)]
mod test_utils {
    use super::*;

    // approx crate에 assert_relative_eq!가 존재함. 허용오차 epsilon 지정 가능
    pub fn float_eq(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 1e-6,
            "expected {expected}, got {actual}"
        );
    }

    pub fn vec2_eq(actual: Vec2, expected: Vec2) {
        float_eq(actual.x, expected.x);
        float_eq(actual.y, expected.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod vec2 {
        use super::*;

        #[test]
        fn add() {
            test_utils::vec2_eq(
                Vec2::new(10.0, 20.0) + Vec2::new(20.0, 25.0),
                Vec2::new(30.0, 45.0),
            );
        }

        #[test]
        fn minus() {
            test_utils::vec2_eq(
                Vec2::new(10.0, 20.0) - Vec2::new(20.0, 25.0),
                Vec2::new(-10.0, -5.0),
            );
        }
    }
}
