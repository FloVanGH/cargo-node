use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    f64,
    rc::Rc,
};

use dces::prelude::Entity;

use crate::{prelude::*, render::RenderContext2D, tree::Tree, utils::prelude::*};

use super::Layout;

/// IMPORTANT: The scroll layout will only work for the text box now. A update will follow!!!!
#[derive(Default)]
pub struct ScrollLayout {
    old_child_size: Cell<(f64, f64)>,
    desired_size: RefCell<DirtySize>,
    old_offset: Cell<(f64, f64)>,
    old_alignment: Cell<(Alignment, Alignment)>,
}

impl ScrollLayout {
    pub fn new() -> Self {
        ScrollLayout::default()
    }
}

impl Layout for ScrollLayout {
    fn measure(
        &self,
        render_context_2_d: &mut RenderContext2D,
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree>,

        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &ThemeValue,
    ) -> DirtySize {
        if Visibility::get(entity, ecm.component_store()) == VisibilityValue::Collapsed {
            self.desired_size.borrow_mut().set_size(0.0, 0.0);
            return self.desired_size.borrow().clone();
        }

        let horizontal_alignment = HorizontalAlignment::get(entity, ecm.component_store());
        let vertical_alignment = VerticalAlignment::get(entity, ecm.component_store());

        if horizontal_alignment != self.old_alignment.get().1
            || vertical_alignment != self.old_alignment.get().0
        {
            self.desired_size.borrow_mut().set_dirty(true);
        }

        let constraint = Constraint::get(entity, ecm.component_store());

        if constraint.width() > 0.0 {
            self.desired_size.borrow_mut().set_width(constraint.width());
        }

        if constraint.height() > 0.0 {
            self.desired_size
                .borrow_mut()
                .set_height(constraint.height());
        }

        if ecm.entity_store().children[&entity].len() > 0 {
            let mut index = 0;

            loop {
                let child = ecm.entity_store().children[&entity][index];

                if let Some(child_layout) = layouts.borrow().get(&child) {
                    let dirty = child_layout
                        .measure(render_context_2_d, child, ecm, layouts, theme)
                        .dirty()
                        || self.desired_size.borrow().dirty();

                    self.desired_size.borrow_mut().set_dirty(dirty);
                }

                if index + 1 < ecm.entity_store().children[&entity].len() {
                    index += 1;
                } else {
                    break;
                }
            }
        }

        let off = ScrollOffset::get(entity, ecm.component_store());

        if self.old_offset.get().0 != off.x || self.old_offset.get().1 != off.y {
            self.old_offset.set((off.x, off.y));
            self.desired_size.borrow_mut().set_dirty(true);
        }

        self.desired_size.borrow().clone()
    }

