// Reverse the file
// take file name from command line
// read lines and save the offset of the line
// read line in the reverse order


use std::io::Read;

fn read_file(path: &std::path::Path) -> std::io::Result<()> {
    if !path.exists() {
        eprintln!("file not found: {:?}", path);
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

    println!("{:?}", line_offsets);

    Ok(())
}


pub fn reverse(path: &std::path::Path) -> std::io::Result<()> {
    read_file(path)?;
    Ok(())
}