use rand::{
    rngs,
    seq::index::{self, IndexVec::*},
    thread_rng, Rng,
};
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
};

const PATH: &str = "words/";

pub fn get_words(amount: u8, restriction: char) -> Result<Vec<String>, Box<dyn Error>> {
    let mut res: Vec<String> = Vec::new();
    if restriction != ' ' {
        let file = File::open(format!("{}{}.txt", PATH, restriction))?;
        let file_size = file.metadata()?.len();
        let mut reader = BufReader::new(file);
        let mut index: Vec<i64> = (0..amount)
            .map(|_| thread_rng().gen_range(0..(file_size / 10) as i64))
            .collect();
        for line in reader.lines() {
            index = index.iter_mut().map(|i| *i - 1).collect();
            if index.contains(&0) {
                res.push(line.unwrap_or(" ".to_string()));
            }
        }
        // for i in index::sample(&mut thread_rng(), file_size as usize, amount as usize) {
        //     reader.seek(SeekFrom::Start(i as u64));
        //     let word = reader.lines().map(|l| l.unwrap()).next();
        // }
    }
    Ok(res)
}
