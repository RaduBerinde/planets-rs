use conrod::position::{Align, Direction, Padding, Position, Relative};
use conrod::widget::Id;
use kiss3d::conrod::color;
use kiss3d::{
    conrod::{self, widget, Colorable, Labelable, Positionable, Sizeable, Widget},
    widget_ids,
    window::Window,
};

use crate::{control::ControlEvent, status::Status};

pub struct Ui {
    ids: Ids,
}

impl Ui {
    pub const WIDTH: conrod::Scalar = 280.0;
    const MARGIN: conrod::Scalar = 10.0;

    pub fn new(window: &mut Window) -> Self {
        let conrod_ui = window.conrod_ui_mut();
        let font_id = conrod_ui
            .fonts
            //.insert_from_file("media/UbuntuMono-R.ttf")
            .insert_from_file("media/NotoSansMono-CondensedMedium.ttf")
            .expect("cannot load font");
        conrod_ui.theme = Self::theme(Some(font_id));

        let ids = Ids::new(conrod_ui.widget_id_generator());
        Self { ids }
    }

    pub fn frame(&self, window: &mut Window, status: Status) -> Vec<ControlEvent> {
        let mut events = Vec::<ControlEvent>::new();
        let ids = &self.ids;
        let ui = &mut window.conrod_ui_mut().set_widgets();

        // `Canvas` is a widget that provides some basic functionality for laying out children widgets.
        // By default, its size is the size of the window. We'll use this as a background for the
        // following widgets, as well as a scrollable container for the children widgets.
        widget::Canvas::new()
            .pad(Self::MARGIN)
            .align_right()
            .w(Self::WIDTH)
            .scroll_kids_vertically()
            .set(ids.canvas, ui);

        let timestamp = status.sim.timestamp.format("%Y-%m-%d %H:%M UTC");
        widget::Text::new(&timestamp.to_string())
            .font_size(14)
            .padded_w_of(ids.canvas, Self::MARGIN)
            .mid_top_of(ids.canvas)
            .align_middle_x_of(ids.canvas)
            .center_justify()
            //.line_spacing(5.0)
            .set(ids.timestamp, ui);

        if !status.sim.running {
            // Play button.
            for _press in widget::Button::new()
                .align_middle_x_of(ids.canvas)
                .down(50.0)
                .w_h(50.0, 50.0)
                .set(ids.start, ui)
            {
                events.push(ControlEvent::StartStop)
            }

            widget::Polygon::centred_fill([[0.0, -14.0], [0.0, 14.0], [20.0, 0.0]])
                .color(color::BLACK)
                .align_middle_y_of(ids.start)
                .x_relative_to(ids.start, 2.0)
                .graphics_for(ids.start)
                .set(ids.start_shape, ui);
        } else {
            // Pause button.
            for _press in widget::Button::new()
                .align_middle_x_of(ids.canvas)
                .down(50.0)
                .w_h(50.0, 50.0)
                .set(ids.pause, ui)
            {
                events.push(ControlEvent::StartStop)
            }

            widget::Rectangle::fill([8.0, 28.0])
                .color(color::BLACK)
                .align_middle_y_of(ids.pause)
                .x_relative_to(ids.pause, -6.0)
                .graphics_for(ids.pause)
                .set(ids.pause_shape_1, ui);

            widget::Rectangle::fill([8.0, 28.0])
                .color(color::BLACK)
                .align_middle_y_of(ids.pause)
                .x_relative_to(ids.pause, 6.0)
                .graphics_for(ids.pause)
                .set(ids.pause_shape_2, ui);
        }

        // ===== Simulation speed =====

        widget::Text::new("Simulation speed (time/wall-sec):")
            .font_size(11)
            .mid_left_of(ids.canvas)
            .down(50.0)
            .left_justify()
            .set(ids.speed_title, ui);

        let choices = status.sim.speed.choice_set();
        let width = (Self::WIDTH - 2.0 * Self::MARGIN) / choices.len() as f64;
        for (i, d) in choices.iter().enumerate() {
            let id = Id::new(ids.speed_1.index() + i);
            let is_set = status.sim.speed.get() == *d;
            for _ in widget::Toggle::new(is_set)
                .label(&duration_short_string(d))
                .label_font_size(10)
                .label_color(if is_set { color::BLACK } else { color::WHITE })
                .label_y(Relative::Scalar(1.0))
                .w_h(width + 1.0, 24.0)
                .x_relative_to(
                    ids.canvas,
                    -Self::WIDTH * 0.5 + Self::MARGIN + (0.5 + i as f64) * width,
                )
                .down_from(ids.speed_title, 8.0)
                .set(id, ui)
            {
                events.push(ControlEvent::SetSpeed(choices.by_index(i)));
            }
        }

        events
    }

    fn theme(font_id: Option<conrod::text::font::Id>) -> conrod::Theme {
        conrod::Theme {
            name: "Theme".to_string(),
            padding: Padding::none(),
            x_position: Position::Relative(Relative::Align(Align::Start), None),
            y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
            background_color: color::Color::Rgba(0.1, 0.1, 0.1, 0.75),
            shape_color: color::LIGHT_CHARCOAL,
            border_color: color::Color::Rgba(0.4, 0.4, 0.4, 0.75),
            border_width: 1.0,
            label_color: color::Color::Rgba(0.8, 0.8, 0.8, 1.0),
            font_id,
            font_size_large: 14,
            font_size_medium: 12,
            font_size_small: 10,
            widget_styling: conrod::theme::StyleMap::default(),
            mouse_drag_threshold: 0.0,
            double_click_threshold: std::time::Duration::from_millis(500),
        }
    }
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    struct Ids {
        canvas,
        timestamp,
        start,
        start_shape,
        revstart,
        revstart_shape,
        pause,
        pause_shape_1,
        pause_shape_2,
        speed_title,
        speed_1,
        speed_2,
        speed_3,
        speed_4,
        speed_5,
        speed_6,
        speed_7,
        speed_8,
        speed_9,
    }
}

fn duration_short_string(d: &chrono::Duration) -> String {
    if d.num_days() > 0 {
        format!("{}d", d.num_days())
    } else if d.num_hours() > 0 {
        format!("{}h", d.num_hours())
    } else if d.num_minutes() > 0 {
        format!("{}m", d.num_minutes())
    } else {
        format!("{}s", d.num_seconds())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        use chrono::Duration;
        let tests = [
            (Duration::seconds(1), "1s"),
            (Duration::seconds(15), "15s"),
            (Duration::minutes(1), "1m"),
            (Duration::minutes(15), "15m"),
            (Duration::hours(1), "1h"),
            (Duration::hours(4), "4h"),
            (Duration::days(1), "1d"),
            (Duration::days(5), "5d"),
            (Duration::days(30), "30d"),
            (Duration::days(90), "90d"),
        ];
        for (d, expected) in tests {
            assert_eq!(duration_short_string(&d), expected);
        }
    }
}
