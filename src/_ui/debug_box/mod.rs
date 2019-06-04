use conrod_core::{
    self,
    widget_ids,
    widget,
    Widget,
    Borderable,
    Colorable,
    Positionable,
    position,
    position::Position,
    position::Place,
    position::Dimension,
};

#[derive(WidgetCommon)]
pub struct SettingsBox {
    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    style: Style,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, WidgetStyle)]
pub struct Style {
    /// Color of the button.
    #[conrod(default = "theme.shape_color")]
    pub color: Option<conrod_core::Color>,
    /// Color of the button's label.
    #[conrod(default = "theme.label_color")]
    pub label_color: Option<conrod_core::Color>,
    /// Font size of the button's label.
    #[conrod(default = "theme.font_size_medium")]
    pub label_font_size: Option<conrod_core::FontSize>,
    /// Specify a unique font for the label.
    #[conrod(default = "theme.font_id")]
    pub label_font_id: Option<Option<conrod_core::text::font::Id>>,
}

widget_ids! {
    struct Ids {
        background_box,
        text,
    }
}

pub struct State {
    ids: Ids,
}

impl SettingsBox
{
    pub fn new() -> Self
    {
        SettingsBox {
            common: widget::CommonBuilder::default(),
            style: Style::default(),
        }
    }

    pub fn label_font_id(mut self, font_id: conrod_core::text::font::Id) -> Self
    {
        self.style.label_font_id = Some(Some(font_id));
        self
    }
}

impl Widget for SettingsBox
{
    type State = State;
    type Style = Style;
    type Event = Option<()>;

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State
    {
        State {
            ids: Ids::new(id_gen),
        }
    }

    fn style(&self) -> Self::Style
    {
        self.style.clone()
    }
    
    fn update(self, args: widget::UpdateArgs<Self>) -> Self::Event
    {
        let widget::UpdateArgs {
            id,
            state,
            rect,
            ui,
            style,
            ..
        } = args;

        widget::BorderedRectangle::new([20.0, 20.0])
            .border(2.0)
            .border_rgb(1.0, 0.0, 0.0)
            .rgb(0.0, 1.0, 0.0)
            .graphics_for(id)
            .set(state.ids.background_box, ui);
        
        widget::Text::new("TEST")
            .and_then(style.label_font_id(&ui.theme), widget::Text::font_id)
            .middle_of(id)
            .font_size(style.label_font_size(&ui.theme))
            .graphics_for(id)
            .color(style.label_color(&ui.theme))
            .set(state.ids.text, ui);

        None
    }

    fn default_x_position(&self, ui: &conrod_core::Ui) -> Position
    {
        Position::Relative(position::Relative::Place(Place::Middle), None)
    }

    fn default_y_position(&self, ui: &conrod_core::Ui) -> Position
    {
        Position::Relative(position::Relative::Place(Place::Middle), None)
    }

    fn default_x_dimension(&self, ui: &conrod_core::Ui) -> Dimension
    {
        Dimension::Of(ui.window, None)
    }

    fn default_y_dimension(&self, ui: &conrod_core::Ui) -> Dimension
    {
        Dimension::Of(ui.window, None)
    }
}