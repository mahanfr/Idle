CC = clang
CFLAGS = -Wall -Wextra -ggdb
SRC_DIR = src
SRC_FILES = -I./$(SRC_DIR) $(SRC_DIR)/*.c
BUILD_DIR = build
EXECUTABLE = $(BUILD_DIR)/idle

.PHONY: all compile run clean

compile: $(BUILD_DIR)
	$(CC) $(CFLAGS) -o $(EXECUTABLE) $(SRC_FILES)

run: compile
	./$(EXECUTABLE)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

clean:
	rm -rf $(BUILD_DIR)/*
