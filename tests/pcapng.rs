extern crate pcap_parser;

use pcap_parser::pcapng::Block;
use pcap_parser::traits::PcapNGBlock;
use pcap_parser::{PcapBlock, PcapNGCapture};

static TEST001_BE: &'static [u8] = include_bytes!("../assets/test001-be.pcapng");
static TEST001_LE: &'static [u8] = include_bytes!("../assets/test001-le.pcapng");

#[test]
fn test_pcapng_capture_from_file_and_iter_le() {
    let cap =
        PcapNGCapture::from_file(TEST001_LE).expect("could not parse file into PcapNGCapture");
    for block in cap.iter() {
        println!("new block");
        match block {
            PcapBlock::NG(Block::EnhancedPacket(epb)) => {
                println!("block total length: {}", epb.block_length());
                println!("captured length: {}", epb.caplen());
            }
            _ => (),
        }
    }
}

#[test]
fn test_pcapng_capture_from_file_and_iter_be() {
    let cap =
        PcapNGCapture::from_file(TEST001_BE).expect("could not parse file into PcapNGCapture");
    for block in cap.iter() {
        println!("new block");
        match block {
            PcapBlock::NG(Block::EnhancedPacket(epb)) => {
                println!("block total length: {}", epb.block_length());
                println!("captured length: {}", epb.caplen());
            }
            _ => (),
        }
    }
}