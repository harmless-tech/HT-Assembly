/*
 * This file is the main entry point of the program.
 * It also serves has the build file for the program. (Using #BUILD and #INFO)
 *
 * Do not put comments in comments!
 */

/* */ /* */
/*

*/

// /* /* If the line comment is removed this would fail to compile. */ */
// Line comments are remove first.

// !!! Entry File Only !!!
#BUILD binary_name m1-test // Sets the name of the binary file.

#INFO name Test                                // Name of the program
#INFO authors Harmless_Tech                    // Authors of the program (Split using commas)
#INFO version 0.0.1                            // Version of the program
#INFO website harmless.tech                    // Website of the program  (https:// will be appended to the front)
#INFO git github.com/harmless-tech/HT-Assembly // Git repo of the program (https:// will be appended to the front)
#INFO license MIT                              // License of the program

/* #INFO custom cus_data1 THE DATA IS ME!     // Custom data is currently not allowed. (May never be allowed)
#INFO custom cus_data2 THE DATA IS ME!!! */

#REQUIRE hta_version 0.0.1 // Requires HTA 0.0.1 or higher. This is only checked by the compiler. The runtime is dependent on the compiler version alone.
// \/ This should be a setting in a formatter.
//#REQUIRE semicolons true   // Every instruction line must end with a semicolon. (All instructions still must be on their own line)
//#REQUIRE native_lib std  // For now the std lib is imported by default.
//#REQUIRE native_lib NAME // Requires a native lib. (Not implemented in m1, except for std!)

// !!! Any File !!!
#NAMESPACE main

#IMPORT add.ha // Imports the file add.ha. (add.ha will keep its namespace though!) Kinda like c/c++ #include.

#DEFINE HELLO_STR "Hello there!" // This only applies to this file. Replaces anything that matches! (Even things in quotes)

// For m1 all instructions need to be on their own line.

ENTRY: // The ENTRY tag will be the start of the program. !!! This must be in the entry file and there can only be one. !!!
      alloc hello str "Hello, World!";
      alloc hello str HELLO_STR; // This is valid, the var will be reallocated with the type.
      varreg hello $2;
      call std.print; // std.print will print the value in the return register.

NEWLINE:


      setreg $2 chr '\n';
      call std.print;

      setreg $0 str HELLO_STR;
      cast u32;
      alloc h_int u32;
      regvar h_int;
      dealloc h_int;

      alloc hello_slash str "Hello // World!"; // Remove me!
      alloc hello u128 20,000; // Commas are allowed in numeric values.

EXIT: exit 0;

//EXIT2: // This tag would error out, because it cannot link to an instruction.
