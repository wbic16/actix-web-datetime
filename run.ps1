# apply cargo/trunk to get leptos running
# to read more: https://book.leptos.dev/getting_started/index.html

cargo build
cargo test
write-host "http://localhost:8000/ should be working now - press Ctrl-c to continue"
cargo run


cargo add leptos --features=csr,nightly --package actix-web-datetime-frontend
rustup override set nightly
rustup target add wasm32-unknown-unknown
set-location frontend
trunk serve --open
write-host "http://localhost:8080/ should be working now"