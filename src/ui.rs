use conrod::position::{Align, Direction, Padding, Position, Relative};
use conrod::widget::Id;
use kiss3d::conrod::{color, UiCell};
use kiss3d::{
    conrod::{self, widget, Colorable, Labelable, Positionable, Sizeable, Widget},
    widget_ids,
    window::Window,
};

use crate::choice::Choice;
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
        let ui = &mut window.conrod_ui_mut().set_widgets();

        // `Canvas` is a widget that provides some basic functionality for laying out children widgets.
        // By default, its size is the size of the window. We'll use this as a background for the
        // following widgets, as well as a scrollable container for the children widgets.
        widget::Canvas::new()
            .pad(Self::MARGIN)
            .align_right()
            .w(Self::WIDTH)
            .scroll_kids_vertically()
            .set(self.ids.canvas, ui);

        let timestamp = status.sim.timestamp.format("%Y-%m-%d %H:%M UTC");
        widget::Text::new(&timestamp.to_string())
            .font_size(16)
            .padded_w_of(self.ids.canvas, Self::MARGIN)
            .mid_top_of(self.ids.canvas)
            .align_middle_x_of(self.ids.canvas)
            .center_justify()
            //.line_spacing(5.0)
            .set(self.ids.timestamp, ui);

        self.simulation_controls(ui, &status, &mut events);
        self.simulation_speed(ui, &status, &mut events);
        self.camera_focus(ui, &status, &mut events);

        events
    }

    fn simulation_controls(
        &self,
        ui: &mut UiCell,
        status: &Status,
        events: &mut Vec<ControlEvent>,
    ) {
        if !status.sim.running {
            // Play button.
            for _ in widget::Button::new()
                .color(ui.theme().label_color.with_luminance(0.1))
                .align_middle_x_of(self.ids.canvas)
                .down(20.0)
                .w_h(40.0, 40.0)
                .set(self.ids.play_pause, ui)
            {
                events.push(ControlEvent::StartStop)
            }

            widget::Polygon::centred_fill([[0.0, -12.0], [0.0, 12.0], [16.0, 0.0]])
                //.color(color::LIGHT_CHARCOAL)
                //.color(ui.theme().label_color)
                .align_middle_y_of(self.ids.play_pause)
                .x_relative_to(self.ids.play_pause, 2.0)
                .graphics_for(self.ids.play_pause)
                .set(self.ids.start_shape, ui);
        } else {
            // Pause button.
            for _ in widget::Button::new()
                .color(ui.theme().label_color.with_luminance(0.1))
                .align_middle_x_of(self.ids.canvas)
                .down(20.0)
                .w_h(40.0, 40.0)
                .set(self.ids.pause, ui)
            {
                events.push(ControlEvent::StartStop)
            }

            widget::Rectangle::fill([7.0, 23.0])
                .align_middle_y_of(self.ids.pause)
                .x_relative_to(self.ids.pause, -5.0)
                .graphics_for(self.ids.pause)
                .set(self.ids.pause_shape_1, ui);

            widget::Rectangle::fill([7.0, 23.0])
                .align_middle_y_of(self.ids.pause)
                .x_relative_to(self.ids.pause, 5.0)
                .graphics_for(self.ids.pause)
                .set(self.ids.pause_shape_2, ui);

            // Reverse toggle.
            for _ in widget::Toggle::new(status.sim.reverse)
                .label("Reverse")
                .label_font_size(12)
                .label_color(if status.sim.reverse {
                    color::BLACK
                } else {
                    ui.theme.label_color
                })
                .label_y(Relative::Scalar(2.0))
                .w_h(60.0, 40.0)
                .align_middle_y_of(self.ids.pause)
                .x_relative_to(self.ids.canvas, Self::WIDTH * 0.5 - Self::MARGIN - 30.0)
                .set(self.ids.reverse, ui)
            {
                events.push(ControlEvent::Reverse);
            }
        }
    }

    fn simulation_speed(&self, ui: &mut UiCell, status: &Status, events: &mut Vec<ControlEvent>) {
        if let Some(new_speed) = self.choice_buttons(
            ui,
            self.ids.speed_title,
            "Simulation speed (time/wall-sec)",
            self.ids.play_pause,
            26.0,
            &status.sim.speed,
            |&d| duration_short_string(&d),
        ) {
            events.push(ControlEvent::SetSpeed(new_speed))
        }
    }

    fn camera_focus(&self, ui: &mut UiCell, status: &Status, events: &mut Vec<ControlEvent>) {
        if let Some(new_camera) = self.choice_buttons(
            ui,
            self.ids.camera_title,
            "Camera",
            self.ids.speed_1,
            30.0,
            &status.render.camera_focus,
            |&d| d.props().name.to_string(),
        ) {
            events.push(ControlEvent::SetCamera(new_camera))
        }
    }

    fn choice_buttons<T: Copy>(
        &self,
        ui: &mut UiCell,
        start_id: Id,
        title: &str,
        down_from: Id,
        height: f64,
        choice: &Choice<T>,
        to_str: impl Fn(&T) -> String,
    ) -> Option<Choice<T>> {
        let title_id = start_id;

        widget::Text::new(title)
            .font_size(12)
            .align_middle_x_of(self.ids.canvas)
            .down_from(down_from, 30.0)
            .center_justify()
            .set(start_id, ui);

        let choices = choice.choice_set();
        let width = (Self::WIDTH - 2.0 * Self::MARGIN) / choices.len() as f64;
        let mut result: Option<Choice<T>> = None;
        for (i, d) in choices.iter().enumerate() {
            let id = Id::new(start_id.index() + 1 + i);
            let is_set = choice.index() == i;
            for _ in widget::Toggle::new(is_set)
                .label(&to_str(d))
                .label_font_size(11)
                .label_color(if is_set {
                    color::BLACK
                } else {
                    ui.theme.label_color
                })
                .label_y(Relative::Scalar(1.0))
                .w_h(width + 1.0, height)
                .x_relative_to(
                    self.ids.canvas,
                    -Self::WIDTH * 0.5 + Self::MARGIN + (0.5 + i as f64) * width,
                )
                .down_from(title_id, 8.0)
                .set(id, ui)
            {
                result = result.or(Some(choices.by_index(i)));
            }
        }
        result
    }

    fn theme(font_id: Option<conrod::text::font::Id>) -> conrod::Theme {
        conrod::Theme {
            name: "Theme".to_string(),
            padding: Padding::none(),
            x_position: Position::Relative(Relative::Align(Align::Start), None),
            y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
            background_color: color::Color::Rgba(0.1, 0.1, 0.1, 0.75),
            shape_color: color::Color::Rgba(0.8, 0.8, 0.8, 1.0),
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
        play_pause,
        start_shape,
        revstart,
        revstart_shape,
        pause,
        pause_shape_1,
        pause_shape_2,
        reverse,
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
        camera_title,
        camera_1,
        camera_2,
        camera_3,
        camera_4,
        camera_5,
        camera_6,
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
