use crate::controller::Controller;
use crate::dbg;
use crate::view::View;
use crate::Message;
use std::cell::RefCell;
use std::rc::Rc;

/// Creates an event loop that starts each time a message is added
pub struct Scheduler {
    controller: Rc<RefCell<Option<Controller>>>,
    view: Rc<RefCell<Option<View>>>,
    events: RefCell<Vec<Message>>,
    running: RefCell<bool>,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            controller: Rc::new(RefCell::new(None)),
            view: Rc::new(RefCell::new(None)),
            events: RefCell::new(Vec::new()),
            running: RefCell::new(false),
        }
    }
    pub fn set_controller(&self, controller: Controller) {
        let mut controller_data = self.controller.borrow_mut();
        controller_data.replace(controller);
    }
    pub fn set_view(&self, view: View) {
        let mut view_data = self.view.borrow_mut();
        view_data.replace(view);
    }
    pub fn add_message(&self, message: Message) {
        let v = wasm_bindgen::JsValue::from_str(&format!("{}", "got message"));
        web_sys::console::log_1(&v);
        let mut running = {
            self.running.borrow().clone()
        };
        {
            let mut events = self.events.borrow_mut(); // TODO use try_borrow
            events.push(message);
        }
        if !running {
            self.run();
        }
    }
    fn run(&self) {
        let mut events_len;
        {
            events_len = self.events.borrow().len().clone(); // TODO use try_borrow
        }
        if events_len == 0 {
            let mut running = self.running.borrow_mut();
            *running = false;
        } else {
            {
                let mut running = self.running.borrow_mut();
                *running = true;
            }
            self.next_message();
        }
    }
    fn next_message(&self) {
        let event = {
            let mut events = self.events.borrow_mut(); // TODO use try_borrow
            events.pop()
        };
        if let Some(event) = event {
            match event {
                Message::Controller(e) => {
                    if let Some(ref mut ag) = *self.controller.borrow_mut() {
                        // TODO use try_borrow
                        ag.call(e);
                    }
                }
                Message::View(e) => {
                    if let Some(ref mut ag) = *self.view.borrow_mut() {
                        // TODO use try_borrow
                        ag.call(e);
                    }
                }
            }
            self.run();
        } else {
            let mut running = self.running.borrow_mut();
            *running = false;
        }
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        dbg("calling drop on Scheduler");
    }
}
