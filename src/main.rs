
use hanna::{title, credits, textbox};

fn main() {
    game();
}

fn game(){
    title();

    println!("\t🛖 say the password to enter my hut...");
    // let x = Textbox::new(String::from("Hello"));
    textbox("🕋", "the obelisk mutters a faint “no”");
    println!("\t🕋 the obelisk mutters a faint “no”");
    println!("\t🚦 a traffic light appears before you, violently flashing several times before finally settling on red with a creaking shudder");

    credits();
}
