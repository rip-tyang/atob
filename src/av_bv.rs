pub mod av_bv {
    use std::collections::HashMap;
    static MAGIC_XOR:u64 = 177451812;
    static MAGIC_ADD:u64 = 8728348608;
    static SEEDS:[u8; 6] = [11,10,3,8,4,6];
    static TEMPLATE:&[u8] = b"BV1  4 1 7  ";
    static TABLE:&str = "fZodR9XQDSUm21yCkr6zBqiveYah8bt4xsWpHnJE7jL5VG3guMTKNPAwcF";

    pub fn atob(av: &u64) -> String {
        let decode = (av ^ MAGIC_XOR) + MAGIC_ADD;
        let mut res = TEMPLATE.to_vec();
        for i in 0..6 {
            let seed = SEEDS[i] as usize;
            let key_pos = decode / 58_u64.pow(i as u32) % 58;
            res[seed] = TABLE.bytes().nth(key_pos as usize).unwrap();
        }
        String::from_utf8(res).unwrap()
    }

    pub fn btoa(bv: &str) -> u64 {
        let mut reverse_table = HashMap::new();
        for (i, c) in TABLE.bytes().enumerate() {
            reverse_table.insert(c as char, i as u64);
        }
        let mut a:u64 = 0;
        for i in 0..6 {
            let seed = SEEDS[i];
            let key_pos = bv.bytes().nth(seed as usize).unwrap();
            let value = reverse_table.get(&(key_pos as char)).unwrap();
            a += *value * 58_u64.pow(i as u32);
        }
        (a - MAGIC_ADD) ^ MAGIC_XOR
    }
}
