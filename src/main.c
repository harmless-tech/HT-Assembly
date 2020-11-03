#include <parson.h>
#include <binn.h>
#include <pthread.h>
#include "hta_shared/logging.h"

int main(int argc, char *argv[]) {
    // Logging
    init_logger();
    //

	// Args
	print(LOG_INFO, "Arg Amount: %d\n", argc);
	for(int i = 0; i < argc; i++)
        print(LOG_INFO, "Arg %d: %s\n", i, argv[i]);

	//

	print(LOG_DEBUG, "WOW\n");

	// Cleanup
	close_logger();
	//

	return 0;
}