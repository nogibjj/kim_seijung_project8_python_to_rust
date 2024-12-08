# Start with a lightweight Python image
FROM python:3.8-slim

# Install Rust using rustup
RUN apt-get update && \
    apt-get install -y curl build-essential && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . "$HOME/.cargo/env" && \
    rustup default stable

# Set the working directory
WORKDIR /workspace

# Copy Python and Rust files
COPY . .

# Install Python dependencies
RUN pip install --upgrade pip && pip install -r requirements.txt

# Build the Rust project
RUN . "$HOME/.cargo/env" && cargo build --release

# Command to run when the container starts, defaults to Python script
CMD ["python", "scripts/python_script"]