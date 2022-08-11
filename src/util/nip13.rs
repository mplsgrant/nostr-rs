/// Gets the number of leading zero bits from a hash
pub fn get_leading_zeroes(h: bitcoin_hashes::sha256::Hash) -> u32 {
    let mut res = 0;
    let mut stop_counting = false;
    h.iter().for_each(|&x| {
        if stop_counting { return; }
        res += x.leading_zeros();
        if x != 0 { stop_counting = true; }
    });

    res
}

/// Returns all possible ID prefixes (hex) that have the specified number of leading zero bits
pub fn get_prefixes_for_difficulty(leading_zero_bits: u8) -> Vec<String> {
    let mut r = vec![];

    if leading_zero_bits == 0 { return r; }

    let hex_chars =
        if leading_zero_bits % 4 == 0 { leading_zero_bits / 4 } 
        else { leading_zero_bits / 4 + 1 };
    let prefix_bits = hex_chars * 4;

    for i in 0..2_u32.pow((prefix_bits - leading_zero_bits) as usize as u32) {
        let p = format!("{:01$x}", i, hex_chars as usize); // https://stackoverflow.com/a/26286238
        r.push(p);
    }

    r
}



#[cfg(test)]
pub mod tests {
    use bitcoin_hashes::hex::FromHex;
    use bitcoin_hashes::sha256::Hash;

    use crate::util::nip13::*;

    #[test]
    fn check_get_leading_zeroes() {
        assert_eq!(4, get_leading_zeroes(Hash::from_hex("0fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(3, get_leading_zeroes(Hash::from_hex("1fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("2fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("3fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(1, get_leading_zeroes(Hash::from_hex("4fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(1, get_leading_zeroes(Hash::from_hex("5fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(1, get_leading_zeroes(Hash::from_hex("6fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(1, get_leading_zeroes(Hash::from_hex("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));

        assert_eq!(0, get_leading_zeroes(Hash::from_hex("8fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(0, get_leading_zeroes(Hash::from_hex("9fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(0, get_leading_zeroes(Hash::from_hex("afffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(0, get_leading_zeroes(Hash::from_hex("bfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(0, get_leading_zeroes(Hash::from_hex("cfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(0, get_leading_zeroes(Hash::from_hex("dfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(0, get_leading_zeroes(Hash::from_hex("efffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(0, get_leading_zeroes(Hash::from_hex("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));

        assert_eq!(2, get_leading_zeroes(Hash::from_hex("20ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("21ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("22ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("23ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("24ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("25ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("26ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("27ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("28ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("29ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("2affffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("2bffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("2cffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("2dffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("2effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
        assert_eq!(2, get_leading_zeroes(Hash::from_hex("2fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap()));
    }

    #[test]
    fn check_find_prefixes_for_pow() {
        assert_eq!(get_prefixes_for_difficulty(0).is_empty(), true);

        assert_eq!(get_prefixes_for_difficulty(1),
                   vec!["0", "1", "2", "3", "4", "5", "6", "7"]);
        assert_eq!(get_prefixes_for_difficulty(2),
                   vec!["0", "1", "2", "3"]);
        assert_eq!(get_prefixes_for_difficulty(3),
                   vec!["0", "1"]);
        assert_eq!(get_prefixes_for_difficulty(4),
                   vec!["0"]);

        assert_eq!(get_prefixes_for_difficulty(5),
                   vec!["00", "01", "02", "03", "04", "05", "06", "07"]);
        assert_eq!(get_prefixes_for_difficulty(6),
                   vec!["00", "01", "02", "03"]);
        assert_eq!(get_prefixes_for_difficulty(7),
                   vec!["00", "01"]);
        assert_eq!(get_prefixes_for_difficulty(8),
                   vec!["00"]);

        assert_eq!(get_prefixes_for_difficulty(9),
                   vec!["000", "001", "002", "003", "004", "005", "006", "007"]);
        assert_eq!(get_prefixes_for_difficulty(10),
                   vec!["000", "001", "002", "003"]);
        assert_eq!(get_prefixes_for_difficulty(11),
                   vec!["000", "001"]);
        assert_eq!(get_prefixes_for_difficulty(12),
                   vec!["000"]);
    }


}