use crate::templates::parser::{TemplateNode, Content, Context};
use std::collections::HashMap;


pub fn write(root: &TemplateNode, context: &HashMap<String, Context>) -> String {
    String::from_utf8(root.get_content(context)).unwrap()
}