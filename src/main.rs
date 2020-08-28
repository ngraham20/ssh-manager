mod connections;
mod menu;
mod navigation;

fn main() {
    if let Ok(result) = connections::load_json() {
        println!("{}", result);
    }
}
