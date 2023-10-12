fn main() {
    let input = std::env::args().skip(1).next().unwrap();

    for (i, wanted_prefix) in ["00000", "000000"].into_iter().enumerate() {
        let got = Md5Miner { wanted_prefix }.mine(&input);
        println!("Part {i} answer: {got:?}", i = i + 1);
    }
}

trait Miner {
    fn mine(&self, input: &str) -> Option<String>;
}

struct Md5Miner<'a> {
    wanted_prefix: &'a str,
}

impl<'a> Md5Miner<'a> {
    fn hash(&self, input: &'a str) -> String {
        format!("{:x}", md5::compute(input))
    }
}

impl<'a> Miner for Md5Miner<'a> {
    fn mine(&self, input: &str) -> Option<String> {
        for i in 1..std::u32::MAX {
            let s = format!("{}{}", input, i);
            let hash = self.hash(&s);
            if hash.starts_with(&self.wanted_prefix) {
                return Some(i.to_string());
            }
        }
        None
    }
}
