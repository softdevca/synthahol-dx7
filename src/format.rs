use std::path::Path;

use crate::SYSEX_HEADER;

const FORMAT_IDENTIFIER: [u8; 6] = SYSEX_HEADER;

pub struct Format;

impl Format {
    pub fn name() -> &'static str {
        "Yamaha DX7"
    }

    pub fn filename_extension() -> &'static str {
        "syx"
    }

    pub fn is_format(_path: &Path, header: &[u8]) -> bool {
        header.starts_with(&FORMAT_IDENTIFIER)
    }
}

#[cfg(test)]
mod test {
    use std::fs::read;

    use crate::tests::test_data_path;

    use super::Format;

    #[test]
    fn filename_extension() {
        assert_eq!(Format::filename_extension(), "syx");
    }

    #[test]
    fn name() {
        assert_eq!(Format::name(), "Yamaha DX7");
    }

    #[test]
    fn init_version_1() {
        let path = test_data_path(&["rom1a.syx"]);
        let contents = read(&path).unwrap();
        assert!(Format::is_format(&path, &contents));
    }

    #[test]
    fn short() {
        let path = test_data_path(&["rom1a.syx"]);
        let contents = read(&path).unwrap();
        let shortened = &contents[..3];
        assert!(!Format::is_format(&path, &shortened));
    }
}
