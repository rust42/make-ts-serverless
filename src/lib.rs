use std::io::{BufRead, Result};
use std::collections::HashMap;
use std::io::{BufReader, ErrorKind};
use std::process::Stdio;
use std::path::{Path, PathBuf};
use clap::{Command, Arg};


mod config;
mod app_error;

use config::Config;
use app_error::AppError;

mod templates;

use templates::{
    BUILD_SRC,
    DEPLOY_FN,
    APP_FILE,
    TEMPLATE_YML,
    NPM_PACKAGE_JSON,
    TS_CONFIG,
    TemplateKind,
    render_template,
};
use crate::templates::Template;

pub struct App {
    config: Config,
}

impl App {
    pub fn new() -> Result<App> {
        App::get_app_config()
            .map(|config| App { config })
    }

    fn get_app_config() -> Result<Config> {
        let app = Command::new("make-ts-app")
            .name("Make TS App")
            .about("A Simple app to generate Serverless Typescript Project")
            .arg(Arg::new("name")
                .required(true)
                .help("Name of app")
                .value_name("APP_NAME")
            )
            .arg(Arg::new("bucket")
                .required(true)
                .help("S3 bucket to store artifacts")
                .value_name("BUCKET")
                .short('b')
                .long("bucket_name"))
            .arg(Arg::new("stack-name")
                .required(true)
                .help("Name of cloudformation stack")
                .value_name("STACK_NAME")
                .short('s')
                .long("stack_name"));

        let matches = app.get_matches();
        let app_name = matches.value_of("name").ok_or(AppError::MissingName)?;
        let bucket_name = matches.value_of("bucket").ok_or(AppError::MissingBucketName)?;
        let stack_name = matches.value_of("stack-name").ok_or(AppError::MissingStackName)?;

        Ok(
            Config {
                app_name: String::from(app_name),
                bucket_name: String::from(bucket_name),
                stack_name: String::from(stack_name),
            }
        )
    }
}

impl App {
    pub fn run(self) -> Result<()> {
        Ok(Template::get_all())
            .map(|templates| (templates, self.make_template_variables()))
            .and_then(|(templates, variables)| self.write_templates(&templates, &variables))
            .and_then(|_| self.install_dependencies())
    }

    fn make_template_variables(&self) -> HashMap<&str, &String> {
        let mut template_vars: HashMap<_, _> = HashMap::new();
        template_vars.insert("<APP_NAME>", &self.config.app_name);
        template_vars.insert("<STACK_NAME>", &self.config.stack_name);
        template_vars.insert("<BUCKET_NAME>", &self.config.bucket_name);
        template_vars
    }

    fn write_templates<'a>(&self, templates: &[Template], variables: &HashMap<&'a str, &'a String>) -> Result<()> {
        let app_dir = self.app_directory_path()?;
        let source_dir = self.source_directory_path()?;

        std::fs::create_dir(&app_dir)?;
        std::fs::create_dir(&source_dir)?;

        for template in templates {
            let content = render_template(template, variables);

            let template_dir = match template.kind {
                TemplateKind::App => &app_dir,
                TemplateKind::Src => &source_dir
            };

            let file_name = format!("{}.{}", template.name, template.extension);
            let file_path = Path::new(&template_dir).join(file_name);
            std::fs::write(file_path, content)?;
        }
        Ok(())
    }

    fn install_dependencies(&self) -> Result<()> {
        let args = [
            "install", "--save-dev", "@types/node", "@types/aws-lambda", "typescript"
        ];

        let stdout = std::process::Command::new("npm")
            .args(&args)
            .current_dir(self.source_directory_path()?)
            .stdout(Stdio::piped())
            .spawn()?
            .stdout
            .ok_or_else(|| std::io::Error::new(ErrorKind::Other, "Could not capture standard output."))?;

        let reader = BufReader::new(stdout);

        reader.lines()
            .filter_map(|line| line.ok())
            .for_each(|line| println!("{}", line));
        Ok(())
    }
}

/// Internal methods
impl App {
    fn source_directory_path(&self) -> Result<PathBuf> {
        let app_directory = self.app_directory_path()?;
        Ok(app_directory.join("src"))
    }

    fn app_directory_path(&self) -> Result<PathBuf> {
        let current_dir = std::env::current_dir()?;
        Ok(current_dir.join(&self.config.app_name))
    }
}
