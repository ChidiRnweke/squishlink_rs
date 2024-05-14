# URL Shortener Service

## Overview

This Rust-based URL shortener service offers a simple yet effective way to convert long URLs into shorter links. Shortened links are generated through a unique combination of an adjective, noun, and number, providing easy-to-remember URLs.

This project is a port of an existing project I made [using pure functional Scala](https://github.com/ChidiRnweke/SquishLink-backend). The frontend of the project can be found in [this repository](https://github.com/ChidiRnweke/SquishLink-frontend) and is deployed on [chidinweke.be/squish](chidinweke.be/squish). 

The version using Rust is the one currently deployed as it only consumes 8 megabytes of memory while idle compared to the 160 megabytes that the Scala version consumes.

## Key Features

- **Shorten URLs**: Converts long URLs into shorter versions using a memorable pattern.
- **Retrieve Original URLs**: Allows users to access the original URL by visiting the shortened link.


## How It Works

The service generates short links by combining an adjective, a noun, and a number, creating identifiable, but sometimes goofy URLs. The application interfaces with a database to store and retrieve original URLs based on their shortened counterparts.

## Architecture

The application's architecture is modular, separating concerns into configuration loading, database operations, and HTTP service handling. This structure simplifies maintenance and enhances the clarity of the codebase.

- **main.rs**: Bootstraps the application and sets up the HTTP server.
- **config.rs**: Loads configuration settings from the environment.
- **shorten.rs**: Contains the logic for URL shortening and database interaction.
- **cleanup.rs**: Runs a background task that cleans links after 7 days.

## Getting Started

The project only contains unit tests. To run the application you must spin up a docker container using 

```bash
docker compose -f docker-compose-dev.yml up -d
```

Running the application or its tests is straightforward:

* To run the service: `cargo run`

The environment variables in the docker compose dev file correspond with the defaults inside the application.

If you want to deploy this application yourself you can modify the CI/CD template found in `.github/workflows/main.yml` as well as `compose.yml` to ensure the application and database are on the same network.

## Testing

To execute tests: `cargo test`. This project only contains a few unit tests, so a database/test containers aren't necessary to run them.
