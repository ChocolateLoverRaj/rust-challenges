pub fn convert(s: String, num_rows: i32) -> String {
    let num_rows = num_rows as usize;
    // We know the final len of the string so there will be no reallocation or wasted memory
    let mut o = String::with_capacity(s.len());
    let step = match num_rows {
        1 => 1,
        num_rows => num_rows * 2 - 2,
    };
    // Append the top row
    {
        let mut i = 0;
        loop {
            if let Some(c) = s.as_bytes().get(i * step).copied() {
                o.push(char::from(c));
            } else {
                break;
            }
            i += 1;
        }
    }
    // Append middle rows
    for row in 1..num_rows - 1 {
        let start_0 = row;
        let start_1 = num_rows * 2 - 2 - row;
        let mut i = 0;
        loop {
            if let Some(c) = s.as_bytes().get(start_0 + i * step).copied() {
                o.push(char::from(c));
            } else {
                break;
            }
            if let Some(c) = s.as_bytes().get(start_1 + i * step).copied() {
                o.push(char::from(c));
            } else {
                break;
            }
            i += 1;
        }
    }
    // Append the last row
    if num_rows > 1 {
        let start = num_rows - 1;
        let mut i = 0;
        loop {
            if let Some(c) = s.as_bytes().get(start + i * step).copied() {
                o.push(char::from(c));
            } else {
                break;
            }
            i += 1;
        }
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_string_to_zigzag() {
        assert_eq!(
            "PAHNAPLSIIGYIR".to_string(),
            convert("PAYPALISHIRING".to_string(), 3)
        );

        assert_eq!(
            "PINALSIGYAHRPI".to_string(),
            convert("PAYPALISHIRING".to_string(), 4)
        );

        assert_eq!("A".to_string(), convert("A".to_string(), 1));
    }
}
