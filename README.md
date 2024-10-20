# RabbitMQ Message Viewer

Connects to a given RabbitMQ instance and allows to view messages.

## Setting up

### Setting up the dependencies

For local development, set up a Docker container with RabbitMQ running. Use the compose file in `dependencies` directory to set one up. 

### Setting up the application

1. Set up the `.env` file based on the contents in `.env.example`. Please make sure that the correct details are populated. If you use the RabbitMQ container above, give `http://localhost:15672` as the URL, and username and password `user` and `password` respectively.
2. Run the backend Rocket server with `cargo run`. It should spin up a server at port `8000`.
3. Spin up the React server with `npm run start`. `package.json` has been updated with `proxy` configs to proxy client requests to port `8000`.
