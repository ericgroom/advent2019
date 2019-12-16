pub fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x.abs();
    let mut y = y.abs();
    if x == 0 || y == 0 {
        return 0;
    }
    while x != y {
        if x > y {
            x = x - y;
        } else {
            y = y - x;
        }
    }

    x
}

pub fn lcm(x: i64, y: i64) -> i64 {
    let gcd = gcd(x, y);
    if gcd <= 0 {
        0
    } else {
        x / gcd * y
    }
}

pub fn digits_of(integer: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut temp = integer;
    while temp > 0 {
        let last_digit = temp % 10;
        result.push(last_digit);
        temp /= 10;
    }
    result.reverse();
    result
}
