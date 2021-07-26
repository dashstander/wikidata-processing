use bzip2::read::BzDecoder;
// use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use::wikidata::Entity;


fn decompress_and_read_ldjson(input: impl Read) {
    let reader = BufReader::new(BzDecoder::new(input));
    let entities = reader
        .lines()
        .map(Result::unwrap)
        //.par_bridge()
        .filter_map(|line| serde_json::from_str(&line.trim_end_matches(",")).ok())
        .filter_map(|record| Entity::from_json(record).ok());
    for ent in entities {
        for (_pid, claim) in ent.claims {
            println!("{}", claim.id);
        }
    }
}


fn main() {
    let path = "/mnt/d/latest-all.json.bz2";
    let compressed_file = File::open(path).expect("No file found");
    decompress_and_read_ldjson(compressed_file);
}