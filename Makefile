TEST_BIN_DIR := examples/bin

test: init_test
	@cargo test

init_test: clean

ifeq "$(wildcard $(TEST_BIN_DIR))" ""
	@mkdir $(TEST_BIN_DIR)
	@mkdir $(TEST_BIN_DIR)/c
	@mkdir $(TEST_BIN_DIR)/cpp
endif

clean:
	@cargo clean
