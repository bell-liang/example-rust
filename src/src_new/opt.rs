use structopt_derive::*;
#[derive(StructOpt, Debug)]
#[structopt(name = "read_csv", about = "Usage")]
pub struct Opt {
    #[structopt(help = "Input file 1")]
    pub input_1: String,
    #[structopt(help = "Input file 1")]
    pub input_2: String,
    #[structopt(help = "Column Name")]
    pub column_name: String,
    #[structopt(help = "Mark Name")]
    pub mark_name: String, 
    #[structopt(help = "Output file, stdout if not present")]
    pub output: Option<String>,
}