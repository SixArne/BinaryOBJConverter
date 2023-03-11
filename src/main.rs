use clap::Parser;

mod OBJParser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments
{
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long, default_value_t = false)]
    reverse: bool,
}

fn main() {
    let args = Arguments::parse();
    // println!("input {}", args.input);
    // println!("output {}", args.output);

    // let path = String::from("./Data/bunny.obj");
    // let binPath = String::from("./Data/bunno.bobj");
    // let textPath = String::from("./Data/bunny_copy.obj");

    let mut objParser = OBJParser::OBJ::new();
    if !args.reverse
    {
        objParser.ReadFile(&args.input);
        objParser.WriteAsBinary(&args.output);
    }
    else 
    {
        objParser.ReadBinary(&args.input);
        objParser.WriteAsText(&args.output);
    }

    println!("exit");
}
