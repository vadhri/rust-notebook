#![allow(dead_code)]

pub fn i18n(days: i32) -> String {
    match days {
        std::i32::MIN..=-2 => format!("arrived {} days ago", days.abs()),
        -1 => format!("arrived yesterday !"),
        0 => format!("arrives today !"),
        1 => format!("arrives tomorrow !"),
        2..=std::i32::MAX => format!("arrives in {} days !", days)
    }
}

pub fn verse(n: u32) -> String {
    let verse = match n {
        0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1)
    };

    verse
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();

    for verse_no in (end..=start).rev() {
        song.push_str(&verse(verse_no));

        println!("{} {}", start, verse_no);

        if verse_no != end {
            song.push_str("\n");
        }
    }

    song
}



pub fn pm_test() {
    for x in -3..4 {
        println!("The delivery {}", i18n(x));
    }

    println!("{:?}",sing(2, 0));
}
