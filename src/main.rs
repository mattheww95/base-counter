use bio::io::fastq;
use bio::io::fastq::Records;
use core::fmt;
use std::default;
use std::path::PathBuf;
use clap::Parser;
use flate2::read::GzDecoder;
use std::io::{BufReader, Read};
use std::fs::File;





#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli{

    #[clap(long, short)]
    forward: PathBuf, // Path to forward reads, zipped or unzipped


    #[clap(long, short)]
    reverse: PathBuf, // Path to reverse reads, zipped or unzipped

    #[clap(long, short)]
    name: String, // Sample name for the output results
}


#[derive(Debug)]
struct BaseCounts{
    a: u64,
    t: u64,
    c: u64,
    g: u64,
    ambig: u64,
}


impl Default for BaseCounts {
    fn default() -> Self {
        return BaseCounts { a: 0, t: 0, c: 0, g: 0, ambig: 0 }
    }
}

impl fmt::Display for BaseCounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{},{},{}", self.a, self.t, self.c, self.g, self.ambig)
    }
}

fn get_reader(file_path: PathBuf) -> Box<dyn Read> {

    let f = match File::open(&file_path) {
        Ok(file) => file,
        Err(error) => panic!("Could not open file: {:?}", error),
    };

    let extension = match file_path.extension() {
        Some(ext) => ext,
        None => panic!("Could not determine file extension: {:?}", file_path),
    }.to_str().unwrap();

    match extension {
        "gz" => Box::new(GzDecoder::new(f)),
        _ => Box::new(f),
    }
}

fn count_bases(basecounts: &mut BaseCounts, reader: &mut Records<BufReader<Box< dyn Read + 'static>>>) -> () {

    while let Some(Ok(record)) = reader.next(){
        for base in record.seq().iter() {
           match base {
                    b'a' | b'A' => basecounts.a += 1,
                    b't' | b'T' => basecounts.t += 1,
                    b'c' | b'C' => basecounts.c += 1,
                    b'g' | b'G' => basecounts.g += 1,
                    _ => basecounts.ambig += 1,
            }
        }
    }

}


fn count_fq_bases(forward: PathBuf, reverse: PathBuf) -> BaseCounts {
    
    let mut basecounts: BaseCounts = default::Default::default();

    let mut reader = fastq::Reader::from_bufread(
        BufReader::new(get_reader(forward)))
        .records();

    count_bases(&mut basecounts, &mut reader);

    reader = fastq::Reader::from_bufread(BufReader::new(get_reader(reverse))).records();
    count_bases(&mut basecounts, &mut reader);

    basecounts

}

fn main() {
    let cli = Cli::parse();
    let counts = count_fq_bases(cli.forward, cli.reverse);
    println!("Sample,A,T,C,G,Ambig");
    println!("{},{}", cli.name, counts);
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::path::Path;

    #[test]
    fn test_count_bases(){
        let p1 = Path::new("test/reads_1.fq");
        let p2 = Path::new("test/reads_2.fq.gz");
        let counts = count_fq_bases(p1.to_path_buf(), p2.to_path_buf());
        assert_eq!(counts.a, 256319);
        assert_eq!(counts.c, 306746);
        assert_eq!(counts.g, 304376);
        assert_eq!(counts.t, 260031);
        assert_eq!(counts.ambig, 46);
    }
}