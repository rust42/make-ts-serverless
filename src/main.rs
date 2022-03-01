use std::any::TypeId;
use std::error::Error;
use std::string::String;
use std::fmt::{Debug, Display, format, Formatter};
use clap::{Command, Arg};
use std::fs::copy;
use std::io::{BufRead, BufReader, ErrorKind, Result, Stdout, stdout};
use std::process::Stdio;
use std::ptr::write;

enum AppError {
    MissingName,
    MissingBucketName,
    MissingStackName,
}

impl AppError {

    fn description(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::MissingName => {
                write!(f, "App name missing")
            },
            AppError::MissingBucketName => {
                write!(f, "Bucket name missing")
            },
            AppError::MissingStackName => {
                write!(f, "Missing stack name")
            }
        }
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.description(f)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.description(f)
    }
}

const TS_CONFIG: &'static str = r#"
{
  "compilerOptions": {
    /* Visit https://aka.ms/tsconfig.json to read more about this file */

    /* Projects */
    // "incremental": true,                              /* Enable incremental compilation */
    // "composite": true,                                /* Enable constraints that allow a TypeScript project to be used with project references. */
    // "tsBuildInfoFile": "./",                          /* Specify the folder for .tsbuildinfo incremental compilation files. */
    // "disableSourceOfProjectReferenceRedirect": true,  /* Disable preferring source files instead of declaration files when referencing composite projects */
    // "disableSolutionSearching": true,                 /* Opt a project out of multi-project reference checking when editing. */
    // "disableReferencedProjectLoad": true,             /* Reduce the number of projects loaded automatically by TypeScript. */

    /* Language and Environment */
    "target": "es2016",                                  /* Set the JavaScript language version for emitted JavaScript and include compatible library declarations. */
    // "lib": [],                                        /* Specify a set of bundled library declaration files that describe the target runtime environment. */
    // "jsx": "preserve",                                /* Specify what JSX code is generated. */
    // "experimentalDecorators": true,                   /* Enable experimental support for TC39 stage 2 draft decorators. */
    // "emitDecoratorMetadata": true,                    /* Emit design-type metadata for decorated declarations in source files. */
    // "jsxFactory": "",                                 /* Specify the JSX factory function used when targeting React JSX emit, e.g. 'React.createElement' or 'h' */
    // "jsxFragmentFactory": "",                         /* Specify the JSX Fragment reference used for fragments when targeting React JSX emit e.g. 'React.Fragment' or 'Fragment'. */
    // "jsxImportSource": "",                            /* Specify module specifier used to import the JSX factory functions when using `jsx: react-jsx*`.` */
    // "reactNamespace": "",                             /* Specify the object invoked for `createElement`. This only applies when targeting `react` JSX emit. */
    // "noLib": true,                                    /* Disable including any library files, including the default lib.d.ts. */
    // "useDefineForClassFields": true,                  /* Emit ECMAScript-standard-compliant class fields. */

    /* Modules */
    "module": "commonjs",                                /* Specify what module code is generated. */
    // "rootDir": "./",                                  /* Specify the root folder within your source files. */
    // "moduleResolution": "node",                       /* Specify how TypeScript looks up a file from a given module specifier. */
    // "baseUrl": "./",                                  /* Specify the base directory to resolve non-relative module names. */
    // "paths": {},                                      /* Specify a set of entries that re-map imports to additional lookup locations. */
    // "rootDirs": [],                                   /* Allow multiple folders to be treated as one when resolving modules. */
    // "typeRoots": [],                                  /* Specify multiple folders that act like `./node_modules/@types`. */
    // "types": [],                                      /* Specify type package names to be included without being referenced in a source file. */
    // "allowUmdGlobalAccess": true,                     /* Allow accessing UMD globals from modules. */
    // "resolveJsonModule": true,                        /* Enable importing .json files */
    // "noResolve": true,                                /* Disallow `import`s, `require`s or `<reference>`s from expanding the number of files TypeScript should add to a project. */

    /* JavaScript Support */
    // "allowJs": true,                                  /* Allow JavaScript files to be a part of your program. Use the `checkJS` option to get errors from these files. */
    // "checkJs": true,                                  /* Enable error reporting in type-checked JavaScript files. */
    // "maxNodeModuleJsDepth": 1,                        /* Specify the maximum folder depth used for checking JavaScript files from `node_modules`. Only applicable with `allowJs`. */

    /* Emit */
    // "declaration": true,                              /* Generate .d.ts files from TypeScript and JavaScript files in your project. */
    // "declarationMap": true,                           /* Create sourcemaps for d.ts files. */
    // "emitDeclarationOnly": true,                      /* Only output d.ts files and not JavaScript files. */
    // "sourceMap": true,                                /* Create source map files for emitted JavaScript files. */
    // "outFile": "./",                                  /* Specify a file that bundles all outputs into one JavaScript file. If `declaration` is true, also designates a file that bundles all .d.ts output. */
    "outDir": "./dist",                                   /* Specify an output folder for all emitted files. */
    // "removeComments": true,                           /* Disable emitting comments. */
    // "noEmit": true,                                   /* Disable emitting files from a compilation. */
    // "importHelpers": true,                            /* Allow importing helper functions from tslib once per project, instead of including them per-file. */
    // "importsNotUsedAsValues": "remove",               /* Specify emit/checking behavior for imports that are only used for types */
    // "downlevelIteration": true,                       /* Emit more compliant, but verbose and less performant JavaScript for iteration. */
    // "sourceRoot": "",                                 /* Specify the root path for debuggers to find the reference source code. */
    // "mapRoot": "",                                    /* Specify the location where debugger should locate map files instead of generated locations. */
    // "inlineSourceMap": true,                          /* Include sourcemap files inside the emitted JavaScript. */
    // "inlineSources": true,                            /* Include source code in the sourcemaps inside the emitted JavaScript. */
    // "emitBOM": true,                                  /* Emit a UTF-8 Byte Order Mark (BOM) in the beginning of output files. */
    // "newLine": "crlf",                                /* Set the newline character for emitting files. */
    // "stripInternal": true,                            /* Disable emitting declarations that have `@internal` in their JSDoc comments. */
    // "noEmitHelpers": true,                            /* Disable generating custom helper functions like `__extends` in compiled output. */
    // "noEmitOnError": true,                            /* Disable emitting files if any type checking errors are reported. */
    // "preserveConstEnums": true,                       /* Disable erasing `const enum` declarations in generated code. */
    // "declarationDir": "./",                           /* Specify the output directory for generated declaration files. */
    // "preserveValueImports": true,                     /* Preserve unused imported values in the JavaScript output that would otherwise be removed. */

    /* Interop Constraints */
    // "isolatedModules": true,                          /* Ensure that each file can be safely transpiled without relying on other imports. */
    // "allowSyntheticDefaultImports": true,             /* Allow 'import x from y' when a module doesn't have a default export. */
    "esModuleInterop": true,                             /* Emit additional JavaScript to ease support for importing CommonJS modules. This enables `allowSyntheticDefaultImports` for type compatibility. */
    // "preserveSymlinks": true,                         /* Disable resolving symlinks to their realpath. This correlates to the same flag in node. */
    "forceConsistentCasingInFileNames": true,            /* Ensure that casing is correct in imports. */

    /* Type Checking */
    "strict": true,                                      /* Enable all strict type-checking options. */
    // "noImplicitAny": true,                            /* Enable error reporting for expressions and declarations with an implied `any` type.. */
    // "strictNullChecks": true,                         /* When type checking, take into account `null` and `undefined`. */
    // "strictFunctionTypes": true,                      /* When assigning functions, check to ensure parameters and the return values are subtype-compatible. */
    // "strictBindCallApply": true,                      /* Check that the arguments for `bind`, `call`, and `apply` methods match the original function. */
    // "strictPropertyInitialization": true,             /* Check for class properties that are declared but not set in the constructor. */
    // "noImplicitThis": true,                           /* Enable error reporting when `this` is given the type `any`. */
    // "useUnknownInCatchVariables": true,               /* Type catch clause variables as 'unknown' instead of 'any'. */
    // "alwaysStrict": true,                             /* Ensure 'use strict' is always emitted. */
    // "noUnusedLocals": true,                           /* Enable error reporting when a local variables aren't read. */
    // "noUnusedParameters": true,                       /* Raise an error when a function parameter isn't read */
    // "exactOptionalPropertyTypes": true,               /* Interpret optional property types as written, rather than adding 'undefined'. */
    // "noImplicitReturns": true,                        /* Enable error reporting for codepaths that do not explicitly return in a function. */
    // "noFallthroughCasesInSwitch": true,               /* Enable error reporting for fallthrough cases in switch statements. */
    // "noUncheckedIndexedAccess": true,                 /* Include 'undefined' in index signature results */
    // "noImplicitOverride": true,                       /* Ensure overriding members in derived classes are marked with an override modifier. */
    // "noPropertyAccessFromIndexSignature": true,       /* Enforces using indexed accessors for keys declared using an indexed type */
    // "allowUnusedLabels": true,                        /* Disable error reporting for unused labels. */
    // "allowUnreachableCode": true,                     /* Disable error reporting for unreachable code. */

    /* Completeness */
    // "skipDefaultLibCheck": true,                      /* Skip type checking .d.ts files that are included with TypeScript. */
    "skipLibCheck": true                                 /* Skip type checking all .d.ts files. */
  }
}
"#;

