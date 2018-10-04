#![feature(type_ascription)]

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

extern crate js_sys;
extern crate web_sys;
use std::rc::Rc;
use std::sync::Mutex;

use js_sys::{Date, JSON};

#[wasm_bindgen]
pub fn run() {
 //   App::new("todos-wasmbindgen");
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(store) = Store::new("todos-wasmbindgen") {
                let mut controller = Controller::new(store, None);
                if let Some(mut view) = View::new() {
                    // let template = new Template();
                    //const view = new View(template);

                    //let mut controller = Controller::new(store, view);
                    view.set_controller(Box::new(&controller));
                    controller.set_view(&mut view);
/*
                    let set_page = Closure::wrap(Box::new(move || {
                        if let Some(location) = document.location() {
                            if let Ok(hash) = location.hash() {
                                controller.set_page(hash);
                            }
                        }
                    }) as Box<FnMut()>);
                    let window_et: web_sys::EventTarget = window.into();
                    window_et.add_event_listener_with_callback(
                        "load",
                        set_page.as_ref().unchecked_ref(),
                    );
                    window_et.add_event_listener_with_callback(
                        "hashchange",
                        set_page.as_ref().unchecked_ref(),
                    );
*/
                }
            }
        }
    }
}

/*
struct App<'app> {
  controller: Controller<'app>,
  view: View<'app>,
  store: Store,
}

impl <'app> App <'app> {
    fn new(name: &str) -> Option<App> {
        let view = View::new();
        let store = Store::new(name);
        if let Some(store) = store {
            let controller = Controller::new(&mut store, None);
            if let Some(view) = view {
                return Some(App {
                  controller,
                  view,
                  store
                });
            }
        }
        None
    }
}
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
            res
        })?
    } else {
        None
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
/*
impl From<Element> for Option<web_sys::HtmlInputElement> {
    fn from(obj: Element) -> Option<web_sys::HtmlInputElement> {
        if let Some(el) = obj.el {
            el.into()
        } else {
            None
        }
    }
}
*/
impl Element {
    fn qs(selector: &str) -> Option<Element> {
        let el = web_sys::window()?
            .document()?
            .query_selector(selector)
            .ok()?;
        Some(Element { el })
    }

    fn add_event_listener<T>(&mut self, event_name: &str, handler: T)
    where
        // TODO rewrite without static
        T: 'static + FnMut(web_sys::Event) -> ()
    {
        let cb = Closure::wrap(Box::new(handler) as Box<FnMut(_)>);
        if let Some(el) = self.el.take() {
            let el_et: web_sys::EventTarget = el.into();
            el_et.add_event_listener_with_callback(event_name, cb.as_ref().unchecked_ref());
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
        handler: T,
        use_capture: bool,
    ) where
        T: Fn(web_sys::Event) -> (),
    {
        let cb = Closure::wrap(Box::new(move |event: web_sys::Event| {
            if let Some(target_element) = event.target() {
                let dyn_el: Option<&web_sys::Element> =
                    wasm_bindgen::JsCast::dyn_ref(&target_element);
                if let Some(target_element) = dyn_el {
                    if let Ok(potential_elements) = target_element.query_selector_all(selector) {
                        //let hasMatch = Array.prototype.indexOf.call(potential_elements, target_element) >= 0;
                        let mut has_match = false;
                        for i in 0..potential_elements.length() {
                            if let Some(el) = potential_elements.get(i) {
                                has_match = true;
                                break;
                            }
                        }

                        if has_match {
                            //handler.call(target_element, event);
                        }
                    }
                }
            }
        }) as Box<FnMut(_)>);

        if let Some(el) = self.el.take() {
            //  if let Some(element_et) = self.into(): Option<web_sys::EventTarget> {
            //let element_et: web_sys::EventTarget = el.into();
            if let Some(dyn_el) = wasm_bindgen::JsCast::dyn_ref(&el): Option<&web_sys::EventTarget>
            {
                dyn_el.add_event_listener_with_callback_and_bool(
                    //element_et.add_event_listener_with_callback_and_bool(
                    event,
                    cb.as_ref().unchecked_ref(),
                    use_capture,
                );
            }
            //if let Some(dyn_el) = wasm_bindgen::JsCast::dyn_ref(element_et): web_sys::Element {
            self.el = Some(el);
            //}
        }
        // TODO
    /*
    	// Attach a handler to event for all elements that match the selector,
    	// now or in the future, based on a root element
    	window.$delegate = function (target, selector, type, handler) {
        let cb = Closure::wrap(Box::new(move |event| {
    		  	let target_element = event.target();
    		  	let potentialElements = window.qsa(selector, target);
    		  	let hasMatch = Array.prototype.indexOf.call(potentialElements, targetElement) >= 0;
    
    		  	if (hasMatch) {
    		  		handler.call(targetElement, event);
    		  	}
        }) as Box<FnMut(_)>);
    
    		// https://developer.mozilla.org/en-US/docs/Web/Events/blur
    		let use_capture = type == 'blur' || type == 'focus';
    
        target.add_event_listener_with_callback(type, cb.as_ref().unchecked_ref(), use_capture);
    };
    */    }
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
        el.set_hidden(visible);
    }
}

