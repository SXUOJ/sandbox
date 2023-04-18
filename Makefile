TEST_BIN_DIR := examples/bin

build_grpc:
	@cargo build --bin grpc --release

build_cmd:
	@cargo build --bin cmd --release

test: init_test
	@cargo test -- --nocapture

init_test: clean
	@mkdir $(TEST_BIN_DIR)
	@mkdir $(TEST_BIN_DIR)/c
	@mkdir $(TEST_BIN_DIR)/cpp

docker_grpc_run:
	@docker-compose up -d --force-recreate --build judger

clean:
	@cargo clean
	@rm -rf $(TEST_BIN_DIR)
