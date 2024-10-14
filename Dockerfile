FROM ubuntu:22.04

# Install Node.js, build-essential, and libssl-dev
RUN apt-get update && \
    apt-get install -y curl build-essential libssl-dev pkg-config && \
    curl -fsSL https://deb.nodesource.com/setup_14.x | bash - && \
    apt-get install -y nodejs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Rust
# Get Rust; NOTE: using sh for better compatibility with other base images
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Set environment variables for Rust
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory
WORKDIR /app

# Copy the entire project to the working directory
COPY . .

# Create a directory for static files
RUN mkdir /app/static

# Build the React frontend
WORKDIR /app/client
RUN npm install
RUN npm run build

# Build the Rust backend
WORKDIR /app
RUN cargo build --release

# Set environment variables for Rocket
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8088

# Expose the port Rocket will run on
EXPOSE 8088

# Command to run the application
CMD ["./target/release/rabbitmq-messages-management"]
