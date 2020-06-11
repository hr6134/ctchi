use crate::core::config::get_configuration;

use std::fs;
use regex::Regex;
use std::collections::HashMap;

pub enum Context {
    BooleanValue(bool),
    SingleValue(String),
    MultiValue(Vec<String>),
}

pub trait Content {
    fn get_content(&self, context: &HashMap<String, Context>) -> Vec<u8>;
}

#[derive(Debug)]
pub enum TemplateNode {
    CtchiIfTagNode(IfTag),
    CtchiImportTagNode(ImportTag),
    CtchiForTagNode(ForTag),
    CtchiTemplateTagNode(TemplateTag),
    CtchiValueNode(CtchiValue),
    HtmlNode(Html),
}

impl Content for TemplateNode {
    fn get_content(&self, context: &HashMap<String, Context>) -> Vec<u8> {
        match self {
            TemplateNode::CtchiTemplateTagNode(e) => e.get_content(context),
            TemplateNode::HtmlNode(e) => e.get_content(context),
            TemplateNode::CtchiValueNode(e) => e.get_content(context),
            TemplateNode::CtchiForTagNode(e) => e.get_content(context),
            TemplateNode::CtchiIfTagNode(e) => e.get_content(context),
            TemplateNode::CtchiImportTagNode(e) => e.get_content(context),
        }
    }
}

impl TemplateNode {
    pub fn from_tag(tag: TemplateTag) -> TemplateNode {
        TemplateNode::CtchiTemplateTagNode(tag)
    }

    pub fn from_html(text: Html) -> TemplateNode {
        TemplateNode::HtmlNode(text)
    }

    pub fn from_value(value: CtchiValue) -> TemplateNode {
        TemplateNode::CtchiValueNode(value)
    }
}

#[derive(Debug)]
pub struct Page {
    pub root: TemplateTag,
}

#[derive(Debug)]
pub struct TemplateTag {
    pub name: String,
    pub children: Vec<TemplateNode>,
    pub size: usize,
}

impl Content for TemplateTag {
    fn get_content(&self, context: &HashMap<String, Context>) -> Vec<u8> {
        let mut result = Vec::new();

        for c in &self.children {
            result.append(&mut c.get_content(context));
        }

        result
    }
}

#[derive(Debug)]
pub struct ForTag {
    pub var_name: String,
    pub param_name: String,
    pub children: Vec<TemplateNode>,
    pub size: usize,
}

impl Content for ForTag {
    fn get_content(&self, context: &HashMap<String, Context>) -> Vec<u8> {
        let mut result = Vec::new();

        let default_value = Context::MultiValue(Vec::new());
        let context_value = match context.get(&self.param_name).unwrap_or(&default_value) {
            Context::MultiValue(e) => e,
            _ => panic!("For tag should have multivalue in context"),
        };

        for v in context_value {
            let mut inner_context = HashMap::<String, Context>::new();
            let local_var_name = &self.var_name;
            inner_context.insert(local_var_name.to_string(), Context::SingleValue(v.to_string()));
            for c in &self.children {
                result.append(&mut c.get_content(&inner_context));
            }
        }

        result
    }
}

#[derive(Debug)]
pub struct IfTag {
    pub var_name: String,
    pub children: Vec<TemplateNode>,
    pub size: usize,
}

impl Content for IfTag {
    fn get_content(&self, context: &HashMap<String, Context>) -> Vec<u8> {
        let mut result = Vec::new();

        let default_value = Context::BooleanValue(false);
        let context_value = match context.get(&self.var_name).unwrap_or(&default_value) {
            Context::BooleanValue(e) => *e,
            _ => panic!("If tag should have boolean value in context"),
        };

        if context_value {
            for c in &self.children {
                result.append(&mut c.get_content(context));
            }
        }

        result
    }
}

#[derive(Debug)]
pub struct ImportTag {
    pub path: String,
    pub size: usize,
}

impl Content for ImportTag {
    fn get_content(&self, context: &HashMap<String, Context>) -> Vec<u8> {
        let node = parse_file(&self.path);
        node.get_content(context)
    }
}

#[derive(Debug)]
pub struct CtchiValue {
    pub value: String,
}

impl Content for CtchiValue {
    fn get_content(&self, _context: &HashMap<String, Context>) -> Vec<u8> {
        let default_value = Context::SingleValue(String::new());

        let result = match _context.get(&self.value).unwrap_or(&default_value) {
            Context::SingleValue(e) => e,
            _ => panic!("Value can only be single value")
        };

        Vec::from(result.as_bytes())
    }
}

#[derive(Debug)]
pub struct Html {
    pub value: String,
}

impl Content for Html {
    fn get_content(&self, _context: &HashMap<String, Context>) -> Vec<u8> {
        Vec::from(self.value.as_bytes())
    }
}

pub fn parse_file(path: &str) -> TemplateNode {
    let config_reader = get_configuration();
    let config = config_reader.inner.lock().unwrap();
    let page = format!("{}/{}", config.base_path, path);
    drop(config);
    let content = fs::read_to_string(page).unwrap_or_else(|error| { error.to_string() });

    parse(&content)
}

