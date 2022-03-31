fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    for line in input.lines() {
        if line.starts_with("toggle ") {
            let (start, end) = line.split(' ').collect::<Vec<_>>();
        }
        else if line.starts_with("turn on ") {

        }
        else if line.starts_with("turn off ") {
            
        }
    }

}
