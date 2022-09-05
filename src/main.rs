mod cli_parsing;

fn main() {
    for ele in cli_parsing::parse_arguments().files {
        println!("{:?}", ele);
    }
}
