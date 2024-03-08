# Use the official Rust image as a parent image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY ./ .

# Use the offline feature of sqlx to avoid database connection during build
ENV SQLX_OFFLINE true

# Install cargo-watch
RUN cargo install cargo-watch

# Build code for debug
RUN cargo build

# Disable SQLX_OFFLINE for runtime
ENV SQLX_OFFLINE false

# Use cargo watch to run the application
CMD ["cargo", "watch", "-w", "/usr/src/app", "-x", "run", "--poll"]

# Expose the 8000 port so that Rocket server could be accessed
EXPOSE 8000
