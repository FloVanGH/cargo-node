use crate::prelude::*;

widget!(
    /// The `FontIconBlock` widget is used to draw text. It is not interactive.
    ///
    /// **CSS element:** `font-icon-block`
    FontIconBlock {
        /// Sets or shares the icon property.
        icon: FontIcon,

        /// Sets or shares the icon brush property.
        brush: IconBrush,

        /// Sets or share the icon font size property.
        icon_size: IconSize,

        /// Sets or shares the icon font property.
        font: IconFont,

        /// Sets or shares the css selector property.
        selector: Selector
    }
);

impl Template for FontIconBlock {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("FontIconBlock")
            .selector("font-icon-block")
            .icon("")
            .brush(colors::LINK_WATER_COLOR)
            .icon_size(fonts::ICON_FONT_SIZE_12)
            .font("Material Icons")
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        Box::new(FontIconRenderObject)
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(FixedSizeLayout::new())
    }
}
