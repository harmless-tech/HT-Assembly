#include <parson.h>
#include <binn.h>
#include <pthread.h>
#include "hta_shared/logging.h"

int main(int argc, char *argv[]) {
    // Logging
    init_logger();
    //

	// Args
	print(LOG_DEBUG, "Arg Amount: %d\n", argc);
	for(int i = 0; i < argc; i++)
        print(LOG_DEBUG, "Arg %d: %s\n", i, argv[i]);
	//

	// Program
	//

	// Cleanup
	close_logger();
	//

	return 0;
}
