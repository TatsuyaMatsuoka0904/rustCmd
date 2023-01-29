use clap::Parser;
mod components;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {


   #[arg(short, long, default_value_t = 1)]
   count: u8,
}



fn main() {
   let args = Args::parse();

   for _ in 0..args.count {
    components::result::angle_koyori();
    components::result::timekeep();
    components::result::rand();
   }
}