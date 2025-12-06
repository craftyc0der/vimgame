PROJECT_NAME = vimgame
WASM_TARGET = wasm32-unknown-unknown
DIST_DIR = dist

.PHONY: all test clean build-native run-native solve-all build-web run-web setup-web

# Default target runs tests and builds both versions
all: test build-native build-web

# Run tests
test:
	cargo test

# Native Build & Run
build-native:
	cargo build --release

run-native:
	cargo run

solve-all:
	cargo run -- --solve-all

# Web Build & Run
setup-web:
	rustup target add $(WASM_TARGET)

build-web: setup-web
	cargo build --target $(WASM_TARGET) --release
	mkdir -p $(DIST_DIR)
	cp target/$(WASM_TARGET)/release/$(PROJECT_NAME).wasm $(DIST_DIR)/$(PROJECT_NAME).wasm
	cp assets/index.html $(DIST_DIR)/index.html
	# Copy assets folder to dist so the game can find them (levels, etc.)
	# We use rsync or cp -r. Using cp -r for simplicity, but excluding index.html to avoid overwrite warning if it was in assets
	mkdir -p $(DIST_DIR)/assets
	cp -r assets/levels $(DIST_DIR)/assets/
	# Download macroquad js bundle if not present
	test -f $(DIST_DIR)/mq_js_bundle.js || curl -L https://raw.githubusercontent.com/not-fl3/macroquad/master/js/mq_js_bundle.js -o $(DIST_DIR)/mq_js_bundle.js

run-web: build-web
	@echo "Serving at http://localhost:8000"
	@echo "Press Ctrl+C to stop"
	open http://localhost:8000
	cd $(DIST_DIR) && python3 -m http.server 8000

clean:
	cargo clean
	rm -rf $(DIST_DIR)
