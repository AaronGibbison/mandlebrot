use crate::MAX_ITERATIONS;

pub fn escape_time(y0: f32, x0: f32) -> u32 {
    let mut iterations = 0;

    let mut x = 0.0;
    let mut y = 0.0;
    let mut x2 = 0.0;
    let mut y2 = 0.0;
    while x2 + y2 <= 4.0 && iterations < MAX_ITERATIONS {
        y = 2.0 * x * y + y0;
        x = x2 - y2 + x0;
        x2 = x * x;
        y2 = y * y;

        iterations += 1;
    }

    return iterations;
}

pub fn escape_time_with_bulb(y0: f32, x0: f32) -> u32 {
    let q = (x0 - 0.25) * (x0 - 0.25) + (y0 * y0);
    let mut iterations = if q * (q + (x0 - 0.25)) <= 0.25 * y0 * y0 { MAX_ITERATIONS } else { 0 };

    let mut x = 0.0;
    let mut y = 0.0;
    let mut x2 = 0.0;
    let mut y2 = 0.0;
    while x2 + y2 <= 4.0 && iterations < MAX_ITERATIONS {
        y = 2.0 * x * y + y0;
        x = x2 - y2 + x0;
        x2 = x * x;
        y2 = y * y;

        iterations += 1;
    }

    return iterations;
}

pub fn escape_time_with_period(y0: f32, x0: f32) -> u32 {
    let mut iterations = 0;

    let mut period = 0;
    let mut xold = 0.0;
    let mut yold = 0.0;

    let mut x = 0.0;
    let mut y = 0.0;
    let mut x2 = 0.0;
    let mut y2 = 0.0;
    while x2 + y2 <= 4.0 && iterations < MAX_ITERATIONS {
        y = 2.0 * x * y + y0;
        x = x2 - y2 + x0;
        x2 = x * x;
        y2 = y * y;

        iterations += 1;
        period += 1;

        if x == xold && y == yold {
            iterations = MAX_ITERATIONS;
        } else if period == 60 {
            period = 0;
            xold = x;
            yold = y;
        }
    }

    return iterations;
}

pub fn escape_time_with_bulb_period(l_set: f32, r_set: f32) -> u32 {
    let q = (r_set - 0.25) * (r_set - 0.25) + (l_set * l_set);
    let mut iterations = if q * (q + (r_set - 0.25)) <= 0.25 * l_set * l_set { MAX_ITERATIONS } else { 0 };

    let mut period = 0;
    let mut r_old = 0.0;
    let mut l_old = 0.0;

    let mut r = 0.0;
    let mut l = 0.0;
    let mut r2 = 0.0;
    let mut l2 = 0.0;

    while r2 + l2 <= 4.0 && iterations < MAX_ITERATIONS {
        l = 2.0 * r * l + l_set;
        r = r2 - l2 + r_set;
        r2 = r * r;
        l2 = l * l;

        iterations += 1;
        period += 1;

        if r == r_old && l == l_old {
            iterations = MAX_ITERATIONS;
        } else if period == 60 {
            period = 0;
            r_old = r;
            l_old = l;
        }
    }

    return iterations;
}