const DEPLOY_FN: &'static str = r#"
#!/usr/env/sh

BUCKET_NAME=<BUCKET_NAME>
STACK_NAME=<STACK_NAME>

cd src
npm run build
cd ../

echo "Building the app ..."
sam build

echo "Packaging ... "
sam package --s3-bucket $BUCKET_NAME --output-template-file output.yaml

echo "Deploying the application ..."
sam deploy --template-file output.yaml --stack-name $STACK_NAME --capabilities CAPABILITY_IAM
"#;

const TEMPLATE_YML: &'static str = r#"
AWSTemplateFormatVersion: "2010-09-09"
Transform: AWS::Serverless-2016-10-31
Description: >
    <APP_NAME> app description

Resources:
    <APP_NAME>Function:
        Type: AWS::Serverless::Function
        Properties:
            CodeUri: src/dist/
            Handler: index.lambdaHandler
            Runtime: nodejs12.x
            Architectures:
                - x86_64

            Events:
                <APP_NAME>Api:
                    Type: Api
                    Properties:
                        Path: /welcome
                        Method: get
Outputs:
    <APP_NAME>Api:
        Description: "API Gateway endpoint URL for prod stage for <APP_NAME> app function"
        Value: !Sub "https://${ServerlessRestApi}.execute-api.${AWS::Region}.amazonaws.com/Prod/welcome"
    <APP_NAME>Function:
        Description: "<APP_NAME> api function arn"
        Value: !GetAtt <APP_NAME>Function.Arn
    <APP_NAME>ApiFunctionIamRole:
        Description: "Implicit IAM Role"
        Value: !GetAtt <APP_NAME>Function.Arn

