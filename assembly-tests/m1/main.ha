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
#REQUIRE semicolons true // Every instruction line must end with a semicolon.
#REQUIRE std true // Default true. Allows the std library to be used.
//#REQUIRE file_type assembly // ...IDK

//#NAMESPACE ht

//#IMPORT additional
//#IMPORT structs.has

#DEFINE HELLO_STR "Hello there!"

// For m1 all instructions need to be on their own line.

ENTRY:
      alloc hello str "Hello, World!";
      alloc hello str HELLO_STR;
      call std.print hello;

NEWLINE:



      call std.print "\n";

      alloc h_int u32 HELLO_STR;
      alloc h_float float HELLO_STR;

EXIT: exit 0;
