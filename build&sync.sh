cargo fmt

echo "BUILDING"
cross build --release --target=arm-unknown-linux-gnueabihf

echo "PUSHING"
# leave this as atmos888 not to mess up the cron job
scp -o StrictHostKeyChecking=no target/arm-unknown-linux-gnueabihf/release/readwrite naughty@naughty:~/atmos999
scp -o StrictHostKeyChecking=no dht.py naughty@naughty:~/dht.py
