# Mikey, BengalBots's Discord Bot

Mikey is BengalBots's Discord Bot built using Rust, [Serenity](https://github.com/serenity-rs/serenity), and [Poise](https://github.com/serenity-rs/poise). It is deployed using [Shuttle](https://www.shuttle.dev/).


# Running Examples

To run specific examples, simply use `cargo run --example` with flags set.
E.g. `cargo run --example rich_embed` will run the "rich_embed" example.

# Deployment

This repository is configured to be deployed to Shuttle. As such, the following commands would be helpful to test and deploy:

`shuttle run` will run Mikey locally with whatever tokens are stored in the `Secrets.dev.toml` file at the root folder.

`shuttle deploy` will deploy the project to whatever project ID is specified in the `./.shuttle/config.toml` file.

Lastly, `shuttle deployment stop` will end the life of whatever deployment is currently running.