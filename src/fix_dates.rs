use std::io::{Read, Write};
use regex::bytes::Regex;


fn fix_dates_in_byte_slice(mut data: &mut [u8], min_date: &[u8]) {
    let re = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();

    while let Some(m) = re.find(data) {
        let range = m.range();
        let date = &mut data[range.clone()];

        if &*date < min_date {
            date.copy_from_slice(min_date);
        }

        data = &mut data[range.end..];
    }
}


pub fn fix_dates(input: &mut (impl Read + ?Sized), output: &mut impl Write, min_date: &[u8]) {
    // We delegate the actual work to a function with customizable buffer size.
    // This makes testing issues with ISO dates spanning buffer size boundaries easier.
    fix_dates_with_buffer_size(input, output, min_date, 64 * 1024)
}


fn fix_dates_with_buffer_size(input: &mut (impl Read + ?Sized), output: &mut impl Write, min_date: &[u8], block_size: usize) {
    // An ISO date may span the bytes which are read by two subsequent iterations.
    // Determine the overlap in bytes which is needed to ensure that we handle all dates correctly.
    const OVERLAP: usize = "2000-01-01".len() - 1;

    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(block_size + OVERLAP, 0);

    let mut data_end = 0;

    loop {
        let read_size = input.read(&mut buffer[data_end..]).unwrap();
        if read_size == 0 {
            break;
        }

        data_end = data_end + read_size;

        fix_dates_in_byte_slice(&mut buffer[..data_end], min_date);

        // If at least one ISO date fits in the data in the buffer, we will do the following:
        // - Write the beginning of the buffer to the output
        // - Move the end of the buffer to the front. This ensures that ISO dates which have been
        //   read only partially are handled correctly in the next iteration.
        if data_end > OVERLAP {
            output.write_all(&buffer[..data_end - OVERLAP]).unwrap();
            buffer.copy_within(data_end - OVERLAP.., 0);
            data_end = OVERLAP;
        }
    }

    output.write_all(&buffer[..data_end]).unwrap();
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;
    use super::*;

    fn byte_slice_test_helper(min_date: &[u8], input: &[u8], expected: &[u8]) {
        let mut data = input.to_vec();
        fix_dates_in_byte_slice(&mut data, &min_date);

        // print differences in results as strings if possible
        match (from_utf8(&data), from_utf8(&expected)) {
            (Ok(data), Ok(expected)) => assert_eq!(data, expected),
            (_, _) => assert_eq!(data, expected)
        }
    }

    #[test]
    fn test_byte_slice_1() {
        byte_slice_test_helper(b"2023-02-10",
                               b"start date: 2023-02-01, end date: 2023-02-28",
                               b"start date: 2023-02-10, end date: 2023-02-28");
    }

    #[test]
    fn test_byte_slice_2() {
        byte_slice_test_helper(b"2023-02-10",
                               b"start date: 2023-02-01, end date: 2023-02-09",
                               b"start date: 2023-02-10, end date: 2023-02-10");
    }

    #[test]
    fn test_byte_slice_no_utf8() {
        byte_slice_test_helper(b"2023-02-10",
                               b"\xff2023-02-09\xff2023-02-10\xff2023-02-11\xff1998-01-31",
                               b"\xff2023-02-10\xff2023-02-10\xff2023-02-11\xff2023-02-10");
    }

    fn read_test_helper(min_date: &[u8], buffer_size: Option<usize>, input: &[u8], expected: &[u8]) {
        let input = input.to_vec();
        let mut output: Vec<u8> = Vec::new();

        match buffer_size {
            None => fix_dates(&mut &(input[..]), &mut output, &min_date),
            Some(size) => fix_dates_with_buffer_size(&mut &(input[..]), &mut output, &min_date, size)
        }

        // print differences in results as strings if possible
        match (from_utf8(&output), from_utf8(&expected)) {
            (Ok(output), Ok(expected)) => assert_eq!(output, expected),
            (_, _) => assert_eq!(output, expected)
        }
    }

    #[test]
    fn test_read_1() {
        read_test_helper(b"2023-02-10",
                         None,
                         b"start date: 2023-02-01, end date: 2023-02-09",
                         b"start date: 2023-02-10, end date: 2023-02-10",
        )
    }

    #[test]
    fn test_read_2() {
        read_test_helper(b"2023-02-10",
                         Some(1),
                         b"start date: 2023-02-01, end date: 2023-02-09",
                         b"start date: 2023-02-10, end date: 2023-02-10",
        )
    }
}
