use ray_tracer::vec3::Vec3;

#[test]
fn test_vec3_new() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

#[test]
fn test_vec3_add() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    let c = a + b;
    assert_eq!(c.x, 5.0);
    assert_eq!(c.y, 7.0);
    assert_eq!(c.z, 9.0);
}

#[test]
fn test_vec3_sub() {
    let a = Vec3::new(5.0, 6.0, 7.0);
    let b = Vec3::new(1.0, 2.0, 3.0);
    let c = a - b;
    assert_eq!(c.x, 4.0);
    assert_eq!(c.y, 4.0);
    assert_eq!(c.z, 4.0);
}

#[test]
fn test_vec3_mul_f64() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let scaled = v * 2.0;
    assert_eq!(scaled.x, 2.0);
    assert_eq!(scaled.y, 4.0);
    assert_eq!(scaled.z, 6.0);
}

#[test]
fn test_vec3_dot() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);
    let dot = a.dot(&b);
    assert_eq!(dot, 1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.0);
}

#[test]
fn test_vec3_length_squared() {
    let v = Vec3::new(3.0, 4.0, 0.0);
    assert_eq!(v.length_squared(), 3.0 * 3.0 + 4.0 * 4.0 + 0.0 * 0.0);
}

#[test]
fn test_vec3_length() {
    let v = Vec3::new(3.0, 4.0, 0.0);
    assert_eq!(v.length(), 5.0);
}

#[test]
fn test_vec3_unit_vector() {
    let v = Vec3::new(3.0, 4.0, 0.0);
    let u = v.unit_vector();
    assert!((u.x - 0.6).abs() < 1e-10);
    assert!((u.y - 0.8).abs() < 1e-10);
    assert!((u.z - 0.0).abs() < 1e-10);
    assert!((u.length() - 1.0).abs() < 1e-10);
}

#[test]
fn test_vec3_reflect() {
    let v = Vec3::new(1.0, -1.0, 0.0); // Coming in at 45 degrees from below
    let n = Vec3::new(0.0, 1.0, 0.0); // Normal pointing up
    let r = Vec3::reflect(&v, &n);
    // Should reflect to (1, 1, 0)
    assert!((r.x - 1.0).abs() < 1e-10);
    assert!((r.y - 1.0).abs() < 1e-10);
    assert!((r.z - 0.0).abs() < 1e-10);
}

#[test]
fn test_vec3_random_in_unit_sphere() {
    let v = Vec3::random_in_unit_sphere();
    assert!(v.length_squared() < 1.0);
}

#[test]
fn test_vec3_random_in_unit_disk() {
    let v = Vec3::random_in_unit_disk();
    assert!(v.length_squared() < 1.0);
    assert_eq!(v.z, 0.0);
}

#[test]
fn test_vec3_element() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.element(0), 1.0);
    assert_eq!(v.element(1), 2.0);
    assert_eq!(v.element(2), 3.0);
    assert_eq!(v.element(3), 3.0); // Default to Z
}

#[test]
fn test_vec3_center() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.center(), v);
}
