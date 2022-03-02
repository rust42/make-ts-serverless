pub const DEPLOY_FN: &'static str = r#"
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
