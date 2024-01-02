use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(STEELBLUE);

    draw_line(&draw, app.time, 200., 0., 0.);
    draw_line(&draw, app.time, 200., 40., 10.);
    draw_line(&draw, app.time, 200., 80., 20.);
    draw_line(&draw, app.time, 200., 120., 30.);
    draw_line(&draw, app.time, 200., 160., 40.);
    draw_line(&draw, app.time, 200., 200., 50.);

    draw.to_frame(&app, &frame).unwrap();
}

fn draw_line(draw: &Draw, time: f32, line_len: f32, offset: f32, y: f32) {
    let grad_length = line_len * 2.;
    let peak_position = (time * 200. + offset) % (line_len + grad_length * 2.);

    let r_from = SILVER.red as f32 / 255.;
    let r_to = STEELBLUE.red as f32 / 255.;
    let g_from = SILVER.green as f32 / 255.;
    let g_to = STEELBLUE.green as f32 / 255.;
    let b_from = SILVER.blue as f32 / 255.;
    let b_to = STEELBLUE.blue as f32 / 255.;

    let r_diff = r_to - r_from;
    let r_step = r_diff / grad_length as f32;
    let g_diff = g_to - g_from;
    let g_step = g_diff / grad_length as f32;
    let b_diff = b_to - b_from;
    let b_step = b_diff / grad_length as f32;

    let line_len = line_len as u32;

    let points = ((grad_length as u32)..(line_len + grad_length as u32)).map(|i| {
        let distanse_to_peak = (peak_position - i as f32).abs();
        let distanse_to_peak = if distanse_to_peak > grad_length {
            grad_length
        } else {
            distanse_to_peak
        };

        let red = (r_step * distanse_to_peak) + r_from;
        let green = (g_step * distanse_to_peak) + g_from;
        let blue = (b_step * distanse_to_peak) + b_from;

        let x = i as f32 - grad_length;

        (pt2(x, y), Rgb::new(red, green, blue))
    });
    draw.polyline().weight(8.).points_colored(points);
}

fn draw_90(draw: &Draw, radius: usize, from_degree: u32, to_degree: u32) {
    let radiuses = (0..=radius).step_by(radius / 20).map(|i| i as f32);

    let r_from = SILVER.red as f32 / 255.;
    let r_to = STEELBLUE.red as f32 / 255.;
    let g_from = SILVER.green as f32 / 255.;
    let g_to = STEELBLUE.green as f32 / 255.;
    let b_from = SILVER.blue as f32 / 255.;
    let b_to = STEELBLUE.blue as f32 / 255.;

    let r_diff = r_to - r_from;
    let r_step = r_diff / 90.;
    let g_diff = g_to - g_from;
    let g_step = g_diff / 90.;
    let b_diff = b_to - b_from;
    let b_step = b_diff / 90.;

    draw.background().color(SILVER);
    for radius in radiuses {
        let points = (from_degree..to_degree).map(|i| {
            let color_mod: f32 = (i - from_degree) as f32;
            let radian = deg_to_rad(i as f32);
            let x = radian.sin() * radius;
            let y = radian.cos() * radius;

            (
                pt2(x, y),
                Rgb::new(
                    (r_step * color_mod) + r_from,
                    (g_step * color_mod) + g_from,
                    (b_step * color_mod) + b_from,
                ),
            )
        });
        draw.polyline().weight(3.).points_colored(points);
    }
}
