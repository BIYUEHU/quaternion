use quaternion_rs::{Quaternion, Vector3};

fn main() {
    let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    let q2 = Quaternion::new(5.0, 6.0, 7.0, 8.0);
    println!("q1: {}", q1.to_string());
    println!("q1 to vector: {:?}", q1.to_vector3());
    println!("q1 + q2 = {}", Quaternion::add(&q1, &q2));
    println!("q1 * q2 = {}", Quaternion::multiply(&q1, &q2));
    println!("q1 - q2 = {}", Quaternion::subtract(&q1, &q2));
    println!("q1 / q2 = {}", Quaternion::divide(&q1, &q2));
    println!("Conjugate of q1: {}", q1.conjugate());
    println!("Sign number of q1: {}", q1.sgn());
    println!("Absolute value of q1: {}", q1.abs());
    println!("Inverse of q1: {}", q1.inverse());
    println!(
        "Rotating vector [1, 0, 0] by q1: {:?}",
        q1.rotate_vector(&Vector3(1.0, 0.0, 0.0))
    );
    println!("Euler angles of q1: {:?}", q1.to_euler_angles());
}
