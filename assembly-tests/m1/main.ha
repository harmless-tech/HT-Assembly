/*
 * This file is the main entry point of the program.
 * It also serves has the build file for the program. (Using #BUILD and #INFO)
 *
 * Do not put comments in comments!
 */

/*  */
/*

*/

// !!! Main File Only !!!
#BUILD file_name m1-test // Sets the name of the binary file.

#INFO name Test                                        // Name of the program
#INFO authors Harmless_Tech                            // Authors of the program (Splits by commas)
#INFO version 0.0.1                                    // Version of the program
#INFO website https://harmless.tech                    // Website of the program
#INFO git https://github.com/harmless-tech/HT-Assembly // Git repo of the program
#INFO license MIT                                      // License of the program

#INFO custo/*m cus_data1 THE DATA IS ME!
#INFO custom cus_data2 THE DATA IS M*/E!!!

#REQUIRE hta_version 0.0.1 // Requires HTA 0.0.1 or higher
#REQUIRE semicolons true   // Every instruction line must end with a semicolon.
#REQUIRE native_lib std    // The std is imported by default, but you can turn it off here.
//#REQUIRE native_lib NAME // Requires a native lib. (Not implemented in m1, except for std!)

// !!! Any File !!!
#NAMESPACE main

#IMPORT add.ha // Imports the file add.ha. (add.ha will keep its namespace though!) Kinda like c/c++ #include.

#DEFINE HELLO_STR "Hello there!" // This only applies to this file. Replaces anything that matches!

// For m1 all instructions need to be on their own line.

ENTRY:
      alloc hello str "Hello, World!";
      alloc hello str HELLO_STR; // This is valid, the var will be reallocated with the type.
      call std.print hello;

NEWLINE:



      call std.print "\n";

      alloc h_int u32 HELLO_STR;
      alloc h_float float HELLO_STR;

EXIT: exit 0;

//EXIT2: // This tag would error out, because it cannot link to an instruction.
