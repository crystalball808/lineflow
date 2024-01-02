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

    let pad: f32 = 0.;
    let win = app.window_rect();
    let rect_size: f32 = app.mouse.x + win.w() / 2.;

    let rect_count = (win.w() - pad * 2.) / rect_size;
    let row_count = (win.h() - pad * 2.) / rect_size;

    let lines_count = (win.h() / (app.mouse.y + win.h() / 2.))
        .trunc()
        .max(1.)
        .min(10.);

    for row_i in 0..row_count as u32 {
        for rect_i in 0..rect_count as u32 {
            let is_vertical = (rect_i + row_i) % 2 == 1;
            let is_reversed = (rect_i + row_i) % 3 == 1;

            let rect = Rect::from_w_h(rect_size, rect_size).top_left_of(
                win.pad_left(rect_i as f32 * (rect_size + pad))
                    .pad_top(row_i as f32 * (rect_size + pad)),
            );

            let reserved_line_space = rect_size / lines_count;

            let line_weight = reserved_line_space * 0.8;

            for line_number in 0..lines_count as u32 {
                let line_start = if is_vertical {
                    pt2(
                        rect.left()
                            + reserved_line_space * line_number as f32
                            + reserved_line_space / 2.,
                        rect.top(),
                    )
                } else {
                    pt2(
                        rect.left(),
                        rect.top()
                            - reserved_line_space * line_number as f32
                            - reserved_line_space / 2.,
                    )
                };

                let line_end = if is_vertical {
                    pt2(
                        rect.left()
                            + reserved_line_space * line_number as f32
                            + reserved_line_space / 2.,
                        rect.bottom(),
                    )
                } else {
                    pt2(
                        rect.right(),
                        rect.top()
                            - reserved_line_space * line_number as f32
                            - reserved_line_space / 2.,
                    )
                };

                let points = if is_reversed {
                    generate_gradient_line_points(
                        line_end,
                        line_start,
                        time,
                        (line_number * 2 + rect_i + row_i) as f32 * 10.,
                    )
                } else {
                    generate_gradient_line_points(
                        line_start,
                        line_end,
                        time,
                        (line_number * 2 + rect_i + row_i) as f32 * 10.,
                    )
                };
                draw.polyline().weight(line_weight).points_colored(points);
            }
        }
    }

    draw.to_frame(&app, &frame).unwrap();
}
