use conrod::position::{Align, Direction, Padding, Position, Relative};
use kiss3d::{
    conrod::{self, widget, Colorable, Positionable, Sizeable, Widget},
    widget_ids,
    window::Window,
};

use crate::{control::ControlEvent, status::Status};

pub struct Ui {
    ids: Ids,
}

impl Ui {
    pub const WIDTH: conrod::Scalar = 280.0;
    const MARGIN: conrod::Scalar = 20.0;

    pub fn new(window: &mut Window) -> Self {
        let conrod_ui = window.conrod_ui_mut();
        //let font_id = conrod_ui
        //    .fonts
        //    .insert_from_file("media/unifont-14.0.04.ttf")
        //    .expect("cannot load font");
        conrod_ui.theme = Self::theme();

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
            .padded_w_of(ids.canvas, Self::MARGIN)
            .mid_top_of(ids.canvas)
            .align_middle_x_of(ids.canvas)
            .center_justify()
            .line_spacing(5.0)
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
                .color(conrod::color::BLACK)
                .align_middle_y_of(ids.start)
                .x_position_relative_to(ids.start, Relative::Scalar(2.0))
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
                .color(conrod::color::BLACK)
                .align_middle_y_of(ids.pause)
                .x_position_relative_to(ids.pause, Relative::Scalar(-6.0))
                .graphics_for(ids.pause)
                .set(ids.pause_shape_1, ui);

            widget::Rectangle::fill([8.0, 28.0])
                .color(conrod::color::BLACK)
                .align_middle_y_of(ids.pause)
                .x_position_relative_to(ids.pause, Relative::Scalar(6.0))
                .graphics_for(ids.pause)
                .set(ids.pause_shape_2, ui);
        }

        events
    }

    fn theme() -> conrod::Theme {
        conrod::Theme {
            name: "Theme".to_string(),
            padding: Padding::none(),
            x_position: Position::Relative(Relative::Align(Align::Start), None),
            y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
            background_color: conrod::color::Color::Rgba(0.1, 0.1, 0.1, 0.75),
            shape_color: conrod::color::LIGHT_CHARCOAL,
            border_color: conrod::color::Color::Rgba(0.4, 0.4, 0.4, 0.75),
            border_width: 1.0,
            label_color: conrod::color::Color::Rgba(0.8, 0.8, 0.8, 1.0),
            font_id: None,
            font_size_large: 20,
            font_size_medium: 16,
            font_size_small: 12,
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
    }
}
