use crate::controller::ControllerMessage;
use crate::dbg;
use crate::store::{ItemList, ItemListTrait};
use crate::{Message, Scheduler};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;

use crate::template::Template;

const ENTER_KEY: u32 = 13;
const ESCAPE_KEY: u32 = 27;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

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

/**
  TODO known bugs
  - Refresh now doesn't show the results despite being stored
  - filter and delete sometimes list the wrong number of items
*/

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
                let e_node: web_sys::Node = parent.into();
                e_node.parent_node().map(|ep| {
                    if let Some(dyn_el) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&ep)
                    {
                        res = Some(dyn_el.dataset().get("id"));
                    }
                });
            }
            res.unwrap()
        })
    } else {
        None
    }
}

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
    pub fn new(sched: Rc<Scheduler>) -> Option<View> {
        let todo_list = Element::qs(".todo-list")?;
        let todo_item_counter = Element::qs(".todo-count")?;
        let clear_completed = Element::qs(".clear-completed")?;
        let main = Element::qs(".main")?;
        let toggle_all = Element::qs(".toggle-all")?;
        let new_todo = Element::qs(".new-todo")?;
        let mut view = View {
            sched: RefCell::new(sched),
            todo_list,
            todo_item_counter,
            clear_completed,
            main,
            toggle_all,
            new_todo,
            callbacks: Vec::new(),
        };

        Some(view)
    }

    pub fn init(&mut self) {
        dbg("got init");
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

                            dbg("sending");
                            let ref sched = *sched.borrow_mut();
                            sched
                                .add_message(Message::Controller(ControllerMessage::SetPage(hash)));
                            // TODO refactor back into fn
                        }
                    }
                }) as Box<FnMut()>);

                let window_et: web_sys::EventTarget = window.into();
                /*
                let c = set_page.as_ref().unchecked_ref();
                window_et.add_event_listener_with_callback(
                    "load",
                    set_page.as_ref().unchecked_ref(),
                );
                self.callbacks.push((window_et, "load".to_string(), set_page));
*/
                dbg("about to add hashchange");
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

    pub fn bind_edit_item(&mut self) {
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
    pub fn show_items(&self, items: ItemList) {
        dbg("shhhhh");
        // TODO what is items?
        if let Some(ref el) = self.todo_list.el {
            dbg("itemss");
            el.set_inner_html(&Template::item_list(items));
        }
    }

    /// Remove an item from the view.
    pub fn remove_item(&self, id: String) {
        let elem = Element::qs(&format!("[data-id=\"{}\"]", id));

        dbg("removing node -a");
        if let Some(elem) = elem {
            if let Some(ref el) = self.todo_list.el {
                if let Some(dyn_el) = wasm_bindgen::JsCast::dyn_ref(el): Option<&web_sys::Node> {
                    if let Some(elem) = elem.into(): Option<web_sys::Node> {
                        dbg("removing node -");
                        dyn_el.remove_child(&elem);
                        //elem.remove();
                    }
                }
            }
        }
    }

    /// Set the number in the 'items left' display.
    pub fn set_items_left(&self, items_left: usize) {
        // TODO what is items left?
        if let Some(ref todo_item_counter) = self.todo_item_counter.el {
            todo_item_counter.set_inner_html(&Template::item_counter(items_left));
        }
    }

    /// Set the visibility of the "Clear completed" button.
    pub fn set_clear_completed_button_visibility(&self, visible: bool) {
        if let Some(ref clear_completed) = self.clear_completed.el {
            dbg("got a button!");
            set_visibility(clear_completed, visible);
        }
    }

    /// Set the visibility of the main content and footer.
    pub fn set_main_visibility(&self, visible: bool) {
        if let Some(ref main) = self.main.el {
            dbg("got a main!");
            set_visibility(main, visible);
        }
    }

    /// Set the checked state of the Complete All checkbox.
    pub fn set_complete_all_checkbox(&mut self, checked: bool) {
        if let Some(toggle_all) = self.toggle_all.el.take() {
            if let Some(toggle_all) =
                wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&toggle_all)
            {
                toggle_all.set_checked(checked);
            }
            self.toggle_all.el = Some(toggle_all);
        }
    }

    /// Change the appearance of the filter buttons based on the route.
    pub fn update_filter_buttons(&self, route: String) {
        if let Some(el) = Element::qs(".filters .selected") {
            if let Some(el) = el.el {
                el.set_class_name("");
            }
        }
        if let Some(el) = Element::qs(&format!(".filters [href=\"#{}\"]", route)) {
            if let Some(el) = el.el {
                el.set_class_name("selected");
            }
        }
    }

    /// Clear the new todo input
    pub fn clear_new_todo(&mut self) {
        if let Some(new_todo) = self.new_todo.el.take() {
            if let Some(input_el) =
                wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&new_todo)
            {
                input_el.set_value("");
            }
            self.new_todo.el = Some(new_todo);
        }
    }

    /// Render an item as either completed or not.
    pub fn set_item_complete(&self, id: String, completed: bool) {
        if let Some(mut list_item) = Element::qs(&format!("[data-id=\"{}\"]", id)) {
            let class_name = if completed { "completed" } else { "" };
            if let Some(ref list_item) = list_item.el {
                list_item.set_class_name(class_name);
            }

            // In case it was toggled from an event and not by clicking the checkbox
            if let Some(el) = list_item.qs_from("input") {
                if let Some(input_el) =
                    wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&el)
                {
                    input_el.set_checked(completed);
                }
            }
        }
    }

    /// Bring an item out of edit mode.
    pub fn edit_item_done(&self, id: String, title: String) {
        let list_item = Element::qs(&format!("[data-id=\"{}\"]", id));

        if let Some(mut list_item) = Element::qs(&format!("[data-id=\"{}\"]", id)) {
            if let Some(input) = list_item.qs_from("input.edit") {
                if let Some(ref list_item_el) = list_item.el {
                    list_item_el.class_list().remove_1("editing");
                }

                if let Some(list_item_label) = list_item.qs_from("label") {
                    let list_item_label_node: web_sys::Node = list_item_label.into();
                    list_item_label_node.set_text_content(Some(title.as_str()));
                }

                if let Some(list_item_node) = list_item.into(): Option<web_sys::Node> {
                    list_item_node.remove_child(&input.into());
                }
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
                        let ref sched = *sched.borrow_mut();
                        sched.add_message(Message::Controller(ControllerMessage::AddItem(
                            String::from(title),
                        )));
                    }
                }
            }
        };
        self.new_todo.add_event_listener("change", cb);
    }

    fn bind_remove_completed(&mut self) {
        let sched = self.sched.clone();
        let handler = move |_| {
            let ref sched = *sched.borrow_mut();
            sched.add_message(Message::Controller(ControllerMessage::RemoveCompleted()));
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
                        let ref sched = *sched.borrow_mut();
                        sched.add_message(Message::Controller(ControllerMessage::ToggleAll(
                            input_el.checked(),
                        )));
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
                        let ref sched = *sched.borrow_mut();
                        sched.add_message(Message::Controller(ControllerMessage::RemoveItem(
                            item_id,
                        )));
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
                            let ref sched = *sched.borrow_mut();
                            sched.add_message(Message::Controller(ControllerMessage::ToggleItem(
                                item_id,
                                input_el.checked(),
                            )));
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
                                    let ref sched = *sched.borrow_mut();
                                    dbg("sending -a");
                                    sched.add_message(Message::Controller(
                                        ControllerMessage::EditItemSave(item, input_el.value()),
                                    ));

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
                                let ref sched = *sched.borrow_mut();
                                sched.add_message(Message::Controller(
                                    ControllerMessage::EditItemCancel(item_id),
                                ));
                            }
                        }
                    }
                }
            },
            false,
        );
    }
}

