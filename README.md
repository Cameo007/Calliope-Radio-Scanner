# Calliope Radio Scanner
### Debug and manipulate the radio traffic between [Calliope minis](https://calliope.cc/)!

# Moved to [Codeberg](https://codeberg.org/Cameo007/Calliope-Radio-Scanner.git)

#### It is 
- The "antenna", written in Scratch on the Calliope Mini itself.
- The middleware, written in Rust communicates between the Calliope via USB/serial and the server. But it can also be used without the server.
- The backend (in Python, Flask). It transmitts data between the Rust middleware and the frontend.
- The frontend, a website, where you can watch the traffic or resend it.

### General information
There are 3 modes in which the Calliope can be:
- `Communicator mode`: In this mode the Calliope listens on a specific channel
- `Scanner mode`: In this mode the Calliope scans for used channels
- `Test mode`: This mode is for testing everything, as with it, you can send dummy data on a specific channel

***

### Calliope Antenna
The Calliope communicates with the Middleware via a serial port.
More information about the used commands are available [here](https://github.com/Cameo007/Calliope-Radio-Scanner/wiki/Rust-Spec)

***

### Rust Middleware
```
		Calliope Radio Scanner - Debug and manipulate the radio traffic between Calliope minis!

		Usage: crs [COMMAND]

		Some Commands:

		 detect                                    List all connected Calliope minis

		 communicator <COM-Port> <CHANNEL>         Use Calliope to listen on a specific channel
		 scanner <COM-Port> <SCANNER-FREQUENCY>    Use Calliope mini to search for channels
		 tester <COM-Port> <CHANNEL>               Run the test mode

		 scanner_restart <COM-Port>                Restart Scanner mode
   ```
#### Environment variables
- `CRS_ENDPOINT`: The API endpoint (default: `http://localhost:8123`)

***

### Backend
The API documentation is available [here](https://github.com/Cameo007/Calliope-Radio-Scanner/wiki/API-Spec#get-actionchannel).

Because it uses [flask](https://flask.palletsprojects.com/), you need to run `pip3 install flask flask_cors`.

***

### Frontend
