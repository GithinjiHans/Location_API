IP2Location API
To run this on your CentOS 7 server, you'll need to follow these steps:

Install Rust
First, you need to install Rust on your CentOS 7 server. You can do this by running the following command:

curl --proto '=https' --tlsv1.2 -sSf <https://sh.rustup.rs> | sh
This command will download and run the Rust installation script. Follow the prompts to complete the installation.

Download IP2Location Database
Next, you need to obtain the IP2Location IP-COUNTRY-REGION-CITY-LATITUDE-LONGITUDE.BIN database. You can download it from the IP2Location website. Once downloaded, place the database file in the IP2Location folder in your project.

Run the Server
To run the server, use the following command from the root of your project:

cargo run --release
This command will compile and execute your Rust program, starting the server.

API Endpoint
The API endpoint is:

GET /
Testing the API
You can test the API using a tool like Postman. Send a GET request to the following URL, replacing <server_addr> and <port> with the appropriate values:

GET <server_addr>:<port>/
This will trigger the API request and you should receive a response with the IP information.
