
pub const BUILD_SRC: &'static str = r#"
#!/usr/env/sh
tsc
cp package.json ./dist/package.json
"#;