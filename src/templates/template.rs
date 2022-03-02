use std::string::String;
use crate::templates::TemplateKind;
use crate::{
    BUILD_SRC,
    DEPLOY_FN,
    APP_FILE,
    TEMPLATE_YML,
    NPM_PACKAGE_JSON,
    TS_CONFIG
};

pub struct Template {
    pub name: String,
    pub extension: String,
    pub content: &'static str,
    pub kind: TemplateKind,
}

impl Template {
    fn new(name: String, extension: String, content: &'static str, kind: TemplateKind) -> Template {
        Template {
            name,
            extension,
            content,
            kind,
        }
    }
}

impl Template {

    pub fn get_all() -> Vec<Template> {
        vec![
            Template::new(
                String::from("package"),
                String::from("json"),
                NPM_PACKAGE_JSON,
                TemplateKind::Src),
            Template::new(
                String::from("index"),
                String::from("ts"),
                APP_FILE,
                TemplateKind::Src,
            ),
            Template::new(
                String::from("tsconfig"),
                String::from("json"),
                TS_CONFIG,
                TemplateKind::Src,
            ),
            Template::new(
                String::from("build"),
                String::from("sh"),
                BUILD_SRC,
                TemplateKind::Src,
            ),
            Template::new(
                String::from("template"),
                String::from("yml"),
                TEMPLATE_YML,
                TemplateKind::App,
            ),
            Template::new(
                String::from("deploy"),
                String::from("sh"),
                DEPLOY_FN,
                TemplateKind::App
            ),
        ]
    }
}