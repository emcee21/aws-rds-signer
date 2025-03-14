
DB_TOKEN_EXPIRES_IN_SECONDS=${DB_TOKEN_EXPIRES_IN_SECONDS:-90}
DB_IDENTIFIER=${DB_IDENTIFIER:-my-db-identifer}
DB_PORT=${DB_PORT:-5432}
DB_REGION=${DB_REGION:-us-east-1}
DB_USER=${DB_USER:-my-db-user}

DB_HOST=$(aws rds describe-db-instances --db-instance-identifier $DB_IDENTIFIER --query "DBInstances[0].Endpoint.Address" --output text)

export DB_TOKEN_EXPIRES_IN_SECONDS DB_PORT DB_REGION DB_USER DB_HOST

cargo test -- --nocapture