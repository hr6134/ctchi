use std::collections::HashMap;
use regex::internal::Char;
use crate::core::html_parser::TemplateNode::CtchiForTagNode;

#[derive(Debug)]
pub enum TemplateNode {
    CtchiIfTagNode(IfTag),
    CtchiImportTagNode(ImportTag),
    CtchiForTagNode(ForTag),
    CtchiTemplateTagNode(TemplateTag),
    CtchiValueNode(CtchiValue),
    HtmlNode(Html),
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
    root: TemplateTag,
}

#[derive(Debug)]
pub struct CtchiTagAttribute {
    name: String,
    value: String,
}

#[derive(Debug)]
pub struct TemplateTag {
    name: String,
    children: Vec<TemplateNode>,
    size: usize,
}

#[derive(Debug)]
pub struct ForTag {
    var_name: String,
    param_name: String,
    children: Vec<TemplateNode>,
    size: usize,
}

#[derive(Debug)]
pub struct IfTag {
    var_name: String,
    children: Vec<TemplateNode>,
    size: usize,
}

#[derive(Debug)]
pub struct ImportTag {
    path: String,
    size: usize,
}

#[derive(Debug)]
pub struct CtchiValue {
    value: String,
}

#[derive(Debug)]
pub struct Html {
    value: String,
}

pub struct HtmlParser;

impl HtmlParser {
    pub fn parse(&mut self, html: &str) -> TemplateNode {
        if !html.starts_with("[template]") {
            return TemplateNode::HtmlNode(Html {
                value: html.to_string()
            })
        }

        self.parse_tag(html)
    }

    pub fn parse_tag(&mut self, html: &str) -> TemplateNode {
        let html_bytes = html.as_bytes();
        let mut children = Vec::new();
        let tag_open_token_raw = self.parse_tag_open_token_raw(html_bytes);

        // pass first [
        let mut i = 1 + tag_open_token_raw.len();

        let single_line_tag = html_bytes[i-1] == b'/';
        let mut tag_open_token = String::from_utf8(tag_open_token_raw).unwrap();

        // pass ending ]
        i += 1;

        // look up for children only if we haven't single line tag
        if !single_line_tag {
            // read children
            while html_bytes[i] != b'[' || html_bytes[i + 1] != b'e' {
                let child = if html_bytes[i] == b'[' && html_bytes[i + 1] == b'[' {
                    self.parse_value(&html[i..html.len()])
                } else if html_bytes[i] == b'[' {
                    self.parse_tag(&html[i..html.len()])
                } else {
                    self.parse_text(&html[i..html.len()])
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
        i += end_name.len();

        let result = self.build_result(&tag_open_token, children, i);

        if !single_line_tag {
            // read tag closing, for validation only
            if !HtmlParser::compare(&html[(i - end_name.len())..html.len()], &end_name) {
                println!("{}", end_name);
                println!("{}", &html[i..html.len()]);
                panic!("Wrong closing tag");
            }
        }

        result
    }

    fn build_result(&self, tag_open_token: &str, children: Vec<TemplateNode>, size: usize) -> TemplateNode {
        let tag_name = tag_open_token.split(" ").collect::<Vec<&str>>()[0];
        let params = self.parse_tag_attributes(&tag_open_token);

        match tag_name {
            "for" => TemplateNode::CtchiForTagNode(ForTag {
                var_name: params.0,
                param_name: params.1,
                children,
                size
            }),
            "if" => TemplateNode::CtchiIfTagNode(IfTag{
                var_name: params.0,
                children,
                size
            }),
            "import" => TemplateNode::CtchiImportTagNode(ImportTag{
                path: params.0,
                size
            }),
            "template" => TemplateNode::from_tag(TemplateTag {
                name: tag_name.to_string(),
                children,
                size
            }),
            _ => panic!("Unknown tag"),
        }
    }

    fn compare(html: &str, tag: &str) -> bool {
        html.starts_with(tag)
    }

    // fixme no need in tuple, use another enum
    fn parse_tag_attributes(&self, tag: &str) -> (String, String) {
        let tokens = tag.split(" ").collect::<Vec<&str>>();
        match tokens[0] {
            "for" => (tokens[1].to_string(), tokens[3].to_string()),
            "if" => (tokens[1].to_string(), "".to_string()),
            "import" => (tokens[1][0..(tokens[1].len()-1)].to_string(), "".to_string()),
            _ => ("".to_string(), "".to_string())
        }
    }

    fn parse_tag_open_token_raw(&self, html_bytes: &[u8]) -> Vec<u8> {
        let mut tag_open_token_raw = Vec::new();
        let mut i = 1;

        while html_bytes[i] != b']' {
            tag_open_token_raw.push(html_bytes[i]);
            i += 1;
        }

        tag_open_token_raw
    }

    fn parse_value(&mut self, html: &str) -> TemplateNode {
        let html_bytes = html.as_bytes();
        let mut value = Vec::new();
        let mut i = 2;

        while html_bytes[i] != b']' || html_bytes[i+1] != b']' {
            value.push(html_bytes[i]);
            i += 1;
        };

        TemplateNode::CtchiValueNode(CtchiValue {
            value: String::from_utf8(value).unwrap()
        })
    }

    fn parse_text(&mut self, html: &str) -> TemplateNode {
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
}