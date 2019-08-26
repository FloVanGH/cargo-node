use crate::{prelude::*, render::RenderContext2D, utils::*};

pub struct RectangleRenderObject;

impl RectangleRenderObject {
    // Renders rectangle without border and radius.
    fn render_rect_path(
        &self,
        render_context_2_d: &mut RenderContext2D,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        brush: Brush,
    ) {
        render_context_2_d.set_fill_style(brush);
        render_context_2_d.fill_rect(x, y, width, height);
    }

    // Renders rectangle with border and without radius.
    fn render_bordered_rect_path(
        &self,
        render_context_2_d: &mut RenderContext2D,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        brush: Brush,
        border_brush: Brush,
        border_thickness: Thickness,
    ) {
        // border
        self.render_rect_path(render_context_2_d, x, y, width, height, border_brush);

        // content
        self.render_rect_path(
            render_context_2_d,
            x + border_thickness.left,
            y + border_thickness.top,
            width - border_thickness.left - border_thickness.right,
            height - border_thickness.top - border_thickness.bottom,
            brush,
        );
    }

    // Builds rectangle path with radius and without border.
    fn render_rounded_rect_path(
        &self,
        render_context_2_d: &mut RenderContext2D,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
        brush: Brush,
    ) {
        let m_pi = 3.14159265;
        let degrees = m_pi / 180.0;

        render_context_2_d.save();
        render_context_2_d.begin_path();
        render_context_2_d.arc(
            x + width - radius,
            y + radius,
            radius,
            -90.0 * degrees,
            0.0 * degrees,
            false,
        );
        render_context_2_d.arc(
            x + width - radius,
            y + height - radius,
            radius,
            0.0 * degrees,
            90.0 * degrees,
            false,
        );
        render_context_2_d.arc(
            x + radius,
            y + height - radius,
            radius,
            90.0 * degrees,
            180.0 * degrees,
            false,
        );
        render_context_2_d.arc(
            x + radius,
            y + radius,
            radius,
            180.0 * degrees,
            270.0 * degrees,
            false,
        );

        render_context_2_d.set_fill_style(brush);

        render_context_2_d.fill();
        render_context_2_d.close_path();
        render_context_2_d.restore();
    }

    // Renders rectangle with border and radius.
    fn render_rounded_bordered_rect_path(
        &self,
        render_context_2_d: &mut RenderContext2D,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
        brush: Brush,
        border_brush: Brush,
        border_thickness: Thickness,
    ) {
        // border
        self.render_rounded_rect_path(
            render_context_2_d,
            x,
            y,
            width,
            height,
            radius,
            border_brush,
        );

        // content
        self.render_rounded_rect_path(
            render_context_2_d,
            x + border_thickness.left,
            y + border_thickness.top,
            width - border_thickness.left - border_thickness.right,
            height - border_thickness.top - border_thickness.right,
            radius,
            brush,
        );
    }
}

impl Into<Box<dyn RenderObject>> for RectangleRenderObject {
    fn into(self) -> Box<dyn RenderObject> {
        Box::new(self)
    }
}

impl RenderObject for RectangleRenderObject {
    fn render_self(&self, context: &mut Context<'_>, global_position: &Point) {
        let (bounds, background, border_radius, border_thickness, border_brush) = {
            let widget = context.widget();
            (
                widget.clone::<Bounds>(),
                widget.get::<Background>().0.clone(),
                widget.clone_or_default::<BorderRadius>().0,
                widget.clone_or_default::<BorderThickness>().0,
                widget.clone_or_default::<BorderBrush>().0,
            )
        };

        let has_thickness = border_thickness.left > 0.0
            || border_thickness.top > 0.0
            || border_thickness.right > 0.0
            || border_thickness.bottom > 0.0;

        if border_radius > 0.0 {
            if has_thickness {
                self.render_rounded_bordered_rect_path(
                    context.render_context_2_d(),
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                    border_radius,
                    background,
                    border_brush,
                    border_thickness,
                );
            } else {
                self.render_rounded_rect_path(
                    context.render_context_2_d(),
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                    border_radius,
                    background,
                );
            }
        } else {
            if has_thickness {
                self.render_bordered_rect_path(
                    context.render_context_2_d(),
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                    background,
                    border_brush,
                    border_thickness,
                );
            } else {
                self.render_rect_path(
                    context.render_context_2_d(),
                    global_position.x + bounds.x(),
                    global_position.y + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                    background,
                );
            }
        }
    }
}
