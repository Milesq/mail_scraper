EXE = add_records.exe
BIN = build

CC = g++
CC_ARGS = -std=c++17 -Wall -lstdc++fs

DLL_DIR = debug

ifeq ($(RELEASE),1)
	DLL_DIR = release
	CARGO_FLAG = --release
	CC_ARGS += -mwindows -O3
else
	CC_ARGS += -DDEBUG_ASSERTION
endif

all: clear src/main.cpp target/$(DLL_DIR)/add_record.dll
	$(CC) src/main.cpp -Ltarget/$(DLL_DIR) -ladd_record -o $(BIN)/$(EXE) $(CC_ARGS)
	cp target\$(DLL_DIR)\add_record.dll $(BIN)
	$(BIN)/$(EXE)

build: clear src/main.cpp target/$(DLL_DIR)/add_record.dll
	$(CC) src/main.cpp -Ltarget/$(DLL_DIR) -ladd_record -o $(BIN)/$(EXE) $(CC_ARGS)
	cp target\$(DLL_DIR)\add_record.dll $(BIN)

target/$(DLL_DIR)/add_record.dll: src/*.rs
	cargo build $(CARGO_FLAG)

clear:
	rm -rf $(BIN)
	mkdir $(BIN)
