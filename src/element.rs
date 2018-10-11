use crate::dbg;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct Element {
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
    pub fn qs(selector: &str) -> Option<Element> {
        let body: web_sys::Element = web_sys::window()?.document()?.body()?.into();
        let el = body.query_selector(selector).ok()?;
        Some(Element { el })
    }

    pub fn add_event_listener<T>(&mut self, event_name: &str, handler: T)
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
    pub fn delegate<T>(
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

    pub fn qs_from(&mut self, selector: &str) -> Option<Element> {
        let mut found_el = None;
        if let Some(el) = self.el.take() {
            found_el = Some(Element {
                el: el.query_selector(selector).ok()?,
            });
            self.el = Some(el);
        }
        found_el
    }

    pub fn set_inner_html(&mut self, value: String) {
        if let Some(el) = self.el.take() {
            el.set_inner_html(&value);
            self.el = Some(el);
        }
    }

    pub fn set_text_content(&mut self, value: String) {
        if let Some(el) = self.el.take() {
            if let Some(node) = wasm_bindgen::JsCast::dyn_ref(&el): Option<&web_sys::Node> {
                node.set_text_content(Some(&value));
            }
            self.el = Some(el);
        }
    }

    pub fn class_list_remove(&mut self, value: String) {
        if let Some(el) = self.el.take() {
            el.class_list().remove_1(&value);
            self.el = Some(el);
        }
    }

    pub fn remove_child(&mut self, mut child: Element) {
        if let Some(child_el) = child.el.take() {
            if let Some(el) = self.el.take() {
                if let Some(el_node) = wasm_bindgen::JsCast::dyn_ref(&el): Option<&web_sys::Node> {
                    let child_node: web_sys::Node = child_el.into();
                    el_node.remove_child(&child_node);
                }
                self.el = Some(el);
            }
        }
    }

    pub fn set_class_name(&mut self, class_name: String) {
        if let Some(el) = self.el.take() {
            el.set_class_name(&class_name);
            self.el = Some(el);
        }
    }

    pub fn set_visibility(&mut self, visible: bool) {
        if let Some(el) = self.el.take() {
            {
                let dyn_el: Option<&web_sys::HtmlElement> = wasm_bindgen::JsCast::dyn_ref(&el);
                if let Some(el) = dyn_el {
                    el.set_hidden(!visible);
                }
            }
            self.el = Some(el);
        }
    }

    pub fn set_value(&mut self, value: String) {
        if let Some(el) = self.el.take() {
            if let Some(el) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&el) {
                el.set_value(&value);
            }
            self.el = Some(el);
        }
    }
    /* TODO use
    pub fn focus(&self) {
        if let Some(el) = self.el.take() {
            if let Some(el) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&el) {
                el.focus();
            }
            self.el = Some(el);
        }
    }
*/

    pub fn set_checked(&mut self, checked: bool) {
        if let Some(el) = self.el.take() {
            if let Some(el) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&el) {
                el.set_checked(checked);
            }
            self.el = Some(el);
        }
    }
}
