pub const FIELD_OF_VIEW: f32 = 90. * std::f32::consts::PI / 180.; //in radians
pub const GRID_SIZE: usize = 100;
pub const Z_FAR: f32 = 100.;
pub const Z_NEAR: f32 = 0.1;
pub const Z_PLANE: f32 = -2.414213; //-1 / tan(pi/8)

//Months sice 1880
//Jan 2011: 1572; Dez 2011: 1583; MAX: 1704
pub const MAX: usize = 1572;
pub const MIN: usize = 1583;
