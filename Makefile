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

# make build_linux version=0.1.0
build_linux:
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target i686-unknown-linux-gnu
	cargo build --release --target aarch64-unknown-linux-gnu
	make zip_file target=x86_64-unknown-linux-gnu version=$(version)
	make zip_file target=i686-unknown-linux-gnu version=$(version)
	make zip_file target=aarch64-unknown-linux-gnu version=$(version)

# make build_windows version=0.1.0
build_windows:
	cargo build --release --target x86_64-pc-windows-msvc
	make zip_file target=x86_64-pc-windows-msvc version=$(version)

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

# make zip_file target=x86_64-unknown-linux-gnu version=0.1.0
zip_file:
	cd target/$(target)/release && \
	tar -zcvf $(version)_$(target).tar.gz herpy && \
	cd ../../../

build_image:
	docker build -t prongbang/herpy:latest .

run_container:
	docker run \
		-p 8080:8080 \
		-v "./herpy.yaml:/etc/herpy/herpy.yaml" \
		--name herpy-api-gateway \
		prongbang/herpy:latest

# make push_image tag=latest
push_image:
	docker tag prongbang/herpy:$(tag) prongbang/herpy:$(tag)
	docker push prongbang/herpy:$(tag)

# make build_macos_release version=0.1.0
build_macos_release:
	make build_macos version=0.1.0

# make build_linux_release version=0.1.0
build_linux_release:
	make build_linux version=0.1.0

# make build_windows_release version=0.1.0
build_windows_release:
	make build_windows version=0.1.0

get_users:
	curl --location 'http://localhost:8080/users' \
    --header 'Authorization: Bearer JWT'

get_orders:
	curl --location 'http://localhost:8080/orders' \
    --header 'Authorization: Bearer JWT'

get_hello:
	curl -X POST --location 'http://localhost:8080/hello' \
    --header 'Authorization: Bearer JWT'

get_hello_krakens:
	curl -X GET --location 'http://localhost:8090/users'