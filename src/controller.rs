use crate::store::*;
use crate::view::{View, ViewMessage};
use js_sys::Date;
use crate::{Scheduler, Message};

use std::rc::{Weak, Rc};
use std::cell::RefCell;

pub struct Controller {
    store: Store,
    view: Option<View>,
//    app: RefCell<Option<Weak<App>>>,
    sched: RefCell<Option<Weak<Scheduler>>>,
    active_route: String,
    last_active_route: String,
}

pub enum ControllerMessage {
    AddItem(String),
    SetPage(String),
}

impl Controller {
    pub fn new(store: Store, view: Option<View>, sched: Weak<Scheduler>) -> Controller {
        let mut controller = Controller {
            store,
            view,
            sched: RefCell::new(Some(sched)),
            active_route: "".into(),
            last_active_route: "".into(),
        };
/*
        if let Some(ref mut view) = controller.view {
            view.bind_edit_item_save(|a, b| {
                controller.edit_item_save(a.into(), b.to_string());
            });
        }
*/
        /* 
  		view.bind_add_item(controller.add_item);
  		view.bind_edit_item_cancel(controller.edit_item_cancel);
  		view.bind_remove_item(controller.remove_item);
  		view.bind_toggle_item((id, completed) => {
  			controller.toggle_completed(id, completed);
  			controller._filter();
  		});
  		view.bind_remove_completed(controller.removeCompletedItems.bind(controller));
  		view.bind_toggle_all(controller.toggleAll.bind(controller));
*/

        controller
    }

    /// Take ownership of the view
    pub fn set_view(&mut self, view: View) {
        self.view = Some(view);
    }

    pub fn call(&mut self, method_name: ControllerMessage) {
        use self::ControllerMessage::*;
        match method_name {
            AddItem(title) => self.add_item(title),
            SetPage(hash) => self.set_page(hash),
        }
    }

    fn add_message(&self, view_message: ViewMessage) {
        if let Some(ref sched) = *self.sched.borrow_mut() {
            if let Some(sched) = sched.upgrade() {
                sched.add_message(Message::View(view_message));
            }
        }
    }

    pub fn set_page(&mut self, raw: String) {
        let v = wasm_bindgen::JsValue::from_str(&format!("{}", "hay 2"));
        web_sys::console::log_1(&v);
        let route = raw.replace(r#"/^#\//"#, "");
        self.active_route = route.clone();
        self._filter(false);
        let v = wasm_bindgen::JsValue::from_str(&format!("{}", "controller 22"));
        web_sys::console::log_1(&v);
        self.add_message(ViewMessage::UpdateFilterButtons(route));
    }

    /// Add an Item to the Store and display it in the list.
    fn add_item(&mut self, title: String) {
        self.store.insert(Item {
            id: Date::now() as usize,
            title,
            completed: false,
        });
        self.add_message(ViewMessage::ClearNewTodo());
        self._filter(true);
    }

    /// Save an Item in edit.
    fn edit_item_save(&mut self, id: usize, title: String) {
        if title.len() > 0 {
            self.store.update(ItemUpdate::Title {
                id,
                title: title.clone(),
            });
            if let Some(ref mut view) = self.view {
                view.edit_item_done(id, &title);
            }
        } else {
            self.remove_item(id);
        }
    }

    /// Cancel the item editing mode.
    fn edit_item_cancel(&mut self, id: usize) {
        if let Some(data) = self.store.find(ItemQuery::Id { id }) {
            if let Some(todo) = data.get(0) {
                let title = &todo.title;
                if let Some(ref mut view) = self.view {
                    view.edit_item_done(id, &title);
                }
            }
        }
    }

    /// Remove the data and elements related to an Item.
    fn remove_item(&mut self, id: usize) {
        self.store.remove(ItemQuery::Id { id });
        self._filter(false);
        if let Some(ref mut view) = self.view {
            view.remove_item(id);
        }
    }

    /// Remove all completed items.
    fn remove_completed_items(&mut self) {
        self.store.remove(ItemQuery::Completed { completed: true });
        self._filter(false);
    }

    /// Update an Item in storage based on the state of completed.
    fn toggle_completed(&mut self, id: usize, completed: bool) {
        self.store.update(ItemUpdate::Completed { id, completed });
        if let Some(ref mut view) = self.view {
            view.set_item_complete(id, completed);
        }
    }

    /// Set all items to complete or active.
    fn toggle_all(&mut self, completed: bool) {
        let mut vals = Vec::new();
        self.store
            .find(ItemQuery::Completed {
                completed: !completed,
            }).map(|data| {
                for item in data.iter() {
                    vals.push(item.id);
                }
            });
        for id in vals.iter() {
            self.toggle_completed(*id, completed);
        }
        self._filter(false);
    }

    /// Refresh the list based on the current route.
    fn _filter(&mut self, force: bool) {
        let route = &self.active_route;

        if force || self.last_active_route != "" || &self.last_active_route != route {
            let query = match route.as_str() {
                "completed" => ItemQuery::Completed { completed: true },
                "active" => ItemQuery::Completed { completed: false },
                _ => ItemQuery::EmptyItemQuery,
            };
            if let Some(res) = self.store.find(query) {
/*
                if let Some(ref mut view) = self.view {
                    view.show_items(res);
                }
*/
                //self.add_message(ViewMessage::ShowItems(res));
            }
        }

        if let Some((total, active, completed)) = self.store.count() {
            self.add_message(ViewMessage::SetItemsLeft(active));
            self.add_message(ViewMessage::SetClearCompletedButtonVisibility(completed > 0));
            self.add_message(ViewMessage::SetCompleteAllCheckbox(completed == total));
            self.add_message(ViewMessage::SetMainVisibility(total > 0));
        }

        self.last_active_route = route.to_string();
    }
}
