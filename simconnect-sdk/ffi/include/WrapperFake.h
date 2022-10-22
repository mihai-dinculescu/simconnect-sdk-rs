// fake defs in lieu of Windows.h that is used for the docs.rs build
#define MAX_PATH 260
#define FALSE 0
#define CONST const
#define CALLBACK __stdcall
#define DECLARE_HANDLE(name) \
    struct name##__          \
    {                        \
        int unused;          \
    };                       \
    typedef struct name##__ *name

typedef unsigned long DWORD;
typedef unsigned char BYTE;
typedef int BOOL;
typedef long LONG;
typedef LONG HRESULT;
typedef void *HANDLE;
typedef char CHAR;
typedef CONST CHAR *LPCSTR;
typedef struct _GUID
{
    unsigned long Data1;
    unsigned short Data2;
    unsigned short Data3;
    unsigned char Data4[8];
} GUID;

DECLARE_HANDLE(HWND);

#include "SimConnect.h"
