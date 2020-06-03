EXE = add_records.exe
DIST = build
INI_FILE = add_record_config.ini

CC = g++
CC_ARGS = -std=c++17 -Wall -lstdc++fs -Iinipp/inipp/

DLL_DIR = debug

ifeq ($(RELEASE),1)
	DLL_DIR = release
	CARGO_FLAG = --release
	CC_ARGS += -mwindows -O3
else
	CC_ARGS += -DDEBUG_ASSERTION
endif

build: dist-dir $(DIST)/add_record.dll $(DIST)/$(EXE) $(DIST)/static build/$(INI_FILE)

run: build
	@echo Running...\n
	@$(DIST)/$(EXE)

$(DIST)/$(EXE): src/*.cpp src/*.hpp
	@echo g++ ...
	@$(CC) src/main.cpp -L$(DIST) -ladd_record -o $(DIST)/$(EXE) $(CC_ARGS)

$(DIST)/add_record.dll: src/*.rs
	cargo build $(CARGO_FLAG)
	@echo Copy dll file
	@cp target\$(DLL_DIR)\add_record.dll $(DIST)/add_record.dll

$(DIST)/static: static/*
	@echo copy www
	@rm -rf $(DIST)/static
	@cp -r static build

build/$(INI_FILE): add_record_config.ini
	@echo ini file
	@cp -p add_record_config.ini $(DIST)/$(INI_FILE)

dist-dir:
	@mkdir -p $(DIST)
