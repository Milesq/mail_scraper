#include "bridge.hpp"
#include "filesystem.hpp"
#include "helpers.hpp"

using namespace std;

class Config
{
public:
    string outputFileName;
    string *fields;
    unsigned int fieldsCount;

    Config(string path);
};

int main(int argc, const char **argv)
{
    str fields[3] = {"E-mail", "Telefon", "Imię i Nazwisko"};

    Config cfg("cfg.ini");

    try
    {
        if (argc == 1)
        {
            config([]() {
                throw CustomError::CONFIG_SERVER_CLOSE;
            });

            return 1;
        }

        assert(argc >= 2, L"Program został uruchomiony nieprawidłowo\n\nPrzenieś ikonkę folderu na aplikację");
        string dir = argv[1];
        assert(fs::exists(dir), L"Podany folder nie istnieje");

        if (fs::is_regular_file(dir))
            add_record(read_file(dir).c_str(), cfg.outputFileName.c_str(), fields, cfg.fieldsCount);
        else if (fs::is_directory(dir))
            for (auto &p : fs::directory_iterator(dir))
            {
                string file = read_file(p.path().string());
                add_record(file.c_str(), cfg.outputFileName.c_str(), fields, cfg.fieldsCount);
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

Config::Config(string path)
{
    this->fields = new string[3];
    this->fieldsCount = 3;
    this->outputFileName = "output.csv";
}
