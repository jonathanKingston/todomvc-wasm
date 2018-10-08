use crate::store::{ItemListSlice, ItemListTrait};
use std::rc::{Rc, Weak};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::Scheduler;

const ENTER_KEY: u32 = 13;
const ESCAPE_KEY: u32 = 27;

fn item_id(element: &web_sys::EventTarget) -> Option<usize> {
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
            res.map(|id| usize::from_str_radix(&id, 10).ok())?
        })?
    } else {
        None
    }
}

pub struct View {
    sched: Option<Rc<Scheduler>>,
    todo_list: Element,
    todo_item_counter: Element,
    clear_completed: Element,
    main: Element,
    toggle_all: Element,
    new_todo: Element,
}

impl  View  {
    pub fn new() -> Option<View> {
        let todo_list = Element::qs(".todo-list")?;
        let todo_item_counter = Element::qs(".todo-count")?;
        let clear_completed = Element::qs(".clear-completed")?;
        let main = Element::qs(".main")?;
        let toggle_all = Element::qs(".toggle-all")?;
        let new_todo = Element::qs(".new-todo")?;
        let view = View {
            sched: None,
            todo_list,
            todo_item_counter,
            clear_completed,
            main,
            toggle_all,
            new_todo,
        };

        /* TODO fixed mixed mutability
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
*/

        Some(view)
    }

    /*
    pub fn set_controller(&mut self, controller: Box<&'a Controller<'a>>) {
        self.controller = Some(controller);
    }
*/

    /// Put an item into edit mode.
    fn edit_item(&self, target: web_sys::Element) {
        let target_node: web_sys::Node = target.into();
        if let Some(parent_element) = target_node.parent_element() {
            let parent_node: web_sys::Node = parent_element.into();
            if let Some(list_item) = parent_node.parent_element() {
                list_item.class_list().add_1("editing");

                if let Some(input) = create_element("input") {
                    input.set_class_name("edit");
                    /* TODO fix move out of borrow
                    if let Some(el) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&target)
                    {
/* TODO fix move out of borrow
                        if let Some(input_el) =
                            wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&target)
                        {
                            input_el.set_value(&el.inner_text());
                        }
*/
                        let list_item_node: web_sys::Node = list_item.into();
                        list_item_node.append_child(&input.into());
                        el.focus();
                    }
*/
                }
            }
        }
    }

    /// Populate the todo list with a list of items.
    pub fn show_items(&self, items: ItemListSlice) {
        // TODO what is items?
        if let Some(ref el) = self.todo_list.el {
            el.set_inner_html(&Template::item_list(items));
        }
    }

    /// Remove an item from the view.
    pub fn remove_item(&self, id: usize) {
        let elem = Element::qs(&format!("[data-id=\"{}\"]", id));

        /*TODO fix moved out of borrow
        if let Some(elem) = elem {
            if let Some(todo_list_node) = self.todo_list.into(): Option<web_sys::Node> {
                if let Some(elem) = elem.into(): Option<web_sys::Node> {
                    todo_list_node.remove_child(&elem);
                }
            }
        }
*/
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
            set_visibility(clear_completed, visible);
        }
    }

    /// Set the visibility of the main content and footer.
    pub fn set_main_visibility(&self, visible: bool) {
        //  let value = if visible == true { "block" } else { "none" };
        //	self.main.style().set_property("display", value)
        if let Some(ref main) = self.main.el {
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
    pub fn set_item_complete(&self, id: usize, completed: bool) {
        if let Some(mut list_item) = Element::qs(&format!("[data-id=\"{}\"]", id)) {
            let class_name = if completed { "completed" } else { "" };
            if let Some(ref list_item) = list_item.el {
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
    pub fn edit_item_done(&self, id: usize, title: &String) {
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
        self.clear_completed
            .add_event_listener("click", move |event: web_sys::Event| {
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
        T: 'static + Fn(Option<usize>) -> (),
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
        T: Fn(Option<usize>, bool) -> (),
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

    pub fn bind_edit_item_save<T>(&mut self, mut handler: T)
    where
        T: FnMut(usize, &str) -> (),
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
                                if let Some(item) = item_id(&target) {
                                    handler(item, input_el.value().trim());
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

    fn bind_edit_item_cancel<T>(&mut self, handler: T)
    where
        T: Fn(Option<usize>) -> (),
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
        T: FnMut(web_sys::Event) -> (),
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