struct Element {
    el: Option<web_sys::Element>,
}

impl From<Element> for Option<web_sys::Node> {
    fn from(obj: Element) -> Option<web_sys::Node> {
        if let Some(el) = obj.el {
            Some(el.into())
        } else {
            None
        }
    }
}

impl From<Element> for Option<web_sys::EventTarget> {
    fn from(obj: Element) -> Option<web_sys::EventTarget> {
        if let Some(el) = obj.el {
            Some(el.into())
        } else {
            None
        }
    }
}

impl Element {
    fn qs(selector: &str) -> Option<Element> {
        let body: web_sys::Element = web_sys::window()?.document()?.body()?.into();
        let el = body.query_selector(selector).ok()?;
        Some(Element { el })
    }

    fn add_event_listener<T>(&mut self, event_name: &str, handler: T)
    where
        // TODO rewrite without static
        T: 'static + FnMut(web_sys::Event) -> (),
    {
        let cb = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
        if let Some(el) = self.el.take() {
            let el_et: web_sys::EventTarget = el.into();
            el_et.add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref());
            cb.forget(); // TODO cycle collect this
            if let Ok(el) = wasm_bindgen::JsCast::dyn_into(el_et): Result<web_sys::Element, _> {
                self.el = Some(el);
            }
        }
    }

    // TODO fix static lifetimes
    fn delegate<T>(
        &mut self,
        // element: web_sys::Element,
        // element: Element,
        selector: &'static str,
        event: &str,
        mut handler: T,
        use_capture: bool,
    ) where
        T: 'static + FnMut(web_sys::Event) -> (),
    {
        if let Some(el) = self.el.take() {
            //  if let Some(element_et) = self.into(): Option<web_sys::EventTarget> {
            //let element_et: web_sys::EventTarget = el.into();
            if let Some(dyn_el) = wasm_bindgen::JsCast::dyn_ref(&el): Option<&web_sys::EventTarget>
            {
                //let rc_el = Rc::new(el);
                //let cl_el = rc_el.clone();
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        //let tg_el: web_sys::Element = self.el;
                        // TODO document selector to the target element
                        let tg_el = document;

                        let cb = Closure::wrap(Box::new(move |event: web_sys::Event| {
                            dbg("got fn call delegated");
                            if let Some(target_element) = event.target() {
                                let dyn_target_el: Option<
                                    &web_sys::Node,
                                > = wasm_bindgen::JsCast::dyn_ref(&target_element);
                                if let Some(target_element) = dyn_target_el {
                                    if let Ok(potential_elements) =
                                        tg_el.query_selector_all(selector)
                                    {
                                        //let hasMatch = Array.prototype.indexOf.call(potential_elements, target_element) >= 0;
                                        dbg("got fn call delegated arse");
                                        let mut has_match = false;
                                        dbg(format!(
                                            "len: {} {}",
                                            potential_elements.length(),
                                            selector
                                        ).as_str());
                                        for i in 0..potential_elements.length() {
                                            if let Some(el) = potential_elements.get(i) {
                                                if target_element.is_equal_node(Some(&el)) {
                                                    has_match = true;
                                                }
                                                break;
                                            }
                                        }

                                        if has_match {
                                            dbg("got fn call delegated match");
                                            //handler.call(target_element, event);
                                            handler(event);
                                        }
                                    }
                                }
                            }
                        }) as Box<FnMut(_)>);

                        dyn_el.add_event_listener_with_callback_and_bool(
                            //element_et.add_event_listener_with_callback_and_bool(
                            event,
                            cb.as_ref().unchecked_ref(),
                            use_capture,
                        );
                        cb.forget(); // TODO cycle collect
                    }
                }
            }
            self.el = Some(el);
        }
    }

    fn qs_from(&mut self, selector: &str) -> Option<web_sys::Element> {
        let el = self.el.take();
        let mut found_el = None;
        if let Some(el) = self.el.take() {
            found_el = el.query_selector(selector).ok()?;
            self.el = Some(el);
        }
        found_el
    }
}

fn set_visibility(el: &web_sys::Element, visible: bool) {
    let dyn_el: Option<&web_sys::HtmlElement> = wasm_bindgen::JsCast::dyn_ref(el);
    if let Some(el) = dyn_el {
        el.set_hidden(!visible);
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
