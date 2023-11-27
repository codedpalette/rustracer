use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::util::{random_double, random_double_ranged};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point = Vec3;

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(a: Vec3, b: Vec3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }

    pub fn random_ranged(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_ranged(min, max),
            random_double_ranged(min, max),
            random_double_ranged(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_ranged(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().normalize()
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if Vec3::dot(on_unit_sphere, normal) > 0.0 {
            // In the same hemisphere as the normal
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    // Return true if the vector is close to zero in all dimensions.
    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(v, n) * n
    }

    pub fn refract(uv: Vec3, n: Vec3, refraction_ratio: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(-uv, n), 1.0);
        let r_out_perpendicular = refraction_ratio * (uv + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perpendicular.length_squared()).abs().sqrt() * n;
        r_out_perpendicular + r_out_parallel
    }
}

// This macro helps us implement math operators on Vector3
// in such a way that it handles binary operators on any
// combination of Vec3, &Vec3 and f64.
macro_rules! impl_binary_operations {
  // $VectorType is something like `Vec3`
  // $Operation is something like `Add`
  // $op_fn is something like `add`
  // $op_symbol is something like `+`
  ($VectorType:ident $Operation:ident $op_fn:ident $op_symbol:tt) => {
    // Implement a + b where a and b are both of type &VectorType.
    // Lower down we'll implement cases where either a or b - or both
    // - are values by forwarding through to this implementation.
    impl<'a, 'b> $Operation<&'a $VectorType> for &'b $VectorType {
      type Output = $VectorType;
      fn $op_fn(self, other: &'a $VectorType) -> $VectorType {
        $VectorType {
          x: self.x $op_symbol other.x,
          y: self.y $op_symbol other.y,
          z: self.z $op_symbol other.z,
        }
      }
    }

    // Implement a + b for the cases...
    //
    //   a: $VectorType,  b: &$VectorType
    //   a: &$VectorType, b: $VectorType
    //   a: $VectorType, b: $VectorType
    //
    // In each case we forward through to the implementation above.
    impl $Operation<$VectorType> for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: $VectorType) -> $VectorType {
        &self $op_symbol &other
      }
    }

    impl<'a> $Operation<&'a $VectorType> for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: &'a $VectorType) -> $VectorType {
        &self $op_symbol other
      }
    }

    impl<'a> $Operation<$VectorType> for &'a $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: $VectorType) -> $VectorType {
        self $op_symbol &other
      }
    }

    // Implement a + b where a is type &$VectorType and b is type f64
    impl<'a> $Operation<f64> for &'a $VectorType {
      type Output = $VectorType;

      fn $op_fn(self, other: f64) -> $VectorType {
        $VectorType {
          x: self.x $op_symbol other,
          y: self.y $op_symbol other,
          z: self.z $op_symbol other
        }
      }
    }

    // Implement a + b where...
    //
    // a is $VectorType and b is f64
    // a is f64 and b is $VectorType
    // a is f64 and b is &$VectorType
    //
    // In each case we forward the logic to the implementation
    // above.
    impl $Operation<f64> for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: f64) -> $VectorType {
        &self $op_symbol other
      }
    }

    impl $Operation<$VectorType> for f64 {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: $VectorType) -> $VectorType {
        &other $op_symbol self
      }
    }

    impl<'a> $Operation<&'a $VectorType> for f64 {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self, other: &'a $VectorType) -> $VectorType {
        other $op_symbol self
      }
    }
  };
}

// It also implements unary operators like - a where a is of
// type Vec3 or &Vec3.
macro_rules! impl_unary_operations {
  // $VectorType is something like `Vec3`
  // $Operation is something like `Neg`
  // $op_fn is something like `neg`
  // $op_symbol is something like `-`
  ($VectorType:ident $Operation:ident $op_fn:ident $op_symbol:tt) => {

    // Implement the unary operator for references
    impl<'a> $Operation for &'a $VectorType {
      type Output = $VectorType;

      fn $op_fn(self) -> Vec3 {
        $VectorType {
          x: $op_symbol self.x,
          y: $op_symbol self.y,
          z: $op_symbol self.z,
        }
      }
    }

    // Have the operator on values forward through to the implementation
    // above
    impl $Operation for $VectorType {
      type Output = $VectorType;

      #[inline]
      fn $op_fn(self) -> Vec3 {
        $op_symbol &self
      }
    }
  };
}

// Implement add-assignment operators like a += b where a and
// b is either &Vec3 or Vec3 (in this case a is always of type
// &mut Vec3).
macro_rules! impl_op_assign {
  // $VectorType is something like `Vec3`
  // $OperationAssign is something like `AddAssign`
  // $op_fn is something like `add_assign`
  // $op_symbol is something like `+=`
  ($VectorType:ident $OperationAssign:ident $op_fn:ident $op_symbol:tt) => {
    // Implement $OperationAssign for RHS &Vec3
    impl<'a> $OperationAssign<&'a $VectorType> for $VectorType {
      fn $op_fn(&mut self, other: &'a $VectorType) {
        *self = $VectorType {
          x: self.x $op_symbol other.x,
          y: self.y $op_symbol other.y,
          z: self.z $op_symbol other.z,
        };
      }
    }

    // Implement $OperationAssign for RHS Vec3 by forwarding through to the
    // implementation above
    impl $OperationAssign for $VectorType {
      #[inline]
      fn $op_fn(&mut self, other: $VectorType) {
        *self = *self $op_symbol &other
      }
    }
  };
}

impl_binary_operations!(Vec3 Add add +);
impl_op_assign!(Vec3 AddAssign add_assign +);

impl_binary_operations!(Vec3 Sub sub -);
impl_op_assign!(Vec3 SubAssign sub_assign -);
impl_unary_operations!(Vec3 Neg neg -);

impl_binary_operations!(Vec3 Mul mul *);
impl_op_assign!(Vec3 MulAssign mul_assign *);

impl_binary_operations!(Vec3 Div div /);
impl_op_assign!(Vec3 DivAssign div_assign /);
