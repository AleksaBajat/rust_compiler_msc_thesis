use my_macro::expand_body;

macro_rules! expand_missing_body {
    ($(#[$attr:meta])* mod $mod_name:ident { $(pub fn $fn_name:ident();)* }) => {
        $(#[$attr])*
        mod $mod_name {
            $(pub fn $fn_name() {
                println!("Function `{}` is not yet implemented.", stringify!($fn_name));
            })*
        }
    };
}

expand_missing_body! {
    mod foo {
        pub fn missing_body();
        pub fn another_missing_body();
    }
}

#[expand_body]
fn missing_body();

fn main() {
    foo::missing_body();
    foo::another_missing_body();
    missing_body();
}
