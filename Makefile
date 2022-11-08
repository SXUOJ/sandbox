TEST_BIN_DIR := test_cases/bin

test: init_test
	@cargo test

init_test: clean

ifeq "$(wildcard $(TEST_BIN_DIR))" ""
	@mkdir test_cases/bin
	@mkdir test_cases/bin/c
	@mkdir test_cases/bin/cpp
endif

clean:
	@cargo clean
