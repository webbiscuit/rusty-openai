run:
    cargo run --bin rusty-openai-runner

generate-api:
    openapi-generator-cli generate -i specs/openapi.yml -g rust -o openaiapi --skip-validate-spec