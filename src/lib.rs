use std::io;

pub fn title() {
    println!("\n\n\n\tHANNA - An atmospheric text-based story.");
    println!("\n\t\t\tWelcome.\n")
}

pub fn credits(){
    println!("\n\n\n\tMade by Carter Facey-Smith.");
    println!("\n\tThank you for playing.")
}

pub fn textbox(emoji: &str, input: &str){
    println!("\t{} {}", emoji, input);    
}

pub fn user_input(prompt: String) -> String{
    textbox("", &prompt);
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).unwrap();
    return input_string;
}

pub fn scene_one(){
    textbox("🛖", "say the password to enter my hut...");
    user_input("".to_string());
    textbox("🕋", "the obelisk mutters a faint “no”");
    textbox("🚦", "a traffic light appears before you, violently flashing several times before finally settling on red with a creaking shudder");
    textbox("\n\t\t🪤\t🪝\t🎰\n\n", "\tthree things appear before you");

}

// pub struct Textbox {
//     pub tab_size: u8,
//     pub text: String,
// }

// impl Textbox{
//     pub fn new(input: String) -> Result<Textbox, &'static str> {
//         let tab_size = 2;
//         let text = input.clone();
        

//         Ok(Textbox { tab_size, text })
//     }
// }