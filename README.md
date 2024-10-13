# RabbitMQ Message Viewer

Connects to a given RabbitMQ instance and allows to view messages.

## Setting up

1. Run the backend Rocket server with `cargo run`. It should spin up a server at port `8000`.
2. Spin up the React server with `npm run start`. `package.json` has been updated with `proxy` configs to proxy client requests to port `8000`.
