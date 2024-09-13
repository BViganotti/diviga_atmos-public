#!/bin/bash

# Format code
cargo fmt

# Function to log messages
log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1"
}

# Function to check if last command was successful
check_status() {
    if [ $? -ne 0 ]; then
        log "ERROR: $1"
        exit 1
    fi
}

# Start building
log "BUILDING"

# Run the build command
CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --release --target arm-unknown-linux-gnueabihf
check_status "Build failed"

log "Build successful"

# If we've made it here, the build was successful. Now we can push.
log "PUSHING"

# Push config file
scp -o StrictHostKeyChecking=no config.toml naughty@192.168.0.216:~/config.toml
check_status "Failed to push config.toml"

# Push binary
scp -o StrictHostKeyChecking=no target/arm-unknown-linux-gnueabihf/release/atmos naughty@192.168.0.216:~/atmosAAA
check_status "Failed to push atmos binary"

log "Push completed successfully"

log "Build and sync process completed"




################################################################################
#cargo fmt

#echo "BUILDING"
#cross build --release --target=arm-unknown-linux-gnueabihf
#CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --release --target arm-unknown-linux-gnueabihf



#docker run --rm -v $(pwd):/app rust_cross_compile/pi

#docker run -v /var/run/docker.sock:/var/run/docker.sock -v $(pwd):/app \
#   -w /app rust_cross_compile/pi cross build --target arm-unknown-linux-gnueabihf



##echo "PUSHING"
# leave this as atmos888 not to mess up the cron job

#scp -o StrictHostKeyChecking=no config.toml naughty@192.168.0.216:~/config.toml
#scp -o StrictHostKeyChecking=no target/arm-unknown-linux-gnueabihf/release/atmos naughty@192.168.0.216:~/atmosAAA

#scp -o StrictHostKeyChecking=no dht.py naughty@naughty:~/dht.py
