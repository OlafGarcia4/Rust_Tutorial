
    fn priv_func(){
        println!("mod_example::private function was called");
    }
    pub fn pub_func(){
        println!("mod_example::public function was called");
    }
    pub fn ind_access(){
        priv_func();
        println!("mod_example::priv_func was called inderectly");
    }
    pub mod nested_mod{
        pub fn nested_mod_func(){
            println!("mod_example::nested_mod::nested_mod_func was called");
        }
        #[allow(dead_code)]
        fn nested_priv_func(){
            println!("mod_example::nested_mod::nested_priv_func was called");
        }
        pub(in crate::mod_example) fn mod_only_visibles(){
            println!("mod_only_visibles was called, only used by mod_example");
            mod_current_use();
        }
        pub(self) fn mod_current_use(){
            println!("mod_example::nested_mod::mod_current_use was called, can only be used by nested_mod");
        }
        pub(super) fn super_mod_use(){
            println!("mod_example::nested_mod::super_mod_use was called, can only be used by the parent module");
        }
    }
    
    pub fn call_public_mods() {
        println!("called call_public_mods");
         print!("\n");
         nested_mod::mod_only_visibles();
        print!("> ");
        nested_mod::super_mod_use();
        print!("\n");
    }

    pub(crate) fn pub_func_crate(){
        println!("pub_func_crate was called\n\n");
    }

    mod private_nested{
        #[allow(dead_code)]
        fn private_nested_func(){
            println!("private_nested_func was called");
        }
        #[allow(dead_code)]
        pub(crate) fn restricted_function(){
            println!("restricted_func was called");
        }
    }

pub fn function() {
    println!("called `function()`");
}
