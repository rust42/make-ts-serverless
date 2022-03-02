use std::io::{BufRead, Result};
use std::collections::HashMap;
use std::io::{BufReader, ErrorKind};
use std::process::Stdio;
use std::path::Path;
use clap::{Command, Arg};


mod config;
mod app_error;

use config::Config;
use app_error::AppError;

mod templates;
use templates:: {
    BUILD_SRC,
    DEPLOY_FN,
    APP_FILE,
    TEMPLATE_YML,
    NPM_PACKAGE_JSON,
    TS_CONFIG,
    TemplateKind,
    render_template,
};

pub fn get_config() -> Result<Config> {
    let app =  Command::new("make-ts-app")
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

pub fn run(config: &Config) -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let current_dir: &str = current_dir.to_str().ok_or(AppError::MissingName)?;
    let app_dir = format!("{current_dir}/{}", config.app_name);
    let source_dir = format!("{app_dir}/src");

    std::fs::create_dir(&app_dir)?;
    std::fs::create_dir(&source_dir)?;

    let templates = templates::Template::get_all();

    let mut template_vars:HashMap<_, _> = HashMap::new();

    template_vars.insert("<APP_NAME>", &config.app_name);
    template_vars.insert("<STACK_NAME>", &config.stack_name);
    template_vars.insert("<BUCKET_NAME>", &config.bucket_name);

    for template in templates {
        let content = render_template(&template, &template_vars);

        let file_path = match template.kind {
            TemplateKind::App => {
                Path::new(&app_dir).join(format!("{}.{}", template.name, template.extension))
            },
            TemplateKind::Src => {
                Path::new(&source_dir).join(format!("{}.{}", template.name, template.extension))
            },
        };

        std::fs::write(file_path, content)?;
    }
    let args = [
        "install", "--save-dev", "@types/node", "@types/aws-lambda", "typescript"
    ];

    let stdout = std::process::Command::new("npm")
        .args(&args)
        .current_dir(source_dir)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| std::io::Error::new(ErrorKind::Other,"Could not capture standard output."))?;

    let reader = BufReader::new(stdout);

    reader.lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));
    Ok(())
}