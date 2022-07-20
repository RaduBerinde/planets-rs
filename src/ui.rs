use conrod::position::{Align, Direction, Padding, Position, Relative};
use kiss3d::{
    conrod::{self, widget, Positionable, Sizeable, Widget},
    widget_ids,
    window::Window,
};

use crate::status::Status;

pub struct Ui {
    ids: Ids,
}

impl Ui {
    pub const WIDTH: conrod::Scalar = 280.0;
    const MARGIN: conrod::Scalar = 20.0;

    pub fn new(window: &mut Window) -> Self {
        let conrod_ui = window.conrod_ui_mut();
        conrod_ui.theme = Self::theme();

        let ids = Ids::new(conrod_ui.widget_id_generator());
        Self { ids }
    }

    pub fn frame(&self, window: &mut Window, status: Status) {
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
    }
}
