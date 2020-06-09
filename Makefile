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
	NPM_TARGET = :release
else
	CC_ARGS += -DDEBUG_ASSERTION
endif

build: dist-dir $(DIST)/add_record.dll $(DIST)/$(EXE) $(DIST)/static $(DIST)/$(INI_FILE)

run: build
	@echo Running...\n
	@$(DIST)/$(EXE)

$(DIST)/$(EXE): src/*.cpp src/*.hpp
	@echo g++ ...
	@$(CC) src/main.cpp -L$(DIST) -ladd_record -o $(DIST)/$(EXE) $(CC_ARGS)

$(DIST)/add_record.dll: src/*.rs src/**/*.rs
	cargo build $(CARGO_FLAG)
	@echo Copy dll file
	@cp target\$(DLL_DIR)\add_record.dll $(DIST)/add_record.dll

$(DIST)/static: www/* parse-mail/pkg/parse_mail.js
	@echo making www
	@rm -rf $(DIST)/static
	@npm run build$(NPM_TARGET)

parse-mail/pkg/parse_mail.js: parse-mail/src/*.rs parse-mail/src/**/*.rs
	@wasm-pack build parse-mail

$(DIST)/$(INI_FILE): add_record_config.ini
	@echo Copy ini file
	@cp -p add_record_config.ini $(DIST)/$(INI_FILE)

dist-dir:
	@mkdir -p $(DIST)

clear:
	rm -rf \
		.cache \
		build/ \
		dist/ \
		parse-mail/pkg \
		parse-mail/target \
		target/debug \
		target/release
