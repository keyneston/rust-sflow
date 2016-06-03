// Standard Imports
use std::io::Cursor;

// Internal Imports
use datagram::Datagram;

// External Test Imports
#[cfg(test)]
use rustc_serialize::hex::FromHex;

const RAW_TEST_DATA: &'static str = "this data is redacted";

#[test]
fn test_decode_datagram() {

    let hex_data = RAW_TEST_DATA.clone().from_hex().unwrap();
    let mut data = Cursor::new(hex_data);

    let d: Datagram = ::utils::Decodeable::read_and_decode(&mut data).unwrap();
    // TODO: Add more tests here.
    assert_eq!(d.sample_record.len(), 6);
}

#[test]
fn test_decode_string() {
    struct TestDecodeStringCase {
        raw_test_data: &'static str,
        result: &'static str,
    };

    let test_cases: Vec<TestDecodeStringCase> = vec![
        TestDecodeStringCase{raw_test_data: "00000006666f6f626172", result: "foobar"},
    ];

    for case in test_cases {
        let mut data = Cursor::new(case.raw_test_data.clone().from_hex().unwrap());
        let res: String = ::utils::Decodeable::read_and_decode(&mut data).unwrap();

        assert_eq!(case.result, res);
    }
}
