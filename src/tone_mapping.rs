use crate::vec3::Vec3;

pub fn aces(color: Vec3) -> Vec3 {
    let c = color * 0.6;
    let x = c.x;
    let y = c.y;
    let z = c.z;
    let a = x * (x * 2.51 + 0.03);
    let b = y * (y * 2.51 + 0.03);
    let c2 = z * (z * 2.51 + 0.03);
    Vec3::new(
        a / (x * (x * 2.43 + 0.59) + 0.14).max(0.0001),
        b / (y * (y * 2.43 + 0.59) + 0.14).max(0.0001),
        c2 / (z * (z * 2.43 + 0.59) + 0.14).max(0.0001),
    )
}

pub fn srgb_encode(c: f64) -> u8 {
    let c = c.clamp(0.0, 0.999);
    if c <= 0.0031308 {
        (c * 12.92 * 255.0 + 0.5) as u8
    } else {
        ((1.055 * c.powf(1.0 / 2.4) - 0.055) * 255.0 + 0.5) as u8
    }
}

pub fn write_pixel(color: Vec3) -> [u8; 3] {
    let r = srgb_encode(color.x);
    let g = srgb_encode(color.y);
    let b = srgb_encode(color.z);
    [r, g, b]
}
