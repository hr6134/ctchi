use crate::templates::parser::{TemplateNode, Content};


pub fn write(root: &TemplateNode) -> String {
    String::from_utf8(root.get_content()).unwrap()
}