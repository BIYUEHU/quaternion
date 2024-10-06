use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub struct Vector3(pub f64, pub f64, pub f64);

#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    pub w: f64,
    pub i: f64,
    pub j: f64,
    pub k: f64,
}

impl Default for Quaternion {
    fn default() -> Self {
        Quaternion {
            w: 1.0,
            i: 0.0,
            j: 0.0,
            k: 0.0,
        }
    }
}

impl Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Quaternion {
    pub fn from_axis_angle(axis: &Vector3, angle: f64) -> Self {
        let half_angle = angle / 2.0;
        let sin_half_angle = half_angle.sin();
        Quaternion {
            w: half_angle.cos(),
            i: axis.0 * sin_half_angle,
            j: axis.1 * sin_half_angle,
            k: axis.2 * sin_half_angle,
        }
    }

    pub fn add(q1: &Quaternion, q2: &Quaternion) -> Self {
        Quaternion::new(q1.w + q2.w, q1.i + q2.i, q1.j + q2.j, q1.k + q2.k)
    }

    pub fn subtract(q1: &Quaternion, q2: &Quaternion) -> Self {
        Quaternion::new(q1.w - q2.w, q1.i - q2.i, q1.j - q2.j, q1.k - q2.k)
    }

    pub fn multiply(q1: &Quaternion, q2: &Quaternion) -> Self {
        Quaternion::new(
            q1.w * q2.w - q1.i * q2.i - q1.j * q2.j - q1.k * q2.k,
            q1.w * q2.i + q1.i * q2.w + q1.j * q2.k - q1.k * q2.j,
            q1.w * q2.j - q1.i * q2.k + q1.j * q2.w + q1.k * q2.i,
            q1.w * q2.k + q1.i * q2.j - q1.j * q2.i + q1.k * q2.w,
        )
    }

    pub fn divide(q1: &Quaternion, q2: &Quaternion) -> Self {
        Quaternion::multiply(q1, &q2.conjugate())
    }

    pub fn scalar_product(q1: &Quaternion, q2: &Quaternion) -> f64 {
        q1.w * q2.w + q1.i * q2.i + q1.j * q2.j + q1.k * q2.k
    }

    pub fn outer_product(q1: &Quaternion, q2: &Quaternion) -> Self {
        Quaternion::new(
            q1.w * q2.w - q1.i * q2.i - q1.j * q2.j - q1.k * q2.k,
            q1.w * q2.i + q1.i * q2.w + q1.j * q2.k - q1.k * q2.j,
            q1.w * q2.j - q1.i * q2.k + q1.j * q2.w + q1.k * q2.i,
            q1.w * q2.k + q1.i * q2.j - q1.j * q2.i + q1.k * q2.w,
        )
    }

    pub fn even_product(q1: &Quaternion, q2: &Quaternion) -> Self {
        Quaternion::new(
            q1.w * q2.w - q1.i * q2.i - q1.j * q2.j - q1.k * q2.k,
            q1.w * q2.i + q1.i * q2.w - q1.j * q2.k + q1.k * q2.j,
            q1.w * q2.j + q1.i * q2.k + q1.j * q2.w - q1.k * q2.i,
            q1.w * q2.k - q1.i * q2.j + q1.j * q2.i + q1.k * q2.w,
        )
    }

    pub fn cross_product(q1: &Quaternion, q2: &Quaternion) -> Self {
        Quaternion::new(
            q1.w * q2.w - q1.i * q2.i - q1.j * q2.j - q1.k * q2.k,
            q1.w * q2.i + q1.i * q2.w + q1.j * q2.k - q1.k * q2.j,
            q1.w * q2.j - q1.i * q2.k + q1.j * q2.w + q1.k * q2.i,
            q1.w * q2.k + q1.i * q2.j - q1.j * q2.i + q1.k * q2.w,
        )
    }

    pub fn new(w: f64, i: f64, j: f64, k: f64) -> Self {
        Quaternion {
            w,
            i,
            j,
            k,
            ..Default::default()
        }
    }

    pub fn rotate_vector(&self, v: &Vector3) -> Vector3 {
        let vq = Quaternion::new(0.0, v.0, v.1, v.2);
        let result = Quaternion::multiply(self, &Quaternion::multiply(&vq, self));
        Vector3(result.i, result.j, result.k)
    }

