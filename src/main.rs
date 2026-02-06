mod cli;

use cli::parse;

fn main() {
    let args = parse();
    
    

    println!("Input: {:?}", args.input);
    println!("Output: {:?}", args.output);
    println!("Force: {}", args.force);
}
