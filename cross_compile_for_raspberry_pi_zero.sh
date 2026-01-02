sudo apt install musl-tools -y

cargo install cross

cross build --target arm-unknown-linux-musleabihf --release