fn create_element(tag: &str) -> Option<web_sys::Element> {
    web_sys::window()?.document()?.create_element(tag).ok()
}

fn escape_html(val: &str) -> &str {
    val
}
// export const escapeForHTML = s => s.replace(/[&<]/g, c => c === '&' ? '&amp;' : '&lt;');

struct Template {}

impl Template {
    /**
     * Format the contents of a todo list.
     *
     * @param {ItemList} items Object containing keys you want to find in the template to replace.
     * @returns {!string} Contents for a todo list
     *
     * @example
     * view.show({
     *	id: 1,
     *	title: "Hello World",
     *	completed: false,
     * })
     */
    fn item_list(items: ItemListSlice) -> String {
        let mut output = String::from("");
        for item in items.iter() {
            output.push_str(
                r#"
  <li data-id="${item.id}"${item.completed ? ' class="completed"' : ''}>
  	<div class="view">
  		<input class="toggle" type="checkbox" ${item.completed ? 'checked' : ''}>
  		<label>${escapeForHTML(item.title)}</label>
  		<button class="destroy"></button>
  	</div>
  </li>"#,
            );
        }
        return output;
    }

    /**
     * Format the contents of an "items left" indicator.
     *
     * @param {number} activeTodos Number of active todos
     *
     * @returns {!string} Contents for an "items left" indicator
     */
    fn item_counter(active_todos: usize) -> String {
        let plural = if active_todos > 1 { "s" } else { "" };
        return format!("{} item{} left", active_todos, plural);
    }
}

const ENTER_KEY: u32 = 13;
const ESCAPE_KEY: u32 = 27;

struct View<'a> {
    controller: Option<Box<&'a Controller<'a>>>,
    todo_list: Element,
    todo_item_counter: Element,
    clear_completed: Element,
    main: Element,
    toggle_all: Element,
    new_todo: Element,
}

impl <'a> View <'a> {
    fn new() -> Option<View<'a>> {
        let todo_list = Element::qs(".todo-list")?;
        let todo_item_counter = Element::qs(".todo-count")?;
        let clear_completed = Element::qs(".clear-completed")?;
        let main = Element::qs(".main")?;
        let toggle_all = Element::qs(".toggle-all")?;
        let new_todo = Element::qs(".new-todo")?;
        let mut view = View {
            controller: None,
            todo_list,
            todo_item_counter,
            clear_completed,
            main,
            toggle_all,
            new_todo,
        };

        view.todo_list.delegate(
            "li label",
            "dblclick",
            |e: web_sys::Event| {
                if let Some(target) = e.target() {
                    if let Ok(el) = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(target) {
                        view.edit_item(el);
                    }
                }
            },
            false,
        );

