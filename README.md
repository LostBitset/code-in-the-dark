# code-in-the-dark

## How to run

### "Virtual Sensor"

This part is a WebSockets server written in Rust.

To run it, `cd virtual_sensor`, and then `cargo run`. 

### "Client App"

This part is a WebSockets client written in Python, which reads data from the `client/session.txt` file.

To run it, `cd client`, then `python main.py`. You will need to install the `python3-websockets` dependency with your system package manager beforehand. 
