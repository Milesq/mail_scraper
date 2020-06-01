#include <windows.h>
#include <fstream>

#ifndef __HELPERS
#define __HELPERS
using namespace std;

enum CustomError
{
    ASSERTION_ERROR,
    FILE_ERROR,
    CONFIG_SERVER_CLOSE
};

void assert(bool assertion, const wchar_t *msg)
{
    if (!assertion)
    {
        MessageBoxW(NULL, msg, L"Błąd", MB_OK);
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

#endif
