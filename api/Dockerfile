# Use the official Rust image as a parent image
FROM rust:latest
ENV ROCKET_ADDRESS=0.0.0.0

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY ./ .

# Install cargo-watch
RUN cargo install cargo-watch

# Build your program for release
RUN cargo build --release

# Use cargo watch to run the application
CMD ["cargo", "watch", "-w", "/usr/src/app", "-x", "run", "--poll"]

EXPOSE 8000
