use store::{ItemList, ItemListTrait};

fn escape_html(val: String) -> String {
    val.replace('&', "&alt;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot").replace('\'', "&#39;")
}

pub struct Template {}

impl Template {
    /// Format the contents of a todo list.
    ///
    /// items `ItemList` contains keys you want to find in the template to replace.
    /// Returns the contents for a todo list
    ///
    pub fn item_list(items: &ItemList) -> String {
        let mut output = String::from("");
        for item in items.iter() {
            let completed_class = if item.completed {
                "class=\"completed\""
            } else {
                ""
            };
            let checked = if item.completed { "checked" } else { "" };
            let title = escape_html(item.title.clone());
            output.push_str(&format!(
                r#"
  <li data-id="{}"{}>
  	<div class="view">
  		<input class="toggle" type="checkbox" {}>
  		<label>{}</label>
  		<button class="destroy"></button>
  	</div>
  </li>"#,
                item.id, completed_class, checked, title
            ));
        }
        output
    }

    ///
    /// Format the contents of an "items left" indicator.
    ///
    /// `active_todos` Number of active todos
    ///
    /// Returns the contents for an "items left" indicator
    pub fn item_counter(active_todos: usize) -> String {
        let plural = if active_todos > 1 { "s" } else { "" };
        let mut template = active_todos.to_string();
        template.push_str(" item");
        template.push_str(plural);
        template.push_str(" left");
        template
    }
}
