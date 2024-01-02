use nannou::prelude::*;

mod gradient_lines;
use gradient_lines::generate_gradient_line_points;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(STEELBLUE);

    let win = app.window_rect();

    let points = generate_gradient_line_points(win.bottom_left(), win.top_right(), app.time, 0.);
    draw.polyline().weight(10.).points_colored(points);
    let points = generate_gradient_line_points(pt2(0., 0.), pt2(130., -80.), app.time, 0.);
    draw.polyline().weight(10.).points_colored(points);

    draw.to_frame(&app, &frame).unwrap();
}
