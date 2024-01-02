use nannou::prelude::*;

mod gradient_lines;
use gradient_lines::generate_gradient_line_points;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let time = app.time;

    draw.background().color(STEELBLUE);

    let pad: f32 = 10.;
    let win = app.window_rect();
    let rect_size: f32 = 100.;

    let rect_count = (win.w() - pad * 2.) / rect_size;
    dbg!(rect_count);

    for rect_i in 0..rect_count as u32 {
        let rect = Rect::from_w_h(rect_size, rect_size)
            .top_left_of(win.pad_left(rect_i as f32 * (rect_size + pad)));

        let lines_count = 4.;
        let reserved_line_space = rect_size / lines_count;

        let line_weight = reserved_line_space * 0.5;

        for line_number in 0..lines_count as u32 {
            let points = generate_gradient_line_points(
                pt2(
                    rect.left(),
                    rect.top()
                        - reserved_line_space * line_number as f32
                        - reserved_line_space / 2.,
                ),
                pt2(
                    rect.right(),
                    rect.top()
                        - reserved_line_space * line_number as f32
                        - reserved_line_space / 2.,
                ),
                time,
                (line_number * 20) as f32,
            );
            draw.polyline().weight(line_weight).points_colored(points);
        }
    }

    // let points = generate_gradient_line_points(win.bottom_left(), win.top_right(), app.time, 0.);
    // draw.polyline().weight(10.).points_colored(points);
    // let points = generate_gradient_line_points(win.top_left(), win.bottom_right(), app.time, 0.);
    // draw.polyline().weight(10.).points_colored(points);

    draw.to_frame(&app, &frame).unwrap();
}
