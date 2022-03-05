RUST_TARGET="aarch64-unknown-linux-gnu" # corresponding with the above, set this to aarch64 or x86_64 -unknown-linux-gnu for ARM or x86 functions.

zipRustLambda() {
  echo "NANA DOING"
	cp ./target/${RUST_TARGET}/release/rust_on_aws ./bootstrap && zip lambda.zip bootstrap && rm bootstrap
}
