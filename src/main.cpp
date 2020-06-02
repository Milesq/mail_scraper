#include <string>
#include <fstream>
#include <vector>
#include <windows.h>

#include <inipp.h>

#include "bridge.hpp"
#include "filesystem.hpp"
#include "helpers.hpp"

using namespace std;
using inipp::Ini;

void success(uint32_t = 0);

int main(int argc, const char **argv)
{
    try
    {
        if (argc == 1)
        {
            config([]() {
                success();
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
            success(1);
        }
        else if (fs::is_directory(dir))
        {
            uint32_t filesCount = 0;
            for (auto &p : fs::directory_iterator(dir))
            {
                ++filesCount;
                string file = read_file(p.path().string());
                execute_add_record(file.c_str());
            }

            success(filesCount);
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

void success(uint32_t filesCount)
{
    wstring w_msg;
    wstringstream wss;
    wss << L"Program zakończył pracę.\n";

    if (filesCount != 0)
    {
        wss << "Przeskanowano "
            << filesCount
            << L" plik/i/ów";
    }

    w_msg.append(wss.str());

    const wchar_t *msg = w_msg.c_str();
    MessageBoxW(NULL, msg, L"Info", MB_OK);
}
