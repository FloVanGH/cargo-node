use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dces::prelude::{Entity, EntityComponentManager, System};

use crate::{prelude::*, shell::WindowShell, tree::Tree};

/// This system is used to initializes the widgets.
pub struct InitSystem {
    pub shell: Rc<RefCell<WindowShell<WindowAdapter>>>,
    pub states: Rc<RefCell<BTreeMap<Entity, Rc<dyn State>>>>,
    pub render_objects: Rc<RefCell<BTreeMap<Entity, Box<dyn RenderObject>>>>,
    pub layouts: Rc<RefCell<BTreeMap<Entity, Box<dyn Layout>>>>,
    pub handlers: Rc<RefCell<BTreeMap<Entity, Vec<Rc<dyn EventHandler>>>>>,
}

impl InitSystem {
    // init css ids.
    fn init_id(&self, node: Entity, store: &mut ComponentStore, root: Entity) {
        // Add css id to global id map.
        let id = if let Ok(selector) = store.borrow_component::<Selector>(node) {
            if let Some(id) = &selector.0.id {
                Some((node, id.clone()))
            } else {
                None
            }
        } else {
            None
        };

        if let Some((entity, id)) = id {
            if let Ok(global) = store.borrow_mut_component::<Global>(root) {
                global.id_map.insert(id, entity);
            }
        }
    }

    // Read all initial data from css
    fn read_init_from_theme(&self, context: &mut Context) {
        context.widget().update_theme_by_state(true);
    }
}

impl System<Tree> for InitSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree>) {
        let root = ecm.entity_store().root;

        #[cfg(feature = "debug")]
        let debug = true;
        #[cfg(not(feature = "debug"))]
        let debug = false;

        if debug {
            crate::shell::log("\n------ Widget tree ------\n".to_string());

            print_tree(root, 0, ecm);

            crate::shell::log("\n------ Widget tree ------\n".to_string());
        }

        // init css ids
        let window_shell = &mut self.shell.borrow_mut();
        let theme = ecm
            .component_store()
            .borrow_component::<Theme>(root)
            .unwrap()
            .0
            .clone();

        let mut current_node = root;

        loop {
            self.init_id(current_node, ecm.component_store_mut(), root);

            {
                let mut context = Context::new(
                    current_node,
                    ecm,
                    window_shell,
                    &theme,
                    self.render_objects.clone(),
                    self.layouts.clone(),
                    self.handlers.clone(),
                    self.states.clone(),
                );

                if let Some(state) = self.states.borrow().get(&current_node) {
                    state.init(&mut context);
                }

                self.read_init_from_theme(&mut context);
            }

            let mut it = ecm.entity_store().start_node(current_node).into_iter();
            it.next();

            if let Some(node) = it.next() {
                current_node = node;
            } else {
                break;
            }
        }
    }
}

fn print_tree(entity: Entity, depth: usize, ecm: &mut EntityComponentManager<Tree>) {
    let name = Name::get(entity, ecm.component_store());
    let selector = Selector::get(entity, ecm.component_store());

    crate::shell::log(format!(
        "{}{} (entity: {}{})",
        "| ".repeat(depth),
        name,
        entity.0,
        selector
    ));

    for child in ecm.entity_store().clone().children.get(&entity).unwrap() {
        print_tree(*child, depth + 1, ecm);
    }
}
