pub trait FormateToString {
    fn formate_file_size(&self) -> Option<String>;
}

impl FormateToString for u64 {
    fn formate_file_size(&self) -> Option<String> {
        let abbreviation: Option<&str> = match *self {
            1 => Some("B"),                             //Byte [1 Byte (B) = 8 bits (b)]
            n if n <= 1_000 => Some("KB"),              //Kilobyte
            n if n <= 1_024 => Some("KiB"),             //Kibibyte
            n if n <= 1_000_000 => Some("MB"),          //Megabyte
            n if n <= 1_048_576 => Some("MiB"),         //Mebibyte
            n if n <= 1_000_000_000 => Some("GB"),      //Gigabyte
            n if n <= 1_073_741_824 => Some("GiB"),     //Gibibyte
            n if n <= 1_000_000_000_000 => Some("TB"),  //Terabyte
            n if n <= 1_099_511_627_776 => Some("TiB"), //Tebibyte
            _ => None,
        };

        if let Some(abb) = abbreviation {
            return Some(format!("{:.2} {}", self, abb));
        }
        None
    }
}
