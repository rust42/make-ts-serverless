use std::collections::HashMap;
use crate::templates::Template;

pub fn render_template(template: &Template, variables: &HashMap<&str, &String>) -> String {
    let mut text = String::from(template.content);
    for (key, value) in variables {
        text = text.replace(key, value)
    }
    return text
}