pub fn parse(html: &str) -> TemplateNode {
    if !html.starts_with("[template]") {
        return TemplateNode::HtmlNode(Html {
            value: html.to_string()
        });
    }

    let escaped_html = escape_page(html);

    parse_tag(escaped_html.as_ref())
}

fn escape_page(html: &str) -> String {
    let open_bracket_replacer = Regex::new(r"\\\[").unwrap();
    let close_bracket_replacer = Regex::new(r"\\]").unwrap();
    let open_replacer = open_bracket_replacer.replace_all(html, "&#x5B;");
    let close_replacer = close_bracket_replacer.replace_all(open_replacer.as_ref(), "&#x5D;");

    close_replacer.to_string()
}

fn parse_tag(html: &str) -> TemplateNode {
    let html_bytes = html.as_bytes();
    let mut children = Vec::new();
    let tag_open_token_raw = parse_tag_open_token_raw(html_bytes);

    // pass first [
    let mut i = 1 + tag_open_token_raw.len();

    let single_line_tag = html_bytes[i - 1] == b'/';
    let tag_open_token = String::from_utf8(tag_open_token_raw).unwrap();

    // pass ending ]
    i += 1;

    // look up for children only if we haven't single line tag
    if !single_line_tag {
        // read children
        while !is_end_tag(&html[i..html.len()]) {
            let child = if html_bytes[i] == b'[' && html_bytes[i + 1] == b'[' {
                parse_value(&html[i..html.len()])
            } else if html_bytes[i] == b'[' {
                parse_tag(&html[i..html.len()])
            } else {
                parse_text(&html[i..html.len()])
            };

            let size = match &child {
                TemplateNode::CtchiTemplateTagNode(e) => e.size,
                TemplateNode::HtmlNode(e) => e.value.len(),
                TemplateNode::CtchiValueNode(e) => e.value.len() + 4,
                TemplateNode::CtchiForTagNode(e) => e.size,
                TemplateNode::CtchiIfTagNode(e) => e.size,
                TemplateNode::CtchiImportTagNode(e) => e.size,
            };
            i += size;

            children.push(child);
        }
    }

    let tag_name = tag_open_token.split(" ").collect::<Vec<&str>>()[0];
    let end_name = format!("[end{}]", tag_name);
    if !single_line_tag {
        i += end_name.len();
    }

    let result = build_result(&tag_open_token, children, i);

    if !single_line_tag {
        // read tag closing, for validation only
        if !compare(&html[(i - end_name.len())..html.len()], &end_name) {
            panic!("Wrong closing tag");
        }
    }

    result
}

fn is_end_tag(html: &str) -> bool {
    let tags = vec!("[endfor]", "[endtemplate]", "[endif]");

    for tag in tags {
        if html.starts_with(tag) {
            return true;
        }
    }

    false
}

fn build_result(tag_open_token: &str, children: Vec<TemplateNode>, size: usize) -> TemplateNode {
    let tag_name = tag_open_token.split(" ").collect::<Vec<&str>>()[0];
    let params = parse_tag_attributes(&tag_open_token);

    match tag_name {
        "for" => TemplateNode::CtchiForTagNode(ForTag {
            var_name: params.0,
            param_name: params.1,
            children,
            size,
        }),
        "if" => TemplateNode::CtchiIfTagNode(IfTag {
            var_name: params.0,
            children,
            size,
        }),
        "import" => TemplateNode::CtchiImportTagNode(ImportTag {
            path: params.0,
            size,
        }),
        "template" => TemplateNode::from_tag(TemplateTag {
            name: tag_name.to_string(),
            children,
            size,
        }),
        _ => panic!("Unknown tag"),
    }
}

fn compare(html: &str, tag: &str) -> bool {
    html.starts_with(tag)
}

// fixme no need in tuple, use another enum
fn parse_tag_attributes(tag: &str) -> (String, String) {
    let tokens = tag.split(" ").collect::<Vec<&str>>();
    match tokens[0] {
        "for" => (tokens[1].to_string(), tokens[3].to_string()),
        "if" => (tokens[1].to_string(), "".to_string()),
        "import" => (tokens[1][1..(tokens[1].len() - 2)].to_string(), "".to_string()),
        _ => ("".to_string(), "".to_string())
    }
}

fn parse_tag_open_token_raw(html_bytes: &[u8]) -> Vec<u8> {
    let mut tag_open_token_raw = Vec::new();
    let mut i = 1;

    while html_bytes[i] != b']' {
        tag_open_token_raw.push(html_bytes[i]);
        i += 1;
    }

    tag_open_token_raw
}

fn parse_value(html: &str) -> TemplateNode {
    let html_bytes = html.as_bytes();
    let mut value = Vec::new();
    let mut i = 2;

    while html_bytes[i] != b']' || html_bytes[i + 1] != b']' {
        value.push(html_bytes[i]);
        i += 1;
    };

    TemplateNode::CtchiValueNode(CtchiValue {
        value: String::from_utf8(value).unwrap()
    })
}

fn parse_text(html: &str) -> TemplateNode {
    let html_bytes = html.as_bytes();
    let mut value = Vec::new();
    let mut i = 0;

    while html_bytes[i] != b'[' {
        value.push(html_bytes[i]);
        i += 1;
    };

    TemplateNode::HtmlNode(Html {
        value: String::from_utf8(value).unwrap()
    })
}
