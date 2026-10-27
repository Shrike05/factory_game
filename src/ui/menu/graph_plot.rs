use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct PlotPoint {
    pub x_pct: f32,
    pub y_pct: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct PlotSegment {
    pub mid_x_px: f32,
    pub mid_y_px: f32,
    pub width_px: f32,
    pub height_px: f32,
    pub angle_rad: f32,
}

/// Converts raw numerical telemetry values into normalized coordinates and angled segments.
pub fn calculate_plot_geometry(
    values: &[f32],
    canvas_w_px: f32,
    canvas_h_px: f32,
) -> (Vec<PlotPoint>, Vec<PlotSegment>) {
    if values.is_empty() {
        return (vec![], vec![]);
    }
    let min = values.iter().copied().fold(f32::INFINITY, f32::min);
    let max = values.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let range = if (max - min).abs() < 0.001 {
        1.0
    } else {
        max - min
    };

    let n = values.len();
    let points: Vec<PlotPoint> = values
        .iter()
        .enumerate()
        .map(|(i, &val)| {
            let x_pct = if n > 1 {
                (i as f32 / (n - 1) as f32) * 100.0
            } else {
                50.0
            };
            let y_pct = ((val - min) / range).clamp(0.0, 1.0) * 100.0;
            PlotPoint { x_pct, y_pct }
        })
        .collect();

    let mut segments = Vec::new();
    for i in 0..(points.len().saturating_sub(1)) {
        let p1 = points[i];
        let p2 = points[i + 1];

        let x1 = (p1.x_pct / 100.0) * canvas_w_px;
        let y1 = (p1.y_pct / 100.0) * canvas_h_px;
        let x2 = (p2.x_pct / 100.0) * canvas_w_px;
        let y2 = (p2.y_pct / 100.0) * canvas_h_px;

        let dx = x2 - x1;
        let dy = y2 - y1;
        let length = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx);

        // Compute midpoint so UiTransform center-rotation pivots correctly
        let mid_x = (x1 + x2) / 2.0;
        let mid_y = (y1 + y2) / 2.0;

        segments.push(PlotSegment {
            mid_x_px: mid_x,
            mid_y_px: mid_y,
            width_px: length,
            height_px: 2.0,
            angle_rad: angle,
        });
    }

    (points, segments)
}

/// Renders a single data point dot at (x_pct, y_pct).
pub fn plot_point_dot(pt: PlotPoint) -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            left: percent(pt.x_pct),
            bottom: percent(pt.y_pct),
            width: px(4.0),
            height: px(4.0),
            margin: UiRect {
                left: px(-2.0),
                right: px(0.0),
                top: px(0.0),
                bottom: px(-2.0),
            },
        }
        BackgroundColor(Color::WHITE)
    }
}

/// Renders an angled 2px line segment between two consecutive telemetry samples.
pub fn plot_line_segment(seg: PlotSegment) -> impl Scene {
    bsn! {
        Node {
            position_type: PositionType::Absolute,
            left: px(seg.mid_x_px),
            bottom: px(seg.mid_y_px),
            width: px(seg.width_px),
            height: px(seg.height_px),
            // Center the segment on its midpoint so rotation pivots cleanly around center
            margin: UiRect {
                left: px(-seg.width_px / 2.0),
                right: px(0.0),
                top: px(0.0),
                bottom: px(-seg.height_px / 2.0),
            },
        }
        BackgroundColor(Color::WHITE)
        UiTransform::from_rotation(Rot2::radians(seg.angle_rad))
    }
}

/// Dynamic plot surface taking raw float values.
pub fn graph_plot_surface(values: &[f32]) -> impl Scene {
    let (points, segments) = calculate_plot_geometry(values, 180.0, 70.0);

    bsn! {
        Node {
            position_type: PositionType::Relative,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            flex_grow: 1.0,
            padding: UiRect::all(px(4.0)),
            border: UiRect::all(px(1.0)),
            overflow: Overflow::clip(),
        }
        BackgroundColor(Color::BLACK)
        BorderColor::all(Color::srgb(0.133, 0.133, 0.133))
        Children [
            // Gridlines
            Node { width: percent(100.0), height: px(1.0) }
            BackgroundColor(Color::srgb(0.067, 0.067, 0.067))
            --
            Node { width: percent(100.0), height: px(1.0) }
            BackgroundColor(Color::srgb(0.067, 0.067, 0.067))
            //--
            // Dynamic Line Segments
            //{segments.into_iter().map(|seg|{
            //    bsn!{@plot_line_segment(seg)}
            //}).collect::<Vec<_>>()}
            --
            // Dynamic Data Point Dots
            {points.into_iter().map(|pt|{
                bsn!{@plot_point_dot(pt)}
            }).collect::<Vec<_>>()}
        ]
    }
}
