///
/// The structure of app is in the following format:
/// App [APP_NAME] passed from commandline
///
/// - App
///     - deploy.sh
///     - template.yaml
///     - src
///         - build.sh
///         - index.sh
///         - package.sh
///
/// Templates inside App root folder are App
/// Templates inside src folder are Src

pub enum TemplateKind {
    /// The template that is usually placed inside app folder
    App,

    /// The template that is placed inside src folder
    Src,
}