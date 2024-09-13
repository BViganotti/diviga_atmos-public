# Atmos: Atmospheric Control System

![Atmos Logo](path/to/logo.png) <!-- Add a logo if you have one -->

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Project Structure](#project-structure)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [API Endpoints](#api-endpoints)
- [Development](#development)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

## Overview

Atmos is a sophisticated atmospheric control system designed to monitor and manage environmental conditions. Built with Rust, it leverages various sensors and relays to control temperature, humidity, and ventilation in a controlled environment. This project is ideal for applications such as home-made charcuterie, wine cellars, or any environment requiring precise atmospheric control.

## Features

- Real-time monitoring of temperature and humidity
- Automated control of cooling, heating, humidification, and dehumidification
- Web-based API for remote monitoring and control
- Integration with InfluxDB for data storage and analysis
- Configurable settings for different environmental requirements
- Robust error handling and logging

## Project Structure

The project is organized into several modules, each responsible for specific functionalities:

- [`src/main.rs`](src/main.rs): The entry point of the application.
- [`src/config.rs`](src/config.rs): Handles configuration settings.
- [`src/error.rs`](src/error.rs): Defines custom error types.
- [`src/initialization.rs`](src/initialization.rs): Initializes shared data and relay pins.
- [`src/monitor_atmosphere.rs`](src/monitor_atmosphere.rs): Monitors and controls atmospheric conditions.
- [`src/read_atmosphere.rs`](src/read_atmosphere.rs): Reads data from atmospheric sensors.
- [`src/relay_ctrl.rs`](src/relay_ctrl.rs): Controls relay operations.
- [`src/request_atmosphere.rs`](src/request_atmosphere.rs): Handles atmospheric data requests.
- [`src/routes/`](src/routes/): Contains API route handlers.
- [`src/shared_data.rs`](src/shared_data.rs): Manages shared data across threads.
- [`src/ventilation.rs`](src/ventilation.rs): Handles ventilation control.
- [`src/webserver.rs`](src/webserver.rs): Sets up and runs the web server.
- [`src/influx_client.rs`](src/influx_client.rs): Manages InfluxDB interactions.

## Installation

1. Ensure you have Rust and Cargo installed. If not, install them from [rustup.rs](https://rustup.rs/).

2. Clone the repository:
   ```
   git clone https://github.com/yourusername/atmos.git
   cd atmos
   ```

3. Install dependencies:
   ```
   cargo build
   ```

4. Set up the necessary hardware (Raspberry Pi, sensors, relays) according to the [hardware setup guide](docs/hardware_setup.md).

## Configuration

1. Copy the example configuration file:
   ```
   cp config.example.toml config.toml
   ```

2. Edit `config.toml` to match your environment and hardware setup.

3. Set up environment variables or use a `.env` file for sensitive information like database credentials.

## Usage

To compile the Atmos system:
```
CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --release --target arm-unknown-linux-gnueabihf
```

The system will start monitoring and controlling the atmosphere based on your configuration settings.

## API Endpoints

Atmos exposes several API endpoints for monitoring and control:

- `GET /api/atmosphere/full`: Retrieves full atmospheric data.
- `POST /change_fridge_status`: Changes the fridge relay status.
- `POST /change_humidifier_status`: Changes the humidifier relay status.
- `POST /change_dehumidifier_status`: Changes the dehumidifier relay status.
- `POST /change_ventilator_status`: Changes the ventilator relay status.

For detailed API documentation, refer to the [API Documentation](docs/api.md).

## Development

To set up the development environment:

1. Install development dependencies:
   ```
   cargo install --path .
   ```

2. Run the project in development mode:
   ```
   cargo run
   ```

3. For hot-reloading during development, you can use `cargo-watch`:
   ```
   cargo install cargo-watch
   cargo watch -x run
   ```

## Testing

Run the test suite with:
```
cargo test
```
 