# Reference: https://betterprogramming.pub/building-an-api-gateway-in-rust-with-hyper-c84aaf549443
# Badge: https://docs.github.com/en/actions/monitoring-and-troubleshooting-workflows/adding-a-workflow-status-badge

install:
	cargo install --path .

# make build_macos version=0.1.0
build_macos:
	cargo build --release --target x86_64-apple-darwin
	cargo build --release --target aarch64-apple-darwin
	make zip_macos_x86_64 version=$(version)
	make zip_macos_arm64 version=$(version)

# make zip_macos_x86_64 version=0.1.0
zip_macos_x86_64:
	cd target/x86_64-apple-darwin/release && \
	tar -zcvf $(version)_Darwin_x86_64.tar.gz herpy && \
	cd ../../../

# make zip_macos_arm64 version=0.1.0
zip_macos_arm64:
	cd target/aarch64-apple-darwin/release && \
	tar -zcvf $(version)_Darwin_arm64.tar.gz herpy && \
	cd ../../../

# make build_macos version=0.1.0
build_macos_release:
	make build_macos version=0.1.0

get_users:
	curl --location 'http://localhost:8080/users' \
    --header 'Authorization: Bearer JWT'

get_orders:
	curl --location 'http://localhost:8080/orders' \
    --header 'Authorization: Bearer JWT'