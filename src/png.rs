use crc::{Crc, Algorithm};
use flate2::write::DeflateEncoder;
use flate2::Compression;
use rand::Rng;
use std::io::{self, Write};

const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
const PNG_CRC_ALG: Algorithm<u32> = Algorithm {
    width: 32,
    poly: 0xedb88320,
    init: 0xffffffff,
    refin: true,
    refout: true,
    xorout: 0xffffffff,
    check: 0xdebb20e3,
    residue: 0x00000000,
};

fn calculate_crc32(data: &[u8]) -> u32 {
    let crc = Crc::<u32>::new(&PNG_CRC_ALG);
    crc.checksum(data)
}

fn create_chunk(chunk_type: &[u8], data: &[u8]) -> Vec<u8> {
    let length = (data.len() as u32).to_be_bytes();
    let crc_data: Vec<u8> = chunk_type.iter().chain(data.iter()).cloned().collect();
    let crc = calculate_crc32(&crc_data).to_be_bytes();
    
    [&length[..], chunk_type, data, &crc[..]].concat()
}

fn create_ihdr_chunk(width: u32, height: u32) -> Vec<u8> {
    let mut ihdr_data = Vec::new();
    ihdr_data.extend_from_slice(&width.to_be_bytes());
    ihdr_data.extend_from_slice(&height.to_be_bytes());
    ihdr_data.extend_from_slice(&[8, 2, 0, 0, 0]); // Bit depth, Color type, Compression, Filter, Interlace

    create_chunk(b"IHDR", &ihdr_data)
}

fn create_idat_chunk(payload: &[u8], attempt: usize) -> Option<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut lbytes = vec![0u8; attempt];
    let mut rbytes = vec![0u8; attempt];
    rng.fill(&mut lbytes[..]);
    rng.fill(&mut rbytes[..]);

    let mut idat_data = Vec::new();
    idat_data.extend_from_slice(&lbytes);
    idat_data.extend_from_slice(payload);
    idat_data.extend_from_slice(&rbytes);

    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&idat_data).unwrap();
    let compressed_data = encoder.finish().unwrap();

    if compressed_data.windows(payload.len()).any(|window| window == payload) {
        Some(create_chunk(b"IDAT", &compressed_data))
    } else {
        None
    }
}

fn create_iend_chunk() -> Vec<u8> {
    create_chunk(b"IEND", &[])
}

fn initialize_png(width: u32, height: u32) -> Vec<u8> {
    let mut image = PNG_SIGNATURE.to_vec();
    println!("✅ PNG Header created");

    let ihdr_chunk = create_ihdr_chunk(width, height);
    image.extend_from_slice(&ihdr_chunk);
    println!("✅ IHDR chunk created with width: {}, height: {}", width, height);

    image
}

fn find_valid_idat_chunk(payload: &[u8], max_attempts: usize) -> io::Result<Option<Vec<u8>>> {
    println!("🕵️  Searching for valid payload placement (Max Attempts: {})...", max_attempts);

    for i in 1..=max_attempts {
        if let Some(idat_chunk) = create_idat_chunk(payload, i) {
            println!("🎯 Payload successfully placed after {} attempts!", i);
            return Ok(Some(idat_chunk));
        }

        if i % 1000 == 0 {
            println!("⌛ Attempt {}: Still searching...", i);
        }
    }

    Ok(None)
}

fn prompt_user_continue() -> io::Result<bool> {
    println!("❓ Do you want to continue searching? [Y/n]: ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(!input.trim().eq_ignore_ascii_case("n"))
}

pub fn build_png(payload: &[u8], width: u32, height: u32) -> io::Result<Vec<u8>> {
    println!("{}", "=".repeat(40));
    println!("🖼️  Starting PNG Generation...");
    println!("{}", "-".repeat(40));

    let mut image = initialize_png(width, height);
    let mut max_attempts = 5;

    loop {
        match find_valid_idat_chunk(payload, max_attempts)? {
            Some(idat_chunk) => {
                image.extend_from_slice(&idat_chunk);
                break;
            }
            None => {
                println!("⚠️  Max attempts reached: {}", max_attempts);
                if !prompt_user_continue()? {
                    println!("❌ Operation aborted by user.");
                    return Ok(image); // Return the incomplete image
                }
                max_attempts *= max_attempts;
            }
        }
    }

    let iend_chunk = create_iend_chunk();
    image.extend_from_slice(&iend_chunk);

    println!("{}", "=".repeat(40));
    println!("🎉 PNG Generation complete!");
    println!("{}", "=".repeat(40));

    Ok(image)
}