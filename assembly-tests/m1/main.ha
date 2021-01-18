#INFO name Test                                        // Name of the program
#INFO authors Harmless_Tech                            // Authors of the program
#INFO version 0.0.1                                    // Version of the program
#INFO website https://harmless.tech                    // Website of the program
#INFO git https://github.com/harmless-tech/HT-Assembly // Git repo of the program
#INFO license MIT                                      // License of the program
//TODO Should DEBUG_INFO be injected has readonly struct?

#HEADER hta_version 0.0.1 greater_equal // Requires HTA 0.0.1 or higher
#HEADER file_type assembly
#HEADER semicolons true

#NAMESPACE ht

//#IMPORT additional
#REQUIRE structs.has

#DEFINE HELLO_STR "Hello there!"

ENTRY: // This tag will be merged with the line below.
    alloc hello str "Hello, World!";
    syscall std.printf hello;
    syscall std.printf "\n";
    exit 0
