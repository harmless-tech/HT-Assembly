#include <ctype.h>
#include "utils.h"

void to_lower(char *arg) {
    for(char *p = arg; *p; p++)
        *p = tolower(*p);
}