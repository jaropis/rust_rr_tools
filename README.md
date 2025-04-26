# rust_rr_tools

RRI:
./rust_rr_tools "rri" "rea" 1000 true 3 false 1024
cargo run "rri" "rea" 1000 true 3 false 1024
MASTER
./rust_rr_tools "txt" "rea" 1000 true 1 false
cargo run "txt" "rea" 1000 false 3 false
