#ifndef UTILS_H
#define UTILS_H

#include <stdlib.h>
#include "logging.h"

#define TEST_ALLOCATION(p, blame) if(p == NULL) \
	{ println(LOG_ERROR, "Failed to allocate memory! (%s)", blame); exit(EXIT_FAILURE); }

void to_lower(char *arg);

#endif // UTILS_H
