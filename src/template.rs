use store::{ItemList, ItemListTrait};

fn escape_html(val: String) -> String {
    // TODO escape me!
    val
}
// export const escapeForHTML = s => s.replace(/[&<]/g, c => c === '&' ? '&amp;' : '&lt;');

pub struct Template {}

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
    pub fn item_list(items: ItemList) -> String {
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

    /**
     * Format the contents of an "items left" indicator.
     *
     * @param {number} activeTodos Number of active todos
     *
     * @returns {!string} Contents for an "items left" indicator
     */
    pub fn item_counter(active_todos: usize) -> String {
        let plural = if active_todos > 1 { "s" } else { "" };
        return format!("{} item{} left", active_todos, plural);
    }
}
