/**
 * A simple logger that prints to the console and a file.
 */

#ifndef LOGGING_H
#define LOGGING_H

#define LOG_INFO "INFO"
#define LOG_WARN "WARN"
#define LOG_ERROR "ERROR"
#define LOG_DEBUG "DEBUG"
#define LOG_TRACE "TRACE"

void init_logger(); //TODO Allow for custom path and name later??
void print(const char *tag, const char *format, ...);
void close_logger();

#endif // LOGGING_H
