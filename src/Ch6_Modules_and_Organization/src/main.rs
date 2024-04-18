mod mod_example;

fn main() {
    use mod_example::*; //using the glob operator to bring the items into the scope
    use mod_example::nested_mod as nested;
    //use mod_example::{self,nested_mod_example} alternative way to call for these paths
    println!("\n\n");

    function();
    ind_access();
    nested::nested_mod_func();
    call_public_mods();
    pub_func_crate();

    //these fucntion will retrun error because main.rs has no visibility as they are private nor do we have inderect access
    //nested_mod::mod_only_visibles();
    //nested_mod::nested_priv_func();

}
