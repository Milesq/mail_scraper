#include <windows.h>
#include <fstream>
#include <vector>

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

vector<string> split(string str, string delim = " ")
{
    vector<string> tokens;
    char *str_c = strdup(str.c_str());
    char *token = NULL;

    token = strtok(str_c, delim.c_str());
    while (token != NULL)
    {
        tokens.push_back(string(token));
        token = strtok(NULL, delim.c_str());
    }

    delete[] str_c;

    return tokens;
}

#endif
