#include <string.h>
#include <parson.h>
#include <binn.h>
#include <pthread.h>
#include <xxhash.h>
#include "../hta_shared/utils.h"
#include "../hta_shared/logging.h"
#include "../hta_shared/stypes.h"
#include "../hta_compiler/hta_compiler.h"

i32 main(i32 argc, char *argv[]) {
    // Logging
    init_logger();
    println(LOG_INFO, "Starting HTA!");
    //

	// Args
	println(LOG_DEBUG, "Arg Amount: %d", argc);
	for(i32 i = 0; i < argc; i++)
        println(LOG_DEBUG, "Arg %d: %s", i, argv[i]);
	//

	// Program
	if(argc >= 3) {
	    char *arg1 = argv[1];
	    char *arg2 = argv[2];
	    to_lower(arg1);

	    if(strcmp(arg1, "compile") == 0) {
	        //TODO Allow for export file name.
	        println(LOG_INFO, "Compiler launched with %s file.", arg2);
	        hta_compile(arg2); //TODO Allow export!
	    }
	    else if(strcmp(arg1, "run") == 0) {
            println(LOG_INFO, "Running %s.", arg2);
	    }
	    else
            println(LOG_ERROR, "Argument 1 was not compile or run!");
	}
	else
        println(LOG_ERROR, "Need 2 args: ht-assembly {compile, run} [FILE_NAME]", argc);
	//

	// Cleanup
	close_logger();
	//

	return 0;
}
