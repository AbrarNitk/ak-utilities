/*
Requirements:

- Reverse the input lines, words in line, the line itself, chars in the word.
- Input can be a file from command line or stdin
- can be very big, can not fit into the memory
- line can also be a very big, can not fit into the memory
- output can be written to a file if given from command line or written to the stdout
*/

// Reverse the file
// take file name from command line
// read lines and save the offset of the line
// read line in the reverse order

use std::io::{Read, Seek};

fn read_file(path: &std::path::Path) -> std::io::Result<Vec<usize>> {
    if !path.exists() {
        eprintln!("file not found: {:?}", path);
        std::process::exit(1);
    }
    let mut buf: [u8; 1024] = [0; 1024];
    let mut file = std::fs::File::options().read(true).open(path)?;
    let mut file_offset: usize = 0;
    let mut line_offsets = vec![0];

    loop {
        let read_bytes = file.read(&mut buf)?;
        if read_bytes == 0 {
            break;
        }

        for byte in buf.iter() {
            file_offset += 1;
            if byte.eq(&b'\n') {
                line_offsets.push(file_offset);
            }
        }
    }

    Ok(line_offsets)
}

// Note function should also accepts the Write, where should it write the bytes
fn process_reverse<W>(
    path: &std::path::Path,
    line_offsets: &[usize],
    w: &mut W,
) -> std::io::Result<()>
    where
        W: std::io::Write,
{
    if !path.exists() {
        eprintln!("file not found: {:?}", path);
        std::process::exit(1);
    }
    let mut buf: [u8; 1024] = [0; 1024];
    let mut file = std::fs::File::options().read(true).open(path)?;
    // Note: reading lines in the reverse order
    for index in (0..(line_offsets.len() - 1)).rev() {
        let offset = line_offsets[index];
        let size = line_offsets[index + 1] - offset;
        println!("offset: {}, size: {}", offset, size);

        // Note: this seek can also be done from backward pointer, because we are reading file from back
        // seeking from the start always is not a good idea, though I need to check the seek system call
        file.seek(std::io::SeekFrom::Start(offset as u64))?;
        // need to read the size bytes
        let mut read_size = size;
        loop {
            let read_bytes = file.read(&mut buf)?;
            if read_bytes == 0 {
                break;
            }
            if read_bytes >= read_size {
                w.write(&buf[..read_size])?;
                break;
            } else {
                read_size = read_size - read_bytes;
            }
        }
    }
    Ok(())
}

pub fn reverse(path: &std::path::Path) -> std::io::Result<()> {
    let line_offsets = read_file(path)?;
    process_reverse(path, line_offsets.as_slice(), &mut std::io::stdout())?;
    Ok(())
}
