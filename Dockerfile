# Stage 1: Install the necessary dependencies
FROM ubuntu:22.04 as builder

RUN apt-get update && \
    apt-get install -y curl build-essential

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust to the PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Stage 2: Build the Rust application
FROM builder as final

WORKDIR /app

RUN apt install -y git



# Keep the container running
CMD tail -f /dev/null
