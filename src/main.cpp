#include <iostream>
#include <fstream>
#include <vector>

#include <inipp.h>

#include "bridge.hpp"
#include "filesystem.hpp"
#include "helpers.hpp"

using namespace std;
using inipp::Ini;

int main(int argc, const char **argv)
{
    try
    {
        if (argc == 1)
        {
            config([]() {
                throw CustomError::CONFIG_SERVER_CLOSE;
            });

            return 0;
        }

        assert(argc >= 2, L"Program został uruchomiony nieprawidłowo\n\nPrzenieś ikonkę folderu na aplikację");
        string dir = argv[1];
        assert(fs::exists(dir), L"Podany folder nie istnieje");

        Ini<char> ini;
        ifstream is("add_record_config.ini");
        ini.parse(is);
        assert(ini.errors.empty(), L"Plik .ini uszkodzony");

        auto sections = ini.sections[""];

        str outputFileName = sections["output_file_name"].c_str();
        vector<string> string_fields = split(sections["fields"], ",");

        vector<str> fields;
        fields.resize(fields.size());

        for (auto field : string_fields)
        {
            fields.push_back(strdup(field.c_str()));
        }

        auto execute_add_record = [&](str s) {
            add_record(s, outputFileName, &fields[0], fields.size());
        };

        if (fs::is_regular_file(dir))
        {
            execute_add_record(read_file(dir).c_str());
        }
        else if (fs::is_directory(dir))
            for (auto &p : fs::directory_iterator(dir))
            {
                string file = read_file(p.path().string());
                execute_add_record(file.c_str());
            }
        else
            assert(false, L"Musisz podać folder albo plik");
    }
    catch (CustomError)
    {
    }

#ifdef DEBUG_ASSERTION
    system("pause");
#endif

    return 0;
}
