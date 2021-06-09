# mail_scraper

App creates http server and give you access to simple admin panel. In admin panel you can add mails and precise fields that should be used.

This is my project for apprenticeships. Main program is written in C++, adding records to output file is written in Rust and compiled to dynamic library.

Mail Parser also is written in Rust and is used two times: in main program and in panel admin. Parser compiles to dynamic library and WASM 
