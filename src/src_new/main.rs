use structopt::StructOpt;
mod opt;
use self::opt::Opt;
mod err;
mod core;
use std::path::PathBuf;
use std::process;
use self::core::{
    read::{load_csv, write_csv},
    write::replace_column,
};

fn main() {
    let opt = Opt::from_args();
    let filename_1 = PathBuf::from(opt.input_1);
    let filename_2 = PathBuf::from(opt.input_2);
    let csv_data_1 = match load_csv(filename_1) {
        Ok(fname) => { fname },
        Err(e) => {
            println!("main error: {:?}", e);
            process::exit(1);
        }
    };
    let csv_data_2 = match load_csv(filename_2) {
        Ok(fname) => { fname },
        Err(e) => {
            println!("main error: {:?}", e);
            process::exit(1);
        }
    };
    let modified_data = match replace_column(csv_data_1, csv_data_2, &opt.column_name, &opt.mark_name) {
        Ok(data) => { data },
        Err(e) => {
            println!("main error: {:?}", e);
            process::exit(1);
        }
    };
    let output_file = &opt.output
        .unwrap_or("output/output.csv".to_string());
    match write_csv(&modified_data, &output_file) {
        Ok(_) => {
            println!("write success!");
        },
        Err(e) => {
            println!("main error: {:?}", e);
            process::exit(1);
        }
    }
}
