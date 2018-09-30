#[macro_use]
extern crate stdweb;

fn main() {
    stdweb::initialize();
    let message = "こんにちは 世界";

    // Write javascript inside Rust code!?
    js! {
        // @{var_name} to use Rust variable in the js! macro
        alert ( @{message} );
        console.log( @{message} );
    }

    stdweb::event_loop();
}
