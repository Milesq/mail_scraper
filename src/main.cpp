#include <windows.h>
#include <experimental/filesystem>
#include <fstream>

#include <iostream>

using namespace std;
namespace fs = experimental::filesystem;

typedef const char *str;

extern "C"
{
    void add_record(str code, str file_output, const str *fields, int length);
}

enum CustomError
{
    ASSERTION_ERROR,
    FILE_ERROR,
};

void assert(bool, string);
void add_record(string);
string read_file(string path);

int main(int argc, const char **argv)
{
    str fields[3] = {"E-mail", "Telefon", "Imię i Nazwisko"};

    try
    {
        assert(argc == 2, "Program został uruchomiony nieprawidłowo\n\nPrzenieś ikonkę folderu na aplikację");
        string dir = argv[1];
        assert(fs::exists(dir), "Podany folder nie istnieje");

        if (fs::is_regular_file(dir))
            add_record(read_file(dir).c_str(), "output.csv", fields, 3);
        else if (fs::is_directory(dir))
            for (auto &p : fs::directory_iterator(dir))
            {
                string file = read_file(p.path().string());
                add_record(file.c_str(), "output.csv", fields, 3);
            }
        else
            assert(false, "Musisz podać folder albo plik");
    }
    catch (CustomError)
    {
    }

#ifdef DEBUG_ASSERTION
    system("pause");
#endif

    return 0;
}

void assert(bool assertion, string msg)
{
    if (!assertion)
    {
        MessageBox(NULL, TEXT(msg.c_str()), TEXT("Error"), MB_OK);
        throw CustomError::ASSERTION_ERROR;
    }
}

string read_file(string path)
{
    ifstream f(path);
    string str;
    if (f.good())
    {
        f.seekg(0, ios::end);
        size_t len = f.tellg();
        f.seekg(0);
        string content(len + 1, '\0');
        f.read(&content[0], len);
        f.close();
        return content;
    }
    throw CustomError::FILE_ERROR;
}
