#ifndef __BRIDGE
#define __BRIDGE
typedef const char *str;
typedef void (*CB)();

extern "C"
{
    void add_record(str code, str file_output, const str *fields, int length);
    void config(CB);
}

#endif
