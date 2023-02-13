# crayon
Crayon is a place for me to showcase different mini-projects, tools, visualizations, etc.
When deciding a name, I was visualizing a bucket full of colourful crayons.
It's chaotic and messy, but the colours draw you in and inspire creativity.
I'm hoping that idea is an appropriate metaphor for this project.

### Development

    cargo build
    cargo test

    ./target/debug/content_generator -h
    ./target/debug/content_generator target/resources

    ./target/debug/crayon -h
    ./target/debug/crayon_server 8080 target/resources/

    curl http://localhost:8080/api
    open http://localhost:8080
