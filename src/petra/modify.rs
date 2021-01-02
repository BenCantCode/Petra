
fn lerp(s: f32, e: f32, i: f32) -> f32 {
    s + (e - s) * i
}

pub fn get_subpixel_value(x: f32, y: f32, data: &Vec<Vec<f32>>, len: usize) -> Option<f32> {
    // P1   P2
    //  (x,y)
    // P3   P4
    // Bilinear interpolation, with linear interpolation/nearest neighbor for edge values.
    let x_adjusted = x % 1.0;
    let y_adjusted = y % 1.0;
    if x.floor() >= 0.0
        && x.ceil() <= (len as f32) - 1.0
        && y.floor() >= 0.0
        && y.ceil() <= (len as f32) - 1.0
    {
        // Full bilinear
        let p1 = data[y.floor() as usize][x.floor() as usize];
        let p2 = data[y.floor() as usize][x.ceil() as usize];
        let p3 = data[y.ceil() as usize][x.floor() as usize];
        let p4 = data[y.ceil() as usize][x.ceil() as usize];
        let top_x_interp = lerp(p1, p2, x_adjusted);
        let bottom_x_interp = lerp(p3, p4, x_adjusted);
        return Some(lerp(top_x_interp, bottom_x_interp, y_adjusted));
    } else if x.floor() < 0.0
        && x.ceil() <= (len as f32) - 1.0
        && x.ceil() >= 0.0
        && y.floor() >= 0.0
        && y.ceil() <= (len as f32) - 1.0
    {
        // left side cut off
        let p2 = data[y.floor() as usize][x.ceil() as usize];
        let p4 = data[y.ceil() as usize][x.ceil() as usize];
        return Some(lerp(p2, p4, y_adjusted));
    } else if x.floor() >= 0.0
        && x.floor() <= (len as f32) - 1.0
        && x.ceil() > (len as f32) - 1.0
        && y.floor() >= 0.0
        && y.ceil() <= (len as f32) - 1.0
    {
        // right side cut off
        let p1 = data[y.floor() as usize][x.floor() as usize];
        let p3 = data[y.ceil() as usize][x.floor() as usize];
        return Some(lerp(p1, p3, y_adjusted));
    } else if x.floor() >= 0.0
        && x.ceil() <= (len as f32) - 1.0
        && y.floor() < 0.0
        && y.ceil() >= 0.0
        && y.ceil() <= (len as f32) - 1.0
    {
        // top cut off
        let p3 = data[y.ceil() as usize][x.floor() as usize];
        let p4 = data[y.ceil() as usize][x.ceil() as usize];
        return Some(lerp(p3, p4, x_adjusted));
    } else if x.floor() >= 0.0
        && x.ceil() <= (len as f32) - 1.0
        && y.floor() >= 0.0
        && y.floor() <= (len as f32) - 1.0
        && y.ceil() > (len as f32) - 1.0
    {
        // bottom cut off
        let p1 = data[y.floor() as usize][x.floor() as usize];
        let p2 = data[y.floor() as usize][x.ceil() as usize];
        return Some(lerp(p1, p2, x_adjusted));
    } else if y.floor() >= 0.0
        && y.floor() <= (len as f32) - 1.0
        && x.floor() >= 0.0
        && x.floor() <= (len as f32) - 1.0
    {
        // If p1 isn't cut off (top left corner)
        return Some(data[y.floor() as usize][x.floor() as usize]);
    } else if y.floor() >= 0.0
        && y.floor() <= (len as f32) - 1.0
        && x.ceil() >= 0.0
        && x.ceil() <= (len as f32) - 1.0
    {
        // If p2 isn't cut off (top right corner)
        return Some(data[y.floor() as usize][x.ceil() as usize]);
    } else if y.ceil() >= 0.0
        && y.ceil() <= (len as f32) - 1.0
        && x.floor() >= 0.0
        && x.floor() <= (len as f32) - 1.0
    {
        // If p3 isn't cut off (bottom left corner)
        return Some(data[y.ceil() as usize][x.floor() as usize]);
    } else if y.ceil() >= 0.0
        && y.ceil() <= (len as f32) - 1.0
        && x.ceil() >= 0.0
        && x.ceil() <= (len as f32) - 1.0
    {
        // If p4 isn't cut off (top right corner)
        return Some(data[y.ceil() as usize][x.ceil() as usize]);
    } else {
        return None;
    }
}

fn get_subpixel_weights(x: f32, y: f32) -> (f32, f32, f32, f32) {
    let x_adjusted = x % 1.0;
    let y_adjusted = y % 1.0;
    let p1_weight = lerp(lerp(1.0, 0.0, x_adjusted), 0.0, y_adjusted);
    let p2_weight = lerp(lerp(0.0, 1.0, x_adjusted), 0.0, y_adjusted);
    let p3_weight = lerp(0.0, lerp(1.0, 0.0, x_adjusted), y_adjusted);
    let p4_weight = lerp(0.0, lerp(0.0, 1.0, x_adjusted), y_adjusted);
    (p1_weight, p2_weight, p3_weight, p4_weight)
}

pub fn modify_data(x: f32, y: f32, change: f32, data: &mut Vec<Vec<f32>>) {
    let (p1, p2, p3, p4) = get_subpixel_weights(x, y);
    data[y.floor() as usize][x.floor() as usize] += p1 * change;
    data[y.floor() as usize][x.ceil() as usize] += p2 * change;
    data[y.ceil() as usize][x.floor() as usize] += p3 * change;
    data[y.ceil() as usize][x.ceil() as usize] += p4 * change;
}