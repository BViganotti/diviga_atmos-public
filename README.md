Atmospheric control system

This project is a home project that monitors the temperature and humidity level in an enclosed space. It is running on a raspberry pi zero w, a 4 channel relay board, Rust, shady soldering and even shadier "webinterface" ( single page of rendered pure HTML, and i mean PURE because there is not even some CSS there ( i was in a hurry )).
The code control the turning on and off a multiple elements, a fridge, a dehumidifier, a fan and a humidifier ( and optionally a heater in winter ). This is all to make home made charcuterie as it is something i'm very fond of. Right now i'm in the process of making Prosciutto.

For this project i went with Rust as i really love the language and was looking for an opportunity ( at the time ) to learn the language. After a few days i went from loving the language to really loving the language and its ecosystem. From the compilation based checks, error reporting, to the cargo ecosystem, incredible libs like cross-rs which is a godsent for cross compilation when you come from C++, Rust is truely amazing.



Atmos Project Documentation
Project Overview
Atmos is a Rust-based atmospheric control system designed to monitor and manage environmental conditions. It uses various sensors and relays to control temperature, humidity, and ventilation in a controlled environment.
Project Structure
The project is organized into several modules, each responsible for specific functionalities:
main.rs: The entry point of the application.
config.rs: Handles configuration settings.
error.rs: Defines custom error types.
initialization.rs: Initializes shared data and relay pins.
monitor_atmosphere.rs: Monitors and controls atmospheric conditions.
read_atmosphere.rs: Reads data from atmospheric sensors.
relay_ctrl.rs: Controls relay operations.
request_atmosphere.rs: Handles atmospheric data requests.
routes/: Contains API route handlers.
shared_data.rs: Manages shared data across threads.
ventilation.rs: Handles ventilation control.
webserver.rs: Sets up and runs the web server.
influx_client.rs: Manages InfluxDB interactions.
Key Components
Main Application Flow
The main application flow is defined in main.rs:

This function sets up the environment, initializes shared data and relay pins, and spawns several asynchronous tasks for atmosphere monitoring, ventilation control, and web server operation.
Initialization
The initialization.rs module handles the setup of shared data and relay pins:

initialize_shared_data(): Creates and initializes the SharedData structure.
initialize_relay_pins(): Sets all relay pins to their initial (off) state.
Web Server
The web server is set up in webserver.rs:

This function creates an HTTP server with various routes for getting atmospheric data and controlling relays.
Relay Control
Relay control operations are handled in routes/relay_control.rs:

These functions handle HTTP POST requests to change the status of various relays (fridge, humidifier, dehumidifier, ventilator).
Atmosphere Monitoring
The monitor_atmosphere.rs module contains logic for monitoring and controlling atmospheric conditions:
;
This module includes functions for handling different aspects of atmosphere control, such as managing the fridge, humidifier, dehumidifier, and ventilator based on current conditions.
InfluxDB Integration
The influx_client.rs module handles interactions with InfluxDB for data storage:

This module provides functionality to write atmosphere data to an InfluxDB database.
Configuration
The project uses environment variables and a configuration file for settings. The Settings struct in config.rs loads and stores these configurations.
API Routes
The project exposes several API routes:
GET /api/atmosphere/full: Retrieves full atmospheric data.
POST /change_fridge_status: Changes the fridge relay status.
POST /change_humidifier_status: Changes the humidifier relay status.
POST /change_dehumidifier_status: Changes the dehumidifier relay status.
POST /change_ventilator_status: Changes the ventilator relay status.
Error Handling
Custom error types are defined in error.rs to handle various error scenarios specific to the Atmos system.
Concurrency and Shared State
The project uses Tokio for asynchronous operations and Arc<Mutex<>> for sharing state across multiple threads safely.
Dependencies
Key dependencies include:
actix-web for the web server
tokio for asynchronous runtime
serde for serialization/deserialization
influxdb for InfluxDB integration
rppal for Raspberry Pi GPIO control
For a full list of dependencies, refer to the Cargo.toml file.
Running the Project
To run the project:
Ensure all dependencies are installed.
Set up the necessary environment variables or configuration file.
Run cargo run in the project directory.
Testing
The project includes unit tests, particularly for the atmosphere monitoring logic. Run tests using cargo test.
Future Improvements
Implement more comprehensive error handling and logging.
Add more detailed documentation for each module.
Implement a front-end interface for easier control and monitoring.
Expand test coverage to include integration tests.
This documentation provides an overview of the Atmos project structure and functionality. For more detailed information on specific functions or modules, refer to the inline comments in the respective files.