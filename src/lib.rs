

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