#INFO name Test                                        // Name of the program
#INFO authors Harmless_Tech                            // Authors of the program (Splits by commas)
#INFO version 0.0.1                                    // Version of the program
#INFO website https://harmless.tech                    // Website of the program
#INFO git https://github.com/harmless-tech/HT-Assembly // Git repo of the program
#INFO license MIT                                      // License of the program
#INFO custom cus_data1 THE DATA IS ME!
#INFO custom cus_data2 THE DATA IS ME!!!
//TODO Should DEBUG_INFO be injected has readonly struct?

#REQUIRE hta_version 0.0.1 // Requires HTA 0.0.1 or higher
#REQUIRE semicolons true // Every cmd line must end with a semicolon.
#REQUIRE std_enable true // Default true
//#REQUIRE file_type assembly // ...IDK

//#NAMESPACE ht

//#IMPORT additional
//#IMPORT structs.has

#DEFINE HELLO_STR "Hello there!"

ENTRY:
      alloc hello str "Hello, World!";
      syscall std.printf hello;
      syscall std.printf "\n";

Exit: exit 0;