    fn arrange(
        &self,
        render_context_2_d: &mut RenderContext2D,
        parent_size: (f64, f64),
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree>,

        layouts: &Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
        theme: &ThemeValue,
    ) -> (f64, f64) {
        if !self.desired_size.borrow().dirty() {
            return self.desired_size.borrow().size();
        }

        let horizontal_alignment = HorizontalAlignment::get(entity, ecm.component_store());
        let vertical_alignment = VerticalAlignment::get(entity, ecm.component_store());
        let margin = Margin::get(entity, ecm.component_store());
        let _padding = Padding::get(entity, ecm.component_store());
        let constraint = Constraint::get(entity, ecm.component_store());

        let size = constraint.perform((
            horizontal_alignment.align_measure(
                parent_size.0,
                self.desired_size.borrow().width(),
                margin.left(),
                margin.right(),
            ),
            vertical_alignment.align_measure(
                parent_size.1,
                self.desired_size.borrow().height(),
                margin.top(),
                margin.bottom(),
            ),
        ));

        if let Ok(bounds) = ecm
            .component_store_mut()
            .borrow_mut_component::<Bounds>(entity)
        {
            bounds.set_width(size.0);
            bounds.set_height(size.1);
        }

        let scroll_viewer_mode = ScrollViewerMode::get(entity, ecm.component_store());

        let available_size = {
            let width = if scroll_viewer_mode.horizontal == ScrollMode::Custom
                || scroll_viewer_mode.horizontal == ScrollMode::Auto
            {
                f64::MAX
            } else {
                size.0
            };

            let height = if scroll_viewer_mode.vertical == ScrollMode::Custom
                || scroll_viewer_mode.vertical == ScrollMode::Auto
            {
                f64::MAX
            } else {
                size.1
            };

            (width, height)
        };

        let off = ScrollOffset::get(entity, ecm.component_store());
        let delta = Delta::get(entity, ecm.component_store());
        let mut offset = (off.x, off.y);

        let old_child_size = self.old_child_size.get();

        if ecm.entity_store().children[&entity].len() > 0 {
            let mut index = 0;

            loop {
                let child = ecm.entity_store().children[&entity][index];

                // let child_margin = get_margin(*child, store);
                let mut child_size = old_child_size;
                let child_vertical_alignment = VerticalAlignment::get(child, ecm.component_store());
                let child_horizontal_alignment =
                    HorizontalAlignment::get(child, ecm.component_store());
                let child_margin = Margin::get(child, ecm.component_store());

                if let Some(child_layout) = layouts.borrow().get(&child) {
                    child_size = child_layout.arrange(
                        render_context_2_d,
                        available_size,
                        child,
                        ecm,
                        layouts,
                        theme,
                    );
                }

                match scroll_viewer_mode.horizontal {
                    ScrollMode::Custom => {
                        if child_size.0 > size.0 {
                            offset.0 = (offset.0 + old_child_size.0 - child_size.0).min(0.0);
                        } else {
                            offset.0 = 0.0;
                        }
                    }
                    ScrollMode::Auto => {
                        // todo: refactor * 1.5
                        offset.0 = (offset.0 + delta.x * 1.5).min(0.0).max(size.0 - child_size.0);
                    }
                    _ => {}
                }

                match scroll_viewer_mode.vertical {
                    ScrollMode::Custom => {
                        if child_size.1 > size.1 {
                            offset.1 = (offset.1 + old_child_size.1 - child_size.1).min(1.1);
                        } else {
                            offset.1 = 1.1;
                        }
                    }
                    ScrollMode::Auto => {
                        // todo: refactor * 1.5
                        offset.1 = (offset.1 + delta.y * 1.5).min(1.1).max(size.1 - child_size.1);
                    }
                    _ => {}
                }

                if let Ok(child_bounds) = ecm
                    .component_store_mut()
                    .borrow_mut_component::<Bounds>(child)
                {
                    // todo: add check
                    if scroll_viewer_mode.horizontal == ScrollMode::Custom
                        || scroll_viewer_mode.horizontal == ScrollMode::Auto
                    {
                        child_bounds.set_x(offset.0);
                    } else {
                        child_bounds.set_x(child_horizontal_alignment.align_position(
                            size.0,
                            child_bounds.width(),
                            child_margin.left(),
                            child_margin.right(),
                        ));
                    }

                    if scroll_viewer_mode.vertical == ScrollMode::Custom
                        || scroll_viewer_mode.vertical == ScrollMode::Auto
                    {
                        child_bounds.set_y(offset.1);
                    } else {
                        child_bounds.set_y(child_vertical_alignment.align_position(
                            size.1,
                            child_bounds.height(),
                            child_margin.top(),
                            child_margin.bottom(),
                        ));
                    }
                }

                if let Ok(off) = ecm
                    .component_store_mut()
                    .borrow_mut_component::<ScrollOffset>(entity)
                {
                    (off.0).x = offset.0;
                    (off.0).y = offset.1;
                }

                self.old_child_size.set(child_size);

                if index + 1 < ecm.entity_store().children[&entity].len() {
                    index += 1;
                } else {
                    break;
                }
            }
        }

        self.desired_size.borrow_mut().set_dirty(false);
        size
    }
}

impl Into<Box<dyn Layout>> for ScrollLayout {
    fn into(self) -> Box<dyn Layout> {
        Box::new(self)
    }
}
