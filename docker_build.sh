# First run this to create a cargo data cache container
# docker create -v /cargo --name cargo-cache tianon/true /bin/true

# Then run this to build the project inside of a container
docker run --rm -v $(pwd):/src --volumes-from cargo-cache fnichol/rust:nightly cargo build --release

# Then you must copy the log config into the release directory
cp log4rs.toml ./target/release

# Then run this to build the superball container with the release target copied into it
docker build --tag='superball:v1' .

# Then do this to start superball inside the container
docker run superball:v1