"#;

const NPM_PACKAGE_JSON: &'static str = r#"
{
  "name": "<APP_NAME>",
  "version": "1.0.0",
  "scripts": {
    "build": "sh build.sh"
  }
}
"#;

const BUILD_SRC: &'static str = r#"#!/usr/env/sh
tsc
cp package.json ./dist/package.json
"#;

const APP_FILE: &'static str = r#"
import { APIGatewayProxyEventV2, APIGatewayProxyResultV2, Handler } from 'aws-lambda';

type LambdaHandler = Handler<APIGatewayProxyEventV2, APIGatewayProxyResultV2>;

const lambdaHandler: LambdaHandler = async (event, context) => {

    console.log(`Event received ${JSON.stringify(event)}`);

    return {
        statusCode: 200,
        body: JSON.stringify({
            message: "Hello from <APP_NAME>"
        }, null, 2)
    }
};

exports.lambdaHandler = lambdaHandler;
"#;

impl Error for AppError {
    fn description(&self) -> &str {
       "AppError occurred"
    }
}

impl From<AppError> for std::io::Error {
    fn from(error: AppError) -> Self {
      std::io::Error::new(ErrorKind::Unsupported, "App name must be provided")
    }
}

fn main() ->  Result<()> {
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
    let current_dir = std::env::current_dir()?;


    let current_dir: &str = current_dir.to_str().ok_or(AppError::MissingName)?;
    let app_dir = format!("{current_dir}/{app_name}");
    let source_dir = format!("{app_dir}/src");

    std::fs::create_dir(&app_dir);
    std::fs::create_dir(&source_dir);
    std::fs::write(format!("{app_dir}/template.yaml"), TEMPLATE_YML.replace("<APP_NAME>", app_name));
    std::fs::write(format!("{app_dir}/deploy.sh"), DEPLOY_FN.replace("<BUCKET_NAME>", bucket_name).replace("<STACK_NAME>", stack_name));
    std::fs::write(format!("{source_dir}/package.json"), NPM_PACKAGE_JSON.replace("<APP_NAME>", app_name));
    std::fs::write(format!("{source_dir}/build.sh"), BUILD_SRC);
    std::fs::write(format!("{source_dir}/index.ts"), APP_FILE.replace("<APP_NAME>", app_name));
    std::fs::write(format!("{source_dir}/tsconfig.json"), TS_CONFIG);
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
