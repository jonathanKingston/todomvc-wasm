use crate::controller::ControllerMessage;
use crate::dbg;
use crate::element::Element;
use crate::store::ItemList;
use crate::{Message, Scheduler};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

use crate::template::Template;

const ENTER_KEY: u32 = 13;
const ESCAPE_KEY: u32 = 27;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

/// Messages that represent the methods to be called on the View
pub enum ViewMessage {
    UpdateFilterButtons(String),
    ClearNewTodo(),
    ShowItems(ItemList),
    SetItemsLeft(usize),
    SetClearCompletedButtonVisibility(bool),
    SetCompleteAllCheckbox(bool),
    SetMainVisibility(bool),
    RemoveItem(String),
    EditItemDone(String, String),
    SetItemComplete(String, bool),
}

fn item_id(element: &web_sys::EventTarget) -> Option<String> {
    //TODO ugly reformat
    let dyn_el: Option<&web_sys::Node> = wasm_bindgen::JsCast::dyn_ref(element);
    if let Some(element_node) = dyn_el {
        element_node.parent_node().map(|parent| {
            let mut res = None;
            if let Some(e) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&parent) {
                if e.dataset().get("id") != "" {
                    res = Some(e.dataset().get("id"))
                }
            };
            if None == res {
                if let Some(ep) = parent.parent_node() {
                    if let Some(dyn_el) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&ep)
                    {
                        res = Some(dyn_el.dataset().get("id"));
                    }
                }
            }
            res.unwrap()
        })
    } else {
        None
    }
}

/// Presentation layer
#[wasm_bindgen]
pub struct View {
    sched: RefCell<Rc<Scheduler>>,
    todo_list: Element,
    todo_item_counter: Element,
    clear_completed: Element,
    main: Element,
    toggle_all: Element,
    new_todo: Element,
    callbacks: Vec<(web_sys::EventTarget, String, Closure<FnMut()>)>,
}

impl View {
    /// Construct a new view
    pub fn new(sched: Rc<Scheduler>) -> Option<View> {
        let todo_list = Element::qs(".todo-list")?;
        let todo_item_counter = Element::qs(".todo-count")?;
        let clear_completed = Element::qs(".clear-completed")?;
        let main = Element::qs(".main")?;
        let toggle_all = Element::qs(".toggle-all")?;
        let new_todo = Element::qs(".new-todo")?;
        Some(View {
            sched: RefCell::new(sched),
            todo_list,
            todo_item_counter,
            clear_completed,
            main,
            toggle_all,
            new_todo,
            callbacks: Vec::new(),
        })
    }

