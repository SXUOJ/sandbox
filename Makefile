TEST_BIN_DIR := core/examples/bin

test: init_test
	@cargo test -- --nocapture

init_test: clean
	@mkdir $(TEST_BIN_DIR)
	@mkdir $(TEST_BIN_DIR)/c
	@mkdir $(TEST_BIN_DIR)/cpp

clean:
	@cargo clean
	@rm -rf $(TEST_BIN_DIR)
