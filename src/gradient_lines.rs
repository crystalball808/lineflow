use nannou::prelude::*;

pub fn generate_gradient_line_points(
    start: Point2,
    end: Point2,
    time: f32,
    offset: f32,
) -> Vec<(Point2, Rgb)> {
    let line_len = f32::sqrt((end.x - start.x).pow(2) + (end.y - start.y).pow(2));
    let grad_len = line_len / 2.;
    let peak_position = (time * 200. + offset) % (line_len + grad_len * 2.);

    let step_x = (end.x - start.x) / line_len;
    let step_y = (end.y - start.y) / line_len;

    let r_from = SILVER.red as f32 / 255.;
    let r_to = STEELBLUE.red as f32 / 255.;
    let g_from = SILVER.green as f32 / 255.;
    let g_to = STEELBLUE.green as f32 / 255.;
    let b_from = SILVER.blue as f32 / 255.;
    let b_to = STEELBLUE.blue as f32 / 255.;

    let r_diff = r_to - r_from;
    let r_step = r_diff / grad_len as f32;
    let g_diff = g_to - g_from;
    let g_step = g_diff / grad_len as f32;
    let b_diff = b_to - b_from;
    let b_step = b_diff / grad_len as f32;

    // let points = ((grad_len as u32)..(line_len + grad_len) as u32)
    let points = (0..line_len as u32)
        .map(|i| {
            let i_grad = i + grad_len as u32;
            let distanse_to_peak = (peak_position - i_grad as f32).abs();
            let distanse_to_peak = if distanse_to_peak > grad_len {
                grad_len
            } else {
                distanse_to_peak
            };

            let red = (r_step * distanse_to_peak) + r_from;
            let green = (g_step * distanse_to_peak) + g_from;
            let blue = (b_step * distanse_to_peak) + b_from;

            let x = (step_x * i as f32) + start.x;
            let y = (step_y * i as f32) + start.y;

            (pt2(x, y), Rgb::new(red, green, blue))
        })
        .collect();

    points
}
