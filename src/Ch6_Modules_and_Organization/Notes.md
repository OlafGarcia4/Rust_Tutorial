Modules

So far we have used binary crates, this section will concentrate on making libraries/modules.
Start by creating a lib cargo project by 

    cargo new project_modules --lib  

We see that instead of getting a main.rs we have a lib.rs file under src/. We can still define a main.rs file if we want.

Here are some rules for when creating and using modules

1. When creating a modules it is importat to star form the root directory of the project, this is because compiler
will start looking for these files here 

2. The compiler will for the associeted files to the module. So if we create a module called house in *src/lib.rs*, the 
compiler will look for files *src/house.rs* or look for the directory *src/house/mod.rs* (*mod.rs* being where the code for mod 
house is)

3.  You can create modules inside modules (submodules), and the compiler will look in the parent directory(in this case the src/house).
So if we create mod kitchen in mod house then it will look for */src/house/kitchen* or *src/house/kitchen/mod.rs* 

4. Once we define the module, we can refer to the code in within from other files (given it has the right visibility). We can call part of this
by using the :: syntax, for example if we want to use sink from kitche, then we call it by saying *crate::house::kitchen::sink*

5. Modules can be difined by the its visibility to the rest of the package. We can set modules as private or public. By default all modules are 
private and to set it we just give it pub before we declared the module (pub mod house). 

6. We can reference parts from mudules by using the ::sytax and the path where the code live, we can instead use the 'use' keyword so we don't need
to call the full path each time, so in the case of calling for sink we just declared '*use crate::house::kitchen::sink*'. We can also call * to bring 
all parts of a module into the scope,however this is not recommended for all cases (*use crate::house::kitche::*)

--- --------------------------------
Taking the house module example, if we look at the file structure this is what we will see 

    Building
    |-Cargo.lock
    |-Cargo.toml
    |-src 
        |-house
            |-kitchen
            |-livingRoom
            |-bathroom
        |-apartment
    ... 

File heirecy is crucial for working with modules. 
Lets look at main in this project. We see that we start with defining '`mod mod_example`',
this will point to the *file mod_example.rs*, this is where our module structure will live.
Lets start with the functions that are not nested


**function()**
- this function we can see is labled as public, this is the basic way to create a function that we
        can use on other classes

**priv_func()**
- This function is not labled as witht the pub tag, this function and any other will by default be 
    set to private and will only be able to used by the file where it lives.

**pub_func()**
- This function can be used by any class that references mod_example.rs, or in other words, makes it visibile
    to the rest of the project 

**ind_access()**
- This function shows how we can access classes that are private through public functions. This is called inderect
    access.

**call_public_mods()**
- This function calls pieces/functions from the even nested mods that we build either from the same or other file  

**pub_func_crate()**
    - This function is the first we see were its declared pub(crate). This makes the function as only visible for the 
    crate where it lives. Meaning that if we use the crate on another project, we can not directly call for this function

We can see how visibility allows us to interact parts from a module. Let look at further at the nested modue 'nested_mod'
and 'private_nested'

`nested_mod`

**nested_mod_func()**
- this functions act like the one function that is on the parent module.

**fn nested_priv_func()**
- This funciton work the same as the previous private function,only called by the module in which it is defined.

**fn mod_only_visibles()**
- This funciton is declared with the pub(in 'path') tag. This definition allows for the specified path to use that specific 
    function given that the path specified is a parent or ancestor of the module. 

**mod_current_use()**
- This functionn is define by pub(&self). This means that only the current module where this function is defined will be the 
    only one to use it, this is the same as leaving the function as private. 

**super_mod_use()**
- This function is defined as pub(super). The super option gives the parent module visibility to the function and only the parent 
    module

`private_nested`
    
    nested modules follow the same rules when it comes to visibility, this module can only be seen by its parent mod_example.

**fn private_nested_func()**
    
**restricted_function()**
- This is defined with the pub(crate). This function will still be restricted to the crate even despite this function being declared 
    as public, only the mod_example::private_nested_func can call this function.



As we can see, defining visibility dictates what can interact with the rest. Looking at main we can see that we use the *'use'* and *'use .. as ..'* syntax to define the path of 
where these functions live in order to use them. Visibility is not only specific to functions but to structures and enum we define. Impl blocks dont need to be decalered as public 
but the functions within should be if the intended porpuse is for other code to use it.

We can also decalared self and super when referencing the funcitons within modules with the :: syntax. This is to remove ambiguity when calling for function or other piece of code. With self
used to call for function with in the module and super to call for functions on the parent module. In mod_only_visibles() we can specify using mod_current_use() by insted saying

    22      self::mod_current_use();


Modules can be define infile and they don't nessercarly need to be separeted into their own files, however it is good practice as this makes it easier to see how the modules visibility will work in 
in the file hierchy. We could just define nested_mod and private_nested on their own crates, or even just define mod_example within the main function. If we do define the module with in the main, we would
not have to worry about much about visibility of the module since the functions will be with in the scope of the project (Then defining the path with self and super becomes more important for clarity).


