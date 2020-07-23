mod app;
mod parse;

fn main() {
    let params = app::params();
    
    println!("Using input file name: {}", params.input);
    

}