        Some(view)
    }

    fn set_controller(&mut self, controller: Box<&'a Controller<'a>>) {
        self.controller = Some(controller);
    }

    /// Put an item into edit mode.
    fn edit_item(&self, target: web_sys::Element) {
        let target_node: web_sys::Node = target.into();
        if let Some(parent_element) = target_node.parent_element() {
            let parent_node: web_sys::Node = parent_element.into();
            if let Some(list_item) = parent_node.parent_element() {
                list_item.class_list().add_1("editing");

                if let Some(input) = create_element("input") {
                    input.set_class_name("edit");

                    if let Some(el) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&target)
                    {
                        if let Some(input_el) =
                            wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target)
                        {
                            input_el.set_value(&el.inner_text());
                        }
                        let list_item_node: web_sys::Node = list_item.into();
                        list_item_node.append_child(&input.into());
                        el.focus();
                    }
                }
            }
        }
    }

    /// Populate the todo list with a list of items.
    fn show_items(&self, items: ItemListSlice) {
        // TODO what is items?
        if let Some(ref el) = self.todo_list.el {
            el.set_inner_html(&Template::item_list(items));
        }
    }

    /// Remove an item from the view.
    fn remove_item(&self, id: usize) {
        let elem = Element::qs(&format!("[data-id=\"{}\"]", id));

        if let Some(elem) = elem {
            if let Some(todo_list_node) = self.todo_list.into(): Option<web_sys::Node> {
                if let Some(elem) = elem.into(): Option<web_sys::Node> {
                    todo_list_node.remove_child(&elem);
                }
            }
        }
    }

    /// Set the number in the 'items left' display.
    fn set_items_left(&self, items_left: usize) {
        // TODO what is items left?
        if let Some(ref todo_item_counter) = self.todo_item_counter.el {
            todo_item_counter.set_inner_html(&Template::item_counter(items_left));
        }
    }

    /// Set the visibility of the "Clear completed" button.
    fn set_clear_completed_button_visibility(&self, visible: bool) {
        if let Some(ref clear_completed) = self.clear_completed.el {
            set_visibility(clear_completed, visible);
        }
    }

    /// Set the visibility of the main content and footer.
    fn set_main_visibility(&self, visible: bool) {
        //  let value = if visible == true { "block" } else { "none" };
        //	self.main.style().set_property("display", value)
        if let Some(ref main) = self.main.el {
            set_visibility(main, visible);
        }
    }

    /// Set the checked state of the Complete All checkbox.
    fn set_complete_all_checkbox(&mut self, checked: bool) {
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
    fn update_filter_buttons(&self, route: String) {
        Element::qs(".filters .selected").map(|el| {
            if let Some(el) = el.el {
                el.set_class_name("");
            }
        });
        Element::qs(&format!(".filters [href=\"#{}\"]", route)).map(|el| {
            if let Some(el) = el.el {
                el.set_class_name("selected");
            }
        });
    }

    /// Clear the new todo input
    fn clear_new_todo(&mut self) {
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
    fn set_item_complete(&self, id: usize, completed: bool) {
        if let Some(mut list_item) = Element::qs(&format!("[data-id=\"{}\"]", id)) {
            let class_name = if completed { "completed" } else { "" };
            if let Some(list_item) = list_item.el {
                list_item.set_class_name(class_name);
            }

            // In case it was toggled from an event and not by clicking the checkbox
            list_item.qs_from("input").map(|el| {
                if let Some(input_el) =
                    wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&el)
                {
                    input_el.set_checked(completed);
                }
            });
        }
    }

    /// Bring an item out of edit mode.
    fn edit_item_done(&self, id: usize, title: &String) {
        let list_item = Element::qs(&format!("[data-id=\"{}\"]", id));

        if let Some(mut list_item) = Element::qs(&format!("[data-id=\"{}\"]", id)) {
            if let Some(input) = list_item.qs_from("input.edit") {
                if let Some(list_item_node) = list_item.into(): Option<web_sys::Node> {
                    list_item_node.remove_child(&input.into());

                    if let Some(list_item) = list_item.el {
                        list_item.class_list().remove_1("editing");
                    }

                    if let Some(list_item_label) = list_item.qs_from("label") {
                        let list_item_label_node: web_sys::Node = list_item_label.into();
                        list_item_label_node.set_text_content(Some(title.as_str()));
                    }
                }
            }
        }
    }

    fn bind_add_item<T>(&mut self, handler: T)
    where
        // TODO rewrite without static
        T: 'static + Fn(&str) -> (),
    {
        let cb = move |event: web_sys::Event| {
            if let Some(target) = event.target() {
                if let Some(input_el) =
                    wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target)
                {
                    let v = input_el.value(); // TODO remove with nll
                    let title = v.trim();
                    if title != "" {
                        handler(title.clone());
                    }
                }
            }
        };
        self.new_todo.add_event_listener("change", cb);
    }

    fn bind_remove_completed<T>(&mut self, handler: T)
    where
        // TODO rewrite without static
        T: 'static + Fn(web_sys::Event) -> (),
    {
/*
        let cb = Closure::wrap(Box::new(move |event: web_sys::Event| {
            handler(event);
        }) as Box<FnMut(_)>);
*/
        self.clear_completed.add_event_listener("click", handler);
/*
        if let Some(clear_completed_et) = self.clear_completed.into(): Option<web_sys::EventTarget>
        {
            clear_completed_et
                .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
*/
    }

    fn bind_toggle_all<T>(&mut self, handler: T) -> ()
    // Closure<dyn FnMut(web_sys::Event) -> ()>
    where
        // TODO rewrite without static
        T: 'static + Fn(bool) -> (),
    {
/*
        let cb = Closure::wrap(Box::new(move |event: web_sys::Event| {
            if let Some(target) = event.target() {
                if let Some(input_el) =
                    wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target)
                {
                    handler(input_el.checked());
                }
            }
        }) as Box<FnMut(_)>);
        if let Some(toggle_all_et) = self.toggle_all.into(): Option<web_sys::EventTarget> {
            toggle_all_et.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        }
*/
        self.clear_completed.add_event_listener("click", move |event: web_sys::Event| {
            if let Some(target) = event.target() {
                if let Some(input_el) =
                    wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target)
                {
                    handler(input_el.checked());
                }
            }
        });
        //cb.forget();
        //cb
        //toggle_all_et
    }

    fn bind_remove_item<T>(&mut self, handler: T)
    where
        // TODO rewrite without static
        T: 'static + Fn(Option<String>) -> (),
    {
        self.todo_list.delegate(
            ".destroy",
            "click",
            |e: web_sys::Event| {
                //TODO |{target}| {
                if let Some(target) = e.target() {
                    handler(item_id(&target));
                }
            },
            false,
        );
    }

    fn bind_toggle_item<T>(&mut self, handler: T)
    where
        T: Fn(Option<String>, bool) -> (),
    {
        self.todo_list.delegate(
            ".toggle",
            "click",
            |e: web_sys::Event| {
                // TODO |{target}| {
                if let Some(target) = e.target() {
                    if let Some(input_el) =
                        wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target)
                    {
                        handler(item_id(&target), input_el.checked());
                    }
                }
            },
            false,
        );
    }

    fn bind_edit_item_save<T>(&mut self, handler: T)
    where
        T: Fn(Option<String>, &str) -> (),
    {
        self.todo_list.delegate(
            "li .edit",
            "blur",
            |e: web_sys::Event| {
                // TODO |{target}| {
                if let Some(target) = e.target() {
                    if let Some(target_el) =
                        wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&target)
                    {
                        if target_el.dataset().get("iscanceled") != "true" {
                            if let Some(input_el) =
                                wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target)
                            {
                                handler(item_id(&target), input_el.value().trim());
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

    fn bind_edit_item_cancel<T>(&mut self, handler: T)
    where
        T: Fn(Option<String>) -> (),
    {
        self.todo_list.delegate(
            "li .edit",
            "keyup",
            |e: web_sys::Event| {
                if let Some(key_e) = wasm_bindgen::JsCast::dyn_ref::<web_sys::KeyboardEvent>(&e) {
                    if key_e.key_code() == ESCAPE_KEY {
                        if let Some(target) = e.target() {
                            if let Some(el) =
                                wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&target)
                            {
                                el.dataset().set("iscanceled", "true");
                                el.blur();
                            }

                            handler(item_id(&target));
                        }
                    }
                }
            },
            false,
        );
    }
}

struct Controller<'c> {
    store: Store,
    view: Rc<Mutex<View<'c>>>,
    active_route: String,
    last_active_route: String,
}

enum ControllerMessages {
    AddItem(String),
}

impl <'c> Controller <'c> {
    fn new(store: Store, view: View) -> Controller<'c> {
        let controller = Controller {
            store,
            view,
            active_route: "".into(),
            last_active_route: "".into(),
        };
        /* 
  		view.bind_add_item(controller.add_item);
  		view.bind_edit_item_save(controller.edit_item_save);
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

    fn set_view(&mut self, view: &'c mut View<'c>) {
        self.view = Some(view);
    }

    fn call(&mut self, method_name: ControllerMessages) {
        use ControllerMessages::*;
        match method_name {
            AddItem(title) => self.add_item(title),
        }
    }

    fn set_page(&mut self, raw: String) {
        let route = raw.replace(r#"/^#\//"#, "");
        self.active_route = route;
        self._filter(false);
        self.view.map(|view| {
            view.update_filter_buttons(route);
        });
    }

    /// Add an Item to the Store and display it in the list.
    fn add_item(&mut self, title: String) {
        self.store.insert(Item {
            id: Date::now() as usize,
            title,
            completed: false,
        });
        self.view.map(|view| {
            view.clear_new_todo();
        });
        self._filter(true);
    }

    /// Save an Item in edit.
    fn edit_item_save(&mut self, id: usize, title: String) {
        if title.len() > 0 {
            self.store.update(ItemUpdate::Title { id, title: title.clone() });
            self.view.map(|view| {
                view.edit_item_done(id, &title);
            });
        } else {
            self.remove_item(id);
        }
    }

    /// Cancel the item editing mode.
    fn edit_item_cancel(&mut self, id: usize) {
        if let Some(data) = self.store.find(ItemQuery::Id { id }) {
            if let Some(todo) = data.get(0) {
                let title = &todo.title;
                self.view.map(|view| {
                    view.edit_item_done(id, &title);
                });
            }
        }
    }

    /// Remove the data and elements related to an Item.
    fn remove_item(&mut self, id: usize) {
        self.store.remove(ItemQuery::Id { id });
        self._filter(false);
        self.view.map(|view| {
            view.remove_item(id);
        });
    }

    /// Remove all completed items.
    fn remove_completed_items(&mut self) {
        self.store.remove(ItemQuery::Completed { completed: true });
        self._filter(false);
    }

    /// Update an Item in storage based on the state of completed.
    fn toggle_completed(&mut self, id: usize, completed: bool) {
        self.store.update(ItemUpdate::Completed { id, completed });
        self.view.map(|view| {
            view.set_item_complete(id, completed);
        });
    }

    /// Set all items to complete or active.
    fn toggle_all(&mut self, completed: bool) {
        self.store
            .find(ItemQuery::Completed {
                completed: !completed,
            }).map(|data| {
                for item in data.iter() {
                    self.toggle_completed(item.id, completed);
                }

                self._filter(false);
            });
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
                self.view.map(|view| {
                     view.show_items(res);
                });
            }
        }

        if let Some((total, active, completed)) = self.store.count() {
            self.view.map(|view| {
                view.set_items_left(active);
                view
                .set_clear_completed_button_visibility(completed > 0);

                view.set_complete_all_checkbox(completed == total);
                view.set_main_visibility(total > 0);
            });
        }

        self.last_active_route = route.to_string();
    }
}

struct Store {
    // TODO check signatures
    //get_local_storage: FnMut() -> Vec<String>,
    //set_local_storage: FnMut(Vec<String>) -> (),
    local_storage: web_sys::Storage,
    data: Option<ItemList>,
    name: String,
}

impl Store {
    /**
     * @param {!string} name Database name
     * @param {function()} [callback] Called when the Store is ready
     */
    fn new(name: &str) -> Option<Store> {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(local_storage)) = window.local_storage() {
                Some(Store {
                    local_storage,
                    data: None,
                    name: String::from(name),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Read the local ItemList from localStorage.
    /// @returns {ItemList} Current array of todos
    /// Uses mut here as the return is something we might want to manipulate
    fn get_local_storage(&mut self) -> &Option<ItemList> {
        let mut item_list = ItemList::new();
        if let Some(ref data) = self.data {
            return &self.data;
        }
        if let Ok(Some(value)) = self.local_storage.get_item(&self.name) {
            if let Ok(data) = JSON::parse(&value) {
                if let Ok(Some(iter)) = js_sys::try_iter(&data) {
                    for item in iter {
                        if let Ok(item) = item {
                            let item: Option<&js_sys::Object> =
                                wasm_bindgen::JsCast::dyn_ref(&item);
                            if let Some(item) = item {
                                let mut temp_item = Item {
                                    title: "".to_string(),
                                    completed: false,
                                    id: 0,
                                };
                                js_sys::Object::entries(item).for_each(&mut |x, _, _| {
                                    let array: js_sys::Array = x.into();
                                    array.shift().as_string().map(|v| match v.as_str() {
                                        "title" => {
                                            if let Some(v) = array.shift().as_string() {
                                                temp_item.title = v;
                                            }
                                        }
                                        "completed" => {
                                            if let Some(v) = array.shift().as_bool() {
                                                temp_item.completed = v;
                                            }
                                        }
                                        "id" => {
                                            if let Some(v) = array.shift().as_f64() {
                                                temp_item.id = v as usize;
                                            }
                                        }
                                        _ => {}
                                    });
                                });
                                item_list.push(temp_item);
                            }
                        }
                    }
                }
            }
        }
        self.data = Some(item_list);
        &self.data
    }

    /// Write the local ItemList to localStorage.
    /// @param {ItemList} todos Array of todos to write
    fn set_local_storage(&mut self, todos: ItemList) {
        fn define_property(object: &js_sys::Object, key: &str, val: JsValue) {
            let descriptor = js_sys::Object::new();
            if let Some(val) = wasm_bindgen::JsCast::dyn_ref(&val): Option<&js_sys::Object> {
                js_sys::Object::define_property(&descriptor, &JsValue::from("value"), val);
                js_sys::Object::define_property(object, &key.into(), &descriptor);
            }
        }
        let array = js_sys::Array::new();
        for item in todos.iter() {
            let mut object = js_sys::Object::new();

            define_property(&mut object, "title", JsValue::from(item.title.clone()));
            define_property(&mut object, "completed", JsValue::from(item.completed));
            define_property(&mut object, "id", JsValue::from(item.id as f64));

            array.push(&JsValue::from(object));
        }
        if let Ok(storage_string) = JSON::stringify(&JsValue::from(array)) {
            let storage_string: String = storage_string.to_string().into();
            self.local_storage
                .set_item(&self.name, storage_string.as_str());
            self.data = Some(todos);
        }
    }

    /**
     * Find items with properties matching those on query.
     *
     * @param {ItemQuery} query Query to match
     * @param {function(ItemList)} callback Called when the query is done
     *
     * @example
     * db.find({completed: true}, data => {
     *	 // data shall contain items whose completed properties are true
     * })
     */
    fn find(&mut self, query: ItemQuery) -> Option<ItemListSlice> {
        self.get_local_storage();
        if let Some(ref todos) = self.data {
            Some(todos.iter().filter(|todo| query.matches(*todo)).collect())
        } else {
            None
        }
    }

    /**
     * Update an item in the Store.
     *
     * @param {ItemUpdate} update Record with an id and a property to update
     * @param {function()} [callback] Called when partialRecord is applied
     */
    fn update(&mut self, update: ItemUpdate) {
        //, callback) {
        let id = update.id();
        self.get_local_storage();
        self.data.take().map(|todos| {
            let todos = todos.into_iter();

            let todos = todos
                .map(|mut todo| {
                    if id == todo.id {
                        todo.update(&update);
                    }
                    todo
                }).collect();
            self.set_local_storage(todos);
        });

        /* isn't this async?
		if (callback) {
			callback();
		}
*/
    }

    /**
     * Insert an item into the Store.
     *
     * @param {Item} item Item to insert
     * @param {function()} [callback] Called when item is inserted
     */
    fn insert(&mut self, item: Item) {
        self.get_local_storage();
        self.data.take().map(|mut todos| {
            todos.push(item);
            self.set_local_storage(todos);
        });
    }

    /**
     * Remove items from the Store based on a query.
     *
     * @param {ItemQuery} query Query matching the items to remove
     * @param {function(ItemList)|function()} [callback] Called when records matching query are removed
     */
    fn remove(&mut self, query: ItemQuery) {
        self.get_local_storage();
        if let Some(todos) = self.data.take() {
            let todos = todos
             .into_iter()
             .filter(|todo| query.matches(todo))
             .collect();
            self.set_local_storage(todos);
        }
    }

    /// Count total, active, and completed todos.
    fn count(&mut self) -> Option<(usize, usize, usize)> {
        self.find(ItemQuery::EmptyItemQuery).map(|data| {
            let total = data.length();

            let mut completed = 0;
            for item in data.iter() {
                if item.completed {
                    completed += 1;
                }
            }
            (total, total - completed, completed)
        })
    }
}

struct Item {
    id: usize,
    title: String,
    completed: bool,
}

impl Item {
    fn update(&mut self, update: &ItemUpdate) {
        match update {
            ItemUpdate::Title { id, title } => {
                self.title = title.to_string();
            }
            ItemUpdate::Completed { id, completed } => {
                self.completed = *completed;
            }
        }
    }
}

trait ItemListTrait<T> {
    fn new() -> Self;
    fn get(&self, i: usize) -> Option<&T>;
    fn length(&self) -> usize;
    fn push(&mut self, item: T);
    fn iter(&self) -> std::slice::Iter<T>;
}

struct ItemList {
    list: Vec<Item>,
}
impl ItemList {
    fn into_iter(self) -> std::vec::IntoIter<Item> {
        self.list.into_iter()
    }
}
impl ItemListTrait<Item> for ItemList {
    fn new() -> ItemList {
        ItemList { list: Vec::new() }
    }
    fn get(&self, i: usize) -> Option<&Item> {
        self.list.get(i)
    }
    fn length(&self) -> usize {
        self.list.len()
    }
    fn push(&mut self, item: Item) {
        self.list.push(item)
    }
    fn iter(&self) -> std::slice::Iter<Item> {
        self.list.iter()
    }
}
use std::iter::FromIterator;
impl<'a> FromIterator<Item> for ItemList {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        let mut c = ItemList::new();
        for i in iter {
            c.push(i);
        }

        c
    }
}

struct ItemListSlice<'a> {
    list: Vec<&'a Item>,
}
impl<'a> ItemListSlice<'a> {
    /*
    fn new(boxed_slice: Box<[&'a Item]>) -> ItemListSlice<'a> {
        ItemListSlice { list: boxed_slice }
    }
*/
}
impl<'a> ItemListTrait<&'a Item> for ItemListSlice<'a> {
    fn new() -> ItemListSlice<'a> {
        ItemListSlice { list: Vec::new() }
    }
    fn get(&self, i: usize) -> Option<&&'a Item> {
        self.list.get(i)
    }
    fn length(&self) -> usize {
        self.list.len()
    }
    fn push(&mut self, item: &'a Item) {
        self.list.push(item)
    }
    fn iter(&self) -> std::slice::Iter<&'a Item> {
        self.list.iter()
    }
}
impl<'a> FromIterator<&'a Item> for ItemListSlice<'a> {
    fn from_iter<I: IntoIterator<Item = &'a Item>>(iter: I) -> Self {
        let mut c = ItemListSlice::new();
        for i in iter {
            c.push(i);
        }
        c
    }
}
enum ItemQuery {
    Id { id: usize },
    Completed { completed: bool },
    EmptyItemQuery,
}
impl ItemQuery {
    fn matches(&self, item: &Item) -> bool {
        match *self {
            ItemQuery::EmptyItemQuery => true,
            ItemQuery::Id { id } => item.id == id,
            ItemQuery::Completed { completed } => item.completed == completed,
        }
    }
}

enum ItemUpdate {
    Title { id: usize, title: String },
    Completed { id: usize, completed: bool },
}
impl ItemUpdate {
    fn id(&self) -> usize {
        match self {
            ItemUpdate::Title { id, title } => *id,
            ItemUpdate::Completed { id, completed } => *id,
        }
    }
}
