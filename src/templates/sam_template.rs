pub const TEMPLATE_YML: &'static str = r#"
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