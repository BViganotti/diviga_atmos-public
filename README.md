Atmospheric control system

This project is a home project that monitors the temperature and humidity level in an enclosed space. It is running on a raspberry pi zero w, a 4 channel relay board, Rust, shady soldering and even shadier "webinterface" ( single page of rendered pure HTML, and i mean PURE because there is not even some CSS there ( i was in a hurry )).
The code control the turning on and off a multiple elements, a fridge, a dehumidifier, a fan and a humidifier ( and optionally a heater in winter ). This is all to make home made charcuterie as it is something i'm very fond of. Right now i'm in the process of making Prosciutto.

For this project i went with Rust as i really love the language and was looking for an opportunity ( at the time ) to learn the language. After a few days i went from loving the language to really loving the language and its ecosystem. From the compilation based checks, error reporting, to the cargo ecosystem, incredible libs like cross-rs which is a godsent for cross compilation when you come from C++, Rust is truely amazing.
