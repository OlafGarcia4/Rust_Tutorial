To use Rust we need to use Cargo. Cargo is a package manager that we can use to build projects
and manage libraries.

To start a new project we first use:

    $ cargo new ___

This will create a new project of the given name, creating 2 files. The \src\ folder will have the 
main.rs file where our code will go and then we have the Cargo.toml file. In the .toml file we can add
packages under the [dependencies] selction of the file that our code in \src\ will use

To compile and build our project we use:

    cargo build

This will generate the executable file for the project. This will be store in the target\debug directory
To then run the project we have to options.

    cargo run
    cargo check

Cargo run will execute the program without having to especify the path of the file. This will check if there is a executable file
built, if not then it will recompile the code to create the executable file and then run it.

Cargo check is like the run however it will not produce a executable. It allows you to check if your code will compile and it is to good habit to check 
regularly thorugh writing code.

    cargo clean

This will do the same thing as a make clean will do.