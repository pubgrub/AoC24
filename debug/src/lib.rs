use std::io::{stdin, stdout, Read, Write};

pub fn p_string_as_grid(s: &mut String, l: usize) {
    let n_lines = s.len() / l;
    s.push_str(&" ".repeat(l));
    for i in 0..=n_lines {
        println!("{}", s.get(i * l..i * l + l).unwrap());
    }
}

pub fn pause() {
    let mut stdout = stdout();
    //stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
