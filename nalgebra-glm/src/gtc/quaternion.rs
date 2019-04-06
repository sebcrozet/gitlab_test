use na::{RealField, UnitQuaternion, U4};

use crate::aliases::{Qua, TMat4, TVec, TVec3};

/// Euler angles of the quaternion `q` as (pitch, yaw, roll).
pub fn quat_euler_angles<N: RealField>(x: &Qua<N>) -> TVec3<N> {
    let q = UnitQuaternion::new_unchecked(*x);
    let a = q.euler_angles();
    TVec3::new(a.2, a.1, a.0)
}

/// Component-wise `>` comparison between two quaternions.
pub fn quat_greater_than<N: RealField>(x: &Qua<N>, y: &Qua<N>) -> TVec<bool, U4> {
    crate::greater_than(&x.coords, &y.coords)
}

/// Component-wise `>=` comparison between two quaternions.
pub fn quat_greater_than_equal<N: RealField>(x: &Qua<N>, y: &Qua<N>) -> TVec<bool, U4> {
    crate::greater_than_equal(&x.coords, &y.coords)
}

/// Component-wise `<` comparison between two quaternions.
pub fn quat_less_than<N: RealField>(x: &Qua<N>, y: &Qua<N>) -> TVec<bool, U4> {
    crate::less_than(&x.coords, &y.coords)
}

/// Component-wise `<=` comparison between two quaternions.
pub fn quat_less_than_equal<N: RealField>(x: &Qua<N>, y: &Qua<N>) -> TVec<bool, U4> {
    crate::less_than_equal(&x.coords, &y.coords)
}

/// Convert a quaternion to a rotation matrix in homogeneous coordinates.
pub fn quat_cast<N: RealField>(x: &Qua<N>) -> TMat4<N> {
    crate::quat_to_mat4(x)
}

/// Computes a right hand look-at quaternion
///
/// # Parameters
///
/// * `direction` - Direction vector point at where to look
/// * `up` - Object up vector
///
pub fn quat_look_at<N: RealField>(direction: &TVec3<N>, up: &TVec3<N>) -> Qua<N> {
    quat_look_at_rh(direction, up)
}

/// Computes a left-handed look-at quaternion (equivalent to a left-handed look-at matrix).
pub fn quat_look_at_lh<N: RealField>(direction: &TVec3<N>, up: &TVec3<N>) -> Qua<N> {
    UnitQuaternion::look_at_lh(direction, up).into_inner()
}

/// Computes a right-handed look-at quaternion (equivalent to a right-handed look-at matrix).
pub fn quat_look_at_rh<N: RealField>(direction: &TVec3<N>, up: &TVec3<N>) -> Qua<N> {
    UnitQuaternion::look_at_rh(direction, up).into_inner()
}

/// The "roll" Euler angle of the quaternion `x` assumed to be normalized.
pub fn quat_roll<N: RealField>(x: &Qua<N>) -> N {
    // FIXME: optimize this.
    quat_euler_angles(x).z
}

/// The "yaw" Euler angle of the quaternion `x` assumed to be normalized.
pub fn quat_yaw<N: RealField>(x: &Qua<N>) -> N {
    // FIXME: optimize this.
    quat_euler_angles(x).y
}

/// The "pitch" Euler angle of the quaternion `x` assumed to be normalized.
pub fn quat_pitch<N: RealField>(x: &Qua<N>) -> N {
    // FIXME: optimize this.
    quat_euler_angles(x).x
}
