TEST_BIN_DIR := examples/bin

test: init_test
	@cargo test -- --nocapture

init_test: clean

ifeq "$(wildcard $(TEST_BIN_DIR))" ""
	@mkdir $(TEST_BIN_DIR)
	@mkdir $(TEST_BIN_DIR)/c
	@mkdir $(TEST_BIN_DIR)/cpp
endif

clean:
	@cargo clean
	@rm -rf $(TEST_BIN_DIR)
