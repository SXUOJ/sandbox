TEST_BIN_DIR := examples/bin

build:
	@cargo build --release

test: init_test
	@cargo test -- --nocapture

init_test: clean
	@mkdir $(TEST_BIN_DIR)
	@mkdir $(TEST_BIN_DIR)/c
	@mkdir $(TEST_BIN_DIR)/cpp

grpc_run:
	@docker-compose up -d --force-recreate --build judger

clean:
	@cargo clean
	@rm -rf $(TEST_BIN_DIR)
