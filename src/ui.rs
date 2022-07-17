use conrod::position::{Align, Direction, Padding, Position, Relative};
use kiss3d::{
    conrod::{self, widget, Positionable, Sizeable, Widget},
    widget_ids,
    window::Window,
};

pub struct Ui {
    ids: Ids,
}

impl Ui {
    pub const WIDTH: conrod::Scalar = 300.0;
    const MARGIN: conrod::Scalar = 30.0;

    pub fn new(window: &mut Window) -> Self {
        let conrod_ui = window.conrod_ui_mut();
        conrod_ui.theme = Self::theme();

        let ids = Ids::new(conrod_ui.widget_id_generator());
        Self { ids }
    }

    pub fn render(&self, window: &mut Window) {
        let mut ui = window.conrod_ui_mut().set_widgets();

        // `Canvas` is a widget that provides some basic functionality for laying out children widgets.
        // By default, its size is the size of the window. We'll use this as a background for the
        // following widgets, as well as a scrollable container for the children widgets.
        //const TITLE: &'static str = "All Widgets";
        widget::Canvas::new()
            .pad(Self::MARGIN)
            .align_right()
            .w(Self::WIDTH)
            .scroll_kids_vertically()
            .set(self.ids.canvas, &mut ui);
    }

    fn theme() -> conrod::Theme {
        conrod::Theme {
            name: "Theme".to_string(),
            padding: Padding::none(),
            x_position: Position::Relative(Relative::Align(Align::Start), None),
            y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
            background_color: conrod::color::Color::Rgba(0.05, 0.05, 0.05, 0.8),
            shape_color: conrod::color::LIGHT_CHARCOAL,
            border_color: conrod::color::WHITE,
            border_width: 1.0,
            label_color: conrod::color::Color::Rgba(0.31, 0.31, 0.29, 1.0),
            font_id: None,
            font_size_large: 26,
            font_size_medium: 18,
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
        title,
    }
}
