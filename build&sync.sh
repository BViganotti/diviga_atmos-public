cargo fmt

echo "BUILDING"
#cross build --release --target=arm-unknown-linux-gnueabihf
CROSS_CONTAINER_OPTS="--platform linux/amd64" cross build --release --target arm-unknown-linux-gnueabihf



#docker run --rm -v $(pwd):/app rust_cross_compile/pi

#docker run -v /var/run/docker.sock:/var/run/docker.sock -v $(pwd):/app \
#   -w /app rust_cross_compile/pi cross build --target arm-unknown-linux-gnueabihf



echo "PUSHING"
# leave this as atmos888 not to mess up the cron job

#scp -o StrictHostKeyChecking=no config.toml naughty@192.168.0.216:~/config.toml
#scp -o StrictHostKeyChecking=no target/arm-unknown-linux-gnueabihf/release/atmos naughty@192.168.0.216:~/atmosAAA

#scp -o StrictHostKeyChecking=no dht.py naughty@naughty:~/dht.py
