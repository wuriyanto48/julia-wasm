use super::shared::{ BASE64_TABLE, EIGHT_BIT_MASK, PADDING };
use std::io::{ Read, Write };

pub fn decode(input: &mut dyn Read, out: &mut dyn Write) -> Result<(), String> {
    let mut base64_hashmap_table: std::collections::HashMap<u8, usize> = std::collections::HashMap::new();
    for (pos, &e) in BASE64_TABLE.iter().enumerate() {
        let cp = e;
        base64_hashmap_table.insert(cp, pos);
    }

    let mut buffer = vec![0 as u8; 4];
    loop {
        let line_read = match input.read(&mut buffer[..]) {
            Ok(o) => o,
            Err(e) => return Err(format!("error reading input {}", e))   
        };

        if line_read <= 0 {
            break;
        }

        let seg_data = &buffer[..line_read];
        
        let mut segment_count = 0;
        let mut dec: u64 = 0;
        for i in seg_data {
            // discard padding
            if *i == PADDING {
                continue;
            }

            let b64_idx = base64_hashmap_table.get(i).unwrap();
            let l_shift: u64 =  18 - segment_count * 6;
            let base64_idx = *b64_idx as u64;
            dec |= base64_idx << l_shift;

            segment_count = segment_count + 1; 
        }

        for i in 0..segment_count-1 {
            let r_shift: u64 = 16 - i * 8;
            let cp = ((dec >> r_shift) & EIGHT_BIT_MASK) as u8;
            if let Err(e) = out.write(&[cp]) {
                return Err(format!("error write buffer out {}", e));
            }
        }
    }

    Ok(())
}