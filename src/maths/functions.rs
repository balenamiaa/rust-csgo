use super::vector::Vector;
use std::ops::{Div, Mul};

pub fn calc_angle(src: Vector, target: Vector) -> Vector {
    use std::f32::consts::PI;
    let delta = target - src;

    *Vector::new(
        -delta.z.atan2(delta.nullify_z().len()),
        {
            let yaw = delta.y.atan2(delta.x);
            if delta.y < 0e0 {
                yaw + 2e0 * PI
            } else {
                yaw
            }
        },
        0e0,
    )
    .normalize_yaw_cs()
    .to_degrees()
}

pub fn get_fov<Tx: Into<f32> + Copy>(src: Vector, target: Vector, r: Tx) -> f32 {
    let mut delta = target - src;
    delta.normalize_yaw_cs();

    Vector::new(
        delta.x.to_radians().abs().sin().mul(r.into()),
        delta.y.to_radians().abs().sin().mul(r.into()),
        0e0,
    )
    .len()
}

pub fn compensate_velocity<Tx: Into<f32> + Copy>(
    src: Vector,
    rel_velocity: Vector,
    r: Tx,
) -> Vector {
    src + rel_velocity / r.into()
}

pub fn smooth_angle<Tx: Into<f32> + Copy>(
    src: Vector,
    target: Vector,
    smooth_percentage: Tx,
) -> Vector {
    let mut delta = target - src;
    delta.normalize_yaw_cs();

    src + delta * smooth_percentage.into()
}
