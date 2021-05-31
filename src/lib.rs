/// recursively parses and runs brainfuck code, putting chars to output vec
/// # Arguments
/// code - reference to brainfuck code in form of char array
/// skip - whether to skip the loop (data byte is 0 when '[' instruction is issued)
/// data - array that is used to compute output
/// data_idx - current position in data
/// output - mock for stdout
fn parse_and_compute(code: &[u8], skip: bool, data: &mut [u8; 30000],
                         data_idx: &mut usize, output: &mut Vec<u8>) -> usize {
    let mut code_idx = 0; // current position in code
    while code_idx < code.len() {
        let c = code[code_idx];
        match c { // TODO potentially add overflow handling
            b'>' if !skip => *data_idx += 1, // move data ptr right
            b'<' if !skip => *data_idx -= 1, // move data ptr left
            b'+' if !skip => data[*data_idx] += 1, // inc data ptr
            b'-' if !skip => data[*data_idx] -= 1, // dec data ptr
            b'.' if !skip => output.push(data[*data_idx]), // put char to out
            b'[' => {
                while !skip && data[*data_idx] != 0 {
                    parse_and_compute(&code[(code_idx + 1)..], false, data, data_idx, output);
                }
                code_idx += parse_and_compute(&code[(code_idx + 1)..], true, data, data_idx, output) + 1;
            },
            b']' => return code_idx,
            _ => ()
        }
        code_idx += 1;
    }
    code_idx
}

pub fn run(code: String) -> String {
    let mut out = Vec::<u8>::new();
    let mut data = [0u8; 30000];
    let mut data_idx = 0usize;
    let code_as_u8_arr: &[u8] = code.as_bytes();

    parse_and_compute(code_as_u8_arr, false, &mut data, &mut data_idx, &mut out);

    String::from_utf8(out).expect("Error creating string from brainfuck output")
}
