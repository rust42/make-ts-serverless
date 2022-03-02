pub const APP_FILE: &'static str = r#"
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