    pub fn init(&mut self) {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                dbg("got doc");

                let sched = self.sched.clone();
                let set_page = Closure::wrap(Box::new(move || {
                    dbg("new hash");
                    if let Some(location) = document.location() {
                        if let Ok(hash) = location.hash() {
                            dbg("calling");
                            // TODO refactor back into fn
                            // Was: &self.add_message(ControllerMessage::SetPage(hash));
                            if let Ok(sched) = &(sched.try_borrow_mut()) {
                                sched.add_message(Message::Controller(ControllerMessage::SetPage(
                                    hash,
                                )));
                            }
                            // TODO refactor back into fn
                        }
                    }
                }) as Box<FnMut()>);

                let window_et: web_sys::EventTarget = window.into();
                window_et.add_event_listener_with_callback(
                    "hashchange",
                    set_page.as_ref().unchecked_ref(),
                );
                set_page.forget(); // Cycle collect this
                                   //self.callbacks.push((window_et, "hashchange".to_string(), set_page));
                self.bind_add_item();
                self.bind_edit_item_save();
                self.bind_edit_item_cancel();
                self.bind_remove_item();
                self.bind_toggle_item();
                self.bind_edit_item();
                self.bind_remove_completed();
                self.bind_toggle_all();
            }
        }
    }

    fn bind_edit_item(&mut self) {
        self.todo_list.delegate(
            "li label",
            "dblclick",
            |e: web_sys::Event| {
                if let Some(target) = e.target() {
                    if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(target) {
                        View::edit_item(el);
                    }
                }
            },
            false,
        );
    }

    /// Put an item into edit mode.
    fn edit_item(target: web_sys::Element) {
        let target_node: web_sys::Node = target.into();
        if let Some(parent_element) = target_node.parent_element() {
            let parent_node: web_sys::Node = parent_element.into();
            if let Some(list_item) = parent_node.parent_element() {
                list_item.class_list().add_1("editing");
                if let Some(input) = create_element("input") {
                    input.set_class_name("edit");
                    let list_item_node: web_sys::Node = list_item.into();
                    list_item_node.append_child(&input.into());
                }
            }
        }
        if let Some(el) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&target_node) {
            if let Some(input_el) =
                wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target_node)
            {
                input_el.set_value(&el.inner_text());
            }
            el.focus();
        }
    }

    /// Used by scheduler to convert a `ViewMessage` into a function call on the `View`
    pub fn call(&mut self, method_name: ViewMessage) {
        use self::ViewMessage::*;
        match method_name {
            UpdateFilterButtons(route) => self.update_filter_buttons(route),
            ClearNewTodo() => self.clear_new_todo(),
            ShowItems(item_list) => self.show_items(item_list),
            SetItemsLeft(count) => self.set_items_left(count),
            SetClearCompletedButtonVisibility(visible) => {
                self.set_clear_completed_button_visibility(visible)
            }
            SetCompleteAllCheckbox(complete) => self.set_complete_all_checkbox(complete),
            SetMainVisibility(complete) => self.set_main_visibility(complete),
            RemoveItem(id) => self.remove_item(id),
            EditItemDone(id, title) => self.edit_item_done(id, title),
            SetItemComplete(id, title) => self.set_item_complete(id, title),
        }
    }

    /// Populate the todo list with a list of items.
    fn show_items(&mut self, items: ItemList) {
        self.todo_list.set_inner_html(Template::item_list(&items));
    }

    /// Gets the selector to find a todo item in the DOM
    fn get_selector_string(id: String) -> String {
        let mut selector = String::from("[data-id=\"");
        selector.push_str(id.as_str());
        selector.push_str("\"]");
        selector
    }

    /// Remove an item from the view.
    fn remove_item(&mut self, id: String) {
        let elem = Element::qs(&View::get_selector_string(id));

        dbg("removing node -a");
        if let Some(elem) = elem {
            self.todo_list.remove_child(elem);
        }
    }

    /// Set the number in the 'items left' display.
    fn set_items_left(&mut self, items_left: usize) {
        // TODO what is items left?
        self.todo_item_counter
            .set_inner_html(Template::item_counter(items_left));
    }

    /// Set the visibility of the "Clear completed" button.
    fn set_clear_completed_button_visibility(&mut self, visible: bool) {
        self.clear_completed.set_visibility(visible);
    }

    /// Set the visibility of the main content and footer.
    fn set_main_visibility(&mut self, visible: bool) {
        self.main.set_visibility(visible);
    }

    /// Set the checked state of the Complete All checkbox.
    fn set_complete_all_checkbox(&mut self, checked: bool) {
        self.toggle_all.set_checked(checked);
    }

    /// Change the appearance of the filter buttons based on the route.
    fn update_filter_buttons(&self, route: String) {
        if let Some(mut el) = Element::qs(".filters .selected") {
            el.set_class_name(String::new());
        }

        let mut selector = String::from(".filters [href=\"");
        selector.push_str(route.as_str());
        selector.push_str("\"]");

        if let Some(mut el) = Element::qs(&selector) {
            el.set_class_name(String::from("selected"));
        }
    }

    /// Clear the new todo input
    fn clear_new_todo(&mut self) {
        self.new_todo.set_value(String::new());
    }

    /// Render an item as either completed or not.
    fn set_item_complete(&self, id: String, completed: bool) {
        if let Some(mut list_item) = Element::qs(&View::get_selector_string(id)) {
            let class_name = if completed { "completed" } else { "" };
            list_item.set_class_name(String::from(class_name));

            // In case it was toggled from an event and not by clicking the checkbox
            if let Some(mut el) = list_item.qs_from("input") {
                el.set_checked(completed);
            }
        }
    }

    /// Bring an item out of edit mode.
    fn edit_item_done(&self, id: String, title: String) {
        if let Some(mut list_item) = Element::qs(&View::get_selector_string(id)) {
            if let Some(input) = list_item.qs_from("input.edit") {
                list_item.class_list_remove(String::from("editing"));

                if let Some(mut list_item_label) = list_item.qs_from("label") {
                    list_item_label.set_text_content(title);
                }

                list_item.remove_child(input);
            }
        }
    }

    fn bind_add_item(&mut self) {
        let sched = self.sched.clone();
        let cb = move |event: web_sys::Event| {
            if let Some(target) = event.target() {
                if let Some(input_el) =
                    wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target)
                {
                    let v = input_el.value(); // TODO remove with nll
                    let title = v.trim();
                    if title != "" {
                        if let Ok(sched) = &(sched.try_borrow_mut()) {
                            sched.add_message(Message::Controller(ControllerMessage::AddItem(
                                String::from(title),
                            )));
                        }
                    }
                }
            }
        };
        self.new_todo.add_event_listener("change", cb);
    }

    fn bind_remove_completed(&mut self) {
        let sched = self.sched.clone();
        let handler = move |_| {
            if let Ok(sched) = &(sched.try_borrow_mut()) {
                sched.add_message(Message::Controller(ControllerMessage::RemoveCompleted()));
            }
        };
        self.clear_completed.add_event_listener("click", handler);
    }

    fn bind_toggle_all(&mut self) {
        let sched = self.sched.clone();
        self.clear_completed
            .add_event_listener("click", move |event: web_sys::Event| {
                if let Some(target) = event.target() {
                    if let Some(input_el) =
                        wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target)
                    {
                        if let Ok(sched) = &(sched.try_borrow_mut()) {
                            sched.add_message(Message::Controller(ControllerMessage::ToggleAll(
                                input_el.checked(),
                            )));
                        }
                    }
                }
            });
    }

    fn bind_remove_item(&mut self) {
        let sched = self.sched.clone();
        self.todo_list.delegate(
            ".destroy",
            "click",
            move |e: web_sys::Event| {
                if let Some(target) = e.target() {
                    if let Some(item_id) = item_id(&target) {
                        if let Ok(sched) = &(sched.try_borrow_mut()) {
                            sched.add_message(Message::Controller(ControllerMessage::RemoveItem(
                                item_id,
                            )));
                        }
                    }
                }
            },
            false,
        );
    }

    fn bind_toggle_item(&mut self) {
        let sched = self.sched.clone();
        self.todo_list.delegate(
            ".toggle",
            "click",
            move |e: web_sys::Event| {
                if let Some(target) = e.target() {
                    if let Some(input_el) =
                        wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target)
                    {
                        if let Some(item_id) = item_id(&target) {
                            if let Ok(sched) = &(sched.try_borrow_mut()) {
                                sched.add_message(Message::Controller(
                                    ControllerMessage::ToggleItem(item_id, input_el.checked()),
                                ));
                            }
                        }
                    }
                }
            },
            false,
        );
    }

    fn bind_edit_item_save(&mut self) {
        let sched = self.sched.clone();

        self.todo_list.delegate(
            "li .edit",
            "blur",
            move |e: web_sys::Event| {
                if let Some(target) = e.target() {
                    if let Some(target_el) =
                        wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&target)
                    {
                        if target_el.dataset().get("iscanceled") != "true" {
                            if let Some(input_el) =
                                wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target)
                            {
                                if let Some(item) = item_id(&target) {
                                    dbg("calling -a");
                                    // TODO refactor back into fn
                                    // Was: &self.add_message(ControllerMessage::SetPage(hash));
                                    if let Ok(sched) = &(sched.try_borrow_mut()) {
                                        dbg("sending -a");
                                        sched.add_message(Message::Controller(
                                            ControllerMessage::EditItemSave(item, input_el.value()),
                                        ));
                                    }

                                    // TODO refactor back into fn
                                }
                            }
                        }
                    }
                }
            },
            true,
        );

        // Remove the cursor from the input when you hit enter just like if it were a real form
        self.todo_list.delegate(
            "li .edit",
            "keypress",
            |e: web_sys::Event| {
                if let Some(key_e) = wasm_bindgen::JsCast::dyn_ref::<web_sys::KeyboardEvent>(&e) {
                    if key_e.key_code() == ENTER_KEY {
                        if let Some(target) = e.target() {
                            if let Some(el) =
                                wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&target)
                            {
                                el.blur();
                            }
                        }
                    }
                }
            },
            false,
        );
    }

    fn bind_edit_item_cancel(&mut self) {
        let sched = self.sched.clone();
        self.todo_list.delegate(
            "li .edit",
            "keyup",
            move |e: web_sys::Event| {
                if let Some(key_e) = wasm_bindgen::JsCast::dyn_ref::<web_sys::KeyboardEvent>(&e) {
                    if key_e.key_code() == ESCAPE_KEY {
                        if let Some(target) = e.target() {
                            if let Some(el) =
                                wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&target)
                            {
                                el.dataset().set("iscanceled", "true");
                                el.blur();
                            }

                            if let Some(item_id) = item_id(&target) {
                                if let Ok(sched) = &(sched.try_borrow_mut()) {
                                    sched.add_message(Message::Controller(
                                        ControllerMessage::EditItemCancel(item_id),
                                    ));
                                }
                            }
                        }
                    }
                }
            },
            false,
        );
    }
}

fn create_element(tag: &str) -> Option<web_sys::Element> {
    web_sys::window()?.document()?.create_element(tag).ok()
}

impl Drop for View {
    fn drop(&mut self) {
        dbg("calling drop on view");
        let callbacks: Vec<(web_sys::EventTarget, String, Closure<FnMut()>)> =
            self.callbacks.drain(..).collect();
        for callback in callbacks {
            callback.0.remove_event_listener_with_callback(
                callback.1.as_str(),
                &callback.2.as_ref().unchecked_ref(),
            );
        }
    }
}
