struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // 僕をイシュマエルとお呼び。何年か前・・・
    let novel = String::from("Call me Ishmael. Some years ago...");
    let i;
    {
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");  // '.'が見つかりませんでした
        i = ImportantExcerpt { part: first_sentence };
    }
    println!("{}", i.part);
}
