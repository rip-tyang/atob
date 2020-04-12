pub mod av_bv {
    use std::collections::HashMap;

    const KEY_LEN:usize = 6;
    const MAGIC_XOR:u64 = 177451812;
    const MAGIC_ADD:u64 = 8728348608;
    static SEEDS:[u8; KEY_LEN] = [11,10,3,8,4,6];
    static TEMPLATE:&[u8] = b"BV1  4 1 7  ";
    static TABLE:&str = "fZodR9XQDSUm21yCkr6zBqiveYah8bt4xsWpHnJE7jL5VG3guMTKNPAwcF";

    pub fn atob(av: &u64) -> String {
        let decode = (av ^ MAGIC_XOR) + MAGIC_ADD;
        let mut res = TEMPLATE.to_vec();
        for i in 0..KEY_LEN {
            let key_pos = decode / 58_u64.pow(i as u32) % 58;
            res[SEEDS[i] as usize] = TABLE.bytes().nth(key_pos as usize).unwrap();
        }
        String::from_utf8(res).unwrap()
    }

    pub fn btoa(bv: &str) -> Option<u64> {
        let mut reverse_table = HashMap::new();
        for (i, c) in TABLE.bytes().enumerate() {
            reverse_table.insert(c as char, i as u64);
        }
        let arr:Option<u64> = (0..KEY_LEN).map(|i| {
            bv.bytes().nth(SEEDS[i] as usize)
              .and_then(|c| reverse_table.get(&(c as char)))
              .map(|idx| idx * 58_u64.pow(i as u32))
        }).sum();
        arr.map(|x| (x - MAGIC_ADD) ^ MAGIC_XOR)
    }
}
