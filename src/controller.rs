use crate::dbg;
use crate::store::*;
use crate::view::{View, ViewMessage};
use crate::{Message, Scheduler};
use js_sys::Date;

use std::cell::RefCell;
use std::rc::Weak;

pub struct Controller {
    store: Store,
    view: Option<View>,
    sched: RefCell<Option<Weak<Scheduler>>>,
    active_route: String,
    last_active_route: String,
}

pub enum ControllerMessage {
    AddItem(String),
    SetPage(String),
    EditItemSave(usize, String),
    ToggleItem(usize, bool),
    EditItemCancel(usize),
    RemoveCompleted(),
    RemoveItem(usize),
    ToggleAll(bool),
}

impl Controller {
    pub fn new(store: Store, view: Option<View>, sched: Weak<Scheduler>) -> Controller {
        Controller {
            store,
            view,
            sched: RefCell::new(Some(sched)),
            active_route: "".into(),
            last_active_route: "".into(),
        }
        /* 
  		view.bind_remove_item(controller.remove_item);
  		view.bind_remove_completed(controller.removeCompletedItems.bind(controller));
*/
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
            EditItemSave(id, value) => self.edit_item_save(id, value),
            EditItemCancel(id) => self.edit_item_cancel(id),
            RemoveCompleted() => self.remove_completed_items(),
            RemoveItem(id) => self.remove_item(id),
            ToggleAll(completed) => self.toggle_all(completed),
            ToggleItem(id, completed) => self.toggle_item(id, completed),
        }
    }

    fn toggle_item(&mut self, id: usize, completed: bool) {
        self.toggle_completed(id, completed);
        self._filter(completed);
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
        if !title.is_empty() {
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
            let mut v = None;
            {
                let store = &mut self.store;
                if let Some(res) = store.find(query) {
                    v = Some(res.into());
                }
            }
            if let Some(res) = v {
                self.add_message(ViewMessage::ShowItems(res));
            }
        }

        if let Some((total, active, completed)) = self.store.count() {
            dbg("heyee4");
            self.add_message(ViewMessage::SetItemsLeft(active));
            dbg("heyee3");
            self.add_message(ViewMessage::SetClearCompletedButtonVisibility(
                completed > 0,
            ));
            dbg("heyee2");
            self.add_message(ViewMessage::SetCompleteAllCheckbox(completed == total));
            dbg("heyee 1");
            self.add_message(ViewMessage::SetMainVisibility(total > 0));
        }

        self.last_active_route = route.to_string();
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        dbg("calling drop on Controller");
    }
}
