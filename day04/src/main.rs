use file;
use regex::Regex;

const TEST: i32 = 0;

fn main() {
    let day = env!("CARGO_PKG_NAME");
    let lines = file::read_file(day, TEST);
    println!("Part 1: {}", solve1(&lines));
    println!("Part 2: {}", solve2(&lines));
}

fn solve1(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let width = lines[0].len() + 2;
    let mut res: Vec<Regex> = vec![];
    res.push(Regex::new(r"XMAS").unwrap());
    res.push(Regex::new(r"SAMX").unwrap());
    for x in -2i32..=0 {
        let offset_str = (width as i32 + x).to_string();
        // going up
        let re_str = String::from(
            "S.{".to_string() + &offset_str + "}A.{" + &offset_str + "}M.{" + &offset_str + "}X",
        );
        res.push(Regex::new(&re_str).unwrap());
        // going down
        let re_str = String::from(
            "X.{".to_string() + &offset_str + "}M.{" + &offset_str + "}A.{" + &offset_str + "}S",
        );
        res.push(Regex::new(&re_str).unwrap());
    }
    let mut search_line = ".".repeat(width);
    for line in lines {
        search_line = search_line + "." + line + ".";
    }
    search_line += &".".repeat(width);
    for re in res {
        let mut found = true;
        let mut start = 0;
        while found {
            let xmas = re.find(&search_line[start..]);
            match xmas {
                None => found = false,
                _ => {
                    start += xmas.unwrap().start() + 1;
                    result += 1;
                }
            }
        }
    }
    result
}

fn solve2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let width = lines[0].len();
    let mut res: Vec<Regex> = vec![];

    let re_str =
        "M.{1}M.{".to_string() + &width.to_string() + "}A.{" + &width.to_string() + "}S.{1}S";
    res.push(Regex::new(&re_str).unwrap());
    let re_str =
        "M.{1}S.{".to_string() + &width.to_string() + "}A.{" + &width.to_string() + "}M.{1}S";
    res.push(Regex::new(&re_str).unwrap());
    let re_str =
        "S.{1}M.{".to_string() + &width.to_string() + "}A.{" + &width.to_string() + "}S.{1}M";
    res.push(Regex::new(&re_str).unwrap());
    let re_str =
        "S.{1}S.{".to_string() + &width.to_string() + "}A.{" + &width.to_string() + "}M.{1}M";
    res.push(Regex::new(&re_str).unwrap());

    let mut search_line = ".".repeat(width);
    for line in lines {
        search_line = search_line + "." + line + ".";
    }
    search_line += &".".repeat(width);

    for re in res {
        let mut found = true;
        let mut start = 0;
        while found {
            let xmas = re.find(&search_line[start..]);
            match xmas {
                None => found = false,
                _ => {
                    start += xmas.unwrap().start() + 1;
                    result += 1;
                }
            }
        }
    }
    result
}
