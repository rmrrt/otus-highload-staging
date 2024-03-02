# Use the official Rust image as a parent image
FROM rust:latest

# Set env variables
ENV ROCKET_ADDRESS=0.0.0.0
ENV CARGO_BUILD_JOBS=4

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY ./ .

# Install cargo-watch
RUN cargo install cargo-watch

# Build code for debug
RUN cargo build

# Use cargo watch to run the application
CMD ["cargo", "watch", "-w", "/usr/src/app", "-x", "run", "--poll"]

#Expose the 8000 port so that Rocket server could be accessed
EXPOSE 8000
