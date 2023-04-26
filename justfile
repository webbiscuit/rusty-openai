run:
    cargo run --bin rusty-openai-runner

generate-api:
    openapi-generator-cli generate -i specs/openaiapi.yml -g rust -o openaiapi_generated --skip-validate-spec