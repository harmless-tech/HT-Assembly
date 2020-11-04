#ifndef UTILS_H
#define UTILS_H

#define TEST_ALLOCATION(p) if(p == NULL) { print(LOG_ERROR, "Failed to allocate memory!"); exit(EXIT_FAILURE); }

void to_lower(char *arg);

#endif // UTILS_H