    pub fn multiply_vector(&self, v: &Vector3) -> Vector3 {
        let vq = Quaternion::new(0.0, v.0, v.1, v.2);
        let result = Quaternion::multiply(self, &vq);
        Vector3(result.i, result.j, result.k)
    }

    pub fn to_euler_angles(&self) -> (f64, f64, f64) {
        let roll = f64::atan2(
            2.0 * (self.w * self.i + self.j * self.k),
            1.0 - 2.0 * (self.i * self.i + self.j * self.j),
        );
        let pitch = f64::asin(2.0 * (self.w * self.j - self.k * self.i));
        let yaw = f64::atan2(
            2.0 * (self.w * self.k + self.i * self.j),
            1.0 - 2.0 * (self.j * self.j + self.k * self.k),
        );
        (roll, pitch, yaw)
    }

    pub fn to_vector3(&self) -> Vector3 {
        Vector3(self.i, self.j, self.k)
    }

    pub fn to_string(&self) -> String {
        let get_sign = |num: f64| if num >= 0.0 { "+" } else { "-" };
        format!(
            "{}{}{}i{}{}j{}{}k",
            self.w,
            get_sign(self.i),
            self.i.abs(),
            get_sign(self.j),
            self.j.abs(),
            get_sign(self.k),
            self.k.abs()
        )
    }

    pub fn conjugate(&self) -> Self {
        Quaternion::new(self.w, -self.i, -self.j, -self.k)
    }

    pub fn abs(&self) -> f64 {
        (self.w * self.w + self.i * self.i + self.j * self.j + self.k * self.k).sqrt()
    }

    pub fn sgn(&self) -> Self {
        let abs = self.abs();
        Quaternion::new(self.w / abs, self.i / abs, self.j / abs, self.k / abs)
    }

    pub fn inverse(&self) -> Self {
        let abs2 = self.abs().powf(2.0);
        Quaternion::new(
            self.w / abs2,
            -self.i / abs2,
            -self.j / abs2,
            -self.k / abs2,
        )
    }

    pub fn arg(&self) -> f64 {
        (self.w / self.abs()).acos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_quaternion() {
        let q = Quaternion::default();
        assert_eq!(q.w, 1.0);
        assert_eq!(q.i, 0.0);
        assert_eq!(q.j, 0.0);
        assert_eq!(q.k, 0.0);
    }

    #[test]
    fn test_from_axis_angle() {
        let axis = Vector3(1.0, 0.0, 0.0); // Rotation around x-axis
        let angle = std::f64::consts::PI; // 180 degrees
        let q = Quaternion::from_axis_angle(&axis, angle);
        assert!((q.w - 0.0).abs() < 1e-6); // w should be 0
        assert!((q.i - 1.0).abs() < 1e-6); // i should be 1
    }

    #[test]
    fn test_addition() {
        let q1 = Quaternion::new(1.0, 1.0, 1.0, 1.0);
        let q2 = Quaternion::new(2.0, 2.0, 2.0, 2.0);
        let result = Quaternion::add(&q1, &q2);
        assert_eq!(result.w, 3.0);
        assert_eq!(result.i, 3.0);
        assert_eq!(result.j, 3.0);
        assert_eq!(result.k, 3.0);
    }

    #[test]
    fn test_subtraction() {
        let q1 = Quaternion::new(2.0, 2.0, 2.0, 2.0);
        let q2 = Quaternion::new(1.0, 1.0, 1.0, 1.0);
        let result = Quaternion::subtract(&q1, &q2);
        assert_eq!(result.w, 1.0);
        assert_eq!(result.i, 1.0);
        assert_eq!(result.j, 1.0);
        assert_eq!(result.k, 1.0);
    }

    #[test]
    fn test_conjugate() {
        let q = Quaternion::new(1.0, -2.0, 3.0, -4.0);
        let conj = q.conjugate();
        assert_eq!(conj.w, 1.0);
        assert_eq!(conj.i, 2.0);
        assert_eq!(conj.j, -3.0);
        assert_eq!(conj.k, 4.0);
    }
}
