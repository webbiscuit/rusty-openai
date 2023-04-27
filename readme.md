# Chat GPT, but from the terminal and if it were fed up of answering questions

![Terminal GPT](Screenshot.png)

## Building

- Install rust
- Install [just](https://github.com/casey/just)
- Install [openapi-generator-cli](https://github.com/OpenAPITools/openapi-generator-cli)
- Set OPENAI_API_KEY to your OpenAI API key (you can use a .env file)
- `just generate-api`
- `just run`

## TODO 
WIP - the generated API doesn't quite work, it looks like OneOf is not yet supported by the Rust generator