HUB ?= default-route-openshift-image-registry.apps.cluster-1d48.1d48.sandbox890.opentlc.com

build: oidc.wasm

oidc.wasm:
	cargo build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/header_append_filter.wasm ./extension.wasm

.PHONY: clean
clean:
	rm extension.wasm || true
	rm -r build || true

.PHONY: container
container: clean build
	mkdir build
	cp container/manifest.yaml build/
	cp extension.wasm build/
	cd build && docker build -t header-append-filter . -f ../container/Dockerfile

container.push: container
	docker push ${HUB}/header-append-filter
