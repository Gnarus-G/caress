set -xe;
crate=$1;

cargo build --all --release;
strip target/release/$crate;
mv target/release/$crate $crate

tar -czvf $crate-$OSTYPE.tar.gz $crate README.md
