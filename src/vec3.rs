
use std::ops;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl ops::Add for Vec3
{
    type Output = Self;
    fn add(self, other: Vec3) -> Self
    {
        Self{ x: self.x + other.x,
              y: self.y + other.y,
              z: self.z + other.z }   
    }
}

impl ops::AddAssign for Vec3
{
    fn add_assign(&mut self, other: Vec3) 
    {
        *self = Vec3 { x: self.x + other.x,
                       y: self.y + other.y,
                       z: self.z + other.z };   
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self{ x: self.x * other.x,
              y: self.y * other.y,
              z: self.z * other.z }   
    }  

}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3{ x: self * other.x,
              y: self * other.y,
              z: self * other.z }   
    }  

}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scale: f64) -> Self {
        Self{ x: self.x * scale,
              y: self.y * scale,
              z: self.z * scale }   
    }  
}

impl ops::MulAssign<f64> for Vec3
{
    fn mul_assign(&mut self, scale: f64) {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scale: f64) -> Self{
        Self{x: self.x / scale,
             y: self.y / scale,
             z: self.z / scale}
    }
}

impl ops::DivAssign<f64> for Vec3 {

    fn div_assign(&mut self, scale: f64) {
        self.x /= scale;
        self.y /= scale;
        self.z /= scale;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self{x: self.x - other.x,
             y: self.y - other.y,
             z: self.z - other.z}
   }
}


impl Vec3 {

    pub fn dot(u: Vec3, v:Vec3 ) -> f64 {
         u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: Vec3, v: Vec3 ) -> Vec3 {
        Vec3{ x: u.y * v.z - u.z * v.y,
              y: u.z * v.x - u.x * v.z,
              z: u.x * v.y - u.y * v.x
        }
   }

   pub fn unit_vector(&mut self) 
   {
        *self /= self.length()
   }

   pub fn get_unit_vector(vec: Vec3) -> Vec3
   {
     let result = vec / vec.length();
     return  result;
   }

    pub fn length_squared(self) -> f64 {
        return self.x * self.x +
               self.y * self.y +
               self.z * self.z
    }

    pub fn length(self) -> f64 {
        return self.length_squared().sqrt();
    }

}


#[test]
fn test_vec3_length()
{
    let mut a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    let mut b = Vec3{x: 4.0, y: 5.0, z: 6.0};

    //Addition
    a += b;
    assert_eq!(a, Vec3{x: 5.0, y: 7.0, z: 9.0});
    assert_eq!(Vec3{x: 1.0, y: 2.0, z: 3.0} + Vec3{x: 4.0, y: 5.0, z: 6.0}, Vec3{x: 5.0, y: 7.0, z: 9.0});

    //Multiply
    assert_eq!(Vec3{x: 1.0, y: 2.0, z: 3.0} * Vec3{x: 1.0, y: 2.0, z: 3.0}, Vec3{x: 1.0, y: 4.0, z: 9.0});
    assert_eq!(Vec3{x: 1.0, y: 2.0, z: 3.0} * 2.0, Vec3{x: 2.0, y: 4.0, z: 6.0});
    assert_eq!(2.0 * Vec3{x: 1.0, y: 2.0, z: 3.0} , Vec3{x: 2.0, y: 4.0, z: 6.0});

    a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    a *= 2.0;
    assert_eq!(a, Vec3{x: 2.0, y: 4.0, z: 6.0});



    //Substraction and Negation
    assert_eq!(-Vec3{x: 1.0, y: 2.0, z: 3.0} , Vec3{x: -1.0, y: -2.0, z: -3.0});

    assert_eq!(Vec3{x: 1.0, y: 2.0, z: 3.0} - Vec3{x: 1.0, y: 2.0, z: 3.0} , Vec3{x: 0.0, y: 0.0, z: 0.0});



    //Divide
    a = Vec3{x: 1.0, y: 2.0, z: 3.0};
    a /= 2.0;
    assert_eq!(a , Vec3{x: 0.5, y: 1.0, z: 1.5});
    assert_eq!(Vec3{x: 1.0, y: 2.0, z: 3.0} / 2.0 , Vec3{x: 0.5, y: 1.0, z: 1.5});




    //Vec 3 functions
    a = Vec3{x: 5.0, y: 10.0, z: 10.0};
    assert_eq!(a.length() , 15.0);

    a = Vec3{x: 5.0, y: 10.0, z: 10.0};
    assert_eq!(a.length_squared() , 225.0);


    a = Vec3{x: 0.0, y: 5.0, z: 0.0};
    b = Vec3{x: 5.0, y: 0.0, z: 0.0};
    assert_eq!(Vec3::dot(a, b) , 0.0);

    a = Vec3{x: 0.0, y: 0.0, z: -10.0};
    b = Vec3{x: 10.0, y: 0.0, z: 0.0};
    assert_eq!(Vec3::cross(a, b) , Vec3{x: 0.0, y: -100.0, z: 0.0});


    a = Vec3{x: 2.0, y: 2.0, z: 2.0};
    a.unit_vector();
    assert_eq!(a , Vec3{x: 1.0 / f64::sqrt(3.0), y: 1.0 / f64::sqrt(3.0), z: 1.0 / f64::sqrt(3.0)});


    a = Vec3{x: 2.0, y: 2.0, z: 2.0};
    a = Vec3::get_unit_vector(a);
    assert_eq!(a , Vec3{x: 1.0 / f64::sqrt(3.0), y: 1.0 / f64::sqrt(3.0), z: 1.0 / f64::sqrt(3.0)});
}