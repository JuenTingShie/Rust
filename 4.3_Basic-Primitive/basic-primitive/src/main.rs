use std::io;

fn main() {
    loop{
        println!("Enter an emoji to show it's unicode: ");

        let mut emoji = String::new();
        io::stdin().read_line(&mut emoji)
                .expect("Fail, try again! ");
        
        println!("{}", emoji.trim().escape_unicode() );
    }
}