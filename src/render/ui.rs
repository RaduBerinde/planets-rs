use conrod::position::{Align, Direction, Padding, Position, Relative};
use conrod::widget::Id;
use kiss3d::conrod::position::Place;
use kiss3d::conrod::{color, UiCell};
use kiss3d::{
    conrod::{self, widget, Colorable, Labelable, Positionable, Sizeable, Widget},
    widget_ids,
    window::Window,
};

use crate::choice::Choice;
use crate::control::ControlEvent;
use crate::state::{RenderState, SimulationState};

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

    pub fn frame(
        &self,
        window: &mut Window,
        sim_state: &dyn SimulationState,
        render_state: &dyn RenderState,
    ) -> Vec<ControlEvent> {
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

        widget::Scrollbar::y_axis(self.ids.canvas)
            .auto_hide(true)
            .set(self.ids.canvas_scrollbar, ui);

        let timestamp = sim_state.timestamp().format("%Y-%m-%d %H:%M UTC");
        widget::Text::new(&timestamp.to_string())
            .font_size(16)
            .padded_w_of(self.ids.canvas, Self::MARGIN)
            .mid_top_of(self.ids.canvas)
            .align_middle_x_of(self.ids.canvas)
            .center_justify()
            //.line_spacing(5.0)
            .set(self.ids.timestamp, ui);

        self.simulation_controls(ui, sim_state, &mut events);
        self.simulation_speed(ui, sim_state, &mut events);
        self.camera_focus(ui, render_state, &mut events);
        self.render_toggles(ui, render_state, &mut events);

        events
    }

    fn simulation_controls(
        &self,
        ui: &mut UiCell,
        sim_state: &dyn SimulationState,
        events: &mut Vec<ControlEvent>,
    ) {
        if !sim_state.is_running() {
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
                .align_middle_y_of(self.ids.play_pause)
                .x_relative_to(self.ids.play_pause, 2.0)
                .graphics_for(self.ids.play_pause)
                .set(self.ids.start_shape, ui);

            // Back button.
            for _ in widget::Button::new()
                .color(ui.theme().label_color.with_luminance(0.1))
                .left_from(self.ids.play_pause, 16.0)
                .w_h(40.0, 40.0)
                .set(self.ids.jump_back, ui)
            {
                events.push(ControlEvent::JumpBack)
            }
            widget::Polygon::centred_fill([[11.0, -12.0], [11.0, 12.0], [0.0, 0.0]])
                .align_middle_y_of(self.ids.jump_back)
                .x_relative_to(self.ids.jump_back, 2.0)
                .graphics_for(self.ids.jump_back)
                .set(self.ids.jump_back_shape_1, ui);

            widget::RoundedRectangle::fill([4.0, 23.0], 2.0)
                .align_middle_y_of(self.ids.jump_back)
                .x_relative_to(self.ids.jump_back, -5.0)
                .graphics_for(self.ids.jump_back)
                .set(self.ids.jump_back_shape_2, ui);

            // Forward button.
            for _ in widget::Button::new()
                .color(ui.theme().label_color.with_luminance(0.1))
                .right_from(self.ids.play_pause, 16.0)
                .w_h(40.0, 40.0)
                .set(self.ids.jump_forward, ui)
            {
                events.push(ControlEvent::JumpForward)
            }
            widget::Polygon::centred_fill([[0.0, -12.0], [0.0, 12.0], [11.0, 0.0]])
                .align_middle_y_of(self.ids.jump_forward)
                .x_relative_to(self.ids.jump_forward, -2.0)
                .graphics_for(self.ids.jump_forward)
                .set(self.ids.jump_forward_shape_1, ui);

            widget::RoundedRectangle::fill([4.0, 23.0], 2.0)
                .align_middle_y_of(self.ids.jump_forward)
                .x_relative_to(self.ids.jump_forward, 5.0)
                .graphics_for(self.ids.jump_forward)
                .set(self.ids.jump_forward_shape_2, ui);
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

            widget::RoundedRectangle::fill([6.0, 23.0], 2.0)
                .align_middle_y_of(self.ids.pause)
                .x_relative_to(self.ids.pause, -5.0)
                .graphics_for(self.ids.pause)
                .set(self.ids.pause_shape_1, ui);

            widget::RoundedRectangle::fill([6.0, 23.0], 2.0)
                .align_middle_y_of(self.ids.pause)
                .x_relative_to(self.ids.pause, 5.0)
                .graphics_for(self.ids.pause)
                .set(self.ids.pause_shape_2, ui);

            // Reverse toggle.
            if self.toggle_switch(
                ui,
                self.ids.reverse_toggle_title,
                "Reverse",
                self.ids.pause,
                Relative::Align(Align::Middle),
                sim_state.is_reverse(),
            ) {
                events.push(ControlEvent::Reverse);
            }
        }
    }

    fn simulation_speed(
        &self,
        ui: &mut UiCell,
        sim_state: &dyn SimulationState,
        events: &mut Vec<ControlEvent>,
    ) {
        if let Some(new_speed) = self.choice_buttons(
            ui,
            self.ids.speed_title,
            "Simulation speed (time/wall-sec)",
            self.ids.play_pause,
            26.0,
            &sim_state.speed(),
            |&d| duration_short_string(&d),
        ) {
            events.push(ControlEvent::SetSpeed(new_speed))
        }
    }

    fn camera_focus(
        &self,
        ui: &mut UiCell,
        render_state: &dyn RenderState,
        events: &mut Vec<ControlEvent>,
    ) {
        if let Some(new_camera) = self.choice_buttons(
            ui,
            self.ids.camera_title,
            "Camera",
            self.ids.speed_1,
            30.0,
            &render_state.camera_focus(),
            |&d| d.description.to_string(),
        ) {
            events.push(ControlEvent::SetCamera(new_camera))
        }
    }

    fn render_toggles(
        &self,
        ui: &mut UiCell,
        render_state: &dyn RenderState,
        events: &mut Vec<ControlEvent>,
    ) {
        if self.toggle_switch(
            ui,
            self.ids.trails_toggle_title,
            "Trails",
            ui.maybe_prev_widget().unwrap(),
            Relative::Direction(Direction::Backwards, 20.0),
            render_state.show_trails(),
        ) {
            events.push(ControlEvent::ToggleTrails)
        }

        if self.toggle_switch(
            ui,
            self.ids.ecliptic_toggle_title,
            "Ecliptic plane",
            ui.maybe_prev_widget().unwrap(),
            Relative::Direction(Direction::Backwards, 20.0),
            render_state.show_ecliptic(),
        ) {
            events.push(ControlEvent::ToggleEcliptic)
        }

        if self.toggle_switch(
            ui,
            self.ids.skybox_toggle_title,
            "Star background",
            ui.maybe_prev_widget().unwrap(),
            Relative::Direction(Direction::Backwards, 20.0),
            render_state.show_skybox(),
        ) {
            events.push(ControlEvent::ToggleSkybox)
        }
    }

    #[allow(clippy::too_many_arguments)]
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
                result = Some(choices.by_index(i));
            }
        }
        result
    }

    // Creates a toggle switch, positioned near the right edge, vertically
    // positioned relative to a given widget. Returns true if the switch was
    // activated.
    fn toggle_switch(
        &self,
        ui: &mut UiCell,
        start_id: Id,
        title: &'static str,
        y_id: Id,
        y: Relative,
        is_set: bool,
    ) -> bool {
        let title_id = start_id;
        let rect_id = Id::new(start_id.index() + 1);
        let circle_id = Id::new(start_id.index() + 2);

        widget::Text::new(title)
            .font_size(12)
            .x_place_on(self.ids.canvas, Place::End(Some(Self::MARGIN + 28.0)))
            .y_position_relative_to(y_id, y)
            .set(title_id, ui);

        let mut rect_color = if is_set {
            ui.theme().shape_color
        } else {
            ui.theme().background_color.highlighted()
        };
        let input = ui.widget_input(rect_id);
        if let Some(mouse) = input.mouse() {
            if mouse.buttons.left().is_down() {
                rect_color = rect_color.clicked();
            } else {
                rect_color = rect_color.highlighted();
            }
        }
        let clicked = input.clicks().left().count() + input.taps().count() > 0;

        widget::RoundedRectangle::fill([28.0, 14.0], 7.0)
            .color(rect_color)
            .x_relative_to(self.ids.canvas, Self::WIDTH * 0.5 - Self::MARGIN - 14.0)
            .y_relative(-2.0)
            //.align_middle_y()
            .set(rect_id, ui);

        widget::Circle::outline(6.0)
            .color(color::BLACK)
            .color(if is_set {
                color::BLACK
            } else {
                color::LIGHT_CHARCOAL
            })
            .align_middle_y()
            .x_relative(if is_set { 7.0 } else { -7.0 })
            .graphics_for(rect_id)
            .set(circle_id, ui);

        clicked
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
        canvas_scrollbar,
        timestamp,
        play_pause,
        jump_back,
        jump_back_shape_1,
        jump_back_shape_2,
        jump_forward,
        jump_forward_shape_1,
        jump_forward_shape_2,
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
        reverse_toggle_title,
        reverse_toggle_rect,
        reverse_toggle_circle,
        trails_toggle_title,
        trails_toggle_rect,
        trails_toggle_circle,
        ecliptic_toggle_title,
        ecliptic_toggle_rect,
        ecliptic_toggle_circle,
        skybox_toggle_title,
        skybox_toggle_rect,
        skybox_toggle_circle,
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
