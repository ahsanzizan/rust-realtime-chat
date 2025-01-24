# Create release directory
mkdir -p chat-app-release

# Build release binaries
cargo build --release

# Copy executables to release folder
cp target/release/server.exe chat-app-release/
cp target/release/client.exe chat-app-release/
