const PC_1: [u8; 56] = [
    57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18, 10, 2, 59, 51, 43, 35, 27, 19, 11, 3, 60,
    52, 44, 36, 63, 55, 47, 39, 31, 23, 15, 7, 62, 54, 46, 38, 30, 22, 14, 6, 61, 53, 45, 37, 29,
    21, 13, 5, 28, 20, 12, 4,
];

const PC_2: [u8; 48] = [
    14, 17, 11, 24, 1, 5, 3, 28, 15, 6, 21, 10, 23, 19, 12, 4, 26, 8, 16, 7, 27, 20, 13, 2, 41, 52,
    31, 37, 47, 55, 30, 40, 51, 45, 33, 48, 44, 49, 39, 56, 34, 53, 46, 42, 50, 36, 29, 32,
];

const SHIFT_SCHEDULE: [u8; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];

fn apply<const N: usize>(original_no_bits: u8, table: [u8; N], data: u64) -> u64 {
    let mut new_data = 0_u64;
    for i in 0..table.len() {
        let masked = data & (1 << (original_no_bits - table[i]));
        new_data |= (masked >> (original_no_bits - table[i])) << (table.len() - i - 1);
    }
    new_data
}

fn circular_shl(data: u64, i: u8) -> u64 {
    ((data << i) | (data >> (28 - i))) & ((1 << 28) - 1)
}

fn create_keys(k: u64) {
    let k_plus = apply(64, PC_1, k);
    eprintln!("K+: {:056b}", k_plus);

    let c0 = (k_plus & 72057593769492480_u64) >> 28;
    let d0 = k_plus & 268435455_u64;
    eprintln!("C0: {:028b}, D0: {:028b}", c0, d0);

    let mut cn_dn = [(c0, d0); 17];
    for i in 1..=16 {
        let cn = circular_shl(cn_dn[i - 1].0, SHIFT_SCHEDULE[i - 1]);
        let dn = circular_shl(cn_dn[i - 1].1, SHIFT_SCHEDULE[i - 1]);
        cn_dn[i] = (cn, dn);
        eprintln!("C{}: {:028b}, D{}: {:028b}", i, cn, i, dn);
    }

    let mut keys = [0_u64; 16];
    for i in 1..=16 {
        keys[i - 1] = apply(56, PC_2, (cn_dn[i].0 << 28) | cn_dn[i].1);
        eprintln!("K{}: {:048b}", i, keys[i - 1]);
    }
}

fn main() {
    create_keys(0b0001001100110100010101110111100110011011101111001101111111110001);
}
