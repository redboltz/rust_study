struct ImportantExcerpt<'a, 'b> {
    part1: &'a str,
    part2: &'b str,
}

fn main() {
    // 僕をイシュマエルとお呼び。何年か前・・・
    let novel1 = String::from("Call me Ishmael. Some years ago...");
    let mut it1 = novel1.split('.');
    let first_sentence1 = it1.next()
        .expect("Could not find a '.'");  // '.'が見つかりませんでした
    let i;
    {
        let novel2 = String::from("AAA. Call me Ishmael. Some years ago...");
        let mut it2 = novel2.split('.');
        let first_sentence2 = it2.next()
            .expect("Could not find a '.'");  // '.'が見つかりませんでした
        i = ImportantExcerpt { part1: first_sentence1, part2: first_sentence2 };
        println!("{}", i.part2);
    }
    println!("{}", i.part1);
}
