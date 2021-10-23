const MAX_SPEED: i32 = 9000;

fn clamp_speed(speed: i32) -> i32 {
    if speed > MAX_SPEED {
        MAX_SPEED
    } else {
        speed
    }
}

fn main() {}

#[test]
fn test_clamp_speed() {
    assert_eq!(clamp_speed(1), 1);
    assert_eq!(clamp_speed(MAX_SPEED), MAX_SPEED);
}
