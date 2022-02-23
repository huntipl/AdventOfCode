fn main() {
    let solution: (i32, i32) = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            line.split('x')
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|mut input| {
            input.sort_unstable();
            let mut faces = vec![
                input[0] * input[1],
                input[0] * input[2],
                input[1] * input[2],
            ];
            faces.sort_unstable();
            let smallest = faces[0];
            let ribbon_base = 2 * input[0] + 2 * input[1];
            let ribbon_bow: i32 = input.iter().product();

            (
                faces.iter().sum::<i32>() * 2 + smallest,
                ribbon_base + ribbon_bow,
            )
        })
        .fold((0, 0), |mut v, (a, b)| {
            v.0 += a;
            v.1 += b;
            v
        });
    // first value is the answer to task1, second to task2
    println!("{solution:#?}");
}
