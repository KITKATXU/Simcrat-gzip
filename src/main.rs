extern crate once_cell;extern crate libc;
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum ArgType {
    None,
    SChar,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    LongInt,
    ULongInt,
    LongLongInt,
    ULongLongInt,
    Double,
    LongDouble,
    Char,
    WideChar,
    String,
    WideString,
    Pointer,
    CountSCharPointer,
    CountShortPointer,
    CountIntPointer,
    CountLongIntPointer,
    CountLongLongIntPointer,
}
#[derive(Debug)]
enum ArgValue {
    SChar(i8),
    UChar(u8),
    Short(i16),
    UShort(u16),
    Int(i32),
    UInt(u32),
    LongInt(i64),
    ULongInt(u64),
    LongLongInt(i64),
    ULongLongInt(u64),
    Float(f32),
    Double(f64),
    LongDouble(f64), // Rust does not have a native long double type
    Char(i32),
    WideChar(u32), // Assuming wint_t is equivalent to u32
    String(String),
    WideString(Vec<u16>), // Assuming wchar_t is equivalent to u16
    Pointer(Box<dyn std::any::Any>),
    CountSCharPointer(Vec<i8>),
    CountShortPointer(Vec<i16>),
    CountIntPointer(Vec<i32>),
    CountLongIntPointer(Vec<i64>),
    CountLongLongIntPointer(Vec<i64>),
}
#[derive(Debug)]
struct Argument {
    arg_type: ArgType,
    value: ArgValue,
}
#[derive(Debug)]
struct Arguments {
    arg: Vec<Argument>,
    direct_alloc_arg: [Argument; 7],
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Bytes {
    Word(i64),
    Bytes { dummy: i32 },
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct CdBuf {
    fd: i32,
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct CharDirective<'a> {
    dir_start: &'a str,
    dir_end: &'a str,
    flags: i32,
    width_start: &'a str,
    width_end: &'a str,
    width_arg_index: usize,
    precision_start: &'a str,
    precision_end: &'a str,
    precision_arg_index: usize,
    conversion: char,
    arg_index: usize,
}
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct CharDirectives<'a> {
    count: usize,
    dir: Vec<CharDirective<'a>>,
    max_width_length: usize,
    max_precision_length: usize,
    direct_alloc_dir: [CharDirective<'a>; 7],
}
type CharType = u8;
type CmpCodeInt = u64;
type CodeInt = i64;
type ComparisonFunction = fn(&(), &()) -> i32;
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Config {
    good_length: Ush,
    max_lazy: Ush,
    nice_length: Ush,
    max_chain: Ush,
}
type CountInt = u64;
type CountShort = u16;
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct CtData {
    fc: FreqOrCode,
    dl: DadOrLen,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum DadOrLen {
    Dad(Ush),
    Len(Ush),
}
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct DirentryT {
    name: String,
}
type Dummy = i32;
type FileT = i32;
type FpucwT = u32;
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum FreqOrCode {
    Freq(Ush),
    Code(Ush),
}
type GLFunctionTakingIntReturningVoidT = Box<dyn Fn(i32)>;
type GlLockT = i32;
type GlOnceT = i32;
type GlRecursiveLockT = i32;
type GlRwlockT = i32;
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Huft {
    e: Uch,
    b: Uch,
    v: HuftValue,
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum HuftValue {
    N(Ush),
    T(Box<Huft>),
}
type IPos = u32;
type IdxT = isize;
type Pos = Ush;
type SaHandlerT = fn(i32);
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct SavedCwd {
    desc: i32,
    name: Option<String>,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum SavedirOption {
    SortNone = 0,
    SortName = 1,
    SortFastRead = 2, // Assign a unique value to SortFastRead
}
type SmallT = u8;
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct TimeTMustBeIntegral {
    _floating_time_t_unsupported: std::marker::PhantomData<()>,
}
#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct TreeDesc<'a> {
    dyn_tree: &'a mut [CtData],
    static_tree: Option<&'a [CtData]>,
    extra_bits: Option<&'a [i32]>,
    extra_base: i32,
    elems: i32,
    max_length: i32,
    max_code: i32,
}
type Uch = u8;
type Ulg = u64;
type Ush = u16;
type Voidp = *mut std::ffi::c_void;
static mut ASCII: i32 = 0;
static mut BASE_DIST: [i32; 30] = [0; 30];
static mut BASE_LENGTH: [i32; 29] = [0; 29];
static mut BB: Ulg = 0;
static mut BITBUF: Ulg = 0;
static mut BITCOUNT: i32 = 0;
static mut BI_BUF: u16 = 0;
static mut BI_VALID: i32 = 0;
static mut BK: u32 = 0;
static mut BLOCKSIZE: u32 = 0;
static BLOCK_MODE: i32 = 0x80;
static mut BLOCK_START: i64 = 0;
static mut BL_COUNT: [Ush; 16] = [0; 16];
static mut BL_DESC: Option<TreeDesc> = None;
const BL_ORDER: [Uch; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
static mut BL_TREE: [CtData; 2 * 19 + 1] = [CtData { fc: FreqOrCode::Freq(0), dl: DadOrLen::Dad(0) }; 2 * 19 + 1];
static BORDER: &[u32] = &[
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];
static mut BYTES_IN: libc::off_t = 0;
static BYTES_OUT: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);
static mut CAUGHT_SIGNALS: libc::sigset_t = libc::sigset_t { __val: [0; 16] };
const COMPARISON_FUNCTION_TABLE: [Option<ComparisonFunction>; 2] = [
    None,
    Some(direntry_cmp_name),
];
static mut COMPRESSED_LEN: libc::off_t = 0;
static CONFIGURATION_TABLE: [Config; 10] = [
    Config { good_length: 0, max_lazy: 0, nice_length: 0, max_chain: 0 },
    Config { good_length: 4, max_lazy: 4, nice_length: 8, max_chain: 4 },
    Config { good_length: 4, max_lazy: 5, nice_length: 16, max_chain: 8 },
    Config { good_length: 4, max_lazy: 6, nice_length: 32, max_chain: 32 },
    Config { good_length: 4, max_lazy: 4, nice_length: 16, max_chain: 16 },
    Config { good_length: 8, max_lazy: 16, nice_length: 32, max_chain: 32 },
    Config { good_length: 8, max_lazy: 16, nice_length: 128, max_chain: 128 },
    Config { good_length: 8, max_lazy: 32, nice_length: 128, max_chain: 256 },
    Config { good_length: 32, max_lazy: 128, nice_length: 258, max_chain: 1024 },
    Config { good_length: 32, max_lazy: 258, nice_length: 258, max_chain: 4096 },
];
static CPDEXT: &[Ush] = &[
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6,
    7, 7, 8, 8, 9, 9, 10, 10, 11, 11,
    12, 12, 13, 13,
];
static CPDIST: &[Ush] = &[
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193,
    257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145,
    8193, 12289, 16385, 24577,
];
static CPLENS: [Ush; 31] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31,
    35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258, 0, 0,
];
static CPLEXT: [Ush; 31] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2,
    3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0, 99, 99,
];
static mut CRC: Ulg = 0xffffffff;
const CRC_32_TAB: [Ulg; 256] = [
    0x00000000, 0x77073096, 0xee0e612c, 0x990951ba, 0x076dc419,
    0x706af48f, 0xe963a535, 0x9e6495a3, 0x0edb8832, 0x79dcb8a4,
    0xe0d5e91e, 0x97d2d988, 0x09b64c2b, 0x7eb17cbd, 0xe7b82d07,
    0x90bf1d91, 0x1db71064, 0x6ab020f2, 0xf3b97148, 0x84be41de,
    0x1adad47d, 0x6ddde4eb, 0xf4d4b551, 0x83d385c7, 0x136c9856,
    0x646ba8c0, 0xfd62f97a, 0x8a65c9ec, 0x14015c4f, 0x63066cd9,
    0xfa0f3d63, 0x8d080df5, 0x3b6e20c8, 0x4c69105e, 0xd56041e4,
    0xa2677172, 0x3c03e4d1, 0x4b04d447, 0xd20d85fd, 0xa50ab56b,
    0x35b5a8fa, 0x42b2986c, 0xdbbbc9d6, 0xacbcf940, 0x32d86ce3,
    0x45df5c75, 0xdcd60dcf, 0xabd13d59, 0x26d930ac, 0x51de003a,
    0xc8d75180, 0xbfd06116, 0x21b4f4b5, 0x56b3c423, 0xcfba9599,
    0xb8bda50f, 0x2802b89e, 0x5f058808, 0xc60cd9b2, 0xb10be924,
    0x2f6f7c87, 0x58684c11, 0xc1611dab, 0xb6662d3d, 0x76dc4190,
    0x01db7106, 0x98d220bc, 0xefd5102a, 0x71b18589, 0x06b6b51f,
    0x9fbfe4a5, 0xe8b8d433, 0x7807c9a2, 0x0f00f934, 0x9609a88e,
    0xe10e9818, 0x7f6a0dbb, 0x086d3d2d, 0x91646c97, 0xe6635c01,
    0x6b6b51f4, 0x1c6c6162, 0x856530d8, 0xf262004e, 0x6c0695ed,
    0x1b01a57b, 0x8208f4c1, 0xf50fc457, 0x65b0d9c6, 0x12b7e950,
    0x8bbeb8ea, 0xfcb9887c, 0x62dd1ddf, 0x15da2d49, 0x8cd37cf3,
    0xfbd44c65, 0x4db26158, 0x3ab551ce, 0xa3bc0074, 0xd4bb30e2,
    0x4adfa541, 0x3dd895d7, 0xa4d1c46d, 0xd3d6f4fb, 0x4369e96a,
    0x346ed9fc, 0xad678846, 0xda60b8d0, 0x44042d73, 0x33031de5,
    0xaa0a4c5f, 0xdd0d7cc9, 0x5005713c, 0x270241aa, 0xbe0b1010,
    0xc90c2086, 0x5768b525, 0x206f85b3, 0xb966d409, 0xce61e49f,
    0x5edef90e, 0x29d9c998, 0xb0d09822, 0xc7d7a8b4, 0x59b33d17,
    0x2eb40d81, 0xb7bd5c3b, 0xc0ba6cad, 0xedb88320, 0x9abfb3b6,
    0x03b6e20c, 0x74b1d29a, 0xead54739, 0x9dd277af, 0x04db2615,
    0x73dc1683, 0xe3630b12, 0x94643b84, 0x0d6d6a3e, 0x7a6a5aa8,
    0xe40ecf0b, 0x9309ff9d, 0x0a00ae27, 0x7d079eb1, 0xf00f9344,
    0x8708a3d2, 0x1e01f268, 0x6906c2fe, 0xf762575d, 0x806567cb,
    0x196c3671, 0x6e6b06e7, 0xfed41b76, 0x89d32be0, 0x10da7a5a,
    0x67dd4acc, 0xf9b9df6f, 0x8ebeeff9, 0x17b7be43, 0x60b08ed5,
    0xd6d6a3e8, 0xa1d1937e, 0x38d8c2c4, 0x4fdff252, 0xd1bb67f1,
    0xa6bc5767, 0x3fb506dd, 0x48b2364b, 0xd80d2bda, 0xaf0a1b4c,
    0x36034af6, 0x41047a60, 0xdf60efc3, 0xa867df55, 0x316e8eef,
    0x4669be79, 0xcb61b38c, 0xbc66831a, 0x256fd2a0, 0x5268e236,
    0xcc0c7795, 0xbb0b4703, 0x220216b9, 0x5505262f, 0xc5ba3bbe,
    0xb2bd0b28, 0x2bb45a92, 0x5cb36a04, 0xc2d7ffa7, 0xb5d0cf31,
    0x2cd99e8b, 0x5bdeae1d, 0x9b64c2b0, 0xec63f226, 0x756aa39c,
    0x026d930a, 0x9c0906a9, 0xeb0e363f, 0x72076785, 0x05005713,
    0x95bf4a82, 0xe2b87a14, 0x7bb12bae, 0x0cb61b38, 0x92d28e9b,
    0xe5d5be0d, 0x7cdcefb7, 0x0bdbdf21, 0x86d3d2d4, 0xf1d4e242,
    0x68ddb3f8, 0x1fda836e, 0x81be16cd, 0xf6b9265b, 0x6fb077e1,
    0x18b74777, 0x88085ae6, 0xff0f6a70, 0x66063bca, 0x11010b5c,
    0x8f659eff, 0xf862ae69, 0x616bffd3, 0x166ccf45, 0xa00ae278,
    0xd70dd2ee, 0x4e048354, 0x3903b3c2, 0xa7672661, 0xd06016f7,
    0x4969474d, 0x3e6e77db, 0xaed16a4a, 0xd9d65adc, 0x40df0b66,
    0x37d83bf0, 0xa9bcae53, 0xdebb9ec5, 0x47b2cf7f, 0x30b5ffe9,
    0xbdbdf21c, 0xcabac28a, 0x53b39330, 0x24b4a3a6, 0xbad03605,
    0xcdd70693, 0x54de5729, 0x23d967bf, 0xb3667a2e, 0xc4614ab8,
    0x5d681b02, 0x2a6f2b94, 0xb40bbe37, 0xc30c8ea1, 0x5a05df1b,
    0x2d02ef8d
];
static DBITS: i32 = 6;
static mut DECOMPRESS: i32 = 0;
static mut DECRYPT: i32 = 0;
const DEPTH: [Uch; 2 * (256 + 1 + 29) + 1] = [0; 2 * (256 + 1 + 29) + 1];
static mut DFD: i32 = -1;
static mut DFNAME: [u8; 1024] = [0; 1024];
static mut DIST_CODE: [Uch; 512] = [0; 512];
static mut DONE: i32 = 0;
const DOT: char = '.';
static mut DYN_DTREE: [CtData; 2 * 30 + 1] = [CtData { fc: FreqOrCode::Freq(0), dl: DadOrLen::Dad(0) }; 2 * 30 + 1];
const DYN_LTREE:usize=0;
static mut D_BUF: [Ush; 0x8000] = [0; 0x8000];
static mut D_DESC: Option<TreeDesc> = None;
static mut ENV: Option<String> = None;
static mut EOFILE: i32 = 0;
static mut EXITING_SIGNAL: i32 = 0;
static mut EXIT_CODE: i32 = 0;
const EXIT_FAILURE: i32 = 1;
static EXTRA_BLBITS: [i32; 19] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 7];
static EXTRA_DBITS: [i32; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];
static EXTRA_LBITS: [i32; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
static mut EXT_HEADER: i32 = 0;
static mut FILE_METHOD: Option<*mut i32> = None;
static mut FILE_TYPE: Option<Box<Ush>> = None;
static mut FLAGS: Uch = 0;
static mut FLAG_BIT: Uch = 0;
static mut FLAG_BUF: [Uch; 0x8000 / 8] = [0; 0x8000 / 8];
static mut FORCE: i32 = 0;
static FOREGROUND: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static mut GOOD_MATCH: u32 = 0;
static HANDLED_SIG: &[i32] = &[2, 1, 13, 15, 24, 25];
static mut HEADER_BYTES: libc::off_t = 0;
static mut HEAP: [i32; 2 * (256 + 1 + 29) + 1] = [0; 2 * (256 + 1 + 29) + 1];
static mut HEAP_LEN: i32 = 0;
static mut HEAP_MAX: i32 = 0;
static mut HUFTS: u32 = 0;
static mut IFD: i32 = 0;
static mut IFILE_SIZE: libc::off_t = 0;
static mut IFNAME: [u8; 1024] = [0; 1024];
static mut INBUF: [Uch; 0x40000 + 64] = [0; 0x40000 + 64];
static mut INPTR: u32 = 0;
static mut INPUT_LEN: libc::off_t = 0;
static mut INSIZE: u32 = 0;
static mut INS_H: u32 = 0;
static mut ISTAT: libc::stat = libc::stat {
    st_dev: 0,
    st_ino: 0,
    st_mode: 0,
    st_nlink: 0,
    st_uid: 0,
    st_gid: 0,
    st_rdev: 0,
    st_size: 0,
    st_blksize: 0,
    st_blocks: 0,
    st_atime: 0,
    st_atime_nsec: 0,
    st_mtime: 0,
    st_mtime_nsec: 0,
    st_ctime: 0,
    st_ctime_nsec: 0,
};
static mut J: i32 = 0;
static mut KEEP: i32 = 0;
static mut LAST_DIST: u32 = 0;
static mut LAST_FLAGS: u32 = 0;
static mut LAST_LIT: u32 = 0;
static mut LAST_MEMBER: i32 = 0;
static LBITS: i32 = 9;
static mut LEAVES: [i32; 26] = [0; 26];
static LENGTH_CODE: [Uch; 256] = [0; 256];
static LEVEL: i32 = 6;
const LICENSE_MSG: &[&str] = &[
    "Copyright (C) 2018 Free Software Foundation, Inc.",
    "Copyright (C) 1993 Jean-loup Gailly.",
    "This is free software.  You may redistribute copies of it under the terms of",
    "the GNU General Public License <https://www.gnu.org/licenses/gpl.html>.",
    "There is NO WARRANTY, to the extent permitted by law.",
    "",
];
static mut LIST: i32 = 0;
static mut LITERAL: [Uch; 256] = [0; 256];
static mut LIT_BASE: [i32; 26] = [0; 26];
const LONGOPTS:usize=0;
static mut LOOKAHEAD: u32 = 0;
static mut LUTIMENSAT_WORKS_REALLY: i32 = 0;
static mut L_DESC: Option<TreeDesc> = None;
static MASK_BITS: &[Ush] = &[
    0x0000,
    0x0001, 0x0003, 0x0007, 0x000f, 0x001f, 0x003f, 0x007f, 0x00ff,
    0x01ff, 0x03ff, 0x07ff, 0x0fff, 0x1fff, 0x3fff, 0x7fff, 0xffff,
];
static mut MATCH_START: u32 = 0;
static mut MAXBITS: i32 = 16;
static mut MAX_CHAIN_LENGTH: u32 = 0;
static mut MAX_LAZY_MATCH: u32 = 0;
static mut MAX_LEN: i32 = 0;
static METHOD: i32 = 8;
static mut NICE_MATCH: i32 = 0;
static NO_NAME: i32 = -1;
static NO_TIME: i32 = -1;
static mut OFD: i32 = 0;
static mut OFNAME: [u8; 1024] = [0; 1024];
static mut OPT_LEN: Ulg = 0;
static mut ORIG_LEN: Ulg = 0;
static mut OUTBUF: [Uch; 0x40000 + 2048] = [0; 0x40000 + 2048];
static mut OUTCNT: u32 = 0;
static mut PARENTS: [i32; 26] = [0; 26];
static mut PART_NB: i32 = 0;
static mut PEEK_BITS: i32 = 0;
static mut PKZIP: i32 = 0;
static mut PRESUME_INPUT_TTY: bool = false;
static mut PREV: [Ush; 1 << 16] = [0; 1 << 16];
static mut PREV_LENGTH: u32 = 0;
static mut PROGRAM_NAME: Option<&'static str> = None;
static mut PT_LEN: [Uch; 1 << 5] = [0; 1 << 5];
static mut PT_TABLE: [Ush; 256] = [0; 256];
static mut QUIET: i32 = 0;
static mut READ_BUF: Option<fn(&mut [u8]) -> i32> = None;
static RECURSIVE: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static mut REMOVE_OFNAME: [u8; 1024] = [0; 1024];
static mut REMOVE_OFNAME_FD: i32 = -1;
static mut RSYNC: i32 = 0;
static mut RSYNC_CHUNK_END: Ulg = 0;
static mut RSYNC_SUM: Ulg = 0;
static mut SAVE_ORIG_NAME: i32 = 0;
const SHORTOPTS: &str = "ab:cdfhH?klLmMnNqrS:tvVZ123456789";
static mut STATIC_DTREE: [CtData; 30] = [CtData { fc: FreqOrCode::Freq(0), dl: DadOrLen::Dad(0) }; 30];
static mut STATIC_LEN: Ulg = 0;
static mut STATIC_LTREE: [CtData; (256 + 1 + 29) + 2] = [CtData { fc: FreqOrCode::Freq(0), dl: DadOrLen::Dad(0) }; (256 + 1 + 29) + 2];
static mut STDIN_WAS_READ: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static mut STRSTART: u32 = 0;
static mut SUBBITBUF: u32 = 0;
static mut SYNCHRONOUS: bool = false;
static mut TEST: i32 = 0;
static mut TIME_STAMP: std::time::Duration = std::time::Duration::new(0, 0);
static mut TOTAL_IN: libc::off_t = 0;
static mut TOTAL_OUT: libc::off_t = 0;
static TO_STDOUT: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static mut UNZIP_CRC: Ulg = 0;
static mut UTIMENSAT_WORKS_REALLY: i32 = 0;
static mut VALID: i32 = 0;
static mut VERBOSE: i32 = 0;
const VERSION: &str = "1.12";
const WINDOW: [Uch; 2 * 0x8000] = [0; 2 * 0x8000];
const WINDOW_SIZE: Ulg = 2 * 0x8000;
static WORK: fn(infile: i32, outfile: i32) -> i32 = std::iter::zip;
static mut ZFILE: FileT = 0;
static mut Z_LEN: usize = 0;
static Z_SUFFIX: Option<&'static str> = None;
fn __printf__(format: &str, args: &[ArgValue]) -> usize {
    // Open a file to write to, for example "output.txt"
    let mut file = match std::fs::File::create("output.txt") {
        Ok(f) => f,
        Err(_) => return 0, // Return 0 if the file cannot be created
    };

    match rpl_vfprintf(&mut file, format, args) {
        Ok(retval) => retval,
        Err(_) => 0,
    }
}
fn __xpg_strerror_r(errnum: i32, buf: &mut [u8]) -> Option<usize>{todo!("proto")}
fn _gl_alloc_nomem() -> Option<*mut ()> {
    None
}
fn _gl_utimens_windows(filename: &str, ts: [std::time::SystemTime; 2]) -> Result<(), ()>{todo!("proto")}
fn abort_gzip() -> ! {
    if let Err(_) = crate::remove_output_file(false) {
        // Handle the error if necessary, or ignore it
    }
    crate::do_exit(1);
}
fn abort_gzip_signal(sig: i32) -> ! {
    remove_output_file(true).ok();
    unsafe {
        if sig == EXITING_SIGNAL {
            std::process::exit(2);
        }
    }
    unsafe {
        libc::signal(sig, libc::SIG_DFL);
        libc::raise(sig);
    }
    unreachable!();
}
fn add_envopt(argcp: &mut usize, argvp: &mut Vec<String>, envvar_name: &str) -> Option<String> {
    let env_val = std::env::var(envvar_name).ok()?;
    let mut env_val = env_val.clone(); // Assuming xstrdup is a clone operation for strings
    let mut nargc = 0;
    let mut p = 0;

    while p < env_val.len() {
        p += env_val[p..].chars().take_while(|&c| c == ' ' || c == '\t').count();
        if p >= env_val.len() {
            break;
        }
        nargc += 1;
        p += env_val[p..].chars().take_while(|&c| c != ' ' && c != '\t').count();
    }

    if nargc == 0 {
        return None;
    }

    *argcp = nargc + 1;
    let mut nargv = Vec::with_capacity(*argcp + 1);
    let oargv = std::mem::take(argvp);
    nargv.push(oargv[0].clone());

    let mut p = 0;
    while nargc > 0 {
        p += env_val[p..].chars().take_while(|&c| c == ' ' || c == '\t').count();
        let start = p;
        p += env_val[p..].chars().take_while(|&c| c != ' ' && c != '\t').count();
        nargv.push(env_val[start..p].to_string());
        nargc -= 1;
    }

    *argvp = nargv;
    Some(env_val)
}
fn aligned_alloc(alignment: usize, size: usize) -> *mut u8{todo!("proto")}
fn asnprintf<'a>(resultbuf: &'a mut String, lengthp: &'a mut usize, format: &'a str, args: &'a [ArgValue]) -> Result<&'a str, ()> {
    vasnprintf(resultbuf, lengthp, format, args)
}
fn at_func2<F>(fd1: i32, file1: &str, fd2: i32, file2: &str, func: F) -> i32 where F: Fn(&str, &str) -> i32{todo!("proto")}
fn atdir_eq(dir: Option<&[u8]>) -> bool {
    let dir = match dir {
        Some(d) if !d.is_empty() => d,
        _ => &[DOT as u8],
    };
    let dirlen = dir.len();
    unsafe {
        std::ptr::eq(&DFNAME[..dirlen], dir) && DFNAME.get(dirlen).copied() == Some(0)
    }
}
fn atdir_set(dir: Option<&[u8]>) -> Result<i32, ()> {
    const TRY_OPENING_DIRECTORIES: bool = true;
    if TRY_OPENING_DIRECTORIES && !atdir_eq(dir) {
        unsafe {
            if DFD >= 0 {
                libc::close(DFD);
            }
        }
        let dir = match dir {
            Some(d) if !d.is_empty() => d,
            _ => DOT.to_string().as_bytes(),
        };
        let dirlen = dir.len();
        unsafe {
            std::ptr::copy_nonoverlapping(dir.as_ptr(), DFNAME.as_mut_ptr(), dirlen);
            DFNAME[dirlen] = b'\0';
            DFD = open_safer(std::str::from_utf8_unchecked(&DFNAME[..dirlen]), 0o00 | 0o200000)?;
        }
    }
    unsafe { Ok(DFD) }
}
fn base_len(name: &str) -> usize {
    let mut len = name.len();
    let prefix_len = 0;
    while len > 1 && name.as_bytes()[len - 1] == b'/' {
        len -= 1;
    }
    if false && len == 1 && name.starts_with("//") && name.len() == 2 {
        return 2;
    }
    if false && prefix_len != 0 && len == prefix_len && name.as_bytes()[prefix_len] == b'/' {
        return prefix_len + 1;
    }
    len
}
fn bi_init(zipfile: FileT) {
    unsafe {
        ZFILE = zipfile;
        BI_BUF = 0;
        BI_VALID = 0;
        if ZFILE != -1 {
            READ_BUF = Some(file_read);
        }
    }
}
fn bi_reverse(code: u32, len: usize) -> u32 {
    let mut res = 0;
    let mut code = code;
    let mut len = len;
    while len > 0 {
        res |= code & 1;
        code >>= 1;
        res <<= 1;
        len -= 1;
    }
    res >> 1
}
fn bi_windup() -> Result<(), ()> {
    unsafe {
        if BI_VALID > 8 {
            if OUTCNT < 0x40000 - 2 {
                OUTBUF[OUTCNT as usize] = (BI_BUF & 0xff) as Uch;
                OUTCNT += 1;
                OUTBUF[OUTCNT as usize] = ((BI_BUF >> 8) & 0xff) as Uch;
                OUTCNT += 1;
            } else {
                OUTBUF[OUTCNT as usize] = (BI_BUF & 0xff) as Uch;
                OUTCNT += 1;
                if OUTCNT == 0x40000 {
                    flush_outbuf()?;
                }
                OUTBUF[OUTCNT as usize] = ((BI_BUF >> 8) & 0xff) as Uch;
                OUTCNT += 1;
                if OUTCNT == 0x40000 {
                    flush_outbuf()?;
                }
            }
        } else if BI_VALID > 0 {
            OUTBUF[OUTCNT as usize] = (BI_BUF & 0xff) as Uch;
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf()?;
            }
        }
        BI_BUF = 0;
        BI_VALID = 0;
    }
    Ok(())
}
fn build_bl_tree() -> usize {
    let mut max_blindex: usize;
    unsafe {
        scan_tree(&mut DYN_DTREE, D_DESC.as_ref().unwrap().max_code as usize);
        scan_tree(&mut DYN_DTREE, L_DESC.as_ref().unwrap().max_code as usize);
        build_tree(BL_DESC.as_mut().unwrap());
        for i in (3..19).rev() {
            if BL_TREE[BL_ORDER[i] as usize].dl.Len() != 0 {
                max_blindex = i;
                break;
            }
        }
        OPT_LEN += 3 * (max_blindex + 1) as u64 + 5 + 5 + 4;
    }
    max_blindex
}
fn build_tree(desc: &mut TreeDesc) {
    let elems = desc.elems as usize;
    let mut max_code = -1;
    let mut node = elems;
    unsafe {
        HEAP_LEN = 0;
        HEAP_MAX = 2 * (256 + 1 + 29) + 1;
    }
    for n in 0..elems {
        if let FreqOrCode::Freq(freq) = desc.dyn_tree[n].fc {
            if freq != 0 {
                unsafe {
                    HEAP[HEAP_LEN as usize + 1] = n as i32;
                    HEAP_LEN += 1;
                }
                max_code = n as i32;
                unsafe {
                    DEPTH[n] = 0;
                }
            } else {
                desc.dyn_tree[n].dl = DadOrLen::Len(0);
            }
        }
    }
    while unsafe { HEAP_LEN < 2 } {
        let new = if max_code < 2 {
            max_code += 1;
            max_code
        } else {
            0
        };
        unsafe {
            HEAP[HEAP_LEN as usize + 1] = new;
            HEAP_LEN += 1;
        }
        desc.dyn_tree[new as usize].fc = FreqOrCode::Freq(1);
        unsafe {
            DEPTH[new as usize] = 0;
            OPT_LEN -= 1;
            if let Some(stree) = desc.static_tree {
                STATIC_LEN -= match stree[new as usize].dl {
                    DadOrLen::Len(len) => len as u64,
                    _ => 0,
                };
            }
        }
    }
    desc.max_code = max_code;
    for n in (1..=unsafe { HEAP_LEN / 2 }).rev() {
        pqdownheap(&mut desc.dyn_tree, n as usize);
    }
    while unsafe { HEAP_LEN >= 2 } {
        let n = unsafe { HEAP[1] };
        unsafe {
            HEAP[1] = HEAP[HEAP_LEN as usize];
            HEAP_LEN -= 1;
        }
        pqdownheap(&mut desc.dyn_tree, 1);
        let m = unsafe { HEAP[1] };
        unsafe {
            HEAP[HEAP_MAX as usize - 1] = n;
            HEAP[HEAP_MAX as usize - 2] = m;
            HEAP_MAX -= 2;
        }
        desc.dyn_tree[node].fc = FreqOrCode::Freq(
            if let FreqOrCode::Freq(freq_n) = desc.dyn_tree[n as usize].fc {
                if let FreqOrCode::Freq(freq_m) = desc.dyn_tree[m as usize].fc {
                    freq_n + freq_m
                } else {
                    0
                }
            } else {
                0
            },
        );
        unsafe {
            DEPTH[node] = (DEPTH[n as usize].max(DEPTH[m as usize]) + 1) as Uch;
        }
        desc.dyn_tree[n as usize].dl = DadOrLen::Dad(node as Ush);
        desc.dyn_tree[m as usize].dl = DadOrLen::Dad(node as Ush);
        unsafe {
            HEAP[1] = node as i32;
        }
        node += 1;
        pqdownheap(&mut desc.dyn_tree, 1);
    }
    unsafe {
        HEAP[HEAP_MAX as usize - 1] = HEAP[1];
        HEAP_MAX -= 1;
    }
    gen_bitlen(desc);
    gen_codes(&mut desc.dyn_tree, max_code as usize);
}
fn calloc<T>(nmemb: usize) -> Box<[T]>{todo!("proto")}
fn canonicalize_file_name(name: &str) -> Option<String>{todo!("proto")}
fn cdb_advance_fd(cdb: &mut CdBuf, dir: &str) -> Result<(), ()> {
    let new_fd = match std::os::unix::fs::OpenOptionsExt::open(
        &std::fs::OpenOptions::new(),
        cdb.fd,
        dir,
        0o200400,
    ) {
        Ok(fd) => fd,
        Err(_) => return Err(()),
    };
    cdb_free(cdb);
    cdb.fd = new_fd;
    Ok(())
}
fn cdb_fchdir(cdb: &CdBuf) -> i32 {
    std::os::unix::io::RawFd::from(cdb.fd);
    unsafe { libc::fchdir(cdb.fd) }
}
fn cdb_free(cdb: &CdBuf) {
    use std::convert::TryInto;
    use std::os::fd::FromRawFd;
    if cdb.fd >= 0 {
        let close_fail = std::os::unix::io::RawFd::from(cdb.fd).try_into().map_or(true, |fd| {
            // Wrap the unsafe call in an unsafe block
            unsafe { std::fs::File::from_raw_fd(fd) }.sync_all().is_err()
        });
        assert!(!close_fail, "! close_fail");
    }
}
fn cdb_init(cdb: &mut CdBuf) {
    cdb.fd = -100;
}
fn chdir_long(mut dir: &str) -> Result<(), ()> {
    let e = std::env::set_current_dir(dir);
    unsafe {
        if e.is_ok() || *libc::__errno_location() != 36 {
            return e.map_err(|_| ());
        }
    }
    let len = dir.len();
    let dir_end = dir.as_ptr().wrapping_add(len);
    let mut cdb = CdBuf { fd: 0 };
    let n_leading_slash = dir.chars().take_while(|&c| c == '/').count();
    cdb_init(&mut cdb);
    assert!(0 < len);
    assert!(4096 <= len);
    if n_leading_slash == 2 {
        let slash = dir[3..].find('/').map(|i| i + 3);
        if let Some(slash) = slash {
            let mut dir_bytes = dir.as_bytes().to_vec();
            dir_bytes[slash] = 0;
            let err = cdb_advance_fd(&mut cdb, std::str::from_utf8(&dir_bytes[..slash]).unwrap());
            dir_bytes[slash] = b'/';
            if err.is_err() {
                return Err(());
            }
            dir = find_non_slash(&dir[slash + 1..]);
        } else {
            unsafe {
                *libc::__errno_location() = 36;
            }
            return Err(());
        }
    } else if n_leading_slash > 0 {
        if cdb_advance_fd(&mut cdb, "/").is_err() {
            return Err(());
        }
        dir = &dir[n_leading_slash..];
    }
    assert!(dir.chars().next() != Some('/'));
    assert!(dir.as_ptr() <= dir_end);
    while unsafe { 4096 <= dir_end.offset_from(dir.as_ptr()) as usize } {
        let slash = dir[..4096].rfind('/');
        if let Some(slash) = slash {
            let mut dir_bytes = dir.as_bytes().to_vec();
            dir_bytes[slash] = 0;
            assert!(slash < 4096);
            let err = cdb_advance_fd(&mut cdb, std::str::from_utf8(&dir_bytes[..slash]).unwrap());
            dir_bytes[slash] = b'/';
            if err.is_err() {
                return Err(());
            }
            dir = find_non_slash(&dir[slash + 1..]);
        } else {
            unsafe {
                *libc::__errno_location() = 36;
            }
            return Err(());
        }
    }
    if dir.as_ptr() < dir_end {
        if cdb_advance_fd(&mut cdb, dir).is_err() {
            return Err(());
        }
    }
    if cdb_fchdir(&cdb) != 0 {
        return Err(());
    }
    cdb_free(&cdb);
    Ok(())
}
fn check_ofname() -> i32 {
    use std::io::{stderr, Write};
    use std::sync::atomic::Ordering;

    unsafe {
        if FORCE == 0 {
            let mut ok = false;
            // Assuming `rpl_fprintf` and `rpl_fflush` are custom functions that need a `File`.
            // You need to open a file or change the function to accept `Stderr`.
            // For demonstration, let's assume you want to print to stderr using standard methods.
            let stderr = &mut stderr();
            writeln!(stderr, "{}: {} already exists;", PROGRAM_NAME.unwrap_or(""), std::str::from_utf8(&OFNAME).unwrap_or("")).unwrap();
            if FOREGROUND.load(Ordering::SeqCst) != 0 && (PRESUME_INPUT_TTY || libc::isatty(0) != 0) {
                write!(stderr, " do you wish to overwrite (y or n)? ").unwrap();
                stderr.flush().unwrap();
                ok = yesno().unwrap_or(false);
            }
            if !ok {
                writeln!(stderr, "\tnot overwritten").unwrap();
                if EXIT_CODE == 0 {
                    EXIT_CODE = 2;
                }
                return 1;
            }
        }
        if xunlink(std::str::from_utf8(&OFNAME).unwrap_or("")).is_err() {
            progerror(std::str::from_utf8(&OFNAME).unwrap_or(""));
            return 1;
        }
    }
    0
}
fn check_zipfile(in_data: i32) -> Result<(), ()> {
    let h = unsafe { &INBUF[INPTR as usize..] };
    unsafe { IFD = in_data };
    let extra_len = (h[26] as u16 | ((h[27] as u16) << 8)) as u32;
    let comment_len = (h[28] as u16 | ((h[29] as u16) << 8)) as u32;
    unsafe { INPTR += 30 + extra_len + comment_len };
    let signature = (h[0] as u64 | ((h[1] as u64) << 8) | ((h[2] as u64) << 16) | ((h[3] as u64) << 24)) as u64;
    if unsafe { INPTR > INSIZE } || signature != 0x04034b50 {
        unsafe {
            rpl_fprintf(
                &mut std::fs::File::create("/dev/stderr").unwrap(),
                "\n{}: {}: not a valid zip file\n",
                &[ArgValue::Str(PROGRAM_NAME.unwrap_or("")), ArgValue::Str(std::str::from_utf8_unchecked(&IFNAME))],
            )?;
            EXIT_CODE = 1;
        }
        return Err(());
    }
    unsafe { METHOD = h[8] as i32 };
    if unsafe { METHOD != 0 && METHOD != 8 } {
        unsafe {
            rpl_fprintf(
                &mut std::fs::File::create("/dev/stderr").unwrap(),
                "\n{}: {}: first entry not deflated or stored -- use unzip\n",
                &[ArgValue::Str(PROGRAM_NAME.unwrap_or("")), ArgValue::Str(std::str::from_utf8_unchecked(&IFNAME))],
            )?;
            EXIT_CODE = 1;
        }
        return Err(());
    }
    unsafe {
        DECRYPT = (h[6] & 1) as i32;
        if DECRYPT != 0 {
            rpl_fprintf(
                &mut std::fs::File::create("/dev/stderr").unwrap(),
                "\n{}: {}: encrypted file -- use unzip\n",
                &[ArgValue::Str(PROGRAM_NAME.unwrap_or("")), ArgValue::Str(std::str::from_utf8_unchecked(&IFNAME))],
            )?;
            EXIT_CODE = 1;
            return Err(());
        }
    }
    unsafe {
        EXT_HEADER = if (h[6] & 8) != 0 { 1 } else { 0 };
        PKZIP = 1;
    }
    Ok(())
}
fn clear_bufs() {
    unsafe {
        OUTCNT = 0;
        INSIZE = 0;
        INPTR = 0;
        BYTES_IN = 0;
    }
    std::sync::atomic::AtomicI64::store(&BYTES_OUT, 0, std::sync::atomic::Ordering::SeqCst);
}
fn clear_ungetc_buffer_preserving_position(fp: &mut std::fs::File) -> Result<(), ()> {
    use std::os::unix::io::AsRawFd;
    use std::os::unix::fs::FileExt;
    use std::io::SeekFrom;

    let fd = fp.as_raw_fd();
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
    if flags & 0x100 != 0 {
        rpl_fseeko(fp, 0, std::io::SeekFrom::Current(0))
    } else {
        Ok(())
    }
}
fn compress_block(ltree: &mut [CtData], dtree: &mut [CtData]) -> Result<(), ()> {
    let mut dist: u32;
    let mut lc: i32;
    let mut lx: usize = 0;
    let mut dx: usize = 0;
    let mut fx: usize = 0;
    let mut flag: Uch = 0;
    let mut code: u32;
    let mut extra: i32;

    while lx < unsafe { LAST_LIT as usize } {
        if (lx & 7) == 0 {
            flag = unsafe { FLAG_BUF[fx] };
            fx += 1;
        }
        lc = unsafe { INBUF[lx] as i32 };
        lx += 1;
        if (flag & 1) == 0 {
            send_bits(match ltree[lc as usize].fc {
                FreqOrCode::Code(c) => c as i32,
                _ => return Err(()),
            }, match ltree[lc as usize].dl {
                DadOrLen::Len(l) => l as i32,
                _ => return Err(()),
            })?;
        } else {
            code = unsafe { LENGTH_CODE[lc as usize] as u32 };
            send_bits(match ltree[(code + 256 + 1) as usize].fc {
                FreqOrCode::Code(c) => c as i32,
                _ => return Err(()),
            }, match ltree[(code + 256 + 1) as usize].dl {
                DadOrLen::Len(l) => l as i32,
                _ => return Err(()),
            })?;
            extra = unsafe { EXTRA_LBITS[code as usize] };
            if extra != 0 {
                lc -= unsafe { BASE_LENGTH[code as usize] };
                send_bits(lc, extra)?;
            }
            dist = unsafe { D_BUF[dx] as u32 };
            dx += 1;
            code = if dist < 256 {
                unsafe { DIST_CODE[dist as usize] as u32 }
            } else {
                unsafe { DIST_CODE[256 + (dist >> 7) as usize] as u32 }
            };
            send_bits(match dtree[code as usize].fc {
                FreqOrCode::Code(c) => c as i32,
                _ => return Err(()),
            }, match dtree[code as usize].dl {
                DadOrLen::Len(l) => l as i32,
                _ => return Err(()),
            })?;
            extra = unsafe { EXTRA_DBITS[code as usize] };
            if extra != 0 {
                dist -= unsafe { BASE_DIST[code as usize] as u32 };
                send_bits(dist as i32, extra)?;
            }
        }
        flag >>= 1;
    }
    send_bits(match ltree[256].fc {
        FreqOrCode::Code(c) => c as i32,
        _ => return Err(()),
    }, match ltree[256].dl {
        DadOrLen::Len(l) => l as i32,
        _ => return Err(()),
    })?;
    Ok(())
}
fn copy(in_data: i32, out: i32) -> Result<(), ()> {
    let mut got: usize;
    unsafe {
        *libc::__errno_location() = 0;
        while INSIZE > INPTR {
            write_buf(out, &mut INBUF[INPTR as usize..INSIZE as usize])?;
            got = read_buffer(in_data, &mut INBUF[..0x40000])?;
            if got == -1_isize as usize {
                read_error();
            }
            BYTES_IN += got as libc::off_t;
            INSIZE = got as u32;
            INPTR = 0;
        }
    }
    Ok(())
}
fn copy_block(buf: &[u8], header: bool) -> Result<(), ()> {
    bi_windup()?;
    let mut len = buf.len() as u32;
    if header {
        let len_u16 = len as Ush;
        let nlen_u16 = !len_u16;
        if unsafe { OUTCNT } < 0x40000 - 2 {
            unsafe {
                OUTBUF[OUTCNT as usize] = (len_u16 & 0xff) as Uch;
                OUTCNT += 1;
                OUTBUF[OUTCNT as usize] = (len_u16 >> 8) as Uch;
                OUTCNT += 1;
                OUTBUF[OUTCNT as usize] = (nlen_u16 & 0xff) as Uch;
                OUTCNT += 1;
                OUTBUF[OUTCNT as usize] = (nlen_u16 >> 8) as Uch;
                OUTCNT += 1;
            }
        } else {
            unsafe {
                OUTBUF[OUTCNT as usize] = (len_u16 & 0xff) as Uch;
                OUTCNT += 1;
                if OUTCNT == 0x40000 {
                    flush_outbuf()?;
                }
                OUTBUF[OUTCNT as usize] = (len_u16 >> 8) as Uch;
                OUTCNT += 1;
                if OUTCNT == 0x40000 {
                    flush_outbuf()?;
                }
                OUTBUF[OUTCNT as usize] = (nlen_u16 & 0xff) as Uch;
                OUTCNT += 1;
                if OUTCNT == 0x40000 {
                    flush_outbuf()?;
                }
                OUTBUF[OUTCNT as usize] = (nlen_u16 >> 8) as Uch;
                OUTCNT += 1;
                if OUTCNT == 0x40000 {
                    flush_outbuf()?;
                }
            }
        }
    }
    for &byte in buf {
        unsafe {
            OUTBUF[OUTCNT as usize] = byte;
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf()?;
            }
        }
    }
    Ok(())
}
fn copy_stat(ifstat: &std::fs::Metadata) -> Result<(), ()> {
    use std::os::unix::fs::MetadataExt;
    use std::os::unix::fs::PermissionsExt;
    let mode = ifstat.permissions().mode() & ((0o400 | 0o200 | 0o100) | ((0o400 | 0o200 | 0o100) >> 3) | (((0o400 | 0o200 | 0o100) >> 3) >> 3));
    let mut timespec = [libc::timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    timespec[0] = get_stat_atime(ifstat).into();
    timespec[1] = get_stat_mtime(ifstat).into();
    let restoring = unsafe {
        DECOMPRESS != 0 && TIME_STAMP.subsec_nanos() as i32 >= 0
            && !(timespec[1].tv_sec == TIME_STAMP.as_secs() as i64
                && timespec[1].tv_nsec == TIME_STAMP.subsec_nanos() as i64)
    };
    if restoring {
        timespec[1] = libc::timespec {
            tv_sec: TIME_STAMP.as_secs() as i64,
            tv_nsec: TIME_STAMP.subsec_nanos() as i64,
        };
    }
    if fdutimens(unsafe { OFD }, Some(unsafe { std::str::from_utf8_unchecked(&OFNAME) }), Some(&timespec)).is_ok() {
        if restoring && unsafe { VERBOSE } > 1 {
            rpl_fprintf(&mut std::fs::File::create("/dev/stderr").unwrap(), "%s: timestamp restored\n", &[ArgValue::String(unsafe { std::str::from_utf8_unchecked(&OFNAME).to_string() })]).ok();
        }
    } else {
        let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
        if unsafe { QUIET == 0 } {
            rpl_fprintf(&mut std::fs::File::create("/dev/stderr").unwrap(), "%s: ", &[ArgValue::String(unsafe { PROGRAM_NAME.unwrap_or("").to_string() })]).ok();
            if unsafe { EXIT_CODE } == 0 {
                unsafe { EXIT_CODE = 2; }
            }
        }
        if unsafe { QUIET == 0 } {
            eprintln!("{}", std::io::Error::from_raw_os_error(e));
        }
    }
    do_chown(unsafe { OFD }, unsafe { std::str::from_utf8_unchecked(&OFNAME) }, u32::MAX, ifstat.gid()).ok();
    let r = unsafe { libc::fchmod(OFD, mode) };
    if r != 0 {
        let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
        if unsafe { QUIET == 0 } {
            rpl_fprintf(&mut std::fs::File::create("/dev/stderr").unwrap(), "%s: ", &[ArgValue::String(unsafe { PROGRAM_NAME.unwrap_or("").to_string() })]).ok();
            if unsafe { EXIT_CODE } == 0 {
                unsafe { EXIT_CODE = 2; }
            }
        }
        if unsafe { QUIET == 0 } {
            eprintln!("{}", std::io::Error::from_raw_os_error(e));
        }
    }
    do_chown(unsafe { OFD }, unsafe { std::str::from_utf8_unchecked(&OFNAME) }, ifstat.uid(), u32::MAX).ok();
    Ok(())
}
fn creat_safer(file: &str, mode: u32) -> Result<i32, ()> {
    use std::os::fd::IntoRawFd;
    use std::os::unix::fs::OpenOptionsExt;
    let fd = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .mode(mode)
        .open(file)
        .map(|file| file.into_raw_fd())
        .map_err(|_| ())?;
    fd_safer(fd)
}
fn create_outfile() -> i32 {
    let mut name_shortened = false;
    let flags = 0o1 | 0o100 | 0o200 | (unsafe { if ASCII != 0 && DECOMPRESS != 0 { 0 } else { 0 } });
    let mut base: &[u8] = unsafe { &OFNAME };
    let mut atfd = -100;
    if unsafe { KEEP == 0 } {
        if let Some(b) = last_component(std::str::from_utf8(&OFNAME).unwrap()) {
            if let Ok(f) = atdir_set(Some(&OFNAME[..b.as_ptr() as usize - OFNAME.as_ptr() as usize])) {
                base = b.as_bytes();
                atfd = f;
            }
        }
    }
    loop {
        let open_errno;
        let mut oldset = unsafe { std::mem::zeroed() };
        unsafe { volatile_strcpy(&mut REMOVE_OFNAME, &OFNAME) };
        unsafe { libc::sigprocmask(0, &CAUGHT_SIGNALS, &mut oldset) };
        unsafe {
            REMOVE_OFNAME_FD = OFD = match openat_safer(atfd, std::str::from_utf8(base).unwrap(), flags) {
                Ok(fd) => fd,
                Err(_) => -1,
            };
        }
        open_errno = unsafe { *libc::__errno_location() };
        unsafe { libc::sigprocmask(2, &oldset, std::ptr::null_mut()) };
        if unsafe { OFD >= 0 } {
            break;
        }
        match open_errno {
            36 => {
                shorten_name(&mut String::from_utf8_lossy(&mut OFNAME).into_owned());
                name_shortened = true;
            }
            17 => {
                if check_ofname() != 0 {
                    unsafe { libc::close(IFD) };
                    return 1;
                }
            }
            _ => {
                progerror(std::str::from_utf8(&OFNAME).unwrap());
                unsafe { libc::close(IFD) };
                return 1;
            }
        }
    }
    if name_shortened && unsafe { DECOMPRESS != 0 } {
        if unsafe { QUIET == 0 } {
            // Assuming ArgValue has a variant `String` for string arguments
            rpl_fprintf(
                &mut std::fs::File::create("/dev/stderr").unwrap(), // Create a File for stderr
                "%s: %s: warning, name truncated\n",
                &[ArgValue::String(unsafe { PROGRAM_NAME.unwrap_or("").to_string() }), ArgValue::String(std::str::from_utf8(&OFNAME).unwrap().to_string())],
            )
            .ok();
            if unsafe { EXIT_CODE == 0 } {
                unsafe { EXIT_CODE = 2 };
            }
        }
    }
    0
}
fn ct_init(attr: Option<Box<Ush>>, methodp: Option<*mut i32>) {
    let mut n: i32;
    let mut bits: i32;
    let mut length: i32;
    let mut code: i32 = 0;
    let mut dist: i32;
    unsafe {
        FILE_TYPE = attr;
        FILE_METHOD = methodp;
        COMPRESSED_LEN = 0;
        INPUT_LEN = 0;
        if let DadOrLen::Len(len) = STATIC_DTREE[0].dl {
            if len != 0 {
                return;
            }
        }
        length = 0;
        for code in 0..28 {
            BASE_LENGTH[code as usize] = length;
            for _ in 0..(1 << EXTRA_LBITS[code as usize]) {
                LENGTH_CODE[length as usize] = code as Uch;
                length += 1;
            }
        }
        LENGTH_CODE[(length - 1) as usize] = code as Uch;
        dist = 0;
        for code in 0..16 {
            BASE_DIST[code as usize] = dist;
            for _ in 0..(1 << EXTRA_DBITS[code as usize]) {
                DIST_CODE[dist as usize] = code as Uch;
                dist += 1;
            }
        }
        dist >>= 7;
        for code in 16..30 {
            BASE_DIST[code as usize] = dist << 7;
            for _ in 0..(1 << (EXTRA_DBITS[code as usize] - 7)) {
                DIST_CODE[(256 + dist) as usize] = code as Uch;
                dist += 1;
            }
        }
        for bits in 0..=15 {
            BL_COUNT[bits as usize] = 0;
        }
        n = 0;
        while n <= 143 {
            STATIC_LTREE[n as usize].dl = DadOrLen::Len(8);
            BL_COUNT[8] += 1;
            n += 1;
        }
        while n <= 255 {
            STATIC_LTREE[n as usize].dl = DadOrLen::Len(9);
            BL_COUNT[9] += 1;
            n += 1;
        }
        while n <= 279 {
            STATIC_LTREE[n as usize].dl = DadOrLen::Len(7);
            BL_COUNT[7] += 1;
            n += 1;
        }
        while n <= 287 {
            STATIC_LTREE[n as usize].dl = DadOrLen::Len(8);
            BL_COUNT[8] += 1;
            n += 1;
        }
        gen_codes(&mut STATIC_LTREE, 256 + 1 + 29 + 1);
        for n in 0..30 {
            STATIC_DTREE[n].dl = DadOrLen::Len(5);
            STATIC_DTREE[n].fc = FreqOrCode::Code(bi_reverse(n as u32, 5) as Ush);
        }
        init_block();
    }
}
fn ct_tally(dist: i32, lc: i32) -> bool {
    unsafe {
        INBUF[LAST_LIT as usize] = lc as Uch;
        LAST_LIT += 1;
        if dist == 0 {
            if let FreqOrCode::Freq(ref mut freq) = DYN_LTREE[lc].fc {
                *freq += 1;
            }
        } else {
            let dist = dist - 1;
            if let FreqOrCode::Freq(ref mut freq) = DYN_LTREE[LENGTH_CODE[lc as usize] as usize + 256 + 1].fc {
                *freq += 1;
            }
            let dcode = if dist < 256 {
                DIST_CODE[dist as usize]
            } else {
                DIST_CODE[256 + (dist >> 7) as usize]
            };
            if let FreqOrCode::Freq(ref mut freq) = DYN_DTREE[dcode as usize].fc {
                *freq += 1;
            }
            D_BUF[LAST_DIST as usize] = dist as Ush;
            LAST_DIST += 1;
            FLAGS |= FLAG_BIT;
        }
        FLAG_BIT <<= 1;
        if (LAST_LIT & 7) == 0 {
            FLAG_BUF[LAST_FLAGS as usize] = FLAGS;
            LAST_FLAGS += 1;
            FLAGS = 0;
            FLAG_BIT = 1;
        }
        if LEVEL > 2 && (LAST_LIT & 0xfff) == 0 {
            let mut out_length: Ulg = (LAST_LIT as Ulg) * 8;
            let in_length: Ulg = STRSTART as Ulg - BLOCK_START as Ulg;
            for dcode in 0..30 {
                if let FreqOrCode::Freq(freq) = DYN_DTREE[dcode].fc {
                    out_length += (freq as Ulg) * (5 + EXTRA_DBITS[dcode] as Ulg);
                }
            }
            out_length >>= 3;
            if LAST_DIST < LAST_LIT / 2 && out_length < in_length / 2 {
                return true;
            }
        }
        LAST_LIT == 0x8000 - 1 || LAST_DIST == 0x8000
    }
}
fn current_timespec() -> libc::timespec {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    ts = gettime();
    ts
}
fn dcnpgettext_expr(domain: &str, msgctxt: &str, msgid: &str, msgid_plural: &str, n: u64, category: i32) -> String {
    let msgctxt_len = msgctxt.len() + 1;
    let msgid_len = msgid.len() + 1;
    let mut buf = [0u8; 1024];
    let msg_ctxt_id = if msgctxt_len + msgid_len <= buf.len() {
        &mut buf[..msgctxt_len + msgid_len]
    } else {
        let size = msgctxt_len + msgid_len;
        let ptr = unsafe { malloc(size) };
        if ptr.is_null() {
            return if n == 1 { msgid.to_string() } else { msgid_plural.to_string() };
        }
        unsafe { std::slice::from_raw_parts_mut(ptr, size) }
    };

    msg_ctxt_id[..msgctxt_len - 1].copy_from_slice(msgctxt.as_bytes());
    msg_ctxt_id[msgctxt_len - 1] = b'\x04';
    msg_ctxt_id[msgctxt_len..msgctxt_len + msgid_len].copy_from_slice(msgid.as_bytes());

    let translation = if n == 1 {
        msgid
    } else {
        msgid_plural
    };

    let found_translation = translation != std::str::from_utf8(&msg_ctxt_id[..msgctxt_len + msgid_len]).unwrap();

    if msg_ctxt_id.as_ptr() != buf.as_ptr() {
        unsafe { rpl_free(msg_ctxt_id.as_mut_ptr() as *mut std::ffi::c_void) };
    }

    if found_translation {
        translation.to_string()
    } else {
        if n == 1 { msgid.to_string() } else { msgid_plural.to_string() }
    }
}
fn dcpgettext_expr(domain: Option<&str>, msgctxt: &str, msgid: &str, category: i32) -> String {
    let msgctxt_len = msgctxt.len() + 1;
    let msgid_len = msgid.len() + 1;
    let mut buf = [0u8; 1024];
    let msg_ctxt_id: &mut [u8];
    let mut allocated = false;

    if msgctxt_len + msgid_len <= buf.len() {
        msg_ctxt_id = &mut buf[..msgctxt_len + msgid_len];
    } else {
        let ptr = unsafe { malloc(msgctxt_len + msgid_len) };
        if ptr.is_null() {
            return msgid.to_string();
        }
        msg_ctxt_id = unsafe { std::slice::from_raw_parts_mut(ptr, msgctxt_len + msgid_len) };
        allocated = true;
    }

    msg_ctxt_id[..msgctxt_len - 1].copy_from_slice(msgctxt.as_bytes());
    msg_ctxt_id[msgctxt_len - 1] = b'\x04';
    msg_ctxt_id[msgctxt_len..msgctxt_len + msgid_len - 1].copy_from_slice(msgid.as_bytes());
    msg_ctxt_id[msgctxt_len + msgid_len - 1] = 0;

    let translation = {
        let _ = category;
        let _ = domain;
        std::str::from_utf8(msg_ctxt_id).unwrap_or(msgid)
    };

    let found_translation = translation != msgid;

    // Ensure the immutable borrow is no longer used before the mutable borrow
    let result = if found_translation {
        translation.to_string()
    } else {
        msgid.to_string()
    };

    if allocated {
        unsafe { rpl_free(msg_ctxt_id.as_mut_ptr() as *mut libc::c_void) };
    }

    result
}
fn decode(count: usize, buffer: &mut [Uch]) -> Result<usize, ()> {
    static mut I: usize = 0;
    let mut r: usize = 0;
    let mut c: u32;

    unsafe {
        while J > 0 {
            J -= 1;
            buffer[r] = buffer[I];
            I = (I + 1) & ((1 << 13) - 1);
            r += 1;
            if r == count {
                return Ok(r);
            }
        }
    }

    loop {
        c = decode_c()?;
        if c == (255 + 256 + 2 - 3) {
            unsafe {
                DONE = 1;
            }
            return Ok(r);
        }
        if c <= 255 {
            buffer[r] = c as Uch;
            r += 1;
            if r == count {
                return Ok(r);
            }
        } else {
            unsafe {
                J = c as i32 - (255 + 1 - 3);
            }
            unsafe {
                I = (r - decode_p() as usize - 1) & ((1 << 13) - 1);
            }
            unsafe {
                while J > 0 {
                    J -= 1;
                    buffer[r] = buffer[I];
                    I = (I + 1) & ((1 << 13) - 1);
                    r += 1;
                    if r == count {
                        return Ok(r);
                    }
                }
            }
        }
    }
}
fn decode_c() -> Result<u32, ()> {
    let mut j: u32;
    let mut mask: u64;
    unsafe {
        if BLOCKSIZE == 0 {
            BLOCKSIZE = getbits(16) as u32;
            if BLOCKSIZE == 0 {
                return Ok(255 + 256 + 2 - 3);
            }
            read_pt_len(16 + 3, 5, 3)?;
            read_c_len()?;
            read_pt_len(13 + 1, 4, usize::MAX)?;
        }
        BLOCKSIZE -= 1;
        j = D_BUF[(BITBUF >> ((8 * 2 * std::mem::size_of::<u8>()) as u64 - 12)) as usize] as u32;
        if j >= (255 + 256 + 2 - 3) {
            mask = 1 << ((8 * 2 * std::mem::size_of::<u8>()) as u64 - 1 - 12);
            loop {
                if BITBUF & mask != 0 {
                    j = PREV[(0x8000 + j) as usize] as u32;
                } else {
                    j = PREV[j as usize] as u32;
                }
                mask >>= 1;
                if j < (255 + 256 + 2 - 3) {
                    break;
                }
            }
        }
        fillbuf(OUTBUF[j as usize] as i32);
    }
    Ok(j)
}
fn decode_p() -> u32 {
    let mut j: u32;
    let mut mask: u32;
    unsafe {
        j = PT_TABLE[(BITBUF >> ((8 * 2 * std::mem::size_of::<u8>()) - 8)) as usize] as u32;
        if j >= (13 + 1) {
            mask = 1 << ((8 * 2 * std::mem::size_of::<u8>()) - 1 - 8);
            while j >= (13 + 1) {
                if BITBUF & (mask as u64) != 0 {
                    j = (PREV[0x8000 + j as usize]) as u32;
                } else {
                    j = PREV[j as usize] as u32;
                }
                mask >>= 1;
            }
        }
        fillbuf(PT_LEN[j as usize] as i32);
        if j != 0 {
            j = (1 << (j - 1)) + getbits((j - 1) as i32) as u32;
        }
    }
    j
}
fn decode_start() {
    crate::huf_decode_start();
    unsafe {
        crate::J = 0;
        crate::DONE = 0;
    }
}
fn deflate(pack_level: i32) -> Result<libc::off_t, ()> {
    let mut hash_head: u16; // Change IPos to u16
    let mut prev_match: u16; // Change IPos to u16
    let mut flush = 0;
    let mut match_available = 0;
    let mut match_length: u32 = 3 - 1;
    lm_init(pack_level)?;
    if pack_level <= 3 {
        return deflate_fast();
    }
    while unsafe { LOOKAHEAD } != 0 {
        unsafe {
            INS_H = ((INS_H << ((15 + 3 - 1) / 3)) ^ (WINDOW[(STRSTART + 3 - 1) as usize] as u32)) & ((1 << 15) - 1);
            hash_head = PREV[(0x8000 + INS_H) as usize];
            PREV[(STRSTART & (0x8000 - 1)) as usize] = hash_head;
            PREV[(0x8000 + INS_H) as usize] = STRSTART as u16; // Cast STRSTART to u16
            PREV_LENGTH = match_length;
            prev_match = MATCH_START as u16; // Cast MATCH_START to u16
            match_length = 3 - 1;
            if hash_head != 0 && PREV_LENGTH < MAX_LAZY_MATCH &&
                STRSTART - hash_head as u32 <= (0x8000 - (258 + 3 + 1)) &&
                STRSTART <= WINDOW_SIZE as u32 - (258 + 3 + 1) {
                match_length = longest_match(hash_head as u32) as u32;
                if match_length > LOOKAHEAD {
                    match_length = LOOKAHEAD;
                }
                if match_length == 3 && STRSTART - MATCH_START > 4096 {
                    match_length -= 1;
                }
            }
            if PREV_LENGTH >= 3 && match_length <= PREV_LENGTH {
                flush = ct_tally((STRSTART - 1 - prev_match as u32) as i32, (PREV_LENGTH - 3) as i32) as i32;
                LOOKAHEAD -= PREV_LENGTH - 1;
                PREV_LENGTH -= 2;
                if RSYNC != 0 {
                    rsync_roll(STRSTART as usize, (PREV_LENGTH + 1) as usize);
                }
                while PREV_LENGTH != 0 {
                    STRSTART += 1;
                    INS_H = ((INS_H << ((15 + 3 - 1) / 3)) ^ (WINDOW[(STRSTART + 3 - 1) as usize] as u32)) & ((1 << 15) - 1);
                    hash_head = PREV[(0x8000 + INS_H) as usize];
                    PREV[(STRSTART & (0x8000 - 1)) as usize] = hash_head;
                    PREV[(0x8000 + INS_H) as usize] = STRSTART as u16; // Cast STRSTART to u16
                    PREV_LENGTH -= 1;
                }
                match_available = 0;
                match_length = 3 - 1;
                STRSTART += 1;
                if RSYNC != 0 && STRSTART > RSYNC_CHUNK_END as u32 {
                    RSYNC_CHUNK_END = 0xFFFFFFFF;
                    flush = 2;
                }
                if flush != 0 {
                    flush_block(
                        if BLOCK_START >= 0 {
                            Some(&WINDOW[BLOCK_START as usize..STRSTART as usize])
                        } else {
                            None
                        },
                        (STRSTART as i64 - BLOCK_START) as u64, // Cast STRSTART to i64 for subtraction
                        flush - 1 != 0,
                        false,
                    )?;
                    BLOCK_START = STRSTART as i64;
                }
            } else if match_available != 0 {
                flush = ct_tally(0, WINDOW[(STRSTART - 1) as usize] as i32) as i32;
                if RSYNC != 0 && STRSTART > RSYNC_CHUNK_END as u32 {
                    RSYNC_CHUNK_END = 0xFFFFFFFF;
                    flush = 2;
                }
                if flush != 0 {
                    flush_block(
                        if BLOCK_START >= 0 {
                            Some(&WINDOW[BLOCK_START as usize..STRSTART as usize])
                        } else {
                            None
                        },
                        (STRSTART as i64 - BLOCK_START) as u64, // Cast STRSTART to i64 for subtraction
                        flush - 1 != 0,
                        false,
                    )?;
                    BLOCK_START = STRSTART as i64;
                }
                if RSYNC != 0 {
                    rsync_roll(STRSTART as usize, 1);
                }
                STRSTART += 1;
                LOOKAHEAD -= 1;
            } else {
                if RSYNC != 0 && STRSTART > RSYNC_CHUNK_END as u32 {
                    RSYNC_CHUNK_END = 0xFFFFFFFF;
                    flush = 2;
                    flush_block(
                        if BLOCK_START >= 0 {
                            Some(&WINDOW[BLOCK_START as usize..STRSTART as usize])
                        } else {
                            None
                        },
                        (STRSTART as i64 - BLOCK_START) as u64, // Cast STRSTART to i64 for subtraction
                        flush - 1 != 0,
                        false,
                    )?;
                    BLOCK_START = STRSTART as i64;
                }
                match_available = 1;
                if RSYNC != 0 {
                    rsync_roll(STRSTART as usize, 1);
                }
                STRSTART += 1;
                LOOKAHEAD -= 1;
            }
            while LOOKAHEAD < (258 + 3 + 1) as u32 && EOFILE == 0 {
                fill_window();
            }
        }
    }
    if match_available != 0 {
        unsafe {
            ct_tally(0, WINDOW[(STRSTART - 1) as usize] as i32);
        }
    }
    unsafe {
        flush_block(
            if BLOCK_START >= 0 {
                Some(&WINDOW[BLOCK_START as usize..STRSTART as usize])
            } else {
                None
            },
            (STRSTART as i64 - BLOCK_START) as u64, // Cast STRSTART to i64 for subtraction
            flush - 1 != 0,
            true,
        )
    }
}
fn deflate_fast() -> Result<libc::off_t, ()> {
    use std::convert::TryInto;
    let mut hash_head: u16; // Assuming IPos is u16 based on the error message
    let mut flush = 0;
    let mut match_length: u32 = 0;
    unsafe {
        PREV_LENGTH = 3 - 1;
        while LOOKAHEAD != 0 {
            INS_H = ((INS_H << ((15 + 3 - 1) / 3)) ^ (WINDOW[(STRSTART + 3 - 1) as usize] as u32)) & ((1 << 15) - 1);
            hash_head = PREV[(STRSTART & (0x8000 - 1)) as usize];
            PREV[(STRSTART & (0x8000 - 1)) as usize] = (PREV[0x8000 + INS_H as usize]);
            PREV[0x8000 + INS_H as usize] = STRSTART as Ush;
            if hash_head != 0 && STRSTART - hash_head as u32 <= (0x8000 - (258 + 3 + 1)) as u32
                && STRSTART <= WINDOW_SIZE as u32 - (258 + 3 + 1) as u32
            {
                match_length = longest_match((hash_head as usize).try_into().unwrap()) as u32;
                if match_length > LOOKAHEAD {
                    match_length = LOOKAHEAD;
                }
            }
            if match_length >= 3 {
                flush = ct_tally((STRSTART - MATCH_START) as i32, (match_length - 3) as i32) as i32;
                LOOKAHEAD -= match_length;
                if RSYNC != 0 {
                    rsync_roll(STRSTART as usize, match_length as usize);
                }
                if match_length <= MAX_LAZY_MATCH {
                    match_length -= 1;
                    while match_length != 0 {
                        STRSTART += 1;
                        INS_H = ((INS_H << ((15 + 3 - 1) / 3)) ^ (WINDOW[(STRSTART + 3 - 1) as usize] as u32)) & ((1 << 15) - 1);
                        hash_head = PREV[(STRSTART & (0x8000 - 1)) as usize];
                        PREV[(STRSTART & (0x8000 - 1)) as usize] = (PREV[0x8000 + INS_H as usize]);
                        PREV[0x8000 + INS_H as usize] = STRSTART as Ush;
                        match_length -= 1;
                    }
                    STRSTART += 1;
                } else {
                    STRSTART += match_length;
                    match_length = 0;
                    INS_H = WINDOW[STRSTART as usize] as u32;
                    INS_H = ((INS_H << ((15 + 3 - 1) / 3)) ^ (WINDOW[(STRSTART + 1) as usize] as u32)) & ((1 << 15) - 1);
                }
            } else {
                flush = ct_tally(0, WINDOW[STRSTART as usize] as i32) as i32;
                if RSYNC != 0 {
                    rsync_roll(STRSTART as usize, 1);
                }
                LOOKAHEAD -= 1;
                STRSTART += 1;
            }
            if RSYNC != 0 && STRSTART > RSYNC_CHUNK_END as u32 {
                RSYNC_CHUNK_END = 0xFFFFFFFF;
                flush = 2;
            }
            if flush != 0 {
                flush_block(
                    if BLOCK_START >= 0 {
                        Some(&WINDOW[BLOCK_START as usize..STRSTART as usize])
                    } else {
                        None
                    },
                    (STRSTART - BLOCK_START as u32) as Ulg,
                    (flush - 1) != 0,
                    false,
                )?;
                BLOCK_START = STRSTART as i64;
            }
            while LOOKAHEAD < (258 + 3 + 1) as u32 && EOFILE == 0 {
                fill_window();
            }
        }
        flush_block(
            if BLOCK_START >= 0 {
                Some(&WINDOW[BLOCK_START as usize..STRSTART as usize])
            } else {
                None
            },
            (STRSTART - BLOCK_START as u32) as Ulg,
            (flush - 1) != 0,
            true,
        )
    }
}
fn dir_len(file: &str) -> Option<usize> {
    let mut prefix_length = 0;
    let length;
    prefix_length += if prefix_length != 0 {
        if file.as_bytes().get(prefix_length) == Some(&b'/') {
            0
        } else {
            0
        }
    } else {
        if file.as_bytes().get(0) == Some(&b'/') {
            if file.as_bytes().get(1) == Some(&b'/') && file.as_bytes().get(2) != Some(&b'/') {
                2
            } else {
                1
            }
        } else {
            0
        }
    };

    if let Some(last_component) = last_component(file) {
        length = last_component.as_ptr() as usize - file.as_ptr() as usize;
        let mut length = length;
        while prefix_length < length {
            if file.as_bytes().get(length - 1) != Some(&b'/') {
                break;
            }
            length -= 1;
        }
        Some(length)
    } else {
        None
    }
}
fn direntry_cmp_name(a: &DirentryT, b: &DirentryT) -> std::cmp::Ordering {
    a.name.cmp(&b.name)
}
fn discard_input_bytes(nbytes: usize, flags: u32) -> Result<(), ()> {
    let mut nbytes = nbytes;
    while nbytes != 0 {
        let c = if unsafe { INPTR < INSIZE } {
            let byte = unsafe { INBUF[INPTR as usize] };
            unsafe { INPTR += 1 };
            byte
        } else {
            fill_inbuf(false)?
        };
        if unsafe { FLAGS & 0x02 } != 0 {
            updcrc(Some(&[c]), 1);
        }
        if nbytes != usize::MAX {
            nbytes -= 1;
        } else if c == 0 {
            break;
        }
    }
    Ok(())
}
fn display_ratio(num: i64, den: i64, file: &mut std::fs::File) -> Result<(), ()> {
    let ratio = if den == 0 { 0.0 } else { 100.0 * num as f64 / den as f64 };
    rpl_fprintf(file, "%5.1f%%", &[ArgValue::Float(ratio as f32)])?;
    Ok(())
}
fn do_chown(fd: i32, name: &str, uid: u32, gid: u32) -> Result<(), ()> {
    match nix::unistd::fchown(fd, Some(nix::unistd::Uid::from_raw(uid)), Some(nix::unistd::Gid::from_raw(gid))) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}
fn do_exit(exitcode: i32) -> ! {
    static mut IN_EXIT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

    unsafe {
        if IN_EXIT.load(std::sync::atomic::Ordering::SeqCst) {
            std::process::exit(exitcode);
        }
        IN_EXIT.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    unsafe {
        if let Some(env) = ENV.take() {
            rpl_free(env.as_ptr() as *mut std::ffi::c_void);
        }
    }

    std::process::exit(exitcode);
}
fn do_list(method: i32) -> Result<(), ()> {
    let mut crc: Ulg;
    static mut FIRST_TIME: bool = true;
    static METHODS: [&str; 9] = [
        "store", "compr", "pack ", "lzh  ", "", "", "", "", "defla"
    ];
    let positive_off_t_width = (((((std::mem::size_of::<libc::off_t>() * 8) - (!((0 as libc::off_t) < (-1 as libc::off_t)))) * 146 + 484) / 485) + (!((0 as libc::off_t) < (-1 as libc::off_t)))) - 1;

    unsafe {
        if FIRST_TIME && METHOD >= 0 {
            FIRST_TIME = false;
            if VERBOSE != 0 {
                __printf__("method  crc     date  time  ", &[]);
            }
            if QUIET == 0 {
                __printf__("%*.*s %*.*s  ratio uncompressed_name\n",
                    &[positive_off_t_width.into(), positive_off_t_width.into(), "compressed".into(),
                      positive_off_t_width.into(), positive_off_t_width.into(), "uncompressed".into()]);
            }
        } else if METHOD < 0 {
            if TOTAL_IN <= 0 || TOTAL_OUT <= 0 {
                return Ok(());
            }
            if VERBOSE != 0 {
                __printf__("                            ", &[]);
            }
            if VERBOSE != 0 || QUIET == 0 {
                fprint_off(&mut std::io::stdout(), TOTAL_IN, positive_off_t_width);
                __printf__(" ", &[]);
                fprint_off(&mut std::io::stdout(), TOTAL_OUT, positive_off_t_width);
                __printf__(" ", &[]);
            }
            display_ratio(TOTAL_OUT - (TOTAL_IN - HEADER_BYTES), TOTAL_OUT, &mut std::io::stdout())?;
            __printf__(" (totals)\n", &[]);
            return Ok(());
        }

        crc = !0;
        if METHOD == 8 && LAST_MEMBER == 0 {
            crc = UNZIP_CRC;
        }

        if VERBOSE != 0 {
            static MONTH_ABBR: [&str; 12] = [
                "Jan", "Feb", "Mar", "Apr", "May", "Jun",
                "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
            ];
            let tm = std::time::UNIX_EPOCH + TIME_STAMP;
            let datetime = tm.duration_since(std::time::UNIX_EPOCH).unwrap();
            let tm = chrono::NaiveDateTime::from_timestamp_opt(datetime.as_secs() as i64, 0);
            __printf__("%5s %08lx ", &[METHODS[METHOD].into(), crc.into()]);
            if let Some(tm) = tm {
                __printf__("%s%3d %02d:%02d ", &[
                    MONTH_ABBR[tm.month0() as usize].into(),
                    tm.day().into(),
                    tm.hour().into(),
                    tm.minute().into()
                ]);
            } else {
                __printf__("??? ?? ??:?? ", &[]);
            }
        }

        fprint_off(&mut std::io::stdout(), BYTES_IN, positive_off_t_width);
        __printf__(" ", &[]);
        fprint_off(&mut std::io::stdout(), BYTES_OUT, positive_off_t_width);
        __printf__(" ", &[]);

        if BYTES_IN == -1 {
            TOTAL_IN = -1;
            BYTES_IN = 0;
            BYTES_OUT = 0.into();
            HEADER_BYTES = 0;
        } else if TOTAL_IN >= 0 {
            TOTAL_IN += BYTES_IN;
        }

        if BYTES_OUT == -1 {
            TOTAL_OUT = -1;
            BYTES_IN = 0;
            BYTES_OUT = 0.into();
            HEADER_BYTES = 0;
        } else if TOTAL_OUT >= 0 {
            TOTAL_OUT += BYTES_OUT;
        }

        display_ratio(BYTES_OUT - (BYTES_IN - HEADER_BYTES), BYTES_OUT, &mut std::io::stdout())?;
        __printf__(" %s\n", &[OFNAME.as_ptr().into()]);
    }

    Ok(())
}
fn dtotimespec(seconds: f64) -> std::time::Duration{todo!("proto")}
fn dup_cloexec(fd: i32) -> Result<i32, ()> {
    crate::rpl_fcntl(fd, 1030)
}
fn dup_safer(fd: i32) -> Result<i32, ()> {
    rpl_fcntl(fd, libc::F_DUPFD_CLOEXEC as usize)
}
fn dup_safer_flag(fd: i32, flag: i32) -> Result<i32, ()> {
    rpl_fcntl(fd, if (flag & 0o2000000) != 0 { 1030 } else { 0 })
}
fn error(status: i32, errnum: i32, message: &str) -> !{todo!("proto")}
fn error_at_line(){todo!("proto")}
fn fd_safer(fd: i32) -> Result<i32, ()> {
    if (0..=2).contains(&fd) {
        let f = dup_safer(fd)?;
        let e = unsafe { *libc::__errno_location() };
        unsafe { libc::close(fd) };
        unsafe { *libc::__errno_location() = e };
        return Ok(f);
    }
    Ok(fd)
}
fn fd_safer_flag(fd: i32, flag: i32) -> Result<i32, ()> {
    if (0..=2).contains(&fd) {
        let f = dup_safer_flag(fd, flag)?;
        let e = unsafe { *libc::__errno_location() }; // Unsafe block for dereferencing
        unsafe { libc::close(fd) }; // Unsafe block for calling libc::close
        unsafe { *libc::__errno_location() = e }; // Unsafe block for dereferencing and assignment
        return Ok(f);
    }
    Ok(fd)
}
fn fdopen(fd: i32, mode: &str) -> Option<std::fs::File>{todo!("proto")}
fn fdopendir(fd: i32) -> Option<Box<usize>>{todo!("proto")}
fn fdutimens(fd: i32, file: Option<&str>, timespec: Option<&[libc::timespec; 2]>) -> Result<(), ()> {
    let mut adjusted_timespec = [libc::timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    let ts = if let Some(timespec) = timespec {
        adjusted_timespec[0] = timespec[0];
        adjusted_timespec[1] = timespec[1];
        Some(&mut adjusted_timespec)
    } else {
        None
    };
    let mut adjustment_needed = 0;
    let mut st = std::mem::MaybeUninit::<libc::stat>::uninit();
    if let Some(ts) = ts {
        adjustment_needed = validate_timespec(ts).unwrap_or(-1);
    }
    if adjustment_needed < 0 {
        return Err(());
    }
    if fd < 0 && file.is_none() {
        return Err(());
    }
    if unsafe { UTIMENSAT_WORKS_REALLY } >= 0 {
        let mut result;
        if adjustment_needed == 2 {
            if fd < 0 {
                if let Some(file) = file {
                    if unsafe { libc::stat(file.as_ptr() as *const i8, st.as_mut_ptr()) } != 0 {
                        return Err(());
                    }
                }
            } else if unsafe { libc::fstat(fd, st.as_mut_ptr()) } != 0 {
                return Err(());
            }
            let st = unsafe { st.assume_init() };
            if let Some(ts) = ts {
                if ts[0].tv_nsec == ((1 << 30) - 2) {
                    ts[0] = get_stat_atime(&st);
                } else if ts[1].tv_nsec == ((1 << 30) - 2) {
                    ts[1] = get_stat_mtime(&st);
                }
            }
            adjustment_needed += 1;
        }
        if fd < 0 {
            if let Some(file) = file {
                if let Some(ts) = ts {
                    result = unsafe { libc::utimensat(libc::AT_FDCWD, file.as_ptr() as *const i8, ts.as_ptr(), 0) };
                    if result > 0 {
                        return Err(());
                    }
                    if result == 0 || unsafe { *libc::__errno_location() } != libc::ENOSYS {
                        unsafe { UTIMENSAT_WORKS_REALLY = 1 };
                        return Ok(());
                    }
                }
            }
        }
        if fd >= 0 {
            if let Some(ts) = ts {
                result = unsafe { libc::futimens(fd, ts.as_ptr()) };
                if result > 0 {
                    return Err(());
                }
                if result == 0 || unsafe { *libc::__errno_location() } != libc::ENOSYS {
                    unsafe { UTIMENSAT_WORKS_REALLY = 1 };
                    return Ok(());
                }
            }
        }
    }
    unsafe { UTIMENSAT_WORKS_REALLY = -1 };
    unsafe { LUTIMENSAT_WORKS_REALLY = -1 };
    if adjustment_needed != 0 || (false && fd < 0) {
        if adjustment_needed != 3 {
            if fd < 0 {
                if let Some(file) = file {
                    if unsafe { libc::stat(file.as_ptr() as *const i8, st.as_mut_ptr()) } != 0 {
                        return Err(());
                    }
                }
            } else if unsafe { libc::fstat(fd, st.as_mut_ptr()) } != 0 {
                return Err(());
            }
        }
        let st = unsafe { st.assume_init() };
        if let Some(ts) = ts {
            if update_timespec(&st, &mut Some(*ts)).is_ok() {
                return Ok(());
            }
        }
    }
    let mut timeval = [libc::timeval { tv_sec: 0, tv_usec: 0 }; 2];
    let t = if let Some(ts) = ts {
        timeval[0].tv_sec = ts[0].tv_sec;
        timeval[0].tv_usec = ts[0].tv_nsec / 1000;
        timeval[1].tv_sec = ts[1].tv_sec;
        timeval[1].tv_usec = ts[1].tv_nsec / 1000;
        Some(&timeval)
    } else {
        None
    };
    if fd < 0 {
        if let Some(file) = file {
            if let Some(t) = t {
                return if unsafe { libc::utimes(file.as_ptr() as *const i8, t.as_ptr()) } == 0 {
                    Ok(())
                } else {
                    Err(())
                };
            }
        }
    } else {
        if let Some(t) = t {
            if unsafe { libc::futimes(fd, t.as_ptr()) } == 0 {
                let abig = t[0].tv_usec >= 500000;
                let mbig = t[1].tv_usec >= 500000;
                if (abig || mbig) && unsafe { libc::fstat(fd, st.as_mut_ptr()) } == 0 {
                    let st = unsafe { st.assume_init() };
                    let adiff = st.st_atime - t[0].tv_sec;
                    let mdiff = st.st_mtime - t[1].tv_sec;
                    let mut tt = None;
                    let mut truncated_timeval = [libc::timeval { tv_sec: 0, tv_usec: 0 }; 2];
                    truncated_timeval[0] = t[0];
                    truncated_timeval[1] = t[1];
                    if abig && adiff == 1 && get_stat_atime_ns(&st) == 0 {
                        tt = Some(&mut truncated_timeval);
                        tt.unwrap()[0].tv_usec = 0;
                    }
                    if mbig && mdiff == 1 && get_stat_mtime_ns(&st) == 0 {
                        tt = Some(&mut truncated_timeval);
                        tt.unwrap()[1].tv_usec = 0;
                    }
                    if let Some(tt) = tt {
                        unsafe { libc::futimes(fd, tt.as_ptr()) };
                    }
                }
                return Ok(());
            }
        }
    }
    if file.is_none() {
        return Err(());
    }
    if let Some(file) = file {
        if let Some(t) = t {
            return if unsafe { libc::utimes(file.as_ptr() as *const i8, t.as_ptr()) } == 0 {
                Ok(())
            } else {
                Err(())
            };
        }
    }
    Err(())
}
fn file_read(buf: &mut [u8]) -> Result<usize, ()> {
    let len = unsafe { read_buffer(IFD, buf)? };
    if len == 0 {
        return Ok(len);
    }
    updcrc(Some(&buf[..len]), len);
    unsafe {
        BYTES_IN += len as libc::off_t;
    }
    Ok(len)
}
fn fill_inbuf(eof_ok: bool) -> Result<u8, ()> {
    let mut len: isize;
    unsafe {
        INSIZE = 0;
        loop {
            match read_buffer(IFD, &mut INBUF[INSIZE as usize..0x40000]) {
                Ok(read_len) => len = read_len as isize,
                Err(_) => {
                    read_error();
                    break;
                }
            }
            if len == 0 {
                break;
            }
            INSIZE += len as u32;
            if INSIZE >= 0x40000 {
                break;
            }
        }
        if INSIZE == 0 {
            if eof_ok {
                return Err(());
            }
            flush_window().map_err(|_| ())?;
            *libc::__errno_location() = 0;
            read_error();
        }
        BYTES_IN += INSIZE as libc::off_t;
        INPTR = 1;
        Ok(INBUF[0])
    }
}
fn fill_window() {
    unsafe {
        let mut more = (WINDOW_SIZE - LOOKAHEAD as Ulg - STRSTART as Ulg) as u32;
        if more == u32::MAX {
            more -= 1;
        } else if STRSTART >= 0x8000 + (0x8000 - (258 + 3 + 1)) {
            std::ptr::copy(
                WINDOW.as_ptr().add(0x8000),
                WINDOW.as_mut_ptr(),
                0x8000,
            );
            MATCH_START -= 0x8000;
            STRSTART -= 0x8000;
            if RSYNC_CHUNK_END != 0xFFFFFFFF {
                RSYNC_CHUNK_END -= 0x8000;
            }
            BLOCK_START -= 0x8000;
            for n in 0..(1 << 15) {
                let m = PREV[0x8000 + n];
                PREV[0x8000 + n] = if m >= 0x8000 { m - 0x8000 } else { 0 };
            }
            for n in 0..0x8000 {
                let m = PREV[n];
                PREV[n] = if m >= 0x8000 { m - 0x8000 } else { 0 };
            }
            more += 0x8000;
        }
        if EOFILE == 0 {
            if let Some(read_buf) = READ_BUF {
                let n = read_buf(&mut WINDOW[(STRSTART + LOOKAHEAD) as usize..(STRSTART + LOOKAHEAD + more) as usize]);
                if n == 0 || n == -1 {
                    EOFILE = 1;
                    WINDOW[(STRSTART + LOOKAHEAD) as usize..(STRSTART + LOOKAHEAD + 2) as usize].fill(0);
                } else {
                    LOOKAHEAD += n as u32;
                }
            }
        }
    }
}
fn fillbuf(n: i32) {
    unsafe {
        BITBUF <<= n;
        let mut n = n;
        while n > BITCOUNT {
            BITBUF |= (SUBBITBUF as Ulg) << (n - BITCOUNT);
            n -= BITCOUNT;
            SUBBITBUF = if INPTR < INSIZE {
                INBUF[INPTR as usize] as u32
            } else {
                match fill_inbuf(true) {
                    Ok(byte) => byte as u32,
                    Err(_) => 0,
                }
            };
            INPTR += 1;
            if SUBBITBUF == std::u32::MAX {
                SUBBITBUF = 0;
            }
            BITCOUNT = 8;
        }
        BITBUF |= (SUBBITBUF as Ulg) >> (BITCOUNT - n);
        BITCOUNT -= n;
    }
}
fn find_non_slash(s: &str) -> &str {
    let n_slash = s.chars().take_while(|&c| c == '/').count();
    &s[n_slash..]
}
fn finish_out() {
    if rpl_fclose(&mut std::io::stdout().lock()).is_err() {
        write_error();
    }
    do_exit(0);
}
fn flush_block(buf: Option<&[u8]>, stored_len: Ulg, pad: bool, eof: bool) -> Result<libc::off_t, ()> {
    let mut opt_lenb: Ulg;
    let mut static_lenb: Ulg;
    let mut max_blindex: i32;
    unsafe {
        FLAG_BUF[LAST_FLAGS as usize] = FLAGS;
        if *FILE_TYPE.as_ref().unwrap() == Box::new(0xffff) {
            set_file_type();
        }
        build_tree(L_DESC.as_mut().unwrap());
        build_tree(D_DESC.as_mut().unwrap());
        max_blindex = build_bl_tree() as i32;
        opt_lenb = (OPT_LEN + 3 + 7) >> 3;
        static_lenb = (STATIC_LEN + 3 + 7) >> 3;
        INPUT_LEN += stored_len as libc::off_t;
        if static_lenb <= opt_lenb {
            opt_lenb = static_lenb;
        }
        if stored_len <= opt_lenb && eof && COMPRESSED_LEN == 0 && false {
            if buf.is_none() {
                gzip_error("block vanished");
            }
            copy_block(buf.unwrap(), false)?;
            COMPRESSED_LEN = (stored_len << 3) as libc::off_t;
            *FILE_METHOD.unwrap() = 0;
        } else if stored_len + 4 <= opt_lenb && buf.is_some() {
            send_bits((0 << 1) + eof as i32, 3)?;
            COMPRESSED_LEN = (COMPRESSED_LEN + 3 + 7) & !7;
            COMPRESSED_LEN += ((stored_len + 4) << 3) as libc::off_t;
            copy_block(buf.unwrap(), true)?;
        } else if static_lenb == opt_lenb {
            send_bits((1 << 1) + eof as i32, 3)?;
            compress_block(&mut STATIC_LTREE, &mut STATIC_DTREE)?;
            COMPRESSED_LEN += 3 + STATIC_LEN as libc::off_t;
        } else {
            send_bits((2 << 1) + eof as i32, 3)?;
            send_all_trees(
                L_DESC.as_ref().unwrap().max_code as usize + 1,
                D_DESC.as_ref().unwrap().max_code as usize + 1,
                max_blindex as usize + 1,
            )?;
            compress_block(&mut DYN_LTREE, &mut DYN_DTREE)?;
            COMPRESSED_LEN += 3 + OPT_LEN as libc::off_t;
        }
        init_block();
        if eof {
            bi_windup()?;
            COMPRESSED_LEN += 7;
        } else if pad && (COMPRESSED_LEN % 8) != 0 {
            send_bits((0 << 1) + eof as i32, 3)?;
            COMPRESSED_LEN = (COMPRESSED_LEN + 3 + 7) & !7;
            copy_block(buf.unwrap_or(&[]), true)?;
        }
        Ok(COMPRESSED_LEN >> 3)
    }
}
fn flush_outbuf() -> Result<(), ()> {
    unsafe {
        if OUTCNT == 0 {
            return Ok(());
        }
        write_buf(OFD, &mut OUTBUF[..OUTCNT as usize])?;
        OUTCNT = 0;
    }
    Ok(())
}
fn flush_window() -> Result<(), ()> {
    if unsafe { OUTCNT } == 0 {
        return Ok(());
    }
    unsafe {
        updcrc(Some(&WINDOW[..OUTCNT as usize]), OUTCNT as usize);
        write_buf(OFD, &mut WINDOW[..OUTCNT as usize])?;
        OUTCNT = 0;
    }
    Ok(())
}
fn fopen(filename: &str, mode: &str) -> Option<std::fs::File>{todo!("proto")}
fn fprint_off<W: std::io::Write>(file: &mut W, offset: i64, width: usize) {
    let mut buf = [0u8; 8 * std::mem::size_of::<i64>()];
    let mut p = buf.len();
    let mut offset = offset;
    if offset < 0 {
        loop {
            p -= 1;
            buf[p] = b'0' - (offset % 10) as u8;
            offset /= 10;
            if offset == 0 {
                break;
            }
        }
        p -= 1;
        buf[p] = b'-';
    } else {
        loop {
            p -= 1;
            buf[p] = b'0' + (offset % 10) as u8;
            offset /= 10;
            if offset == 0 {
                break;
            }
        }
    }
    let num_len = buf.len() - p;
    let padding = if width > num_len { width - num_len } else { 0 };
    for _ in 0..padding {
        let _ = file.write_all(b" ");
    }
    let _ = file.write_all(&buf[p..]);
}
fn fpurge(fp: &mut std::fs::File) -> i32 {
    match fp.sync_all() {
        Ok(_) => 0, // Return 0 on success
        Err(_) => -1, // Return -1 or any other error code on failure
    }
}
fn free_cwd(cwd: &mut SavedCwd) {
    if cwd.desc >= 0 {
        unsafe {
            libc::close(cwd.desc);
        }
    }
    if let Some(name) = cwd.name.take() {
        unsafe {
            rpl_free(name.as_ptr() as *mut std::ffi::c_void);
        }
    }
}
fn freea(p: *mut u8) {
    const SA_ALIGNMENT_MAX: usize = 16; // Replace 16 with the actual value needed

    if (p as usize) & (SA_ALIGNMENT_MAX - 1) != 0 {
        std::process::abort();
    }
    if (p as usize) & SA_ALIGNMENT_MAX != 0 {
        let offset = unsafe { *p.offset(-1) as isize };
        let mem = unsafe { p.offset(-offset) };
        rpl_free(mem as *mut std::ffi::c_void);
    }
}
fn fseterr(fp: &mut std::fs::File) {
    use std::os::unix::io::AsRawFd;
    use std::os::unix::io::FromRawFd;
    let raw_fd = fp.as_raw_fd();
    let mut file = unsafe { std::fs::File::from_raw_fd(raw_fd) };
    let flags = unsafe { libc::fcntl(raw_fd, libc::F_GETFL) };
    unsafe { libc::fcntl(raw_fd, libc::F_SETFL, flags | 0x0020) };
    std::mem::forget(file);
}
fn gen_bitlen(desc: &mut TreeDesc) {
    let tree = &mut desc.dyn_tree;
    let extra = desc.extra_bits.unwrap_or(&[]);
    let base = desc.extra_base as usize;
    let max_code = desc.max_code as usize;
    let max_length = desc.max_length as usize;
    let stree = desc.static_tree;
    let mut h = 0; // Initialize h
    let mut n;
    let mut m;
    let mut bits;
    let mut xbits;
    let mut f;
    let mut overflow = 0;

    unsafe {
        for bits in 0..=15 {
            BL_COUNT[bits] = 0;
        }
        tree[HEAP[HEAP_MAX as usize] as usize].dl = DadOrLen::Len(0);
        for h in (HEAP_MAX + 1) as usize..(2 * (256 + 1 + 29) + 1) {
            n = HEAP[h] as usize;
            bits = match tree[n].dl {
                DadOrLen::Len(len) => len as usize + 1,
                _ => 0,
            };
            if bits > max_length {
                bits = max_length;
                overflow += 1;
            }
            tree[n].dl = DadOrLen::Len(bits as Ush);
            if n > max_code {
                continue;
            }
            BL_COUNT[bits] += 1;
            xbits = if n >= base { extra[n - base] } else { 0 };
            f = match tree[n].fc {
                FreqOrCode::Freq(freq) => freq,
                _ => 0,
            };
            OPT_LEN += f as Ulg * (bits as Ulg + xbits as Ulg);
            if let Some(stree) = stree {
                STATIC_LEN += f as Ulg * (match stree[n].dl {
                    DadOrLen::Len(len) => len as Ulg,
                    _ => 0,
                } + xbits as Ulg);
            }
        }
        if overflow == 0 {
            return;
        }
        loop {
            bits = max_length - 1;
            while BL_COUNT[bits] == 0 {
                bits -= 1;
            }
            BL_COUNT[bits] -= 1;
            BL_COUNT[bits + 1] += 2;
            BL_COUNT[max_length] -= 1;
            overflow -= 2;
            if overflow <= 0 {
                break;
            }
        }
        for bits in (1..=max_length).rev() {
            n = BL_COUNT[bits] as usize; // Convert to usize
            while n != 0 {
                h -= 1;
                m = HEAP[h] as usize;
                if m > max_code {
                    continue;
                }
                if let DadOrLen::Len(len) = tree[m].dl {
                    if len as usize != bits {
                        if let FreqOrCode::Freq(freq) = tree[m].fc {
                            OPT_LEN += (bits as Ulg - len as Ulg) * freq as Ulg;
                        }
                        tree[m].dl = DadOrLen::Len(bits as Ush);
                    }
                }
                n -= 1;
            }
        }
    }
}
fn gen_codes(tree: &mut [CtData], max_code: usize) {
    let mut next_code = [0u16; 16];
    let mut code: Ush = 0;
    for bits in 1..=15 {
        next_code[bits] = code;
        code = (code + unsafe { BL_COUNT[bits - 1] }) << 1;
    }
    for n in 0..=max_code {
        if let DadOrLen::Len(len) = tree[n].dl {
            if len != 0 {
                if let FreqOrCode::Code(ref mut code) = tree[n].fc {
                    *code = bi_reverse(next_code[len as usize] as u32, len as usize) as u16;
                }
                next_code[len as usize] += 1;
            }
        }
    }
}
fn get_handler(a: &libc::sigaction) -> Option<libc::sighandler_t> {
    // Access the sa_sigaction field and interpret it as a signal handler
    let handler = unsafe { a.sa_sigaction as libc::sighandler_t };

    match handler {
        libc::SIG_DFL | libc::SIG_IGN => None,
        _ => Some(handler),
    }
}
fn get_input_size_and_time() {
    unsafe {
        IFILE_SIZE = -1;
        TIME_STAMP = std::time::Duration::new(0, std::u32::MAX);
        if (ISTAT.st_mode & 0o170000) == 0o100000 {
            IFILE_SIZE = ISTAT.st_size;
            if NO_TIME == 0 || LIST != 0 {
                TIME_STAMP = get_stat_mtime(&ISTAT);
            }
        }
    }
}
fn get_method(in_data: i32) -> i32 {
    // Define the missing variables or functions
    let unzip = || { /* define the unzip function or closure */ };
    let unpack = || { /* define the unpack function or closure */ };
    let unlzw = || { /* define the unlzw function or closure */ };
    let unlzh = || { /* define the unlzh function or closure */ };

    // Define ArgValue and rpl_fprintf to match the expected usage
    enum ArgValue {
        Str(String),
        Ptr(*const u8),
        Int(i32),
        U8(u8),
    }

    fn rpl_fprintf(fp: &mut std::io::Stderr, format: &str, args: &[ArgValue]) -> Result<usize, ()> {
        // Implement the function to handle the arguments
        Ok(0)
    }

    let mut flags: u8; // Assuming Uch is u8
    let mut magic = [0u8; 10];
    let mut imagic0: i32;
    let mut imagic1: i32;
    let mut stamp: u32; // Assuming Ulg is u32

    unsafe {
        if FORCE != 0 && TO_STDOUT.load(std::sync::atomic::Ordering::Relaxed) != 0 {
            imagic0 = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(true).unwrap() } as i32;
            magic[0] = imagic0 as u8;
            INPTR += 1;
            imagic1 = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(true).unwrap() } as i32;
            magic[1] = imagic1 as u8;
            INPTR += 1;
        } else {
            magic[0] = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() };
            INPTR += 1;
            imagic0 = 0;
            if magic[0] != 0 {
                magic[1] = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() };
                INPTR += 1;
                imagic1 = 0;
            } else {
                imagic1 = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(true).unwrap() } as i32;
                magic[1] = imagic1 as u8;
                INPTR += 1;
            }
        }
        METHOD = -1;
        PART_NB += 1;
        HEADER_BYTES = 0;
        LAST_MEMBER = 0;
        if magic.starts_with(&[0x1F, 0x8B]) || magic.starts_with(&[0x1F, 0x9E]) {
            METHOD = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() } as i32;
            INPTR += 1;
            if METHOD != 8 {
                rpl_fprintf(
                    &mut std::io::stderr(),
                    "%s: %s: unknown method %d -- not supported\n",
                    &[
                        ArgValue::Str(PROGRAM_NAME.unwrap().to_string()),
                        ArgValue::Ptr(IFNAME.as_ptr()),
                        ArgValue::Int(METHOD),
                    ],
                ).unwrap();
                EXIT_CODE = 1;
                return -1;
            }
            WORK = unzip;
            flags = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() };
            INPTR += 1;
            if (flags & 0x20) != 0 {
                rpl_fprintf(
                    &mut std::io::stderr(),
                    "%s: %s is encrypted -- not supported\n",
                    &[
                        ArgValue::Str(PROGRAM_NAME.unwrap().to_string()),
                        ArgValue::Ptr(IFNAME.as_ptr()),
                    ],
                ).unwrap();
                EXIT_CODE = 1;
                return -1;
            }
            if (flags & 0xC0) != 0 {
                rpl_fprintf(
                    &mut std::io::stderr(),
                    "%s: %s has flags 0x%x -- not supported\n",
                    &[
                        ArgValue::Str(PROGRAM_NAME.unwrap().to_string()),
                        ArgValue::Ptr(IFNAME.as_ptr()),
                        ArgValue::U8(flags),
                    ],
                ).unwrap();
                EXIT_CODE = 1;
                if FORCE <= 1 {
                    return -1;
                }
            }
            stamp = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() } as u32;
            INPTR += 1;
            stamp |= (if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() } as u32) << 8;
            INPTR += 1;
            stamp |= (if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() } as u32) << 16;
            INPTR += 1;
            stamp |= (if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() } as u32) << 24;
            INPTR += 1;
            if stamp != 0 && NO_TIME == 0 {
                if u64::from(stamp) <= std::time::Duration::MAX.as_secs() {
                    TIME_STAMP = std::time::Duration::new(stamp.into(), 0);
                } else {
                    if QUIET == 0 {
                        rpl_fprintf(
                            &mut std::io::stderr(),
                            "%s: %s: MTIME %lu out of range for this platform\n",
                            &[
                                ArgValue::Str(PROGRAM_NAME.unwrap().to_string()),
                                ArgValue::Ptr(IFNAME.as_ptr()),
                                ArgValue::Int(stamp as i32),
                            ],
                        ).unwrap();
                    }
                    if EXIT_CODE == 0 {
                        EXIT_CODE = 2;
                    }
                    TIME_STAMP = std::time::Duration::new(std::time::Duration::MAX.as_secs(), 0);
                }
            }
            magic[8] = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() };
            INPTR += 1;
            magic[9] = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() };
            INPTR += 1;
            if (flags & 0x02) != 0 {
                magic[2] = 8;
                magic[3] = flags;
                magic[4] = (stamp & 0xff) as u8;
                magic[5] = ((stamp >> 8) & 0xff) as u8;
                magic[6] = ((stamp >> 16) & 0xff) as u8;
                magic[7] = (stamp >> 24) as u8;
                updcrc(None, 0);
                updcrc(Some(&magic), 10);
            }
            if (flags & 0x04) != 0 {
                let mut lenbuf = [0u8; 2];
                let mut len = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() } as u32;
                INPTR += 1;
                len |= (if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() } as u32) << 8;
                INPTR += 1;
                if VERBOSE != 0 {
                    rpl_fprintf(
                        &mut std::io::stderr(),
                        "%s: %s: extra field of %u bytes ignored\n",
                        &[
                            ArgValue::Str(PROGRAM_NAME.unwrap().to_string()),
                            ArgValue::Ptr(IFNAME.as_ptr()),
                            ArgValue::Int(len as i32),
                        ],
                    ).unwrap();
                }
                if (flags & 0x02) != 0 {
                    updcrc(Some(&lenbuf), 2);
                }
                discard_input_bytes(len as usize, flags as u32).unwrap();
            }
            if (flags & 0x08) != 0 {
                if NO_NAME != 0 || (TO_STDOUT.load(std::sync::atomic::Ordering::Relaxed) != 0 && LIST == 0) || PART_NB > 1 {
                    discard_input_bytes(usize::MAX, flags as u32).unwrap();
                } else {
                    let mut p = gzip_base_name(std::str::from_utf8(&OFNAME).unwrap()).unwrap();
                    let base = p.clone();
                    loop {
                        let c = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() } as char;
                        INPTR += 1;
                        p.push(c);
                        if c == '\0' {
                            break;
                        }
                        if p.len() >= OFNAME.len() {
                            gzip_error("corrupted input -- file name too large");
                        }
                    }
                    if (flags & 0x02) != 0 {
                        updcrc(Some(base.as_bytes()), p.len());
                    }
                    let base_name = gzip_base_name(&base).unwrap();
                    std::ptr::copy_nonoverlapping(base_name.as_ptr(), base.as_mut_ptr(), base_name.len() + 1);
                    if LIST == 0 {
                        // Do something with base if needed
                    }
                }
            }
            if (flags & 0x10) != 0 {
                discard_input_bytes(usize::MAX, flags as u32).unwrap();
            }
            if (flags & 0x02) != 0 {
                let crc16 = (updcrc(Some(&magic), 0) & 0xffff) as u32;
                let mut header16 = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() } as u32;
                INPTR += 1;
                header16 |= (if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() } as u32) << 8;
                INPTR += 1;
                if header16 != crc16 {
                    rpl_fprintf(
                        &mut std::io::stderr(),
                        "%s: %s: header checksum 0x%04x != computed checksum 0x%04x\n",
                        &[
                            ArgValue::Str(PROGRAM_NAME.unwrap().to_string()),
                            ArgValue::Ptr(IFNAME.as_ptr()),
                            ArgValue::Int(header16 as i32),
                            ArgValue::Int(crc16 as i32),
                        ],
                    ).unwrap();
                    EXIT_CODE = 1;
                    if FORCE <= 1 {
                        return -1;
                    }
                }
            }
            if PART_NB == 1 {
                HEADER_BYTES = INPTR as libc::off_t + 2 * 4;
            }
        } else if magic.starts_with(&[0x50, 0x4B, 0x03, 0x04]) && INPTR == 2 && INBUF.starts_with(&[0x50, 0x4B, 0x03, 0x04]) {
            INPTR = 0;
            WORK = unzip;
            if check_zipfile(in_data).is_err() {
                return -1;
            }
            LAST_MEMBER = 1;
        } else if magic.starts_with(&[0x1F, 0x1E]) {
            WORK = unpack;
            METHOD = 2;
        } else if magic.starts_with(&[0x1F, 0x9D]) {
            WORK = unlzw;
            METHOD = 1;
            LAST_MEMBER = 1;
        } else if magic.starts_with(&[0x1F, 0xA0]) {
            WORK = unlzh;
            METHOD = 3;
            LAST_MEMBER = 1;
        } else if FORCE != 0 && TO_STDOUT.load(std::sync::atomic::Ordering::Relaxed) != 0 && LIST == 0 {
            METHOD = 0;
            WORK = std::fs::copy;
            if imagic1 != -1 {
                INPTR -= 1;
            }
            LAST_MEMBER = 1;
            if imagic0 != -1 {
                write_buf(1, &mut magic[..1]).unwrap();
            }
        }
        if METHOD >= 0 {
            return METHOD;
        }
        if PART_NB == 1 {
            rpl_fprintf(
                &mut std::io::stderr(),
                "\n%s: %s: not in gzip format\n",
                &[
                    ArgValue::Str(PROGRAM_NAME.unwrap().to_string()),
                    ArgValue::Ptr(IFNAME.as_ptr()),
                ],
            ).unwrap();
            EXIT_CODE = 1;
            return -1;
        } else {
            if magic[0] == 0 {
                let mut inbyte = imagic1;
                while inbyte == 0 {
                    inbyte = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(true).unwrap() } as i32;
                    INPTR += 1;
                }
                if inbyte == -1 {
                    if VERBOSE != 0 {
                        if QUIET == 0 {
                            rpl_fprintf(
                                &mut std::io::stderr(),
                                "\n%s: %s: decompression OK, trailing zero bytes ignored\n",
                                &[
                                    ArgValue::Str(PROGRAM_NAME.unwrap().to_string()),
                                    ArgValue::Ptr(IFNAME.as_ptr()),
                                ],
                            ).unwrap();
                        }
                        if EXIT_CODE == 0 {
                            EXIT_CODE = 2;
                        }
                    }
                    return -3;
                }
            }
            if QUIET == 0 {
                rpl_fprintf(
                    &mut std::io::stderr(),
                    "\n%s: %s: decompression OK, trailing garbage ignored\n",
                    &[
                        ArgValue::Str(PROGRAM_NAME.unwrap().to_string()),
                        ArgValue::Ptr(IFNAME.as_ptr()),
                    ],
                ).unwrap();
            }
            if EXIT_CODE == 0 {
                EXIT_CODE = 2;
            }
            return -2;
        }
    }
}
fn get_stat_atime(st: &std::fs::Metadata) -> std::time::SystemTime {
    st.accessed().unwrap_or(std::time::SystemTime::UNIX_EPOCH)
}
fn get_stat_atime_ns(st: &std::os::linux::raw::stat) -> i64 {
    st.st_atime_nsec as i64
}
fn get_stat_birthtime(st: &std::fs::Metadata) -> std::time::SystemTime {
    std::time::UNIX_EPOCH - std::time::Duration::new(1, 0)
}
fn get_stat_birthtime_ns(st: &std::os::linux::raw::stat) -> i64 {
    0
}
fn get_stat_ctime(st: &std::fs::Metadata) -> std::time::SystemTime {
    st.created().unwrap_or(std::time::SystemTime::UNIX_EPOCH)
}
fn get_stat_ctime_ns(st: &libc::stat) -> i64 {
    st.st_ctime_nsec
}
fn get_stat_mtime(st: &std::fs::Metadata) -> std::time::SystemTime {
    st.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH)
}
fn get_stat_mtime_ns(st: &std::fs::Metadata) -> i64 {
    st.modified()
        .expect("Failed to get modified time")
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .subsec_nanos() as i64
}
fn get_suffix(name: &str) -> Option<String> {
    let mut suffix = String::new();
    let known_suffixes = [
        None,
        Some(".gz"),
        Some(".z"),
        Some(".taz"),
        Some(".tgz"),
        Some("-gz"),
        Some("-z"),
        Some("_z"),
        None,
        None,
    ];
    let mut suffix_of_builtin = false;
    let z_suffix = unsafe { Z_SUFFIX.unwrap_or("") };
    let z_len = unsafe { Z_LEN };

    for suf in known_suffixes.iter().skip(1).take_while(|s| s.is_some()) {
        let suflen = suf.unwrap().len();
        if z_len < suflen && z_suffix.ends_with(&suf.unwrap()[suflen - z_len..]) {
            suffix_of_builtin = true;
            break;
        }
    }

    let mut z_lower = xstrdup(z_suffix);
    let mut z_lower_vec = z_lower.into_vec(); // Convert Box<[u8]> to Vec<u8>
    let mut z_lower_string = String::from_utf8(z_lower_vec).unwrap(); // Convert Vec<u8> to String
    strlwr(&mut z_lower_string);
    let z_lower_str = z_lower_string.as_str();

    let mut known_suffixes = known_suffixes;
    known_suffixes[if suffix_of_builtin {
        known_suffixes.len() - 2
    } else {
        0
    }] = Some(z_lower_str);

    let suf = known_suffixes.iter().skip(if suffix_of_builtin { 1 } else { 0 });

    let nlen = name.len();
    if nlen <= 32 {
        suffix.push_str(name);
    } else {
        suffix.push_str(&name[nlen - 32..]);
    }
    strlwr(&mut suffix);
    let slen = suffix.len();

    let mut match_suffix = None;
    for s in suf.take_while(|s| s.is_some()) {
        let s = s.unwrap();
        let s_len = s.len();
        if slen > s_len && suffix.chars().nth(slen - s_len - 1) != Some('/') && suffix.ends_with(s) {
            match_suffix = Some(name[nlen - s_len..].to_string());
            break;
        }
    }

    // No need to free z_lower_string as Rust handles memory automatically
    match_suffix
}
fn getbits(n: i32) -> u64 {
    let shift_amount = (8 * 2 * std::mem::size_of::<u8>()) as i32 - n;
    let x = unsafe { BITBUF >> shift_amount as usize };
    fillbuf(n);
    x
}
fn getcrc() -> Ulg {
    std::sync::atomic::AtomicU64::new(unsafe { CRC }).load(std::sync::atomic::Ordering::Relaxed) ^ 0xffffffff
}
fn getprogname() -> Option<Box<str>> {
    std::env::args().next().map(|s| s.into_boxed_str())
}
fn gettime() -> libc::timespec {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe {
        libc::clock_gettime(libc::CLOCK_REALTIME, &mut ts);
    }
    ts
}
fn gettime_res() -> i64{todo!("proto")}
fn gzip_base_name(fname: &str) -> Option<String> {
    let mut fname = last_component(fname)?.to_string();
    if ('A' as u8) == ('a' as u8) {
        strlwr(&mut fname);
    }
    Some(fname)
}
fn gzip_error(m: &str) -> ! {
    let stderr = &mut std::io::stderr();
    let _ = rpl_fprintf(stderr, "\n{}: {}: {}\n", &[ArgValue::Str(PROGRAM_NAME.unwrap_or("unknown")), ArgValue::Bytes(unsafe { &IFNAME }), ArgValue::Str(m)]);
    abort_gzip();
}
fn help() {
    static HELP_MSG: &[&str] = &[
        "Compress or uncompress FILEs (by default, compress FILES in-place).",
        "",
        "Mandatory arguments to long options are mandatory for short options too.",
        "",
        "  -c, --stdout      write on standard output, keep original files unchanged",
        "  -d, --decompress  decompress",
        "  -f, --force       force overwrite of output file and compress links",
        "  -h, --help        give this help",
        "  -k, --keep        keep (don't delete) input files",
        "  -l, --list        list compressed file contents",
        "  -L, --license     display software license",
        "  -n, --no-name     do not save or restore the original name and timestamp",
        "  -N, --name        save or restore the original name and timestamp",
        "  -q, --quiet       suppress all warnings",
        "  -r, --recursive   operate recursively on directories",
        "      --rsyncable   make rsync-friendly archive",
        "  -S, --suffix=SUF  use suffix SUF on compressed files",
        "      --synchronous synchronous output (safer if system crashes, but slower)",
        "  -t, --test        test compressed file integrity",
        "  -v, --verbose     verbose mode",
        "  -V, --version     display version number",
        "  -1, --fast        compress faster",
        "  -9, --best        compress better",
        "",
        "With no FILE, or when FILE is -, read standard input.",
        "",
        "Report bugs to <bug-gzip@gnu.org>.",
    ];

    unsafe {
        __printf__("Usage: %s [OPTION]... [FILE]...\n", &[ArgValue::Str(PROGRAM_NAME.unwrap_or(""))]);
    }

    for &msg in HELP_MSG {
        __printf__("%s\n", &[ArgValue::Str(msg)]);
    }
}
fn huf_decode_start() {
    init_getbits();
    unsafe {
        BLOCKSIZE = 0;
    }
}
fn huft_build(b: &[u32], n: u32, s: u32, d: &[Ush], e: &[Ush], t: &mut Option<Box<Huft>>, m: &mut i32) -> Result<(), ()> {
    use std::convert::TryInto;
    let mut a: u32;
    let mut c = [0u32; 17];
    let mut f: u32;
    let mut g: i32;
    let mut h: i32;
    let mut i: u32;
    let mut j: u32;
    let mut k: i32;
    let mut l: i32;
    let mut p: std::slice::Iter<u32>;
    let mut q: *mut Huft = std::ptr::null_mut();
    let mut r: Huft;
    let mut u: [*mut Huft; 16] = [std::ptr::null_mut(); 16];
    let mut v = [0u32; 288];
    let mut w: i32;
    let mut x = [0u32; 17];
    let mut xp: std::slice::IterMut<u32>;
    let mut y: i32;
    let mut z: u32;

    c.fill(0);
    p = b.iter();
    i = n;
    while i != 0 {
        c[*p.next().unwrap() as usize] += 1;
        i -= 1;
    }

    if c[0] == n {
        let q = unsafe { malloc(3 * std::mem::size_of::<Huft>()) as *mut Huft };
        if q.is_null() {
            return Err(());
        }
        unsafe { HUFTS += 3 };
        unsafe {
            (*q.add(0)).v = HuftValue::T(Box::new(Huft { e: 0, b: 0, v: HuftValue::N(0) }));
            (*q.add(1)).e = 99;
            (*q.add(1)).b = 1;
            (*q.add(2)).e = 99;
            (*q.add(2)).b = 1;
        }
        *t = Some(unsafe { Box::from_raw(q.add(1)) });
        *m = 1;
        return Ok(());
    }

    l = *m;
    for j in 1..=16 {
        if c[j] != 0 {
            break;
        }
    }
    k = j as i32;
    if l < j as i32 {
        l = j as i32;
    }
    for i in (1..=16).rev() {
        if c[i] != 0 {
            break;
        }
    }
    g = i as i32;
    if l > i as i32 {
        l = i as i32;
    }
    *m = l;

    let mut y = 1 << j;
    while j < i {
        y -= c[j as usize];
        if y < 0 {
            return Err(());
        }
        j += 1;
        y <<= 1;
    }
    y -= c[i as usize];
    if y < 0 {
        return Err(());
    }
    c[i as usize] += y as u32;

    x[1] = 0;
    j = 0;
    p = c.iter().skip(1);
    xp = x.iter_mut().skip(2);
    while i > 0 {
        *xp.next().unwrap() = j + *p.next().unwrap();
        j = *xp.as_slice().last().unwrap();
        i -= 1;
    }

    p = b.iter();
    i = 0;
    while i < n {
        if let Some(&j) = p.next() {
            if j != 0 {
                v[x[j as usize] as usize] = i;
                x[j as usize] += 1;
            }
        }
        i += 1;
    }

    n = x[g as usize];
    x[0] = 0;
    p = v.iter();
    h = -1;
    w = -l;
    u[0] = std::ptr::null_mut();
    q = std::ptr::null_mut();
    z = 0;

    for k in k..=g {
        a = c[k as usize];
        while a != 0 {
            while k as i32 > w + l {
                h += 1;
                w += l;
                z = (g - w) as u32;
                if z > l as u32 {
                    z = l as u32;
                }
                f = 1 << (k as i32 - w);
                if f > a + 1 {
                    f -= a + 1;
                    xp = c.iter().skip(k as usize);
                    if (k as i32 - w) < z as i32 {
                        while (k as i32 - w) < z as i32 {
                            if (f <<= 1) <= *xp.next().unwrap() {
                                break;
                            }
                            f -= *xp.next().unwrap();
                        }
                    }
                }
                z = 1 << (k as i32 - w);
                q = unsafe { malloc(((z + 1) as usize * std::mem::size_of::<Huft>()) as usize) as *mut Huft };
                if q.is_null() {
                    if h != 0 {
                        unsafe { huft_free(Some(Box::from_raw(u[0]))) };
                    }
                    return Err(());
                }
                unsafe { HUFTS += z + 1 };
                *t = Some(unsafe { Box::from_raw(q.add(1)) });
                t = &mut unsafe { (*q).v.t };
                *t = None;
                u[h as usize] = q.add(1);
                if h != 0 {
                    x[h as usize] = i;
                    r = Huft {
                        b: l as Uch,
                        e: (16 + (k as i32 - w)) as Uch,
                        v: HuftValue::T(Box::new(Huft { e: 0, b: 0, v: HuftValue::N(0) })),
                    };
                    j = i >> (w - l);
                    unsafe { *u[h as usize - 1].add(j as usize) = r };
                }
            }
            r = Huft {
                b: (k as i32 - w) as Uch,
                e: if p.as_slice().len() == 0 {
                    99
                } else if *p.as_slice().get(0).unwrap() < s {
                    if *p.as_slice().get(0).unwrap() < 256 {
                        16
                    } else {
                        15
                    }
                } else {
                    e[*p.as_slice().get(0).unwrap() as usize - s as usize].try_into().unwrap()
                },
                v: if p.as_slice().len() == 0 {
                    HuftValue::N(0)
                } else if *p.as_slice().get(0).unwrap() < s {
                    HuftValue::N(*p.as_slice().get(0).unwrap() as Ush)
                } else {
                    HuftValue::N(d[*p.as_slice().get(0).unwrap() as usize - s as usize])
                },
            };
            if p.as_slice().len() != 0 {
                p.next();
            }
            f = 1 << (k as i32 - w);
            for j in (i >> w)..z {
                unsafe { *q.add(j as usize) = r };
            }
            for j in (1 << (k as i32 - 1)).. {
                if i & j != 0 {
                    i ^= j;
                } else {
                    break;
                }
            }
            i ^= j;
            while (i & ((1 << w) - 1)) != x[h as usize] {
                h -= 1;
                w -= l;
            }
            a -= 1;
        }
    }
    if y != 0 && g != 1 {
        Err(())
    } else {
        Ok(())
    }
}
fn huft_free(t: Option<Box<Huft>>) {
    let mut p = t;
    while let Some(mut current) = p {
        if let HuftValue::T(ref next) = current.v {
            p = Some(next.clone());
        } else {
            p = None;
        }
        std::mem::drop(current);
    }
}
fn icalloc<T>(n: IdxT, s: IdxT) -> Option<*mut T> {
    if (18446744073709551615_u64) < n as u64 {
        if s != 0 {
            return _gl_alloc_nomem().map(|ptr| ptr as *mut T);
        }
        return Some(Box::into_raw(Box::new([] as [T; 0])) as *mut T);
    }
    if (18446744073709551615_u64) < s as u64 {
        if n != 0 {
            return _gl_alloc_nomem().map(|ptr| ptr as *mut T);
        }
        return Some(Box::into_raw(Box::new([] as [T; 0])) as *mut T);
    }
    Some(Box::into_raw(calloc::<T>(n as usize * s as usize)) as *mut T)
}
fn imalloc(s: IdxT) -> Option<*mut u8> {
    if s >= 0 && (s as u64) <= 18446744073709551615 {
        Some(unsafe { malloc(s as usize) })
    } else {
        _gl_alloc_nomem().map(|ptr| ptr as *mut u8)
    }
}
fn inflate() -> i32 {
    let mut e: i32 = 0;
    let mut r: i32;
    let mut h: u32;
    unsafe {
        OUTCNT = 0;
        BK = 0;
        BB = 0;
        h = 0;
        loop {
            HUFTS = 0;
            r = match inflate_block(&mut e) {
                Ok(_) => 0,
                Err(_) => return -1,
            };
            if r != 0 {
                return r;
            }
            if HUFTS > h {
                h = HUFTS;
            }
            if e != 0 {
                break;
            }
        }
        while BK >= 8 {
            BK -= 8;
            INPTR -= 1;
        }
        OUTCNT = OUTCNT;
        let _ = flush_window();
    }
    0
}
fn inflate_block(e: &mut i32) -> Result<(), ()> {
    let mut t: u32;
    let mut w: u32;
    let mut b: Ulg;
    let mut k: u32;
    unsafe {
        b = BB;
        k = BK;
        w = OUTCNT;
        while k < 1 {
            b |= ((INPTR < INSIZE).then(|| INBUF[INPTR as usize]).unwrap_or_else(|| {
                OUTCNT = w;
                fill_inbuf(false).unwrap_or(0)
            }) as Ulg)
                << k;
            INPTR += 1;
            k += 8;
        }
        *e = (b & 1) as i32;
        b >>= 1;
        k -= 1;
        while k < 2 {
            b |= ((INPTR < INSIZE).then(|| INBUF[INPTR as usize]).unwrap_or_else(|| {
                OUTCNT = w;
                fill_inbuf(false).unwrap_or(0)
            }) as Ulg)
                << k;
            INPTR += 1;
            k += 8;
        }
        t = (b & 3) as u32;
        b >>= 2;
        k -= 2;
        BB = b;
        BK = k;
    }
    match t {
        2 => inflate_dynamic(),
        0 => inflate_stored(),
        1 => inflate_fixed(),
        _ => Ok(()),
    }
}
fn inflate_codes(tl: &[Huft], td: &[Huft], bl: usize, bd: usize) -> Result<(), ()> {
    use std::convert::TryInto;
    let mut e;
    let mut n;
    let mut d;
    let mut w;
    let mut t;
    let ml;
    let md;
    let mut b;
    let mut k;
    unsafe {
        b = BB;
        k = BK;
        w = OUTCNT as usize;
    }
    ml = MASK_BITS[bl];
    md = MASK_BITS[bd];
    loop {
        while k < bl.try_into().unwrap() {
            b |= (unsafe {
                if INPTR < INSIZE {
                    INBUF[INPTR as usize]
                } else {
                    OUTCNT = w as u32;
                    fill_inbuf(false)?
                }
            } as Ulg) << k;
            k += 8;
        }
        t = &tl[(b & ml as Ulg) as usize];
        e = t.e;
        while e > 16 {
            if e == 99 {
                return Err(());
            }
            b >>= t.b as Ulg;
            k -= t.b as u32;
            e -= 16;
            while k < e.into() {
                b |= (unsafe {
                    if INPTR < INSIZE {
                        INBUF[INPTR as usize]
                    } else {
                        OUTCNT = w as u32;
                        fill_inbuf(false)?
                    }
                } as Ulg) << k;
                k += 8;
            }
            t = match &t.v {
                HuftValue::T(next) => &next[((b & MASK_BITS[e as usize] as Ulg) as usize)],
                _ => return Err(()),
            };
            e = t.e;
        }
        b >>= t.b as Ulg;
        k -= t.b as u32;
        if e == 16 {
            unsafe {
                WINDOW[w] = match t.v {
                    HuftValue::N(n) => n as Uch,
                    _ => return Err(()),
                };
            }
            w += 1;
            if w == 0x8000 {
                unsafe {
                    OUTCNT = w as u32;
                    flush_window()?;
                }
                w = 0;
            }
        } else {
            if e == 15 {
                break;
            }
            while k < e.into() {
                b |= (unsafe {
                    if INPTR < INSIZE {
                        INBUF[INPTR as usize]
                    } else {
                        OUTCNT = w as u32;
                        fill_inbuf(false)?
                    }
                } as Ulg) << k;
                k += 8;
            }
            n = match t.v {
                HuftValue::N(n) => n as usize,
                _ => return Err(()),
            } + ((b & MASK_BITS[e as usize] as Ulg) as usize);
            b >>= e as Ulg;
            k -= e as u32;
            while k < bd.try_into().unwrap() {
                b |= (unsafe {
                    if INPTR < INSIZE {
                        INBUF[INPTR as usize]
                    } else {
                        OUTCNT = w as u32;
                        fill_inbuf(false)?
                    }
                } as Ulg) << k;
                k += 8;
            }
            t = &td[(b & md as Ulg) as usize];
            e = t.e;
            while e > 16 {
                if e == 99 {
                    return Err(());
                }
                b >>= t.b as Ulg;
                k -= t.b as u32;
                e -= 16;
                while k < e.into() {
                    b |= (unsafe {
                        if INPTR < INSIZE {
                            INBUF[INPTR as usize]
                        } else {
                            OUTCNT = w as u32;
                            fill_inbuf(false)?
                        }
                    } as Ulg) << k;
                    k += 8;
                }
                t = match &t.v {
                    HuftValue::T(next) => &next[((b & MASK_BITS[e as usize] as Ulg) as usize)],
                    _ => return Err(()),
                };
                e = t.e;
            }
            b >>= t.b as Ulg;
            k -= t.b as u32;
            while k < e.into() {
                b |= (unsafe {
                    if INPTR < INSIZE {
                        INBUF[INPTR as usize]
                    } else {
                        OUTCNT = w as u32;
                        fill_inbuf(false)?
                    }
                } as Ulg) << k;
                k += 8;
            }
            d = w - match t.v {
                HuftValue::N(n) => n as usize,
                _ => return Err(()),
            } - ((b & MASK_BITS[e as usize] as Ulg) as usize);
            b >>= e as Ulg;
            k -= e as u32;
            loop {
                let e = if 0x8000 - (d & 0x7FFF) > w {
                    0x8000 - (d & 0x7FFF)
                } else {
                    w
                };
                let e = if e > n { n } else { e };
                n -= e;
                if e <= (if d < w { w - d } else { d - w }) {
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            WINDOW.as_ptr().add(d),
                            WINDOW.as_mut_ptr().add(w),
                            e,
                        );
                    }
                    w += e;
                    d += e;
                } else {
                    for _ in 0..e {
                        unsafe {
                            WINDOW[w] = WINDOW[d];
                        }
                        w += 1;
                        d += 1;
                    }
                }
                if w == 0x8000 {
                    unsafe {
                        OUTCNT = w as u32;
                        flush_window()?;
                    }
                    w = 0;
                }
                if n == 0 {
                    break;
                }
            }
        }
    }
    unsafe {
        OUTCNT = w as u32;
        BB = b;
        BK = k;
    }
    Ok(())
}
fn inflate_dynamic() -> Result<(), ()> {
    let mut i: usize;
    let mut j: usize;
    let mut l: usize;
    let mut m: usize;
    let mut n: usize;
    let mut w: u32;
    let mut tl: Option<Box<Huft>> = None;
    let mut td: Option<Box<Huft>> = None;
    let mut bl: i32;
    let mut bd: i32;
    let mut nb: usize;
    let mut nl: usize;
    let mut nd: usize;
    let mut ll = [0u32; 286 + 30];
    let mut b: Ulg;
    let mut k: usize;

    unsafe {
        b = BB;
        k = BK as usize;
        w = OUTCNT;
    }

    while k < 5 {
        b |= (unsafe { INPTR < INSIZE } as Ulg * unsafe { INBUF[INPTR as usize] as Ulg }) << k;
        k += 8;
        unsafe { INPTR += 1; }
    }
    nl = 257 + (b & 0x1f) as usize;
    b >>= 5;
    k -= 5;

    while k < 5 {
        b |= (unsafe { INPTR < INSIZE } as Ulg * unsafe { INBUF[INPTR as usize] as Ulg }) << k;
        k += 8;
        unsafe { INPTR += 1; }
    }
    nd = 1 + (b & 0x1f) as usize;
    b >>= 5;
    k -= 5;

    while k < 4 {
        b |= (unsafe { INPTR < INSIZE } as Ulg * unsafe { INBUF[INPTR as usize] as Ulg }) << k;
        k += 8;
        unsafe { INPTR += 1; }
    }
    nb = 4 + (b & 0xf) as usize;
    b >>= 4;
    k -= 4;

    if nl > 286 || nd > 30 {
        return Err(());
    }

    for j in 0..nb {
        while k < 3 {
            b |= (unsafe { INPTR < INSIZE } as Ulg * unsafe { INBUF[INPTR as usize] as Ulg }) << k;
            k += 8;
            unsafe { INPTR += 1; }
        }
        ll[unsafe { BORDER[j] } as usize] = (b & 7) as u32;
        b >>= 3;
        k -= 3;
    }

    for j in nb..19 {
        ll[unsafe { BORDER[j] } as usize] = 0;
    }

    bl = 7;
    if huft_build(&ll, 19, 19, &[], &[], &mut tl, &mut bl)? != 0 {
        if tl.is_some() {
            huft_free(tl);
        }
        return Err(());
    }

    if tl.is_none() {
        return Err(());
    }

    n = nl + nd;
    m = unsafe { MASK_BITS[bl as usize] } as usize;
    i = 0;
    l = 0;

    while i < n {
        while k < bl as usize {
            b |= (unsafe { INPTR < INSIZE } as Ulg * unsafe { INBUF[INPTR as usize] as Ulg }) << k;
            k += 8;
            unsafe { INPTR += 1; }
        }
        let td_ref = &tl.as_ref().unwrap()[((b & m as Ulg) as usize)];
        j = td_ref.b as usize;
        b >>= j;
        k -= j;

        match td_ref.e {
            99 => {
                huft_free(tl);
                return Err(());
            }
            _ => {
                j = match td_ref.v {
                    HuftValue::N(n) => n as usize,
                    _ => return Err(()),
                };
                if j < 16 {
                    ll[i] = j as u32;
                    l = j;
                    i += 1;
                } else if j == 16 {
                    while k < 2 {
                        b |= (unsafe { INPTR < INSIZE } as Ulg * unsafe { INBUF[INPTR as usize] as Ulg }) << k;
                        k += 8;
                        unsafe { INPTR += 1; }
                    }
                    j = 3 + (b & 3) as usize;
                    b >>= 2;
                    k -= 2;
                    if i + j > n {
                        return Err(());
                    }
                    for _ in 0..j {
                        ll[i] = l as u32;
                        i += 1;
                    }
                } else if j == 17 {
                    while k < 3 {
                        b |= (unsafe { INPTR < INSIZE } as Ulg * unsafe { INBUF[INPTR as usize] as Ulg }) << k;
                        k += 8;
                        unsafe { INPTR += 1; }
                    }
                    j = 3 + (b & 7) as usize;
                    b >>= 3;
                    k -= 3;
                    if i + j > n {
                        return Err(());
                    }
                    for _ in 0..j {
                        ll[i] = 0;
                        i += 1;
                    }
                    l = 0;
                } else {
                    while k < 7 {
                        b |= (unsafe { INPTR < INSIZE } as Ulg * unsafe { INBUF[INPTR as usize] as Ulg }) << k;
                        k += 8;
                        unsafe { INPTR += 1; }
                    }
                    j = 11 + (b & 0x7f) as usize;
                    b >>= 7;
                    k -= 7;
                    if i + j > n {
                        return Err(());
                    }
                    for _ in 0..j {
                        ll[i] = 0;
                        i += 1;
                    }
                    l = 0;
                }
            }
        }
    }

    huft_free(tl);
    unsafe {
        BB = b;
        BK = k as u32;
    }

    bl = LBITS;
    if huft_build(&ll, nl as u32, 257, &CPLENS, &CPLEXT, &mut tl, &mut bl)? != 0 {
        if tl.is_some() {
            huft_free(tl);
        }
        return Err(());
    }

    bd = DBITS;
    if huft_build(&ll[nl..], nd as u32, 0, &CPDIST, &CPDEXT, &mut td, &mut bd)? != 0 {
        if td.is_some() {
            huft_free(td);
        }
        huft_free(tl);
        return Err(());
    }

    let err = inflate_codes(tl.as_ref().unwrap(), td.as_ref().unwrap(), bl as usize, bd as usize).is_err();
    huft_free(tl);
    huft_free(td);
    if err {
        Err(())
    } else {
        Ok(())
    }
}
fn inflate_fixed() -> Result<(), ()> {
    use std::convert::TryInto;
    let mut l = [0u32; 288];
    for i in 0..144 {
        l[i] = 8;
    }
    for i in 144..256 {
        l[i] = 9;
    }
    for i in 256..280 {
        l[i] = 7;
    }
    for i in 280..288 {
        l[i] = 8;
    }
    let mut bl = 7;
    let mut tl: Option<Box<Huft>> = None;
    if huft_build(&l, 288, 257, &CPLENS, &CPLEXT, &mut tl, &mut bl).is_err() {
        return Err(());
    }
    for i in 0..30 {
        l[i] = 5;
    }
    let mut bd = 5;
    let mut td: Option<Box<Huft>> = None;
    if huft_build(&l, 30, 0, &CPDIST, &CPDEXT, &mut td, &mut bd).is_err() {
        huft_free(tl);
        return Err(());
    }
    if inflate_codes(tl.as_deref().unwrap(), td.as_deref().unwrap(), bl.try_into().unwrap(), bd.try_into().unwrap()).is_err() {
        huft_free(tl);
        huft_free(td);
        return Err(());
    }
    huft_free(tl);
    huft_free(td);
    Ok(())
}
fn inflate_stored() -> Result<(), ()> {
    let mut n: u32;
    let mut w: u32;
    let mut b: Ulg;
    let mut k: u32;
    unsafe {
        b = BB;
        k = BK;
        w = OUTCNT;
    }
    n = k & 7;
    b >>= n;
    k -= n;
    while k < 16 {
        b |= (unsafe {
            if INPTR < INSIZE {
                INBUF[INPTR as usize]
            } else {
                OUTCNT = w;
                fill_inbuf(false)?
            }
        } as Ulg) << k;
        k += 8;
    }
    n = (b & 0xffff) as u32;
    b >>= 16;
    k -= 16;
    while k < 16 {
        b |= (unsafe {
            if INPTR < INSIZE {
                INBUF[INPTR as usize]
            } else {
                OUTCNT = w;
                fill_inbuf(false)?
            }
        } as Ulg) << k;
        k += 8;
    }
    if n != ((!b) & 0xffff) as u32 {
        return Err(());
    }
    b >>= 16;
    k -= 16;
    while n != 0 {
        while k < 8 {
            b |= (unsafe {
                if INPTR < INSIZE {
                    INBUF[INPTR as usize]
                } else {
                    OUTCNT = w;
                    fill_inbuf(false)?
                }
            } as Ulg) << k;
            k += 8;
        }
        unsafe {
            WINDOW[w as usize] = b as Uch;
        }
        w += 1;
        if w == 0x8000 {
            unsafe {
                OUTCNT = w;
                flush_window()?;
            }
            w = 0;
        }
        b >>= 8;
        k -= 8;
        n -= 1;
    }
    unsafe {
        OUTCNT = w;
        BB = b;
        BK = k;
    }
    Ok(())
}
fn init_block() {
    for n in 0..(256 + 1 + 29) {
        unsafe { crate::DYN_LTREE[n].fc = crate::FreqOrCode::Freq(0) };
    }
    for n in 0..30 {
        unsafe { crate::DYN_DTREE[n].fc = crate::FreqOrCode::Freq(0) };
    }
    for n in 0..19 {
        unsafe { crate::BL_TREE[n].fc = crate::FreqOrCode::Freq(0) };
    }
    unsafe { crate::DYN_LTREE[256].fc = crate::FreqOrCode::Freq(1) };
    unsafe {
        crate::OPT_LEN = 0;
        crate::STATIC_LEN = 0;
        crate::LAST_LIT = 0;
        crate::LAST_DIST = 0;
        crate::LAST_FLAGS = 0;
        crate::FLAGS = 0;
        crate::FLAG_BIT = 1;
    }
}
fn init_getbits() {
    unsafe {
        BITBUF = 0;
        SUBBITBUF = 0;
        BITCOUNT = 0;
    }
    fillbuf((8 * 2 * std::mem::size_of::<u8>() as i32));
}
fn input_eof() -> bool {
    if unsafe { DECOMPRESS == 0 || LAST_MEMBER != 0 } {
        return true;
    }
    if unsafe { INPTR == INSIZE } {
        if unsafe { INSIZE != 0x40000 } || crate::fill_inbuf(true).is_err() {
            return true;
        }
        unsafe {
            INPTR = 0;
        }
    }
    false
}
fn install_signal_handlers() {
    let nsigs = HANDLED_SIG.len();
    let mut act = std::mem::MaybeUninit::<libc::sigaction>::uninit();
    unsafe {
        libc::sigemptyset(&mut CAUGHT_SIGNALS);
        for &sig in HANDLED_SIG.iter().take(nsigs) {
            libc::sigaction(sig, std::ptr::null(), act.as_mut_ptr());
            let act = act.assume_init();
            if act.sa_sigaction != libc::SIG_IGN {
                libc::sigaddset(&mut CAUGHT_SIGNALS, sig);
            }
        }
        let mut act = libc::sigaction {
            sa_sigaction: abort_gzip_signal as usize,
            sa_mask: CAUGHT_SIGNALS,
            sa_flags: 0,
            sa_restorer: None,
        };
        for &sig in HANDLED_SIG.iter().take(nsigs) {
            if libc::sigismember(&CAUGHT_SIGNALS, sig) == 1 {
                if sig == HANDLED_SIG[0] {
                    FOREGROUND.store(1, Ordering::SeqCst);
                }
                libc::sigaction(sig, &act, std::ptr::null_mut());
            }
        }
    }
}
fn irealloc(p: *mut u8, s: IdxT) -> Option<*mut u8> {
    if s <= (18446744073709551615u64 as IdxT) {
        let new_size = if s == 0 { 1 } else { s as usize };
        let ptr = unsafe { realloc(p, new_size) };
        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    } else {
        // Convert the result of _gl_alloc_nomem() to Option<*mut u8>
        _gl_alloc_nomem().map(|ptr| ptr as *mut u8)
    }
}
fn ireallocarray(p: *mut (), n: IdxT, s: IdxT) -> Result<*mut (), ()> {
    let (n, s) = if n == 0 || s == 0 { (1, 1) } else { (n, s) };
    if n <= 18446744073709551615 && s <= 18446744073709551615 {
        std::alloc::realloc(p as *mut u8, std::alloc::Layout::array::<u8>(n as usize * s as usize).unwrap(), n as usize * s as usize)
            .as_mut()
            .map(|ptr| ptr as *mut ())
            .ok_or(())
    } else {
        _gl_alloc_nomem().ok_or(())
    }
}
fn last_component(name: &str) -> Option<&str> {
    let mut base = name.trim_start_matches('/');
    let mut last_was_slash = false;
    for (i, c) in base.char_indices() {
        if c == '/' {
            last_was_slash = true;
        } else if last_was_slash {
            base = &base[i..];
            last_was_slash = false;
        }
    }
    if base.is_empty() {
        None
    } else {
        Some(base)
    }
}
fn license() {
    let p = LICENSE_MSG;
    if let Some(program_name) = unsafe { PROGRAM_NAME } {
        __printf__("%s %s\n", &[ArgValue::Str(program_name), ArgValue::Str(VERSION)]);
    }
    for &msg in p {
        __printf__("%s\n", &[ArgValue::Str(msg)]);
    }
}
fn lm_init(pack_level: i32) -> Result<(), ()> {
    if pack_level < 1 || pack_level > 9 {
        gzip_error("bad pack level");
    }

    unsafe {
        PREV[0x8000..].fill(0);

        RSYNC_CHUNK_END = 0xFFFFFFFF;
        RSYNC_SUM = 0;
        MAX_LAZY_MATCH = CONFIGURATION_TABLE[pack_level as usize].max_lazy as u32;
        GOOD_MATCH = CONFIGURATION_TABLE[pack_level as usize].good_length as u32;
        NICE_MATCH = CONFIGURATION_TABLE[pack_level as usize].nice_length as i32;
        MAX_CHAIN_LENGTH = CONFIGURATION_TABLE[pack_level as usize].max_chain as u32;
        STRSTART = 0;
        BLOCK_START = 0;

        LOOKAHEAD = match READ_BUF {
            Some(read_buf) => read_buf(&mut WINDOW[..if std::mem::size_of::<i32>() <= 2 { 0x8000 } else { 2 * 0x8000 }]),
            None => return Err(()),
        } as u32;

        if LOOKAHEAD == 0 || LOOKAHEAD == std::u32::MAX {
            EOFILE = 1;
            LOOKAHEAD = 0;
            return Ok(());
        }

        EOFILE = 0;

        while LOOKAHEAD < (258 + 3 + 1) && EOFILE == 0 {
            fill_window();
        }

        INS_H = 0;
        for j in 0..(3 - 1) {
            INS_H = ((INS_H << ((15 + 3 - 1) / 3)) ^ (WINDOW[j] as u32)) & ((1 << 15) - 1);
        }
    }

    Ok(())
}
fn longest_match(cur_match: IPos) -> usize {
    let mut chain_length = unsafe { MAX_CHAIN_LENGTH };
    let scan = &WINDOW[unsafe { STRSTART as usize }..];
    let mut best_len = unsafe { PREV_LENGTH } as usize;
    let limit = if unsafe { STRSTART } > (0x8000 - (258 + 3 + 1)) as u32 {
        unsafe { STRSTART } - (0x8000 - (258 + 3 + 1)) as u32
    } else {
        0
    };
    let strend = &WINDOW[(unsafe { STRSTART } + 258) as usize..];
    let mut scan_end1 = scan[best_len - 1];
    let mut scan_end = scan[best_len];

    if unsafe { PREV_LENGTH } >= unsafe { GOOD_MATCH } {
        chain_length >>= 2;
    }

    let mut cur_match = cur_match;
    while cur_match > limit && chain_length != 0 {
        let match_pos = cur_match as usize;
        let match_slice = &WINDOW[match_pos..];

        if match_slice[best_len] != scan_end ||
           match_slice[best_len - 1] != scan_end1 ||
           match_slice[0] != scan[0] ||
           match_slice[1] != scan[1] {
            cur_match = unsafe { PREV[cur_match as usize & (0x8000 - 1)].into() };
            chain_length -= 1;
        } else {
            let mut scan_iter = scan.iter().skip(2);
            let mut match_iter = match_slice.iter().skip(2);
            let mut len = 2;
            while let (Some(&s), Some(&m)) = (scan_iter.next(), match_iter.next()) {
                if s != m || len >= 258 {
                    break;
                }
                len += 1;
            }
            if len > best_len {
                unsafe { MATCH_START = cur_match; }
                best_len = len;
                if len >= unsafe { NICE_MATCH } as usize {
                    break;
                }
                scan_end1 = scan[best_len - 1];
                scan_end = scan[best_len];
            }
            cur_match = unsafe { PREV[cur_match as usize & (0x8000 - 1)].into() };
            chain_length -= 1;
        }
    }

    best_len
}
fn lutimens(file: &str, timespec: Option<&[libc::timespec; 2]>) -> Result<(), ()> {
    use std::os::unix::fs::MetadataExt;
    let mut adjusted_timespec = [libc::timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    let ts = if let Some(timespec) = timespec {
        adjusted_timespec[0] = timespec[0];
        adjusted_timespec[1] = timespec[1];
        Some(&mut adjusted_timespec)
    } else {
        None
    };
    let mut adjustment_needed = 0;
    if let Some(ts) = ts {
        adjustment_needed = validate_timespec(ts)?;
    }
    if adjustment_needed < 0 {
        return Err(());
    }
    unsafe {
        if LUTIMENSAT_WORKS_REALLY >= 0 {
            let result;
            if adjustment_needed == 2 {
                let st = match std::fs::symlink_metadata(file) {
                    Ok(st) => st,
                    Err(_) => return Err(()),
                };
                if let Some(ts) = ts {
                    if ts[0].tv_nsec == ((1 << 30) - 2) {
                        ts[0] = get_stat_atime(&st);
                    } else if ts[1].tv_nsec == ((1 << 30) - 2) {
                        ts[1] = get_stat_mtime(&st);
                    }
                }
                adjustment_needed += 1;
            }
            result = unsafe { libc::utimensat(libc::AT_FDCWD, file.as_ptr() as *const i8, ts.map_or(std::ptr::null(), |ts| ts.as_ptr()), libc::AT_SYMLINK_NOFOLLOW) };
            if result > 0 {
                unsafe { *libc::__errno_location() = 38; }
            }
            if result == 0 || unsafe { *libc::__errno_location() } != 38 {
                unsafe {
                    UTIMENSAT_WORKS_REALLY = 1;
                    LUTIMENSAT_WORKS_REALLY = 1;
                }
                return Ok(());
            }
        }
        LUTIMENSAT_WORKS_REALLY = -1;
    }
    if adjustment_needed != 0 {
        let st = match std::fs::symlink_metadata(file) {
            Ok(st) => st,
            Err(_) => return Err(()),
        };
        if let Some(ts) = ts {
            if update_timespec(&st, &mut Some(ts.clone())).is_ok() {
                return Ok(());
            }
        }
    }
    let st = match std::fs::symlink_metadata(file) {
        Ok(st) => st,
        Err(_) => return Err(()),
    };
    if (st.mode() & libc::S_IFMT) != libc::S_IFLNK {
        return fdutimens(-1, Some(file), ts.as_ref().map(|ts| &**ts));
    }
    unsafe { *libc::__errno_location() = 38; }
    Err(())
}
fn make_ofname() -> i32 {
    let mut ofname = unsafe { IFNAME };
    let mut suff = get_suffix(std::str::from_utf8(&ofname).unwrap());

    if unsafe { DECOMPRESS != 0 } {
        if suff.is_none() {
            if unsafe { !RECURSIVE.load(std::sync::atomic::Ordering::SeqCst) != 0 && TEST != 0 } {
                return 0;
            }
            if unsafe { VERBOSE != 0 || (!RECURSIVE.load(std::sync::atomic::Ordering::SeqCst) != 0 && QUIET == 0) } {
                if unsafe { QUIET == 0 } {
                    rpl_fprintf(
                        &mut std::fs::File::create("stderr.log").unwrap(), // Example: log to a file
                        "%s: %s: unknown suffix -- ignored\n",
                        &[
                            ArgValue::String(unsafe { PROGRAM_NAME.unwrap_or("").to_string() }),
                            ArgValue::String(std::str::from_utf8(&unsafe { IFNAME }).unwrap().to_string())
                        ],
                    ).ok();
                }
                if unsafe { EXIT_CODE == 0 } {
                    unsafe { EXIT_CODE = 2 };
                }
            }
            return 2;
        }
        let mut suff_str = suff.unwrap();
        strlwr(&mut suff_str);
        if suff_str == ".tgz" || suff_str == ".taz" {
            suff_str = ".tar".to_string();
        } else {
            suff_str.clear();
        }
    } else if suff.is_some() && unsafe { FORCE == 0 } {
        if unsafe { VERBOSE != 0 || (!RECURSIVE.load(std::sync::atomic::Ordering::SeqCst) != 0 && QUIET == 0) } {
            rpl_fprintf(
                &mut std::fs::File::create("stderr.log").unwrap(), // Example: log to a file
                "%s: %s already has %s suffix -- unchanged\n",
                &[
                    ArgValue::String(unsafe { PROGRAM_NAME.unwrap_or("").to_string() }),
                    ArgValue::String(std::str::from_utf8(&unsafe { IFNAME }).unwrap().to_string()),
                    ArgValue::String(suff.unwrap())
                ],
            ).ok();
        }
        return 2;
    } else {
        unsafe { SAVE_ORIG_NAME = 0 };
        if ofname.len() + unsafe { Z_LEN } >= 1024 {
            return goto_name_too_long();
        }
        ofname.clone_from_slice(unsafe { Z_SUFFIX.unwrap().as_bytes() });
    }
    return 0;

    fn goto_name_too_long() -> i32 {
        if unsafe { QUIET == 0 } {
            rpl_fprintf(
                &mut std::fs::File::create("stderr.log").unwrap(), // Example: log to a file
                "%s: %s: file name too long\n",
                &[
                    ArgValue::String(unsafe { PROGRAM_NAME.unwrap_or("").to_string() }),
                    ArgValue::String(std::str::from_utf8(&unsafe { IFNAME }).unwrap().to_string())
                ],
            ).ok();
        }
        if unsafe { EXIT_CODE == 0 } {
            unsafe { EXIT_CODE = 2 };
        }
        2
    }
}
fn make_simple_name(name: &mut String){todo!("proto")}
fn make_table(nchar: usize, bitlen: &[Uch], tablebits: usize, table: &mut [Ush]) -> Result<(), ()> {
    let mut count = [0u16; 17];
    let mut weight = [0u16; 17];
    let mut start = [0u16; 18];
    let mut prev = [0u16; 1 << 16];
    let mut i: usize;
    let mut k: usize;
    let mut len: usize;
    let mut ch: usize;
    let mut jutbits: usize;
    let mut avail: usize;
    let mut nextcode: usize;
    let mut mask: usize;

    for i in 1..=16 {
        count[i] = 0;
    }
    for &b in bitlen.iter().take(nchar) {
        count[b as usize] += 1;
    }
    start[1] = 0;
    for i in 1..=16 {
        start[i + 1] = start[i] + (count[i] << (16 - i));
    }
    if (start[17] & 0xffff) != 0 {
        return Err(());
    }
    jutbits = 16 - tablebits;
    for i in 1..=tablebits {
        start[i] >>= jutbits;
        weight[i] = 1 << (tablebits - i);
    }
    for i in (tablebits + 1)..=16 {
        weight[i] = 1 << (16 - i);
    }
    i = (start[tablebits + 1] as usize) >> jutbits;
    if i != 0 {
        k = 1 << tablebits;
        while i != k {
            table[i] = 0;
            i += 1;
        }
    }
    avail = nchar;
    mask = 1 << (15 - tablebits);
    for ch in 0..nchar {
        len = bitlen[ch] as usize;
        if len == 0 {
            continue;
        }
        nextcode = (start[len] as usize) + (weight[len] as usize);
        if len <= tablebits {
            if (1 << tablebits) < nextcode {
                return Err(());
            }
            for i in (start[len] as usize)..nextcode {
                table[i] = ch as Ush;
            }
        } else {
            k = start[len] as usize;
            let mut p_index = k >> jutbits;
            i = len - tablebits;
            while i != 0 {
                if table[p_index] == 0 {
                    prev[avail] = 0;
                    prev[avail + 0x8000] = 0;
                    table[p_index] = avail as Ush;
                    avail += 1;
                }
                if (k & mask) != 0 {
                    p_index = prev[table[p_index] as usize + 0x8000] as usize;
                } else {
                    p_index = prev[table[p_index] as usize] as usize;
                }
                k <<= 1;
                i -= 1;
            }
            prev[p_index] = ch as Ush;
        }
        start[len] = nextcode as u16;
    }
    Ok(())
}
fn make_timespec(s: i64, ns: i64) -> std::time::Duration {
    std::time::Duration::new(s as u64, ns as u32)
}
fn malloc(size: usize) -> *mut u8{todo!("proto")}
fn mdir_name(file: &str) -> Result<String, ()> {
    let length = match dir_len(file) {
        Some(len) => len,
        None => return Err(()),
    };
    let append_dot = length == 0 || (length == 0 && file.chars().nth(2) != Some('\0') && file.chars().nth(2) != Some('/'));
    let mut dir = String::with_capacity(length + append_dot as usize + 1);
    dir.push_str(&file[..length]);
    if append_dot {
        dir.push('.');
    }
    Ok(dir)
}
fn mfile_name_concat<'a>(dir: &'a str, base: &'a str) -> Option<(String, Option<usize>)> {
    let dirbase = last_component(dir)?;
    let dirbaselen = base_len(dirbase);
    let dirlen = dirbase.as_ptr() as usize - dir.as_ptr() as usize + dirbaselen;
    let baselen = base.len();
    let mut sep = '\0';

    if dirbaselen > 0 {
        if !dir[dirlen - 1..dirlen].eq("/") && !base.starts_with('/') {
            sep = '/';
        }
    } else if base.starts_with('/') {
        sep = '.';
    }

    let mut result = String::with_capacity(dirlen + (sep != '\0') as usize + baselen + 1);
    result.push_str(&dir[..dirlen]);
    if sep != '\0' {
        result.push(sep);
    }
    let base_start_index = result.len();
    result.push_str(base);

    Some((result, if sep != '\0' { Some(base_start_index) } else { None }))
}
fn mmalloca(n: usize) -> Option<*mut u8> {
    const SA_ALIGNMENT_MAX: usize = 8; // Define the alignment requirement here
    type SmallT = u8; // Define SmallT as needed
    type IdxT = usize; // Define IdxT as needed

    let alignment2_mask = 2 * SA_ALIGNMENT_MAX - 1;
    let plus = std::mem::size_of::<SmallT>() + alignment2_mask;
    let nplus: IdxT = n.checked_add(plus as usize)?;

    if nplus.checked_mul(1).is_some() {
        let mem = unsafe { malloc(nplus as usize) };
        if !mem.is_null() {
            let umem = mem as usize;
            let umemplus = umem.checked_add(std::mem::size_of::<SmallT>() + SA_ALIGNMENT_MAX - 1).unwrap();
            let offset = ((umemplus & !alignment2_mask) + SA_ALIGNMENT_MAX - umem) as IdxT;
            let vp = unsafe { mem.add(offset as usize) };
            let p = vp as *mut SmallT;
            unsafe {
                *p.offset(-1) = offset as SmallT;
            }
            return Some(p as *mut u8);
        }
    }
    None
}
fn my_main(args: Vec<String>) -> Result<(), ()> {
    use std::convert::TryInto;
    use std::ffi::CString;
    use std::os::raw::c_char;

    let mut file_count;
    let mut proglen;
    let mut argv_copy = args.clone();
    let mut env_argc = 0;
    let mut env_argv: Option<Vec<String>> = None;

    // Define missing variables
    let mut optind = 1;
    let mut optarg = String::new();
    const ENV_OPTION: i32 = 1000; // Example value, adjust as needed

    unsafe {
        // Assuming PROGRAM_NAME is a mutable static variable
        PROGRAM_NAME = Some(gzip_base_name(&args[0]).to_string());
        proglen = PROGRAM_NAME.as_ref().map_or(0, |s| s.len());
        if proglen > 4 && PROGRAM_NAME.as_ref().map_or(false, |s| s.ends_with(".exe")) {
            if let Some(ref mut name) = PROGRAM_NAME {
                let new_len = proglen - 4;
                *name = &name[..new_len].to_string();
            }
        }
        ENV = add_envopt(&mut env_argc, &mut argv_copy, "GZIP");
        env_argv = ENV.as_ref().map(|_| argv_copy.clone());
        Z_SUFFIX = Some(".gz");
        Z_LEN = Z_SUFFIX.as_ref().unwrap().len();
    }

    loop {
        let mut optc;
        let mut longind = -1;
        unsafe {
            if let Some(ref env_argv) = env_argv {
                if env_argv[optind] == "--" {
                    optc = ENV_OPTION + '-' as i32;
                } else {
                    // Convert Vec<String> to Vec<*mut c_char>
                    let c_env_argv: Vec<*mut c_char> = env_argv.iter()
                        .map(|arg| CString::new(arg.as_str()).unwrap().into_raw())
                        .collect();
                    optc = libc::getopt_long(env_argc.try_into().unwrap(), c_env_argv.as_ptr(), SHORTOPTS.as_ptr(), LONGOPTS.as_ptr(), &mut longind);
                    if optc >= 0 {
                        optc += ENV_OPTION;
                    } else {
                        if optind != env_argc {
                            rpl_fprintf(
                                &mut std::fs::File::create("/dev/stderr").unwrap(),
                                "%s: %s: non-option in GZIP environment variable\n",
                                &[PROGRAM_NAME.as_ref().unwrap(), env_argv[optind].as_str()],
                            );
                            try_help();
                        }
                        if env_argc != 1 && QUIET == 0 {
                            rpl_fprintf(
                                &mut std::fs::File::create("/dev/stderr").unwrap(),
                                "%s: warning: GZIP environment variable is deprecated; use an alias or script\n",
                                &[PROGRAM_NAME.as_ref().unwrap()],
                            );
                        }
                        rpl_free(c_env_argv.as_ptr() as *mut std::ffi::c_void);
                        env_argv = None;
                        optind = 1;
                        longind = -1;
                    }
                }
            }
            if env_argv.is_none() {
                // Convert Vec<String> to Vec<*mut c_char>
                let c_args: Vec<*mut c_char> = args.iter()
                    .map(|arg| CString::new(arg.as_str()).unwrap().into_raw())
                    .collect();
                optc = libc::getopt_long(args.len().try_into().unwrap(), c_args.as_ptr(), SHORTOPTS.as_ptr(), LONGOPTS.as_ptr(), &mut longind);
            }
            if optc < 0 {
                break;
            }
            match optc {
                x if x == b'a' as i32 => ASCII = 1,
                x if x == b'b' as i32 => {
                    MAXBITS = optarg.parse().unwrap_or(0);
                    for c in optarg.chars() {
                        if !c.is_digit(10) {
                            rpl_fprintf(
                                &mut std::fs::File::create("/dev/stderr").unwrap(),
                                "%s: -b operand is not an integer\n",
                                &[PROGRAM_NAME.as_ref().unwrap()],
                            );
                            try_help();
                        }
                    }
                }
                x if x == b'c' as i32 => TO_STDOUT.store(1, std::sync::atomic::Ordering::SeqCst),
                x if x == b'd' as i32 => DECOMPRESS = 1,
                x if x == b'f' as i32 => FORCE += 1,
                x if x == b'h' as i32 || x == b'H' as i32 => {
                    help();
                    finish_out();
                }
                x if x == b'k' as i32 => KEEP = 1,
                x if x == b'l' as i32 => {
                    LIST = 1;
                    DECOMPRESS = 1;
                    TEST = 1;
                    TO_STDOUT.store(1, std::sync::atomic::Ordering::SeqCst);
                }
                x if x == b'L' as i32 => {
                    license();
                    finish_out();
                }
                x if x == b'm' as i32 => NO_TIME = 1,
                x if x == b'M' as i32 => NO_TIME = 0,
                x if x == b'n' as i32 || x == b'n' as i32 + ENV_OPTION => {
                    NO_NAME = 1;
                    NO_TIME = 1;
                }
                x if x == b'N' as i32 || x == b'N' as i32 + ENV_OPTION => {
                    NO_NAME = 0;
                    NO_TIME = 0;
                }
                PRESUME_INPUT_TTY_OPTION => PRESUME_INPUT_TTY = true,
                x if x == b'q' as i32 || x == b'q' as i32 + ENV_OPTION => {
                    QUIET = 1;
                    VERBOSE = 0;
                }
                x if x == b'r' as i32 => RECURSIVE.store(1, std::sync::atomic::Ordering::SeqCst),
                RSYNCABLE_OPTION | RSYNCABLE_OPTION + ENV_OPTION => RSYNC = 1,
                x if x == b'S' as i32 => {
                    Z_LEN = optarg.len();
                    Z_SUFFIX = Some(optarg.clone());
                }
                SYNCHRONOUS_OPTION => SYNCHRONOUS = true,
                x if x == b't' as i32 => {
                    TEST = 1;
                    DECOMPRESS = 1;
                    TO_STDOUT.store(1, std::sync::atomic::Ordering::SeqCst);
                }
                x if x == b'v' as i32 || x == b'v' as i32 + ENV_OPTION => {
                    VERBOSE += 1;
                    QUIET = 0;
                }
                x if x == b'V' as i32 => {
                    version();
                    finish_out();
                }
                x if x == b'Z' as i32 => {
                    rpl_fprintf(
                        &mut std::fs::File::create("/dev/stderr").unwrap(),
                        "%s: -Z not supported in this version\n",
                        &[PROGRAM_NAME.as_ref().unwrap().as_str()],
                    );
                    try_help();
                }
                x if (b'1' as i32 + ENV_OPTION..=b'9' as i32 + ENV_OPTION).contains(&x) => {
                    optc -= ENV_OPTION;
                    LEVEL = optc - b'0' as i32;
                }
                x if (b'1' as i32..=b'9' as i32).contains(&x) => LEVEL = optc - b'0' as i32,
                _ => {
                    if optc >= ENV_OPTION && optc != ENV_OPTION + b'?' as i32 {
                        rpl_fprintf(&mut std::fs::File::create("/dev/stderr").unwrap(), "%s: ", &[PROGRAM_NAME.as_ref().unwrap().as_str()]);
                        if longind < 0 {
                            rpl_fprintf(&mut std::fs::File::create("/dev/stderr").unwrap(), "-%c: ", &[(optc - ENV_OPTION) as u8 as char]);
                        } else {
                            rpl_fprintf(&mut std::fs::File::create("/dev/stderr").unwrap(), "--%s: ", &[LONGOPTS[longind as usize].name.into()]);
                        }
                        rpl_fprintf(
                            &mut std::fs::File::create("/dev/stderr").unwrap(),
                            "option not valid in GZIP environment variable\n",
                            &[],
                        );
                    }
                    try_help();
                }
            }
        }
    }

    unsafe {
        if NO_TIME < 0 {
            NO_TIME = DECOMPRESS;
        }
        if NO_NAME < 0 {
            NO_NAME = DECOMPRESS;
        }
        file_count = args.len() - optind;
        if ASCII != 0 && QUIET == 0 {
            rpl_fprintf(
                &mut std::fs::File::create("/dev/stderr").unwrap(),
                "%s: option --ascii ignored on this system\n",
                &[PROGRAM_NAME.as_ref().unwrap()],
            );
        }
        if Z_LEN == 0 || Z_LEN > 30 {
            rpl_fprintf(
                &mut std::fs::File::create("/dev/stderr").unwrap(),
                "%s: invalid suffix '%s'\n",
                &[PROGRAM_NAME.as_ref().unwrap(), Z_SUFFIX.as_ref().unwrap()],
            );
            do_exit(1);
        }
        EXITING_SIGNAL = if QUIET != 0 { 13 } else { 0 };
        install_signal_handlers();
        if file_count != 0 {
            if TO_STDOUT.load(std::sync::atomic::Ordering::SeqCst) != 0 && TEST == 0 && (DECOMPRESS == 0 || ASCII == 0) {
                // Do nothing
            }
            while optind < args.len() {
                treat_file(&args[optind])?;
                optind += 1;
            }
        } else {
            treat_stdin()?;
        }
        if STDIN_WAS_READ.load(std::sync::atomic::Ordering::SeqCst) && std::fs::File::open("/dev/stdin").is_err() {
            IFNAME.copy_from_slice(b"stdin");
            read_error();
        }
        if LIST != 0 {
            if QUIET == 0 && file_count > 1 {
                do_list(-1)?;
            }
            if rpl_fflush(Some(&mut std::fs::File::create("/dev/stdout").unwrap())) != Ok(()) {
                write_error();
            }
        }
        if TO_STDOUT.load(std::sync::atomic::Ordering::SeqCst) != 0
            && ((SYNCHRONOUS && std::fs::File::open("/dev/stdout").is_err() && std::io::Error::last_os_error().raw_os_error() != Some(22))
                || std::fs::File::open("/dev/stdout").is_err())
            && std::io::Error::last_os_error().raw_os_error() != Some(9)
        {
            write_error();
        }
        do_exit(EXIT_CODE);
    }
}
fn nonnull<T>(p: *const T) -> *const T {
    if p.is_null() {
        xalloc_die();
    }
    p
}
fn npgettext_aux<'a>(domain: &'a str, msg_ctxt_id: &'a str, msgid: &'a str, msgid_plural: &'a str, n: u64, category: i32) -> &'a str {
    let translation = if n == 1 { msg_ctxt_id } else { msgid_plural };
    if std::ptr::eq(translation, msg_ctxt_id) || std::ptr::eq(translation, msgid_plural) {
        if n == 1 { msgid } else { msgid_plural }
    } else {
        translation
    }
}
fn open_and_stat(name: &str, flags: i32, st: &mut libc::stat) -> Result<i32, ()> {
    let mut fd: i32;
    let mut atfd = libc::AT_FDCWD;
    let mut base = name;
    unsafe {
        if TO_STDOUT.load(std::sync::atomic::Ordering::SeqCst) == 0 && FORCE == 0 {
            if true {
                FLAGS |= 0o400000;
            } else {
                if libc::lstat(name.as_ptr() as *const i8, st) != 0 {
                    return Err(());
                } else if (st.st_mode & libc::S_IFMT) == libc::S_IFLNK {
                    *libc::__errno_location() = libc::ELOOP;
                    return Err(());
                }
            }
        }
        if KEEP == 0 {
            if let Some(b) = last_component(name) {
                if let Ok(f) = atdir_set(Some(name.as_bytes())) {
                    base = b;
                    atfd = f;
                }
            }
        }
        fd = openat_safer(atfd, base, FLAGS.into())?;
        if libc::fstat(fd, st) != 0 {
            let e = *libc::__errno_location();
            libc::close(fd);
            *libc::__errno_location() = e;
            return Err(());
        }
    }
    Ok(fd)
}
fn open_input_file(iname: &str, sbuf: &mut libc::stat) -> Result<i32, ()> {
    let mut ilen;
    let mut z_suffix_errno = 0;
    static SUFFIXES: [&str; 6] = [""; 6];
    let mut suf = &SUFFIXES[..];
    let mut s: Option<String> = None;
    let mut fd;
    let open_flags = libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_NONBLOCK
        | (unsafe { if ASCII != 0 && DECOMPRESS == 0 { 0 } else { 0 } });
    unsafe {
        SUFFIXES[0] = Z_SUFFIX.unwrap_or("");
    }
    if std::mem::size_of::<[u8; 1024]>() - 1 <= iname.len() {
        return name_too_long(iname);
    }
    unsafe {
        IFNAME[..iname.len()].copy_from_slice(iname.as_bytes());
    }
    fd = open_and_stat(iname, open_flags, sbuf)?;
    if fd >= 0 {
        return Ok(fd);
    }
    if unsafe { DECOMPRESS == 0 || *libc::__errno_location() != libc::ENOENT } {
        progerror(iname);
        return Err(());
    }
    s = get_suffix(iname);
    if s.is_some() {
        progerror(iname);
        return Err(());
    }
    ilen = iname.len();
    if unsafe { Z_SUFFIX == Some(".gz") } {
        suf = &suf[1..];
    }
    while let Some(&s0) = suf.first() {
        s = Some(s0.to_string());
        unsafe {
            IFNAME[..iname.len()].copy_from_slice(iname.as_bytes());
        }
        if let Some(ref s_str) = s {
            if std::mem::size_of::<[u8; 1024]>() <= ilen + s_str.len() {
                return name_too_long(iname);
            }
            unsafe {
                IFNAME[ilen..ilen + s_str.len()].copy_from_slice(s_str.as_bytes());
            }
            fd = open_and_stat(std::str::from_utf8(&IFNAME[..ilen + s_str.len()]).unwrap(), open_flags, sbuf)?;
            if fd >= 0 {
                return Ok(fd);
            }
            if unsafe { *libc::__errno_location() != libc::ENOENT } {
                progerror(std::str::from_utf8(&IFNAME[..ilen + s_str.len()]).unwrap());
                return Err(());
            }
            if unsafe { s0 == Z_SUFFIX.unwrap_or("") } {
                z_suffix_errno = unsafe { *libc::__errno_location() };
            }
        }
        suf = &suf[1..];
    }
    unsafe {
        IFNAME[..iname.len()].copy_from_slice(iname.as_bytes());
        if let Some(z_suffix) = Z_SUFFIX {
            IFNAME[iname.len()..iname.len() + z_suffix.len()]
                .copy_from_slice(z_suffix.as_bytes());
        }
        *libc::__errno_location() = z_suffix_errno;
    }
    progerror(std::str::from_utf8(&IFNAME[..iname.len() + Z_SUFFIX.unwrap_or("").len()]).unwrap());
    Err(())
}
fn open_safer(file: &str, flags: i32) -> Result<i32, ()> {
    let mut mode: libc::mode_t = 0;
    unsafe {
        if FLAGS & 0o100 != 0 {
            let mut ap: std::ffi::VaListImpl;
            std::ffi::VaListImpl::<>::with_copy(|va_list| {
                ap = va_list;
                mode = ap.arg::<libc::mode_t>();
            });
        }
    }
    fd_safer(unsafe { libc::open(file.as_ptr() as *const i8, flags, mode) })
}
fn openat_proc_name(buf: &mut [u8], fd: i32, file: &str) -> Result<*mut u8, ()> {
    let mut result = buf.as_mut_ptr();
    let dirlen;
    if file.is_empty() {
        buf[0] = 0;
        return Ok(buf.as_mut_ptr());
    }
    const PROC_SELF_FD_DIR_SIZE_BOUND: usize = {
        let int_bits = std::mem::size_of::<i32>() * 8;
        let max_digits = ((int_bits - (0 < -1) as usize) * 146 + 484) / 485 + (0 < -1) as usize;
        "/proc/self/fd/%d/".len() - "%d".len() + max_digits
    };
    static mut PROC_STATUS: i32 = 0;
    unsafe {
        if PROC_STATUS == 0 {
            let proc_self_fd = libc::open(
                b"/proc/self/fd\0".as_ptr() as *const i8,
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            );
            if proc_self_fd < 0 {
                PROC_STATUS = -1;
            } else {
                let mut dotdot_buf = [0u8; PROC_SELF_FD_DIR_SIZE_BOUND + "../fd".len() - 1];
                let dotdot_buf_ptr = dotdot_buf.as_mut_ptr() as *mut i8;
                libc::sprintf(
                    dotdot_buf_ptr,
                    b"/proc/self/fd/%d/../fd\0".as_ptr() as *const i8,
                    proc_self_fd,
                );
                PROC_STATUS = if libc::access(dotdot_buf_ptr, libc::F_OK) != 0 {
                    -1
                } else {
                    1
                };
                libc::close(proc_self_fd);
            }
        }
        if PROC_STATUS < 0 {
            return Err(());
        } else {
            let bufsize = PROC_SELF_FD_DIR_SIZE_BOUND + file.len();
            if buf.len() < bufsize {
                result = libc::malloc(bufsize) as *mut u8;
                if result.is_null() {
                    return Err(());
                }
            }
            dirlen = libc::sprintf(
                result as *mut i8,
                b"/proc/self/fd/%d/\0".as_ptr() as *const i8,
                fd,
            );
        }
    }
    unsafe {
        std::ptr::copy_nonoverlapping(file.as_ptr(), result.add(dirlen as usize), file.len());
        *result.add(dirlen as usize + file.len()) = 0;
    }
    Ok(result)
}
fn openat_restore_fail(errnum: i32) -> ! {
    error(EXIT_FAILURE, errnum, "failed to return to initial working directory");
    std::process::abort();
}
fn openat_safer(fd: i32, file: &str, flags: i32) -> Result<i32, ()> {
    let mut mode: libc::mode_t = 0;
    unsafe {
        if FLAGS & 0o100 != 0 {
            let mut ap = std::ffi::VaList::new();
            ap.with_copy(|ap| {
                mode = ap.arg::<libc::mode_t>();
            });
        }
    }
    fd_safer(unsafe { libc::openat(fd, std::ffi::CString::new(file).unwrap().as_ptr(), flags, mode) })
}
fn openat_save_fail(errnum: i32) -> ! {
    error(EXIT_FAILURE, errnum, "unable to record current working directory");
    std::process::abort();
}
fn opendir(dir_name: &str) -> Option<usize>{todo!("proto")}
fn opendir_safer(name: &str) -> Result<Box<usize>, ()> {
    if let Some(dp) = opendir(name) {
        let fd = unsafe { libc::dirfd(dp as *mut libc::DIR) };
        if (0..=2).contains(&fd) {
            let f = rpl_fcntl(fd, 1030)?;
            let newdp = fdopendir(f);
            let e = unsafe { *libc::__errno_location() };
            if newdp.is_none() {
                unsafe { libc::close(f) };
            }
            unsafe { libc::closedir(dp as *mut libc::DIR) };
            unsafe { *libc::__errno_location() = e };
            return newdp.ok_or(());
        }
        return Ok(Box::new(dp));
    }
    Err(())
}
fn pgettext_aux<'a>(domain: &'a str, msg_ctxt_id: &'a str, msgid: &'a str, category: i32) -> &'a str {
    let translation = {
        let _ = category;
        let _ = domain;
        msg_ctxt_id
    };
    if std::ptr::eq(translation, msg_ctxt_id) {
        msgid
    } else {
        translation
    }
}
fn pipe_safer() -> Result<[i32; 2], ()> {
    let mut fd = [0; 2];
    if unsafe { libc::pipe(fd.as_mut_ptr()) } == 0 {
        for i in 0..2 {
            match fd_safer(fd[i]) {
                Ok(safe_fd) => fd[i] = safe_fd,
                Err(_) => {
                    let saved_errno = unsafe { *libc::__errno_location() };
                    unsafe { libc::close(fd[1 - i]) };
                    unsafe { *libc::__errno_location() = saved_errno };
                    return Err(());
                }
            }
        }
        Ok(fd)
    } else {
        Err(())
    }
}
fn pqdownheap(tree: &mut [CtData], mut k: usize) {
    let mut v = unsafe { HEAP[k] };
    let mut j = k << 1;
    while j <= unsafe { HEAP_LEN as usize } {
        if j < unsafe { HEAP_LEN as usize } &&
           (match tree[unsafe { HEAP[j + 1] as usize }].fc {
                FreqOrCode::Freq(freq1) => freq1,
                FreqOrCode::Code(_) => 0,
            } < match tree[unsafe { HEAP[j] as usize }].fc {
                FreqOrCode::Freq(freq2) => freq2,
                FreqOrCode::Code(_) => 0,
            } || (match tree[unsafe { HEAP[j + 1] as usize }].fc {
                FreqOrCode::Freq(freq1) => freq1,
                FreqOrCode::Code(_) => 0,
            } == match tree[unsafe { HEAP[j] as usize }].fc {
                FreqOrCode::Freq(freq2) => freq2,
                FreqOrCode::Code(_) => 0,
            } && unsafe { DEPTH[unsafe { HEAP[j + 1] as usize }] } <= unsafe { DEPTH[unsafe { HEAP[j] as usize }] })) {
            j += 1;
        }
        if match tree[v as usize].fc {
            FreqOrCode::Freq(freq1) => freq1,
            FreqOrCode::Code(_) => 0,
        } < match tree[unsafe { HEAP[j] as usize }].fc {
            FreqOrCode::Freq(freq2) => freq2,
            FreqOrCode::Code(_) => 0,
        } || (match tree[v as usize].fc {
            FreqOrCode::Freq(freq1) => freq1,
            FreqOrCode::Code(_) => 0,
        } == match tree[unsafe { HEAP[j] as usize }].fc {
            FreqOrCode::Freq(freq2) => freq2,
            FreqOrCode::Code(_) => 0,
        } && unsafe { DEPTH[v as usize] } <= unsafe { DEPTH[unsafe { HEAP[j] as usize }] }) {
            break;
        }
        unsafe { HEAP[k] = HEAP[j] };
        k = j;
        j <<= 1;
    }
    unsafe { HEAP[k] = v };
}
fn printf_fetchargs(args: &[ArgValue], a: &mut Arguments) -> Result<(), ()> {
    for (i, ap) in a.arg.iter_mut().enumerate() {
        match ap.arg_type {
            ArgType::SChar => {
                if let ArgValue::Int(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::SChar(*val as i8);
                } else {
                    return Err(());
                }
            }
            ArgType::UChar => {
                if let ArgValue::Int(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::UChar(*val as u8);
                } else {
                    return Err(());
                }
            }
            ArgType::Short => {
                if let ArgValue::Int(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::Short(*val as i16);
                } else {
                    return Err(());
                }
            }
            ArgType::UShort => {
                if let ArgValue::Int(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::UShort(*val as u16);
                } else {
                    return Err(());
                }
            }
            ArgType::Int => {
                if let ArgValue::Int(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::Int(*val);
                } else {
                    return Err(());
                }
            }
            ArgType::UInt => {
                if let ArgValue::UInt(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::UInt(*val);
                } else {
                    return Err(());
                }
            }
            ArgType::LongInt => {
                if let ArgValue::LongInt(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::LongInt(*val);
                } else {
                    return Err(());
                }
            }
            ArgType::ULongInt => {
                if let ArgValue::ULongInt(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::ULongInt(*val);
                } else {
                    return Err(());
                }
            }
            ArgType::LongLongInt => {
                if let ArgValue::LongLongInt(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::LongLongInt(*val);
                } else {
                    return Err(());
                }
            }
            ArgType::ULongLongInt => {
                if let ArgValue::ULongLongInt(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::ULongLongInt(*val);
                } else {
                    return Err(());
                }
            }
            ArgType::Double => {
                if let ArgValue::Double(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::Double(*val);
                } else {
                    return Err(());
                }
            }
            ArgType::LongDouble => {
                if let ArgValue::LongDouble(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::LongDouble(*val);
                } else {
                    return Err(());
                }
            }
            ArgType::Char => {
                if let ArgValue::Int(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::Char(*val);
                } else {
                    return Err(());
                }
            }
            ArgType::WideChar => {
                if let ArgValue::WideChar(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::WideChar(*val);
                } else {
                    return Err(());
                }
            }
            ArgType::String => {
                if let ArgValue::String(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::String(val.clone());
                } else {
                    return Err(());
                }
            }
            ArgType::WideString => {
                if let ArgValue::WideString(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::WideString(val.clone());
                } else {
                    return Err(());
                }
            }
            ArgType::Pointer => {
                if let ArgValue::Pointer(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::Pointer(Box::new(val.clone()));
                } else {
                    return Err(());
                }
            }
            ArgType::CountSCharPointer => {
                if let ArgValue::CountSCharPointer(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::CountSCharPointer(val.clone());
                } else {
                    return Err(());
                }
            }
            ArgType::CountShortPointer => {
                if let ArgValue::CountShortPointer(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::CountShortPointer(val.clone());
                } else {
                    return Err(());
                }
            }
            ArgType::CountIntPointer => {
                if let ArgValue::CountIntPointer(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::CountIntPointer(val.clone());
                } else {
                    return Err(());
                }
            }
            ArgType::CountLongIntPointer => {
                if let ArgValue::CountLongIntPointer(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::CountLongIntPointer(val.clone());
                } else {
                    return Err(());
                }
            }
            ArgType::CountLongLongIntPointer => {
                if let ArgValue::CountLongLongIntPointer(val) = args.get(i).ok_or(())? {
                    ap.value = ArgValue::CountLongLongIntPointer(val.clone());
                } else {
                    return Err(());
                }
            }
            _ => return Err(()),
        }
    }
    Ok(())
}
fn printf_frexp(x: f64, expptr: &mut i32) -> f64 {
    if x == 0.0 {
        *expptr = 0;
        return 0.0;
    }

    let bits = x.to_bits();
    let exponent = ((bits >> 52) & 0x7FF) as i32 - 1022; // Extract the exponent
    let mantissa_bits = (bits & 0xFFFFFFFFFFFFF) | 0x10000000000000; // Extract the mantissa and set the implicit leading 1

    *expptr = exponent;
    f64::from_bits((0x3FF0000000000000 | mantissa_bits) & 0xFFFFFFFFFFFFF) * 0.5
}
fn printf_frexpl(x: f64) -> (f64, i32) {
    let mut exponent: i32 = 0;
    let mut x = x;
    x = f64::exp(x, &mut exponent);
    x = x + x;
    exponent -= 1;
    if exponent < (-16381) - 1 {
        x = f64::exp(x, exponent - ((-16381) - 1));
        exponent = (-16381) - 1;
    }
    (x, exponent)
}
fn printf_parse(format: &str, d: &mut CharDirectives, a: &mut Arguments) -> Result<(), ()> {
    let mut cp = format.chars();
    let mut arg_posn = 0;
    let mut d_allocated = 7;
    let mut a_allocated = 7;
    let mut max_width_length = 0;
    let mut max_precision_length = 0;
    d.count = 0;
    d.dir = d.direct_alloc_dir.to_vec();
    a.arg = a.direct_alloc_arg.to_vec();
    while let Some(c) = cp.next() {
        if c == '%' {
            let mut arg_index = usize::MAX;
            let dp = &mut d.dir[d.count];
            dp.dir_start = &format[format.len() - cp.as_str().len() - 1..];
            dp.flags = 0;
            dp.width_start = "";
            dp.width_end = "";
            dp.width_arg_index = usize::MAX;
            dp.precision_start = "";
            dp.precision_end = "";
            dp.precision_arg_index = usize::MAX;
            dp.arg_index = usize::MAX;
            if let Some(c) = cp.clone().next() {
                if c.is_digit(10) {
                    let mut np = cp.clone();
                    while let Some(c) = np.clone().next() {
                        if !c.is_digit(10) {
                            break;
                        }
                        np.next();
                    }
                    if let Some('$') = np.clone().next() {
                        let mut n = 0;
                        while let Some(c) = cp.clone().next() {
                            if !c.is_digit(10) {
                                break;
                            }
                            n = match xsum(n, c.to_digit(10).unwrap() as usize) {
                                Ok(val) => val,
                                Err(_) => return Err(()),
                            };
                            cp.next();
                        }
                        if n == 0 {
                            return Err(());
                        }
                        arg_index = n - 1;
                        // Manually advance the iterator instead of using skip
                        for _ in 0..1 {
                            np.next();
                        }
                        cp = np;
                    }
                }
            }
            loop {
                match cp.clone().next() {
                    Some('\'') => {
                        dp.flags |= 1;
                        cp.next();
                    }
                    Some('-') => {
                        dp.flags |= 2;
                        cp.next();
                    }
                    Some('+') => {
                        dp.flags |= 4;
                        cp.next();
                    }
                    Some(' ') => {
                        dp.flags |= 8;
                        cp.next();
                    }
                    Some('#') => {
                        dp.flags |= 16;
                        cp.next();
                    }
                    Some('0') => {
                        dp.flags |= 32;
                        cp.next();
                    }
                    Some('I') => {
                        dp.flags |= 64;
                        cp.next();
                    }
                    _ => break,
                }
            }
            if let Some('*') = cp.clone().next() {
                dp.width_start = &format[format.len() - cp.as_str().len()..];
                cp.next();
                dp.width_end = &format[format.len() - cp.as_str().len()..];
                if max_width_length < 1 {
                    max_width_length = 1;
                }
                if let Some(c) = cp.clone().next() {
                    if c.is_digit(10) {
                        let mut np = cp.clone();
                        while let Some(c) = np.clone().next() {
                            if !c.is_digit(10) {
                                break;
                            }
                            np.next();
                        }
                        if let Some('$') = np.clone().next() {
                            let mut n = 0;
                            while let Some(c) = cp.clone().next() {
                                if !c.is_digit(10) {
                                    break;
                                }
                                n = match xsum(n, c.to_digit(10).unwrap() as usize) {
                                    Ok(val) => val,
                                    Err(_) => return Err(()),
                                };
                                cp.next();
                            }
                            if n == 0 {
                                return Err(());
                            }
                            dp.width_arg_index = n - 1;
                            // Manually advance the iterator instead of using skip
                            for _ in 0..1 {
                                np.next();
                            }
                            cp = np;
                        }
                    }
                }
                if dp.width_arg_index == usize::MAX {
                    dp.width_arg_index = arg_posn;
                    arg_posn += 1;
                    if dp.width_arg_index == usize::MAX {
                        return Err(());
                    }
                }
                let n = dp.width_arg_index;
                if n >= a_allocated {
                    a_allocated = match xsum(a_allocated, a_allocated) {
                        Ok(val) => val,
                        Err(_) => return Err(()),
                    };
                    if a_allocated <= n {
                        a_allocated = match xsum(n, 1) {
                            Ok(val) => val,
                            Err(_) => return Err(()),
                        };
                    }
                    let memory_size = match xsum(a_allocated, std::mem::size_of::<Argument>()) {
                        Ok(val) => val,
                        Err(_) => return Err(()),
                    };
                    let memory = if a.arg.as_ptr() != a.direct_alloc_arg.as_ptr() {
                        std::alloc::realloc(a.arg.as_mut_ptr() as *mut u8, std::alloc::Layout::array::<Argument>(a_allocated).unwrap(), memory_size)
                    } else {
                        std::alloc::alloc(std::alloc::Layout::array::<Argument>(a_allocated).unwrap())
                    };
                    if memory.is_null() {
                        return Err(());
                    }
                    if a.arg.as_ptr() == a.direct_alloc_arg.as_ptr() {
                        std::ptr::copy_nonoverlapping(a.arg.as_ptr(), memory as *mut Argument, a.arg.len());
                    }
                    a.arg = std::slice::from_raw_parts_mut(memory as *mut Argument, a_allocated).to_vec();
                }
                while a.arg.len() <= n {
                    a.arg.push(Argument { arg_type: ArgType::None });
                }
                if a.arg[n].arg_type == ArgType::None {
                    a.arg[n].arg_type = ArgType::Int;
                } else if a.arg[n].arg_type != ArgType::Int {
                    return Err(());
                }
            } else if let Some(c) = cp.clone().next() {
                if c.is_digit(10) {
                    dp.width_start = &format[format.len() - cp.as_str().len()..];
                    while let Some(c) = cp.clone().next() {
                        if !c.is_digit(10) {
                            break;
                        }
                        cp.next();
                    }
                    dp.width_end = &format[format.len() - cp.as_str().len()..];
                    let width_length = dp.width_end.len() - dp.width_start.len();
                    if max_width_length < width_length {
                        max_width_length = width_length;
                    }
                }
            }
            if let Some('.') = cp.clone().next() {
                cp.next();
                if let Some('*') = cp.clone().next() {
                    dp.precision_start = &format[format.len() - cp.as_str().len() - 1..];
                    cp.next();
                    dp.precision_end = &format[format.len() - cp.as_str().len()..];
                    if max_precision_length < 2 {
                        max_precision_length = 2;
                    }
                    if let Some(c) = cp.clone().next() {
                        if c.is_digit(10) {
                            let mut np = cp.clone();
                            while let Some(c) = np.clone().next() {
                                if !c.is_digit(10) {
                                    break;
                                }
                                np.next();
                            }
                            if let Some('$') = np.clone().next() {
                                let mut n = 0;
                                while let Some(c) = cp.clone().next() {
                                    if !c.is_digit(10) {
                                        break;
                                    }
                                    n = match xsum(n, c.to_digit(10).unwrap() as usize) {
                                        Ok(val) => val,
                                        Err(_) => return Err(()),
                                    };
                                    cp.next();
                                }
                                if n == 0 {
                                    return Err(());
                                }
                                dp.precision_arg_index = n - 1;
                                // Manually advance the iterator instead of using skip
                                for _ in 0..1 {
                                    np.next();
                                }
                                cp = np;
                            }
                        }
                    }
                    if dp.precision_arg_index == usize::MAX {
                        dp.precision_arg_index = arg_posn;
                        arg_posn += 1;
                        if dp.precision_arg_index == usize::MAX {
                            return Err(());
                        }
                    }
                    let n = dp.precision_arg_index;
                    if n >= a_allocated {
                        a_allocated = match xsum(a_allocated, a_allocated) {
                            Ok(val) => val,
                            Err(_) => return Err(()),
                        };
                        if a_allocated <= n {
                            a_allocated = match xsum(n, 1) {
                                Ok(val) => val,
                                Err(_) => return Err(()),
                            };
                        }
                        let memory_size = match xsum(a_allocated, std::mem::size_of::<Argument>()) {
                            Ok(val) => val,
                            Err(_) => return Err(()),
                        };
                        let memory = if a.arg.as_ptr() != a.direct_alloc_arg.as_ptr() {
                            std::alloc::realloc(a.arg.as_mut_ptr() as *mut u8, std::alloc::Layout::array::<Argument>(a_allocated).unwrap(), memory_size)
                        } else {
                            std::alloc::alloc(std::alloc::Layout::array::<Argument>(a_allocated).unwrap())
                        };
                        if memory.is_null() {
                            return Err(());
                        }
                        if a.arg.as_ptr() == a.direct_alloc_arg.as_ptr() {
                            std::ptr::copy_nonoverlapping(a.arg.as_ptr(), memory as *mut Argument, a.arg.len());
                        }
                        a.arg = std::slice::from_raw_parts_mut(memory as *mut Argument, a_allocated).to_vec();
                    }
                    while a.arg.len() <= n {
                        a.arg.push(Argument { arg_type: ArgType::None });
                    }
                    if a.arg[n].arg_type == ArgType::None {
                        a.arg[n].arg_type = ArgType::Int;
                    } else if a.arg[n].arg_type != ArgType::Int {
                        return Err(());
                    }
                } else {
                    dp.precision_start = &format[format.len() - cp.as_str().len() - 1..];
                    while let Some(c) = cp.clone().next() {
                        if !c.is_digit(10) {
                            break;
                        }
                        cp.next();
                    }
                    dp.precision_end = &format[format.len() - cp.as_str().len()..];
                    let precision_length = dp.precision_end.len() - dp.precision_start.len();
                    if max_precision_length < precision_length {
                        max_precision_length = precision_length;
                    }
                }
            }
            let mut type_ = ArgType::None;
            let mut flags = 0;
            loop {
                match cp.clone().next() {
                    Some('h') => {
                        flags |= 1 << (flags & 1);
                        cp.next();
                    }
                    Some('L') => {
                        flags |= 4;
                        cp.next();
                    }
                    Some('l') => {
                        flags += 8;
                        cp.next();
                    }
                    Some('j') => {
                        if std::mem::size_of::<i64>() > std::mem::size_of::<i32>() {
                            flags += 16;
                        } else if std::mem::size_of::<i64>() > std::mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp.next();
                    }
                    Some('z') | Some('Z') => {
                        if std::mem::size_of::<usize>() > std::mem::size_of::<i32>() {
                            flags += 16;
                        } else if std::mem::size_of::<usize>() > std::mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp.next();
                    }
                    Some('t') => {
                        if std::mem::size_of::<isize>() > std::mem::size_of::<i32>() {
                            flags += 16;
                        } else if std::mem::size_of::<isize>() > std::mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp.next();
                    }
                    _ => break,
                }
            }
            let c = cp.next().unwrap();
            match c {
                'd' | 'i' => {
                    if flags >= 16 || (flags & 4) != 0 {
                        type_ = ArgType::LongLongInt;
                    } else if flags >= 8 {
                        type_ = ArgType::LongInt;
                    } else if (flags & 2) != 0 {
                        type_ = ArgType::SChar;
                    } else if (flags & 1) != 0 {
                        type_ = ArgType::Short;
                    } else {
                        type_ = ArgType::Int;
                    }
                }
                'o' | 'u' | 'x' | 'X' => {
                    if flags >= 16 || (flags & 4) != 0 {
                        type_ = ArgType::ULongLongInt;
                    } else if flags >= 8 {
                        type_ = ArgType::ULongInt;
                    } else if (flags & 2) != 0 {
                        type_ = ArgType::UChar;
                    } else if (flags & 1) != 0 {
                        type_ = ArgType::UShort;
                    } else {
                        type_ = ArgType::UInt;
                    }
                }
                'f' | 'F' | 'e' | 'E' | 'g' | 'G' | 'a' | 'A' => {
                    if flags >= 16 || (flags & 4) != 0 {
                        type_ = ArgType::LongDouble;
                    } else {
                        type_ = ArgType::Double;
                    }
                }
                'c' => {
                    if flags >= 8 {
                        type_ = ArgType::WideChar;
                    } else {
                        type_ = ArgType::Char;
                    }
                }
                'C' => {
                    type_ = ArgType::WideChar;
                }
                's' => {
                    if flags >= 8 {
                        type_ = ArgType::WideString;
                    } else {
                        type_ = ArgType::String;
                    }
                }
                'S' => {
                    type_ = ArgType::WideString;
                }
                'p' => {
                    type_ = ArgType::Pointer;
                }
                'n' => {
                    if flags >= 16 || (flags & 4) != 0 {
                        type_ = ArgType::CountLongLongIntPointer;
                    } else if flags >= 8 {
                        type_ = ArgType::CountLongIntPointer;
                    } else if (flags & 2) != 0 {
                        type_ = ArgType::CountSCharPointer;
                    } else if (flags & 1) != 0 {
                        type_ = ArgType::CountShortPointer;
                    } else {
                        type_ = ArgType::CountIntPointer;
                    }
                }
                '%' => {
                    type_ = ArgType::None;
                }
                _ => return Err(()),
            }
            if type_ != ArgType::None {
                dp.arg_index = arg_index;
                if dp.arg_index == usize::MAX {
                    dp.arg_index = arg_posn;
                    arg_posn += 1;
                    if dp.arg_index == usize::MAX {
                        return Err(());
                    }
                }
                let n = dp.arg_index;
                if n >= a_allocated {
                    a_allocated = match xsum(a_allocated, a_allocated) {
                        Ok(val) => val,
                        Err(_) => return Err(()),
                    };
                    if a_allocated <= n {
                        a_allocated = match xsum(n, 1) {
                            Ok(val) => val,
                            Err(_) => return Err(()),
                        };
                    }
                    let memory_size = match xsum(a_allocated, std::mem::size_of::<Argument>()) {
                        Ok(val) => val,
                        Err(_) => return Err(()),
                    };
                    let memory = if a.arg.as_ptr() != a.direct_alloc_arg.as_ptr() {
                        std::alloc::realloc(a.arg.as_mut_ptr() as *mut u8, std::alloc::Layout::array::<Argument>(a_allocated).unwrap(), memory_size)
                    } else {
                        std::alloc::alloc(std::alloc::Layout::array::<Argument>(a_allocated).unwrap())
                    };
                    if memory.is_null() {
                        return Err(());
                    }
                    if a.arg.as_ptr() == a.direct_alloc_arg.as_ptr() {
                        std::ptr::copy_nonoverlapping(a.arg.as_ptr(), memory as *mut Argument, a.arg.len());
                    }
                    a.arg = std::slice::from_raw_parts_mut(memory as *mut Argument, a_allocated).to_vec();
                }
                while a.arg.len() <= n {
                    a.arg.push(Argument { arg_type: ArgType::None });
                }
                if a.arg[n].arg_type == ArgType::None {
                    a.arg[n].arg_type = type_;
                } else if a.arg[n].arg_type != type_ {
                    return Err(());
                }
            }
            dp.conversion = c;
            dp.dir_end = &format[format.len() - cp.as_str().len()..];
            d.count += 1;
            if d.count >= d_allocated {
                d_allocated = match xsum(d_allocated, d_allocated) {
                    Ok(val) => val,
                    Err(_) => return Err(()),
                };
                let memory_size = match xsum(d_allocated, std::mem::size_of::<CharDirective>()) {
                    Ok(val) => val,
                    Err(_) => return Err(()),
                };
                let memory = if d.dir.as_ptr() != d.direct_alloc_dir.as_ptr() {
                    std::alloc::realloc(d.dir.as_mut_ptr() as *mut u8, std::alloc::Layout::array::<CharDirective>(d_allocated).unwrap(), memory_size)
                } else {
                    std::alloc::alloc(std::alloc::Layout::array::<CharDirective>(d_allocated).unwrap())
                };
                if memory.is_null() {
                    return Err(());
                }
                if d.dir.as_ptr() == d.direct_alloc_dir.as_ptr() {
                    std::ptr::copy_nonoverlapping(d.dir.as_ptr(), memory as *mut CharDirective, d.count);
                }
                d.dir = std::slice::from_raw_parts_mut(memory as *mut CharDirective, d_allocated).to_vec();
            }
        }
    }
    d.dir[d.count].dir_start = &format[format.len() - cp.as_str().len()..];
    d.max_width_length = max_width_length;
    d.max_precision_length = max_precision_length;
    Ok(())
}
fn progerror(string: &str) {
    // Fix the dereferencing issue by removing the `*` operator
    let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);

    // Use eprintln! instead of rpl_fprintf for printing to stderr
    eprintln!("{}: {}", unsafe { PROGRAM_NAME.unwrap_or("unknown") }, string);

    // Set the exit code
    unsafe {
        EXIT_CODE = 1;
    }
}
fn read_buffer(fd: i32, buf: &mut [u8]) -> Result<usize, ()> {
    let mut cnt = buf.len();
    if 0x7fffffff < cnt {
        cnt = 0x7fffffff;
    }
    let len = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut std::ffi::c_void, cnt) };
    if len < 0 && unsafe { *libc::__errno_location() } == 11 {
        if let Ok(flags) = rpl_fcntl(fd, 3) {
            if flags >= 0 {
                if flags & 0o4000 == 0 {
                    unsafe { *libc::__errno_location() = 11 };
                } else if rpl_fcntl(fd, 4).is_ok() { // Removed the third argument
                    return Ok(unsafe { libc::read(fd, buf.as_mut_ptr() as *mut std::ffi::c_void, cnt) } as usize);
                }
            }
        }
    }
    if len < 0 {
        Err(())
    } else {
        Ok(len as usize)
    }
}
fn read_byte() -> Result<Uch, ()> {
    let b = {
        let inptr = unsafe { INPTR };
        let insize = unsafe { INSIZE };
        if inptr < insize {
            let byte = unsafe { INBUF[inptr as usize] };
            unsafe { INPTR += 1; }
            Ok(byte)
        } else {
            fill_inbuf(false)
        }
    };
    match b {
        Ok(byte) => Ok(byte),
        Err(_) => {
            gzip_error("invalid compressed data -- unexpected end of file");
        }
    }
}
fn read_c_len() -> Result<(), ()> {
    let mut i: usize;
    let mut c: usize;
    let n = getbits(9) as usize;
    if n == 0 {
        c = getbits(9) as usize;
        for i in 0..(255 + 256 + 2 - 3) {
            unsafe { OUTBUF[i] = 0; }
        }
        for i in 0..4096 {
            unsafe { D_BUF[i] = c as Ush; }
        }
    } else {
        i = 0;
        while i < n {
            c = unsafe { PT_TABLE[(BITBUF >> ((8 * 2 * std::mem::size_of::<u8>()) - 8)) as usize] as usize };
            if c >= (16 + 3) {
                let mut mask = 1u64 << ((8 * 2 * std::mem::size_of::<u8>()) - 1 - 8);
                loop {
                    if unsafe { BITBUF & mask } != 0 {
                        c = unsafe { (PREV[0x8000 + c]) as usize };
                    } else {
                        c = unsafe { PREV[c] as usize };
                    }
                    mask >>= 1;
                    if c < (16 + 3) {
                        break;
                    }
                }
            }
            fillbuf(unsafe { PT_LEN[c] as i32 });
            if c <= 2 {
                if c == 0 {
                    c = 1;
                } else if c == 1 {
                    c = getbits(4) as usize + 3;
                } else {
                    c = getbits(9) as usize + 20;
                }
                while c > 0 {
                    unsafe { OUTBUF[i] = 0; }
                    i += 1;
                    c -= 1;
                }
            } else {
                unsafe { OUTBUF[i] = (c - 2) as Uch; }
                i += 1;
            }
        }
        while i < (255 + 256 + 2 - 3) {
            unsafe { OUTBUF[i] = 0; }
            i += 1;
        }
        make_table(255 + 256 + 2 - 3, unsafe { &OUTBUF }, 12, unsafe { &mut D_BUF })?;
    }
    Ok(())
}
fn read_error() -> ! {
    let e = unsafe { *libc::__errno_location() };
    let stderr = &mut i8::stderr();
    let _ = rpl_fprintf(stderr, "\n%s: ", &[ArgValue::String(PROGRAM_NAME.unwrap_or("").to_string())]);
    if e != 0 {
        unsafe { *libc::__errno_location() = e; }
        eprintln!("{}", std::io::Error::from_raw_os_error(e));
    } else {
        let ifname_str = std::str::from_utf8(&IFNAME).unwrap_or("").to_string();
        let _ = rpl_fprintf(stderr, "%s: unexpected end of file\n", &[ArgValue::String(ifname_str)]);
    }
    abort_gzip();
}
fn read_pt_len(nn: usize, nbit: usize, i_special: usize) -> Result<(), ()> {
    let mut i: usize;
    let mut c: u64;
    let mut n: u64;
    let mut mask: u64;
    n = getbits(nbit as i32);
    if n == 0 {
        c = getbits(nbit as i32);
        for i in 0..nn {
            unsafe { PT_LEN[i] = 0; }
        }
        for i in 0..256 {
            unsafe { PT_TABLE[i] = c as Ush; }
        }
    } else {
        i = 0;
        while i < n as usize {
            c = unsafe { BITBUF >> ((8 * 2 * std::mem::size_of::<u8>()) - 3) };
            if c == 7 {
                mask = 1 << ((8 * 2 * std::mem::size_of::<u8>()) - 1 - 3);
                while mask & unsafe { BITBUF } != 0 {
                    mask >>= 1;
                    c += 1;
                }
                if 16 < c {
                    gzip_error("Bad table\n");
                }
            }
            fillbuf(if c < 7 { 3 } else { c as i32 - 3 });
            unsafe { PT_LEN[i] = c as Uch; }
            i += 1;
            if i == i_special {
                c = getbits(2);
                while c > 0 {
                    unsafe { PT_LEN[i] = 0; }
                    i += 1;
                    c -= 1;
                }
            }
        }
        while i < nn {
            unsafe { PT_LEN[i] = 0; }
            i += 1;
        }
        make_table(nn, unsafe { &PT_LEN }, 8, unsafe { &mut PT_TABLE });
    }
    Ok(())
}
fn read_tree() -> Result<(), ()> {
    let mut len: i32;
    let mut base: i32;
    let mut n: i32;
    let mut max_leaves = 1;
    unsafe {
        ORIG_LEN = 0;
        for _ in 0..4 {
            ORIG_LEN = (ORIG_LEN << 8) | read_byte()? as u64;
        }
        MAX_LEN = read_byte()? as i32;
        if !(0 < MAX_LEN && MAX_LEN <= 25) {
            gzip_error("invalid compressed data -- Huffman code bit length out of range");
        }
        n = 0;
        for len in 1..=MAX_LEN {
            LEAVES[len as usize] = read_byte()? as i32;
            if max_leaves - if len == MAX_LEN { 1 } else { 0 } < LEAVES[len as usize] {
                gzip_error("too many leaves in Huffman tree");
            }
            max_leaves = (max_leaves - LEAVES[len as usize] + 1) * 2 - 1;
            n += LEAVES[len as usize];
        }
        if 256 <= n {
            gzip_error("too many leaves in Huffman tree");
        }
        LEAVES[MAX_LEN as usize] += 1;
        base = 0;
        for len in 1..=MAX_LEN {
            LIT_BASE[len as usize] = base;
            for _ in 0..LEAVES[len as usize] {
                LITERAL[base as usize] = read_byte()?;
                base += 1;
            }
        }
        LEAVES[MAX_LEN as usize] += 1;
    }
    Ok(())
}
fn realloc(ptr: *mut u8, size: usize) -> *mut u8{todo!("proto")}
fn remove_output_file(signals_already_blocked: bool) -> Result<(), ()> {
    let mut fd: i32;
    let mut oldset: libc::sigset_t = unsafe { std::mem::zeroed() };
    if !signals_already_blocked {
        unsafe {
            libc::sigprocmask(0, &CAUGHT_SIGNALS, &mut oldset);
        }
    }
    fd = unsafe { REMOVE_OFNAME_FD };
    if 0 <= fd {
        let mut fname = [0u8; 1024];
        unsafe {
            REMOVE_OFNAME_FD = -1;
            libc::close(fd);
            volatile_strcpy(&mut fname, &REMOVE_OFNAME);
        }
        xunlink(std::str::from_utf8(&fname).unwrap_or_default())?;
    }
    if !signals_already_blocked {
        unsafe {
            libc::sigprocmask(2, &oldset, std::ptr::null_mut());
        }
    }
    Ok(())
}
fn restore_cwd(cwd: &SavedCwd) -> i32 {
    if cwd.desc >= 0 {
        return unsafe { libc::fchdir(cwd.desc) };
    } else if let Some(ref name) = cwd.name {
        return match chdir_long(name) {
            Ok(_) => 0,
            Err(_) => -1,
        };
    }
    -1
}
fn rpl_fclose(fp: &mut std::fs::File) -> Result<(), ()> {
    use std::io::Seek;
    use std::os::fd::AsRawFd;
    let mut saved_errno = None;
    let fd = fp.as_raw_fd();
    if fd < 0 {
        return fp.sync_all().map_err(|_| ());
    }
    if (fp.metadata().map(|_| true).unwrap_or(false) || fp.seek(std::io::SeekFrom::Current(0)).is_ok())
        && rpl_fflush(Some(fp)).is_err()
    {
        saved_errno = Some(std::io::Error::last_os_error().raw_os_error().unwrap_or(0));
    }
    let result = fp.sync_all().map_err(|_| ());
    if let Some(errno) = saved_errno {
        std::io::Error::from_raw_os_error(errno);
        return Err(());
    }
    result
}
fn rpl_fcntl(fd: i32, action: usize) -> Result<i32, ()> {
    let mut args = std::vec::Vec::new();
    let mut result = -1;

    match action {
        0 => {
            let target = args.pop().ok_or(())?;
            result = rpl_fcntl_dupfd(fd, target);
        }
        1030 => {
            let target = args.pop().ok_or(())?;
            result = rpl_fcntl_dupfd_cloexec(fd, target)?;
        }
        _ => {
            match action {
                1 | 3 | 1025 | 9 | 1032 | 1034 | 11 => {
                    result = unsafe { libc::fcntl(fd, action as i32) };
                }
                1033 | 0 | 1030 | 1026 | 2 | 4 | 1024 | 8 | 1031 | 10 => {
                    let x = args.pop().ok_or(())?;
                    result = unsafe { libc::fcntl(fd, action as i32, x) };
                }
                _ => {
                    let p = args.pop().ok_or(())?;
                    result = unsafe { libc::fcntl(fd, action as i32, p) };
                }
            }
        }
    }

    if result == -1 {
        Err(())
    } else {
        Ok(result)
    }
}
fn rpl_fcntl_dupfd(fd: i32, target: i32) -> i32 {
    std::os::unix::io::RawFd::from(fd);
    let result = unsafe { libc::fcntl(fd, libc::F_DUPFD, target) };
    result
}
fn rpl_fcntl_dupfd_cloexec(fd: i32, target: i32) -> Result<i32, ()> {
    let mut result;
    static mut HAVE_DUPFD_CLOEXEC: i32 = 0;
    unsafe {
        if HAVE_DUPFD_CLOEXEC >= 0 {
            result = libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, target);
            if result >= 0 || *libc::__errno_location() != libc::EINVAL {
                HAVE_DUPFD_CLOEXEC = 1;
            } else {
                result = rpl_fcntl_dupfd(fd, target);
                if result >= 0 {
                    HAVE_DUPFD_CLOEXEC = -1;
                }
            }
        } else {
            result = rpl_fcntl_dupfd(fd, target);
        }

        if result >= 0 && HAVE_DUPFD_CLOEXEC == -1 {
            let flags = libc::fcntl(result, libc::F_GETFD);
            if flags < 0 || libc::fcntl(result, libc::F_SETFD, flags | libc::FD_CLOEXEC) == -1 {
                let saved_errno = *libc::__errno_location();
                libc::close(result);
                *libc::__errno_location() = saved_errno;
                result = -1;
            }
        }
    }
    if result >= 0 {
        Ok(result)
    } else {
        Err(())
    }
}
fn rpl_fflush(stream: Option<&mut std::fs::File>) -> Result<(), ()> {
    use std::io::Write;
    if stream.is_none() || !stream.as_ref().map_or(false, |s| s.metadata().map(|m| m.len() > 0).unwrap_or(false)) {
        return std::io::stdout().flush().map_err(|_| ());
    }
    if let Some(s) = stream {
        clear_ungetc_buffer_preserving_position(s)?;
    }
    std::io::stdout().flush().map_err(|_| ())
}
fn rpl_fprintf(fp: &mut std::fs::File, format: &str, args: &[ArgValue]) -> Result<usize, ()> {
    use std::io::Write;
    let mut buf = [0u8; 2000];
    let mut output = String::new();
    let mut lenbuf = buf.len();
    match vasnprintf(&mut output, &mut lenbuf, format, args) {
        Ok(output_str) => {
            if fp.write_all(output_str.as_bytes()).is_err() {
                if output_str.as_ptr() != buf.as_ptr() {
                    rpl_free(output_str.as_ptr() as *mut std::ffi::c_void);
                }
                return Err(());
            }
            if output_str.as_ptr() != buf.as_ptr() {
                rpl_free(output_str.as_ptr() as *mut std::ffi::c_void);
            }
            let len = lenbuf; // Move this line here, after the borrow is no longer needed.
            if len > 0x7fffffff {
                unsafe {
                    *libc::__errno_location() = 75; // Wrap this in an unsafe block.
                }
                fseterr(fp);
                return Err(());
            }
            Ok(len)
        }
        Err(_) => {
            fseterr(fp);
            Err(())
        }
    }
}
fn rpl_free(p: *mut std::ffi::c_void) {
    unsafe {
        // Access errno and store its current value
        let mut err = [*libc::__errno_location(), *libc::__errno_location()];

        // Set errno to 0
        *libc::__errno_location() = 0;

        // Free the memory pointed to by p
        libc::free(p);

        // Restore errno if it was changed
        *libc::__errno_location() = err[(*libc::__errno_location() == 0) as usize];
    }
}
fn rpl_fseek(fp: &mut std::fs::File, offset: i64, whence: std::io::SeekFrom) -> Result<(), ()> {
    rpl_fseeko(fp, offset, whence)
}
fn rpl_fseeko(fp: &mut std::fs::File, offset: i64, whence: std::io::SeekFrom) -> Result<(), ()> {
    use std::convert::TryInto;
    use std::os::unix::io::AsRawFd;
    use std::os::unix::fs::FileExt;
    use std::io::Seek;

    let raw_fd = fp.as_raw_fd();
    let pos = unsafe { libc::lseek(raw_fd, offset, match whence {
        std::io::SeekFrom::Start(pos) => (pos as i64).try_into().unwrap(),
        std::io::SeekFrom::End(pos) => pos.try_into().unwrap(),
        std::io::SeekFrom::Current(pos) => pos.try_into().unwrap(),
    }) };

    if pos == -1 {
        return Err(());
    }

    fp.seek(std::io::SeekFrom::Start(pos as u64)).map_err(|_| ())?;
    Ok(())
}
fn rpl_strerror_r(errnum: i32, buf: &mut [u8]) -> Result<(), ()> {
    if buf.len() <= 1 {
        if !buf.is_empty() {
            buf[0] = b'\0';
        }
        return Err(());
    }
    buf[0] = b'\0';
    {
        let msg: Option<&str> = None;
        if let Some(msg) = msg {
            return safe_copy(buf, msg);
        }
    }
    {
        let mut ret: i32;
        let saved_errno = unsafe { *libc::__errno_location() };
        {
            ret = 0;
            let result = unsafe {
                libc::strerror_r(errnum, buf.as_mut_ptr() as *mut i8, buf.len())
            };
            if result != 0 {
                ret = result;
            }
            if buf[0] == b'\0' {
                let errstring = unsafe {
                    std::ffi::CStr::from_ptr(libc::strerror(errnum))
                };
                ret = if let Ok(errstring) = errstring.to_str() {
                    safe_copy(buf, errstring).map_or_else(|_| unsafe { *libc::__errno_location() }, |_| 0)
                } else {
                    unsafe { *libc::__errno_location() }
                };
            }
        }
        if ret == 22 && buf[0] == b'\0' {
            let msg = format!("Unknown error {}", errnum);
            let _ = safe_copy(buf, &msg);
        }
        unsafe { *libc::__errno_location() = saved_errno };
        if ret == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}
fn rpl_vfprintf(fp: &mut std::fs::File, format: &str, args: &[ArgValue]) -> Result<usize, ()> {
    let mut buf = [0u8; 2000];
    let mut output = String::new();
    let mut lenbuf = buf.len();
    match vasnprintf(&mut output, &mut lenbuf, format, args) {
        Ok(output_str) => {
            let len = lenbuf; // Move this line after the match block to ensure lenbuf is not borrowed.
            if std::io::Write::write(fp, output_str.as_bytes()).unwrap_or(0) < len {
                if output_str.as_ptr() != buf.as_ptr() {
                    rpl_free(output_str.as_ptr() as *mut std::ffi::c_void);
                }
                return Err(());
            }
            if output_str.as_ptr() != buf.as_ptr() {
                rpl_free(output_str.as_ptr() as *mut std::ffi::c_void);
            }
            if len > 0x7fffffff {
                unsafe {
                    *libc::__errno_location() = 75; // Wrap this in an unsafe block.
                }
                fseterr(fp);
                return Err(());
            }
            Ok(len)
        }
        Err(_) => {
            fseterr(fp);
            Err(())
        }
    }
}
fn rsync_roll(start: usize, num: usize) {
    unsafe {
        if start < 4096 {
            for i in start..4096 {
                if i == start + num {
                    return;
                }
                RSYNC_SUM += WINDOW[i] as Ulg;
            }
            let mut num = num - (4096 - start);
            let mut start = 4096;
            for i in start..start + num {
                RSYNC_SUM += WINDOW[i] as Ulg;
                RSYNC_SUM -= WINDOW[i - 4096] as Ulg;
                if RSYNC_CHUNK_END == 0xFFFFFFFF && (RSYNC_SUM % 4096 == 0) {
                    RSYNC_CHUNK_END = i as Ulg;
                }
            }
        } else {
            for i in start..start + num {
                RSYNC_SUM += WINDOW[i] as Ulg;
                RSYNC_SUM -= WINDOW[i - 4096] as Ulg;
                if RSYNC_CHUNK_END == 0xFFFFFFFF && (RSYNC_SUM % 4096 == 0) {
                    RSYNC_CHUNK_END = i as Ulg;
                }
            }
        }
    }
}
fn safe_copy(buf: &mut [u8], msg: &str) -> Result<(), ()> {
    let len = msg.len();
    let moved = if len < buf.len() { len } else { buf.len() - 1 };
    unsafe {
        std::ptr::copy_nonoverlapping(msg.as_ptr(), buf.as_mut_ptr(), moved);
    }
    buf[moved] = b'\0';
    if len < buf.len() {
        Ok(())
    } else {
        Err(())
    }
}
fn save_cwd(cwd: &mut SavedCwd) -> Result<(), ()> {
    cwd.name = None;
    cwd.desc = open_safer(".", libc::O_RDONLY | libc::O_DIRECTORY)?;
    if false {
        cwd.desc = fd_safer_flag(cwd.desc, libc::O_DIRECTORY)?;
    }
    if cwd.desc < 0 {
        cwd.name = std::env::current_dir().ok().map(|p| p.to_string_lossy().into_owned());
        return if cwd.name.is_some() { Ok(()) } else { Err(()) };
    }
    Ok(())
}
fn savedir(dir: &str, option: SavedirOption) -> Result<String, ()> {
    match opendir_safer(dir) {
        Ok(dirp) => {
            let name_space = streamsavedir(dirp.as_ref() as *const _ as *mut libc::DIR, option)?;
            if unsafe { libc::closedir(dirp.as_ref() as *const _ as *mut libc::DIR) } != 0 {
                rpl_free(name_space.as_ptr() as *mut std::ffi::c_void);
                Err(())
            } else {
                Ok(name_space)
            }
        }
        Err(_) => Err(()),
    }
}
fn scan_tree(tree: &mut [CtData], max_code: usize) {
    let mut n: usize;
    let mut prevlen = -1;
    let mut curlen;
    let mut nextlen = match tree[0].dl {
        DadOrLen::Len(len) => len,
        _ => 0,
    };
    let mut count = 0;
    let mut max_count = 7;
    let mut min_count = 4;
    if nextlen == 0 {
        max_count = 138;
        min_count = 3;
    }
    if let DadOrLen::Len(ref mut len) = tree[max_code + 1].dl {
        *len = 0xffff;
    }
    for n in 0..=max_code {
        curlen = nextlen;
        nextlen = match tree[n + 1].dl {
            DadOrLen::Len(len) => len,
            _ => 0,
        };
        if {
            count += 1;
            count < max_count && curlen == nextlen
        } {
            continue;
        } else if count < min_count {
            if let FreqOrCode::Freq(ref mut freq) = BL_TREE[curlen as usize].fc {
                *freq += count as u16;
            }
        } else if curlen != 0 {
            if curlen != prevlen {
                if let FreqOrCode::Freq(ref mut freq) = BL_TREE[curlen as usize].fc {
                    *freq += 1;
                }
            }
            if let FreqOrCode::Freq(ref mut freq) = BL_TREE[16].fc {
                *freq += 1;
            }
        } else if count <= 10 {
            if let FreqOrCode::Freq(ref mut freq) = BL_TREE[17].fc {
                *freq += 1;
            }
        } else {
            if let FreqOrCode::Freq(ref mut freq) = BL_TREE[18].fc {
                *freq += 1;
            }
        }
        count = 0;
        prevlen = curlen;
        if nextlen == 0 {
            max_count = 138;
            min_count = 3;
        } else if curlen == nextlen {
            max_count = 6;
            min_count = 3;
        } else {
            max_count = 7;
            min_count = 4;
        }
    }
}
fn send_all_trees(lcodes: usize, dcodes: usize, blcodes: usize) -> Result<(), ()> {
    send_bits((lcodes - 257) as i32, 5)?;
    send_bits((dcodes - 1) as i32, 5)?;
    send_bits((blcodes - 4) as i32, 4)?;
    for rank in 0..blcodes {
        let len = match unsafe { BL_TREE[BL_ORDER[rank] as usize].dl } {
            DadOrLen::Len(len) => len,
            _ => return Err(()),
        };
        send_bits(len as i32, 3)?;
    }
    send_tree(unsafe { std::slice::from_raw_parts(DYN_LTREE as *const CtData, lcodes) }, lcodes - 1)?;
    send_tree(unsafe { &DYN_DTREE[..dcodes] }, dcodes - 1)?;
    Ok(())
}
fn send_bits(value: i32, length: i32) -> Result<(), ()> {
    unsafe {
        if BI_VALID > (8 * 2 * std::mem::size_of::<u8>() as i32) - length {
            BI_BUF |= (value << BI_VALID) as u16;
            if OUTCNT < 0x40000 - 2 {
                OUTBUF[OUTCNT as usize] = (BI_BUF & 0xff) as Uch;
                OUTCNT += 1;
                OUTBUF[OUTCNT as usize] = (BI_BUF >> 8) as Uch;
                OUTCNT += 1;
            } else {
                OUTBUF[OUTCNT as usize] = (BI_BUF & 0xff) as Uch;
                OUTCNT += 1;
                if OUTCNT == 0x40000 {
                    flush_outbuf()?;
                }
                OUTBUF[OUTCNT as usize] = (BI_BUF >> 8) as Uch;
                OUTCNT += 1;
                if OUTCNT == 0x40000 {
                    flush_outbuf()?;
                }
            }
            BI_BUF = (value as Ush >> ((8 * 2 * std::mem::size_of::<u8>()) as i32 - BI_VALID) as usize) as u16;
            BI_VALID += length - (8 * 2 * std::mem::size_of::<u8>() as i32);
        } else {
            BI_BUF |= (value << BI_VALID) as u16;
            BI_VALID += length;
        }
    }
    Ok(())
}
fn send_tree(tree: &[CtData], max_code: usize) -> Result<(), ()> {
    let mut n: usize;
    let mut prevlen = u16::MAX; // Use u16::MAX to represent an invalid previous length
    let mut curlen;
    let mut nextlen = match tree[0].dl {
        DadOrLen::Len(len) => len,
        _ => return Err(()),
    };
    let mut count = 0;
    let mut max_count = 7;
    let mut min_count = 4;
    if nextlen == 0 {
        max_count = 138;
        min_count = 3;
    }
    for n in 0..=max_code {
        curlen = nextlen;
        nextlen = match tree.get(n + 1).map(|t| t.dl) {
            Some(DadOrLen::Len(len)) => len,
            _ => 0,
        };
        count += 1;
        if count < max_count && curlen == nextlen {
            continue;
        } else if count < min_count {
            while count != 0 {
                send_bits(
                    match unsafe { BL_TREE[curlen as usize].fc } {
                        FreqOrCode::Code(code) => code as i32,
                        _ => return Err(()),
                    },
                    match unsafe { BL_TREE[curlen as usize].dl } {
                        DadOrLen::Len(len) => len as i32,
                        _ => return Err(()),
                    },
                )?;
                count -= 1;
            }
        } else if curlen != 0 {
            if curlen != prevlen {
                send_bits(
                    match unsafe { BL_TREE[curlen as usize].fc } {
                        FreqOrCode::Code(code) => code as i32,
                        _ => return Err(()),
                    },
                    match unsafe { BL_TREE[curlen as usize].dl } {
                        DadOrLen::Len(len) => len as i32,
                        _ => return Err(()),
                    },
                )?;
                count -= 1;
            }
            send_bits(
                match unsafe { BL_TREE[16].fc } {
                    FreqOrCode::Code(code) => code as i32,
                    _ => return Err(()),
                },
                match unsafe { BL_TREE[16].dl } {
                    DadOrLen::Len(len) => len as i32,
                    _ => return Err(()),
                },
            )?;
            send_bits(count - 3, 2)?;
        } else if count <= 10 {
            send_bits(
                match unsafe { BL_TREE[17].fc } {
                    FreqOrCode::Code(code) => code as i32,
                    _ => return Err(()),
                },
                match unsafe { BL_TREE[17].dl } {
                    DadOrLen::Len(len) => len as i32,
                    _ => return Err(()),
                },
            )?;
            send_bits(count - 3, 3)?;
        } else {
            send_bits(
                match unsafe { BL_TREE[18].fc } {
                    FreqOrCode::Code(code) => code as i32,
                    _ => return Err(()),
                },
                match unsafe { BL_TREE[18].dl } {
                    DadOrLen::Len(len) => len as i32,
                    _ => return Err(()),
                },
            )?;
            send_bits(count - 11, 7)?;
        }
        count = 0;
        prevlen = curlen;
        if nextlen == 0 {
            max_count = 138;
            min_count = 3;
        } else if curlen == nextlen {
            max_count = 6;
            min_count = 3;
        } else {
            max_count = 7;
            min_count = 4;
        }
    }
    Ok(())
}
fn set_cloexec_flag(desc: i32, value: bool) -> i32 {
    match rpl_fcntl(desc, 1) {
        Ok(flags) => {
            let newflags = if value { flags | 1 } else { flags & !1 };
            if flags == newflags || rpl_fcntl(desc, 2).unwrap_or(-1) == newflags {
                0
            } else {
                -1
            }
        }
        Err(_) => -1,
    }
}
fn set_file_type() {
    let mut n = 0;
    let mut ascii_freq: u32 = 0;
    let mut bin_freq: u32 = 0;
    unsafe {
        while n < 7 {
            bin_freq += match DYN_LTREE[n].fc {
                crate::FreqOrCode::Freq(freq) => freq as u32,
                _ => 0,
            };
            n += 1;
        }
        while n < 128 {
            ascii_freq += match DYN_LTREE[n].fc {
                crate::FreqOrCode::Freq(freq) => freq as u32,
                _ => 0,
            };
            n += 1;
        }
        while n < 256 {
            bin_freq += match DYN_LTREE[n].fc {
                crate::FreqOrCode::Freq(freq) => freq as u32,
                _ => 0,
            };
            n += 1;
        }
        FILE_TYPE = Some(Box::new(if bin_freq > (ascii_freq >> 2) { 0 } else { 1 }));
        if *FILE_TYPE.as_ref().unwrap() == Box::new(0) && false {
            crate::warning("-l used on binary file");
        }
    }
}
fn setcrc(c: Ulg){todo!("proto")}
fn settime(timespec: &libc::timespec) -> Result<(), ()>{todo!("proto")}
fn shorten_name(name: &mut String) {
    let mut len = name.len();
    let mut trunc: Option<usize> = None;
    let mut plen;
    let mut min_part = 3;
    if unsafe { DECOMPRESS != 0 } {
        if len <= 1 {
            gzip_error("name too short");
        }
        name.truncate(len - 1);
        return;
    }
    let suffix = get_suffix(name).unwrap_or_else(|| gzip_error("can't recover suffix\n"));
    let suffix_pos = name.len() - suffix.len();
    name.truncate(suffix_pos);
    unsafe { SAVE_ORIG_NAME = 1; }
    if len > 4 && &name[suffix_pos - 4..suffix_pos] == ".tar" {
        name.replace_range(suffix_pos - 4..suffix_pos, ".tgz");
        return;
    }
    loop {
        let mut p = last_component(name).unwrap_or("");
        while !p.is_empty() {
            plen = p.find('.').unwrap_or(p.len());
            p = &p[plen..];
            if plen > min_part {
                trunc = Some(name.len() - p.len() - 1);
            }
            if !p.is_empty() {
                p = &p[1..];
            }
        }
        if trunc.is_some() || min_part == 0 {
            break;
        }
        min_part -= 1;
    }
    if let Some(trunc_pos) = trunc {
        let mut chars = name.chars().collect::<Vec<_>>();
        for i in trunc_pos..chars.len() - 1 {
            chars[i] = chars[i + 1];
        }
        chars.pop();
        *name = chars.into_iter().collect();
    } else {
        let trunc_pos = name.rfind('.').unwrap_or_else(|| gzip_error("internal error in shorten_name"));
        if name[trunc_pos + 1..].is_empty() {
            name.truncate(trunc_pos);
        }
    }
    if let Some(z_suffix) = unsafe { Z_SUFFIX } {
        name.push_str(z_suffix);
    }
}
fn stat_time_normalize(result: i32, st: &std::os::linux::raw::stat) -> i32 {
    result
}
fn strdup(s: &str) -> Box<str>{todo!("proto")}
fn streamsavedir(dirp: *mut libc::DIR, option: SavedirOption) -> Result<String, ()> {
    let mut name_space: Vec<u8> = Vec::new();
    let mut entries: Vec<DirentryT> = Vec::new();
    let cmp = match option {
        SavedirOption::SortNone => None,
        SavedirOption::SortName => COMPARISON_FUNCTION_TABLE[0],
        SavedirOption::SortFastRead => COMPARISON_FUNCTION_TABLE[1],
    };

    if dirp.is_null() {
        return Err(());
    }

    loop {
        let dp = unsafe { libc::readdir(dirp) };
        if dp.is_null() {
            break;
        }

        let entry = unsafe { std::ffi::CStr::from_ptr((*dp).d_name.as_ptr()) };
        let entry_str = entry.to_str().unwrap_or("");

        if !entry_str.starts_with('.') || entry_str.len() > 1 {
            let entry_size = entry_str.len() + 1;
            if let Some(cmp_fn) = cmp {
                if entries.len() == entries.capacity() {
                    entries.reserve(1);
                }
                entries.push(DirentryT {
                    name: entry_str.to_string(),
                });
            } else {
                if name_space.len() + entry_size > name_space.capacity() {
                    name_space.reserve(entry_size);
                }
                name_space.extend_from_slice(entry_str.as_bytes());
                name_space.push(0);
            }
        }
    }

    if unsafe { *libc::__errno_location() } != 0 {
        return Err(());
    }

    if let Some(cmp_fn) = cmp {
        if !entries.is_empty() {
            entries.sort_by(|a, b| cmp_fn(&(), &()));
        }
        name_space.clear();
        for entry in entries {
            name_space.extend_from_slice(entry.name.as_bytes());
            name_space.push(0);
        }
    }

    name_space.push(0);
    Ok(String::from_utf8(name_space).unwrap_or_default())
}
fn strip_trailing_slashes(file: &mut String) -> bool {
    if let Some(base) = last_component(file) {
        let base_len = base_len(base);
        let had_slash = base_len < base.len();
        file.truncate(file.len() - (base.len() - base_len));
        had_slash
    } else {
        false
    }
}
fn strlwr(s: &mut String) {
    let lowercased: String = s.chars().map(|c| c.to_lowercase().next().unwrap()).collect();
    *s = lowercased;
}
fn strndup(s: &str, n: usize) -> Option<String>{todo!("proto")}
fn timespec_add(t1: usize, t2: usize) -> usize{todo!("proto")}
fn timespec_cmp(a: &std::time::Duration, b: &std::time::Duration) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    match a.as_secs().cmp(&b.as_secs()) {
        std::cmp::Ordering::Equal => a.subsec_nanos().cmp(&b.subsec_nanos()),
        other => other,
    }
}
fn timespec_sign(a: &std::time::Duration) -> i32 {
    let secs = a.as_secs() as i64;
    let nsecs = a.subsec_nanos() as i64;
    ((secs > 0) as i32 - (secs < 0) as i32) + ((!secs != 0) as i32 & (nsecs != 0) as i32)
}
fn timespec_sub(t1: &usize, t2: &usize) -> usize{todo!("proto")}
fn timespectod(a: std::time::Duration) -> f64 {
    a.as_secs() as f64 + a.subsec_nanos() as f64 / 1e9
}
fn tmpfile() -> Option<std::fs::File>{todo!("proto")}
fn treat_dir(fd: i32, dir: &str) -> Result<(), ()> {
    let dirp = unsafe { libc::fdopendir(fd) };
    if dirp.is_null() {
        progerror(dir);
        unsafe { libc::close(fd) };
        return Ok(());
    }
    let entries = match streamsavedir(dirp, SavedirOption::SortNone) {
        Ok(entries) => entries,
        Err(_) => {
            progerror(dir);
            return Ok(());
        }
    };
    if unsafe { libc::closedir(dirp) } != 0 {
        progerror(dir);
    }
    let mut entry = entries.as_str();
    while !entry.is_empty() {
        let len = dir.len();
        let entrylen = entry.find('\0').unwrap_or(entry.len());
        let current_entry = &entry[..entrylen];
        if current_entry == "." || current_entry == ".." {
            entry = &entry[entrylen + 1..];
            continue;
        }
        if len + entrylen < 1024 - 2 {
            let mut nbuf = String::with_capacity(1024);
            nbuf.push_str(dir);
            if !last_component(nbuf.as_str()).unwrap_or("").is_empty() && !nbuf.ends_with('/') {
                nbuf.push('/');
            }
            nbuf.push_str(current_entry);
            treat_file(&nbuf)?;
        } else {
            let _ = rpl_fprintf(
                &mut std::io::stderr(),
                "%s: %s/%s: pathname too long\n",
                &[ArgValue::String(PROGRAM_NAME.unwrap_or("unknown").to_string()), ArgValue::String(dir.to_string()), ArgValue::String(current_entry.to_string())],
            );
            unsafe { EXIT_CODE = 1 };
        }
        entry = &entry[entrylen + 1..];
    }
    rpl_free(entries.as_ptr() as *mut std::ffi::c_void);
    Ok(())
}
fn treat_file(iname: &str) -> Result<(), ()> {
    if iname == "-" {
        let cflag = TO_STDOUT.load(std::sync::atomic::Ordering::SeqCst);
        treat_stdin()?;
        TO_STDOUT.store(cflag, std::sync::atomic::Ordering::SeqCst);
        return Ok(());
    }
    IFD = open_input_file(iname, unsafe { &mut ISTAT })?;
    if unsafe { ISTAT.st_mode & libc::S_IFMT } == libc::S_IFDIR {
        if RECURSIVE.load(std::sync::atomic::Ordering::SeqCst) != 0 {
            treat_dir(IFD, iname)?;
            return Ok(());
        }
        unsafe { libc::close(IFD) };
        if unsafe { QUIET == 0 } {
            eprintln!(
                "{}: {} is a directory -- ignored",
                PROGRAM_NAME.unwrap_or(""),
                std::str::from_utf8(&IFNAME).unwrap_or("")
            );
            if unsafe { EXIT_CODE } == 0 {
                unsafe { EXIT_CODE = 2 };
            }
        }
        return Ok(());
    }
    if TO_STDOUT.load(std::sync::atomic::Ordering::SeqCst) == 0 {
        if unsafe { ISTAT.st_mode & libc::S_IFMT } != libc::S_IFREG {
            if unsafe { QUIET == 0 } {
                eprintln!(
                    "{}: {} is not a directory or a regular file - ignored",
                    PROGRAM_NAME.unwrap_or(""),
                    std::str::from_utf8(&IFNAME).unwrap_or("")
                );
                if unsafe { EXIT_CODE } == 0 {
                    unsafe { EXIT_CODE = 2 };
                }
            }
            unsafe { libc::close(IFD) };
            return Ok(());
        }
        if unsafe { ISTAT.st_mode & libc::S_ISUID } != 0 {
            if unsafe { QUIET == 0 } {
                eprintln!(
                    "{}: {} is set-user-ID on execution - ignored",
                    PROGRAM_NAME.unwrap_or(""),
                    std::str::from_utf8(&IFNAME).unwrap_or("")
                );
                if unsafe { EXIT_CODE } == 0 {
                    unsafe { EXIT_CODE = 2 };
                }
            }
            unsafe { libc::close(IFD) };
            return Ok(());
        }
        if unsafe { ISTAT.st_mode & libc::S_ISGID } != 0 {
            if unsafe { QUIET == 0 } {
                eprintln!(
                    "{}: {} is set-group-ID on execution - ignored",
                    PROGRAM_NAME.unwrap_or(""),
                    std::str::from_utf8(&IFNAME).unwrap_or("")
                );
                if unsafe { EXIT_CODE } == 0 {
                    unsafe { EXIT_CODE = 2 };
                }
            }
            unsafe { libc::close(IFD) };
            return Ok(());
        }
        if unsafe { FORCE == 0 } {
            if unsafe { ISTAT.st_mode & libc::S_ISVTX } != 0 {
                if unsafe { QUIET == 0 } {
                    eprintln!(
                        "{}: {} has the sticky bit set - file ignored",
                        PROGRAM_NAME.unwrap_or(""),
                        std::str::from_utf8(&IFNAME).unwrap_or("")
                    );
                    if unsafe { EXIT_CODE } == 0 {
                        unsafe { EXIT_CODE = 2 };
                    }
                }
                unsafe { libc::close(IFD) };
                return Ok(());
            }
            if unsafe { ISTAT.st_nlink } >= 2 {
                if unsafe { QUIET == 0 } {
                    eprintln!(
                        "{}: {} has {} other link{} -- file ignored",
                        PROGRAM_NAME.unwrap_or(""),
                        std::str::from_utf8(&IFNAME).unwrap_or(""),
                        unsafe { ISTAT.st_nlink } - 1,
                        if unsafe { ISTAT.st_nlink } == 2 { "" } else { "s" }
                    );
                    if unsafe { EXIT_CODE } == 0 {
                        unsafe { EXIT_CODE = 2 };
                    }
                }
                unsafe { libc::close(IFD) };
                return Ok(());
            }
        }
    }
    get_input_size_and_time();
    if TO_STDOUT.load(std::sync::atomic::Ordering::SeqCst) != 0 && unsafe { TEST == 0 } {
        unsafe { OFNAME.copy_from_slice(b"stdout\0") };
    } else if make_ofname() != 0 {
        unsafe { libc::close(IFD) };
        return Ok(());
    }
    clear_bufs();
    unsafe { PART_NB = 0 };
    if unsafe { DECOMPRESS } != 0 {
        METHOD = get_method(IFD);
        if METHOD < 0 {
            unsafe { libc::close(IFD) };
            return Ok(());
        }
    }
    if TO_STDOUT.load(std::sync::atomic::Ordering::SeqCst) != 0 {
        unsafe { OFD = 1 };
    } else {
        if create_outfile() != 0 {
            return Ok(());
        }
        if unsafe { DECOMPRESS == 0 && SAVE_ORIG_NAME != 0 && VERBOSE == 0 && QUIET == 0 } {
            eprintln!(
                "{}: {} compressed to {}",
                PROGRAM_NAME.unwrap_or(""),
                std::str::from_utf8(&IFNAME).unwrap_or(""),
                std::str::from_utf8(&OFNAME).unwrap_or("")
            );
        }
    }
    if unsafe { SAVE_ORIG_NAME == 0 } {
        unsafe { SAVE_ORIG_NAME = !NO_NAME };
    }
    if unsafe { VERBOSE != 0 && LIST == 0 } {
        eprintln!("{}:\t", std::str::from_utf8(&IFNAME).unwrap_or(""));
    }
    loop {
        if WORK(IFD, unsafe { OFD }) != 0 {
            METHOD = -1;
            break;
        }
        if input_eof() {
            break;
        }
        METHOD = get_method(IFD);
        if METHOD < 0 {
            break;
        }
        BYTES_OUT.store(0, std::sync::atomic::Ordering::SeqCst);
    }
    if unsafe { libc::close(IFD) } != 0 {
        read_error();
    }
    if unsafe { LIST != 0 } {
        do_list(METHOD)?;
        return Ok(());
    }
    if TO_STDOUT.load(std::sync::atomic::Ordering::SeqCst) == 0 {
        // Assuming copy_stat can be modified to accept &libc::stat
        copy_stat(unsafe { &ISTAT })?;
        if (unsafe { SYNCHRONOUS }
            && ((unsafe { DFD } >= 0
                && unsafe { libc::fdatasync(DFD) } != 0
                && unsafe { *libc::__errno_location() } != 22)
                || (unsafe { libc::fsync(OFD) } != 0 && unsafe { *libc::__errno_location() } != 22)))
            || unsafe { libc::close(OFD) } != 0
        {
            write_error();
        }
        if unsafe { KEEP == 0 } {
            let mut oldset: libc::sigset_t = unsafe { std::mem::zeroed() };
            let unlink_errno;
            let ifbase = last_component(std::str::from_utf8(&IFNAME).unwrap_or(""));
            let ufd = if atdir_eq(Some(&IFNAME)) {
                unsafe { DFD }
            } else {
                -1
            };
            let res;
            unsafe { libc::sigprocmask(0, &CAUGHT_SIGNALS, &mut oldset) };
            unsafe { REMOVE_OFNAME_FD = -1 };
            res = if ufd < 0 {
                xunlink(std::str::from_utf8(&IFNAME).unwrap_or(""))
            } else {
                if unsafe { libc::unlinkat(ufd, ifbase.unwrap_or("").as_ptr() as *const i8, 0) } == 0 {
                    Ok(())
                } else {
                    Err(())
                }
            };
            unlink_errno = if res.is_ok() { 0 } else { unsafe { *libc::__errno_location() } };
            unsafe { libc::sigprocmask(2, &oldset, std::ptr::null_mut()) };
            if unlink_errno != 0 {
                if unsafe { QUIET == 0 } {
                    eprint!("{}: ", PROGRAM_NAME.unwrap_or(""));
                    if unsafe { EXIT_CODE } == 0 {
                        unsafe { EXIT_CODE = 2 };
                    }
                }
                if unsafe { QUIET == 0 } {
                    unsafe { *libc::__errno_location() = unlink_errno };
                    unsafe { libc::perror(std::str::from_utf8(&IFNAME).unwrap_or("").as_ptr() as *const i8) };
                }
            }
        }
    }
    if METHOD == -1 {
        if TO_STDOUT.load(std::sync::atomic::Ordering::SeqCst) == 0 {
            remove_output_file(false)?;
        }
        return Ok(());
    }
    if unsafe { VERBOSE != 0 } {
        if unsafe { TEST != 0 } {
            eprint!(" OK");
        } else if unsafe { DECOMPRESS != 0 } {
            display_ratio(
                BYTES_OUT.load(std::sync::atomic::Ordering::SeqCst) - (unsafe { BYTES_IN } - unsafe { HEADER_BYTES }),
                BYTES_OUT.load(std::sync::atomic::Ordering::SeqCst),
                &mut std::fs::File::create("/dev/stderr").unwrap(),
            )?;
        } else {
            display_ratio(
                unsafe { BYTES_IN } - (BYTES_OUT.load(std::sync::atomic::Ordering::SeqCst) - unsafe { HEADER_BYTES }),
                unsafe { BYTES_IN },
                &mut std::fs::File::create("/dev/stderr").unwrap(),
            )?;
        }
        if unsafe { TEST == 0 } {
            eprintln!(
                " -- {} {}",
                if unsafe { KEEP != 0 } { "created" } else { "replaced with" },
                std::str::from_utf8(&OFNAME).unwrap_or("")
            );
        }
    }
    Ok(())
}
fn treat_stdin() -> Result<(), ()> {
    unsafe {
        if !FORCE != 0 && !LIST != 0
            && (PRESUME_INPUT_TTY || libc::isatty(if DECOMPRESS != 0 { 0 } else { 1 }) != 0)
        {
            if !QUIET != 0 {
                let mut stderr = i8::stderr();
                let program_name = PROGRAM_NAME.unwrap_or("");
                let action = if DECOMPRESS != 0 { "read from" } else { "written to" };
                let compression = if DECOMPRESS != 0 { "de" } else { "" };
                rpl_fprintf(
                    &mut stderr,
                    "%s: compressed data not %s a terminal. Use -f to force %scompression.\nFor help, type: %s -h\n",
                    &[ArgValue::Str(program_name), ArgValue::Str(action), ArgValue::Str(compression), ArgValue::Str(program_name)],
                )?;
            }
            do_exit(1);
        }
        if DECOMPRESS != 0 || ASCII == 0 {
            // No operation
        }
        if !TEST != 0 && (DECOMPRESS == 0 || ASCII == 0) {
            // No operation
        }
        IFNAME.copy_from_slice(b"stdin\0");
        OFNAME.copy_from_slice(b"stdout\0");
        if libc::fstat(0, &mut ISTAT) != 0 {
            progerror("standard input");
            do_exit(1);
        }
        get_input_size_and_time();
        clear_bufs();
        TO_STDOUT.store(1, std::sync::atomic::Ordering::SeqCst);
        PART_NB = 0;
        IFD = 0;
        STDIN_WAS_READ.store(true, std::sync::atomic::Ordering::SeqCst);
        if DECOMPRESS != 0 {
            METHOD = get_method(IFD);
            if METHOD < 0 {
                do_exit(EXIT_CODE);
            }
        }
        loop {
            if WORK(0, 1) != 0 {
                return Ok(());
            }
            if input_eof() {
                break;
            }
            METHOD = get_method(IFD);
            if METHOD < 0 {
                return Ok(());
            }
            BYTES_OUT.store(0, std::sync::atomic::Ordering::SeqCst);
        }
        if LIST != 0 {
            do_list(METHOD)?;
            return Ok(());
        }
        if VERBOSE != 0 {
            let mut stderr = i8::stderr();
            if TEST != 0 {
                rpl_fprintf(&mut stderr, " OK\n", &[])?;
            } else if DECOMPRESS == 0 {
                display_ratio(
                    BYTES_IN - (BYTES_OUT.load(std::sync::atomic::Ordering::SeqCst) - HEADER_BYTES),
                    BYTES_IN,
                    &mut stderr,
                )?;
                rpl_fprintf(&mut stderr, "\n", &[])?;
            }
        }
    }
    Ok(())
}
fn try_help() -> ! {
    let stderr = &mut std::io::stderr();
    let program_name = unsafe { PROGRAM_NAME.unwrap_or("program") };
    rpl_fprintf(stderr, &format!("Try `{} --help' for more information.\n", program_name), &[]).ok();
    do_exit(1);
}
fn unlzh(in_data: i32, out: i32) -> Result<(), ()> {
    let mut n: usize;
    unsafe {
        IFD = in_data;
        OFD = out;
    }
    decode_start();
    while unsafe { DONE == 0 } {
        n = decode((1 << 13) as usize, &mut WINDOW[..])?;
        if n > 0 {
            write_buf(out, &mut WINDOW[..n])?;
        }
    }
    Ok(())
}
fn unlzw(in_data: i32, out: i32) -> i32 {
    let mut stackp: *mut u8;
    let mut code: i64;
    let mut finchar: i32;
    let mut oldcode: i64;
    let mut incode: i64;
    let mut inbits: i64;
    let mut posbits: i64;
    let mut outpos: i32;
    let mut bitmask: u32;
    let mut free_ent: i64;
    let mut maxcode: i64;
    let mut maxmaxcode: i64;
    let mut n_bits: i32;
    let mut rsize: i32;
    let mut block_mode: i32; // Use a local mutable variable for BLOCK_MODE
    unsafe {
        let mut maxbits = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() } as i32;
        block_mode = maxbits & 0x80;
        if (maxbits & 0x60) != 0 {
            if QUIET == 0 {
                eprintln!(
                    "\n{}: {:?}: warning, unknown flags 0x{:x}\n",
                    PROGRAM_NAME.unwrap(),
                    IFNAME.as_ptr(),
                    (maxbits & 0x60) as u32
                );
                if EXIT_CODE == 0 {
                    EXIT_CODE = 2;
                }
            }
        }
        maxbits &= 0x1f;
        maxmaxcode = 1 << maxbits;
        if maxbits > 16 {
            eprintln!(
                "\n{}: {:?}: compressed with {} bits, can only handle {} bits\n",
                PROGRAM_NAME.unwrap(),
                IFNAME.as_ptr(),
                maxbits,
                16
            );
            EXIT_CODE = 1;
            return 1;
        }
        rsize = INSIZE as i32;
        n_bits = 9;
        maxcode = (1 << n_bits) - 1;
        bitmask = (1 << n_bits) - 1;
        oldcode = -1;
        finchar = 0;
        outpos = 0;
        posbits = (INPTR as i64) << 3;
        free_ent = if block_mode != 0 { 256 + 1 } else { 256 };
        std::ptr::write_bytes(PREV.as_mut_ptr(), 0, 256);
        for code in (0..=255).rev() {
            WINDOW[code as usize] = code as u8;
        }
        loop {
            let mut i: i32;
            let mut e: i32;
            let mut o: i32 = 0;
            rsize = (posbits >> 3) as i32;
            e = if o <= INSIZE as i32 { INSIZE as i32 - o } else { 0 };
            for i in 0..e {
                INBUF[i as usize] = INBUF[(i + o) as usize];
            }
            INSIZE = e as u32;
            posbits = 0;
            if INSIZE < 64 {
                rsize = read_buffer(in_data, &mut INBUF[INSIZE as usize..]).unwrap_or_else(|_| read_error()) as i32;
                INSIZE += rsize as u32;
                BYTES_IN += rsize as libc::off_t;
            }
            inbits = if rsize != 0 {
                ((INSIZE as i64 - INSIZE as i64 % n_bits as i64) << 3)
            } else {
                ((INSIZE as i64) << 3) - (n_bits as i64 - 1)
            };
            while inbits > posbits {
                if free_ent > maxcode {
                    posbits = (posbits - 1) + ((n_bits as i64) << 3) - (posbits - 1 + ((n_bits as i64) << 3)) % ((n_bits as i64) << 3);
                    n_bits += 1;
                    if n_bits == maxbits {
                        maxcode = maxmaxcode;
                    } else {
                        maxcode = (1 << n_bits) - 1;
                    }
                    bitmask = (1 << n_bits) - 1;
                    continue;
                }
                let p = &INBUF[(posbits >> 3) as usize..];
                code = (((p[0] as i64) | ((p[1] as i64) << 8) | ((p[2] as i64) << 16)) >> (posbits & 0x7)) & bitmask as i64;
                posbits += n_bits as i64;
                if oldcode == -1 {
                    if 256 <= code {
                        gzip_error("corrupt input.");
                    }
                    OUTBUF[outpos as usize] = code as u8;
                    finchar = code as i32;
                    oldcode = code;
                    continue;
                }
                if code == 256 && block_mode != 0 {
                    std::ptr::write_bytes(PREV.as_mut_ptr(), 0, 256);
                    free_ent = 256 + 1 - 1;
                    posbits = (posbits - 1) + ((n_bits as i64) << 3) - (posbits - 1 + ((n_bits as i64) << 3)) % ((n_bits as i64) << 3);
                    n_bits = 9;
                    maxcode = (1 << n_bits) - 1;
                    bitmask = (1 << n_bits) - 1;
                    continue;
                }
                incode = code;
                stackp = D_BUF.as_mut_ptr().add(0x8000 - 1) as *mut u8;
                if code >= free_ent {
                    if code > free_ent {
                        if outpos > 0 {
                            write_buf(out, &mut OUTBUF[..outpos as usize]).unwrap();
                        }
                        gzip_error(if TO_STDOUT.load(std::sync::atomic::Ordering::SeqCst) != 0 {
                            "corrupt input."
                        } else {
                            "corrupt input. Use zcat to recover some data."
                        });
                    }
                    *stackp = finchar as u8;
                    code = oldcode;
                }
                while code >= 256 {
                    *stackp = WINDOW[code as usize];
                    code = PREV[code as usize] as i64;
                }
                *stackp = WINDOW[code as usize];
                finchar = WINDOW[code as usize] as i32;
                let mut i = D_BUF.as_mut_ptr().add(0x8000 - 1).cast::<u8>().offset_from(stackp as *const u8) as i32;
                if outpos + i >= 0x40000 {
                    while i > 0 {
                        let chunk_size = std::cmp::min(i, 0x40000 - outpos);
                        if chunk_size > 0 {
                            std::ptr::copy_nonoverlapping(stackp, OUTBUF.as_mut_ptr().add(outpos as usize), chunk_size as usize);
                            outpos += chunk_size;
                        }
                        if outpos >= 0x40000 {
                            write_buf(out, &mut OUTBUF[..outpos as usize]).unwrap();
                            outpos = 0;
                        }
                        stackp = stackp.add(chunk_size as usize);
                        i = D_BUF.as_mut_ptr().add(0x8000 - 1).cast::<u8>().offset_from(stackp as *const u8) as i32;
                    }
                } else {
                    std::ptr::copy_nonoverlapping(stackp, OUTBUF.as_mut_ptr().add(outpos as usize), i as usize);
                    outpos += i;
                }
                if code < maxmaxcode {
                    PREV[code as usize] = oldcode as u16;
                    WINDOW[code as usize] = finchar as u8;
                    free_ent = code + 1;
                }
                oldcode = incode;
            }
            if rsize == 0 {
                break;
            }
        }
        if outpos > 0 {
            write_buf(out, &mut OUTBUF[..outpos as usize]).unwrap();
        }
    }
    0
}
fn unpack(in_data: i32, out: i32) -> Result<(), ()> {
    let mut len: i32;
    let eob: u32;
    let mut peek: u32;
    let peek_mask: u32;
    unsafe {
        IFD = in_data;
        OFD = out;
    }
    let mut tree_desc = TreeDesc::default(); // Assuming TreeDesc has a default implementation
    build_tree(&mut tree_desc);
    unsafe {
        VALID = 0;
        BITBUF = 0;
    }
    peek_mask = (1 << unsafe { PEEK_BITS }) - 1;
    eob = unsafe { LEAVES[MAX_LEN as usize] } as u32 - 1;

    loop {
        while unsafe { VALID } < unsafe { PEEK_BITS } {
            unsafe {
                BITBUF = (BITBUF << 8) | read_byte()? as u64;
                VALID += 8;
            }
        }
        peek = ((unsafe { BITBUF } >> (unsafe { VALID } - unsafe { PEEK_BITS })) & peek_mask as u64) as u32;
        len = unsafe { OUTBUF[peek as usize] } as i32;
        if len > 0 {
            peek >>= unsafe { PEEK_BITS } - len;
        } else {
            let mut mask: u32 = peek_mask;
            len = unsafe { PEEK_BITS };
            while peek < unsafe { PARENTS[len as usize] } as u32 {
                len += 1;
                mask = (mask << 1) + 1;
                while unsafe { VALID } < len {
                    unsafe {
                        BITBUF = (BITBUF << 8) | read_byte()? as u64;
                        VALID += 8;
                    }
                }
                peek = ((unsafe { BITBUF } >> (unsafe { VALID } - len)) & mask as u64) as u32;
            }
        }
        if peek == eob && len == unsafe { MAX_LEN } {
            break;
        }
        unsafe {
            WINDOW[OUTCNT as usize] = LITERAL[(peek + LIT_BASE[len as usize] as u32) as usize];
            OUTCNT += 1;
            if OUTCNT == 0x8000 {
                flush_window()?;
            }
            VALID -= len;
        }
    }
    flush_window()?;
    if unsafe { ORIG_LEN } != (std::sync::atomic::AtomicI64::load(&BYTES_OUT, std::sync::atomic::Ordering::SeqCst) as Ulg & 0xffffffff) {
        gzip_error("invalid compressed data--length error");
    }
    Ok(())
}
fn unzip(in_data: i32, out: i32) -> i32 {
    let mut orig_crc: Ulg = 0;
    let mut orig_len: Ulg = 0;
    let mut n: i32;
    let mut buf: [Uch; 16] = [0; 16];
    let mut err: i32 = 0;
    unsafe {
        IFD = in_data;
        OFD = out;
        updcrc(None, 0);
        if PKZIP != 0 && EXT_HEADER == 0 {
            orig_crc = (INBUF[14] as Ulg | ((INBUF[15] as Ulg) << 8) | ((INBUF[16] as Ulg) << 16) | ((INBUF[17] as Ulg) << 24));
            orig_len = (INBUF[22] as Ulg | ((INBUF[23] as Ulg) << 8) | ((INBUF[24] as Ulg) << 16) | ((INBUF[25] as Ulg) << 24));
        }
        if METHOD == 8 {
            let res = inflate();
            if res == 3 {
                xalloc_die();
            } else if res != 0 {
                gzip_error("invalid compressed data--format violated");
            }
        } else if PKZIP != 0 && METHOD == 0 {
            let mut n = (INBUF[22] as Ulg | ((INBUF[23] as Ulg) << 8) | ((INBUF[24] as Ulg) << 16) | ((INBUF[25] as Ulg) << 24));
            if n != (INBUF[18] as Ulg | ((INBUF[19] as Ulg) << 8) | ((INBUF[20] as Ulg) << 16) | ((INBUF[21] as Ulg) << 24)) - if DECRYPT != 0 { 12 } else { 0 } {
                eprintln!("len {}, siz {}", n, (INBUF[18] as Ulg | ((INBUF[19] as Ulg) << 8) | ((INBUF[20] as Ulg) << 16) | ((INBUF[21] as Ulg) << 24)));
                gzip_error("invalid compressed data--length mismatch");
            }
            while n != 0 {
                n -= 1;
                let c = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() };
                INPTR += 1;
                WINDOW[OUTCNT as usize] = c;
                OUTCNT += 1;
                if OUTCNT == 0x8000 {
                    flush_window().unwrap();
                }
            }
            flush_window().unwrap();
        } else {
            gzip_error("internal error, invalid method");
        }
        if PKZIP == 0 {
            for n in 0..8 {
                buf[n] = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() };
                INPTR += 1;
            }
            orig_crc = (buf[0] as Ulg | ((buf[1] as Ulg) << 8) | ((buf[2] as Ulg) << 16) | ((buf[3] as Ulg) << 24));
            orig_len = (buf[4] as Ulg | ((buf[5] as Ulg) << 8) | ((buf[6] as Ulg) << 16) | ((buf[7] as Ulg) << 24));
        } else if EXT_HEADER != 0 {
            for n in 0..16 {
                buf[n] = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).unwrap() };
                INPTR += 1;
            }
            orig_crc = (buf[4] as Ulg | ((buf[5] as Ulg) << 8) | ((buf[6] as Ulg) << 16) | ((buf[7] as Ulg) << 24));
            orig_len = (buf[12] as Ulg | ((buf[13] as Ulg) << 8) | ((buf[14] as Ulg) << 16) | ((buf[15] as Ulg) << 24));
        }
        if orig_crc != updcrc(Some(&OUTBUF), 0) {
            eprintln!("\n{}: {:?}: invalid compressed data--crc error", PROGRAM_NAME.unwrap(), IFNAME);
            err = 1;
        }
        if orig_len != (BYTES_OUT.load(std::sync::atomic::Ordering::SeqCst) as Ulg & 0xffffffff) {
            eprintln!("\n{}: {:?}: invalid compressed data--length error", PROGRAM_NAME.unwrap(), IFNAME);
            err = 1;
        }
        if PKZIP != 0 && INPTR + 4 < INSIZE && (INBUF[INPTR as usize] as Ulg | ((INBUF[INPTR as usize + 1] as Ulg) << 8) | ((INBUF[INPTR as usize + 2] as Ulg) << 16) | ((INBUF[INPTR as usize + 3] as Ulg) << 24)) == 0x04034b50 {
            if TO_STDOUT.load(std::sync::atomic::Ordering::SeqCst) != 0 {
                if QUIET == 0 {
                    eprintln!("{}: {:?} has more than one entry--rest ignored", PROGRAM_NAME.unwrap(), IFNAME);
                }
                if EXIT_CODE == 0 {
                    EXIT_CODE = 2;
                }
            } else {
                eprintln!("{}: {:?} has more than one entry -- unchanged", PROGRAM_NAME.unwrap(), IFNAME);
                err = 1;
            }
        }
        EXT_HEADER = 0;
        PKZIP = 0;
        UNZIP_CRC = orig_crc;
        if err == 0 {
            return 0;
        }
        EXIT_CODE = 1;
        if TEST == 0 {
            abort_gzip();
        }
    }
    err
}
fn update_timespec(statbuf: &std::fs::Metadata, ts: &mut Option<[libc::timespec; 2]>) -> Result<(), ()> {
    if let Some(timespec) = ts {
        if timespec[0].tv_nsec == ((1 << 30) - 2)
            && timespec[1].tv_nsec == ((1 << 30) - 2)
        {
            return Err(());
        }
        if timespec[0].tv_nsec == ((1 << 30) - 1)
            && timespec[1].tv_nsec == ((1 << 30) - 1)
        {
            *ts = None;
            return Ok(());
        }
        if timespec[0].tv_nsec == ((1 << 30) - 2) {
            timespec[0] = std::time::SystemTime::into(get_stat_atime(statbuf));
        } else if timespec[0].tv_nsec == ((1 << 30) - 1) {
            timespec[0] = gettime();
        }
        if timespec[1].tv_nsec == ((1 << 30) - 2) {
            timespec[1] = std::time::SystemTime::into(get_stat_mtime(statbuf));
        } else if timespec[1].tv_nsec == ((1 << 30) - 1) {
            timespec[1] = gettime();
        }
    }
    Ok(())
}
fn updcrc(s: Option<&[Uch]>, n: usize) -> Ulg {
    let mut c: Ulg;
    if s.is_none() {
        c = 0xffffffff;
    } else {
        c = unsafe { CRC };
        let mut slice = s.unwrap();
        if n > 0 {
            for &byte in slice.iter().take(n) {
                c = CRC_32_TAB[((c as usize) ^ (byte as usize)) & 0xff] ^ (c >> 8);
            }
        }
    }
    unsafe {
        CRC = c;
    }
    c ^ 0xffffffff
}
fn utimens(file: &str, timespec: &[libc::timespec; 2]) -> Result<(), ()> {
    fdutimens(-1, Some(file), Some(timespec))
}
fn validate_timespec(timespec: &mut [libc::timespec; 2]) -> Result<i32, ()> {
    let mut result = 0;
    let mut utime_omit_count = 0;
    const TIMESPEC_HZ: i64 = 1_000_000_000; // Assuming TIMESPEC_HZ is defined somewhere

    if (timespec[0].tv_nsec != ((1 << 30) - 1)
        && timespec[0].tv_nsec != ((1 << 30) - 2)
        && !(0 <= timespec[0].tv_nsec && timespec[0].tv_nsec < TIMESPEC_HZ))
        || (timespec[1].tv_nsec != ((1 << 30) - 1)
            && timespec[1].tv_nsec != ((1 << 30) - 2)
            && !(0 <= timespec[1].tv_nsec && timespec[1].tv_nsec < TIMESPEC_HZ))
    {
        unsafe {
            *libc::__errno_location() = 22;
        }
        return Err(());
    }

    if timespec[0].tv_nsec == ((1 << 30) - 1) || timespec[0].tv_nsec == ((1 << 30) - 2) {
        timespec[0].tv_sec = 0;
        result = 1;
        if timespec[0].tv_nsec == ((1 << 30) - 2) {
            utime_omit_count += 1;
        }
    }

    if timespec[1].tv_nsec == ((1 << 30) - 1) || timespec[1].tv_nsec == ((1 << 30) - 2) {
        timespec[1].tv_sec = 0;
        result = 1;
        if timespec[1].tv_nsec == ((1 << 30) - 2) {
            utime_omit_count += 1;
        }
    }

    Ok(result + (utime_omit_count == 1) as i32)
}
fn vasnprintf<'a>(resultbuf: &'a mut String, lengthp: &'a mut usize, format: &'a str, args: &'a [ArgValue]) -> Result<&'a str, ()> {
    // TODO
    Ok("")
}
fn version() {
    license();
    __printf__("\n", &[]);
    __printf__("Written by Jean-loup Gailly.\n", &[]);
}
fn volatile_strcpy(dst: &mut [u8], src: &[u8]) {
    let mut src_iter = src.iter();
    let mut dst_iter = dst.iter_mut();
    while let (Some(&s), Some(d)) = (src_iter.next(), dst_iter.next()) {
        *d = s;
        if s == 0 {
            break;
        }
    }
}
fn warning(m: &str) {
    unsafe {
        if QUIET == 0 {
            let stderr = std::fs::File::create("/dev/stderr").unwrap();
            let program_name = PROGRAM_NAME; // Assuming PROGRAM_NAME is a &str
            let ifname_str = std::str::from_utf8(&IFNAME).unwrap(); // Assuming IFNAME is a byte slice
            let _ = rpl_fprintf(&mut stderr, "%s: %s: warning: %s\n", &[program_name, ifname_str, m]);
        }
        if EXIT_CODE == 0 {
            EXIT_CODE = 2;
        }
    }
}
fn wcsdup(s: &str) -> Box<str>{todo!("proto")}
fn write_buf(fd: i32, buf: &mut [u8]) -> Result<(), ()> {
    let cnt = buf.len();
    std::sync::atomic::AtomicI64::fetch_add(&BYTES_OUT, cnt as i64, std::sync::atomic::Ordering::SeqCst);
    if unsafe { TEST } != 0 {
        return Ok(());
    }
    let mut remaining = cnt;
    let mut offset = 0;
    while remaining > 0 {
        match write_buffer(fd, &buf[offset..]) {
            Ok(n) if n == remaining => return Ok(()),
            Ok(n) => {
                remaining -= n;
                offset += n;
            }
            Err(_) => write_error(),
        }
    }
    Ok(())
}
fn write_buffer(fd: i32, buf: &[u8]) -> Result<usize, ()> {
    let cnt = std::cmp::min(buf.len(), 0x7fffffff as usize);
    let result = unsafe {
        libc::write(fd, buf.as_ptr() as *const libc::c_void, cnt)
    };

    if result >= 0 {
        Ok(result as usize)
    } else {
        Err(())
    }
}
fn write_error() -> ! {
    use std::io::{self, Write};

    // Get the last OS error code
    let e = std::io::Error::last_os_error().raw_os_error().unwrap_or(0);

    // Assuming PROGRAM_NAME is a global variable or constant
    if let Some(program_name) = unsafe { PROGRAM_NAME } {
        // Assuming ArgValue is defined with a variant that can hold a string
        // and rpl_fprintf is defined to accept the correct arguments
        let _ = rpl_fprintf(&mut io::stderr(), "\n{}: ", &[ArgValue::String(program_name.to_string())]);
    }

    // Write the error message to stderr
    let _ = io::stderr().write_all(unsafe { &OFNAME });

    // Call abort_gzip to terminate the program
    abort_gzip();
}
fn x2nrealloc(p: Option<*mut u8>, pn: &mut usize, s: usize) -> *mut u8 {
    let mut n = *pn;
    let p = if p.is_none() {
        if n == 0 {
            const DEFAULT_MXFAST: usize = 64 * std::mem::size_of::<usize>() / 4;
            n = DEFAULT_MXFAST / s;
            n += (n == 0) as usize;
        }
        None
    } else {
        if n.checked_add((n >> 1) + 1).is_none() {
            xalloc_die();
        }
        Some(p.unwrap())
    };
    let new_p = unsafe { xreallocarray(p.unwrap_or(std::ptr::null_mut()), n, s) };
    *pn = n;
    new_p
}
fn x2realloc(p: Option<&mut [u8]>, ps: &mut usize) -> *mut u8 {
    crate::x2nrealloc(p.map(|slice| slice.as_mut_ptr()), ps, 1)
}
fn xalloc_die() -> ! {
    crate::error(crate::EXIT_FAILURE, 0, "memory exhausted");
    std::process::abort();
}
fn xcalloc<T: Default + std::clone::Clone>(n: usize) -> Vec<T> {
    vec![T::default(); n]
}
fn xcharalloc(n: usize) -> Option<*mut u8> {
    if std::mem::size_of::<u8>() == 1 {
        Some(xmalloc(n).as_ptr())
    } else {
        xnmalloc(n, std::mem::size_of::<u8>())
    }
}
fn xicalloc<T>(n: IdxT, s: IdxT) -> *const T {
    icalloc::<T>(n, s).map_or(std::ptr::null(), |p| nonnull(p))
}
fn ximalloc(s: IdxT) -> *mut u8 {
    imalloc(s).map_or(std::ptr::null_mut(), |p| nonnull(p) as *mut u8)
}
fn ximemdup(p: &[u8]) -> Vec<u8> {
    let s = p.len() as IdxT;
    let mut buffer = std::vec::Vec::with_capacity(s as usize);
    buffer.extend_from_slice(p);
    buffer
}
fn ximemdup0(p: &[u8]) -> Vec<u8> {
    let mut result = std::vec::Vec::with_capacity(p.len() + 1);
    result.extend_from_slice(p);
    result.push(0);
    result
}
fn xinmalloc(n: IdxT, s: IdxT) -> Result<*const (), ()> {
    crate::xireallocarray(std::ptr::null_mut(), n, s)
}
fn xirealloc(p: *mut u8, s: IdxT) -> *mut u8 {
    match irealloc(p, s) {
        Some(ptr) => ptr, // Directly return the pointer if it's valid
        None => std::ptr::null_mut(), // Return a null pointer if None
    }
}
fn xireallocarray(p: *mut (), n: IdxT, s: IdxT) -> Result<*const (), ()> {
    ireallocarray(p, n, s).map(|ptr| nonnull(ptr))
}
fn xizalloc<T>(s: IdxT) -> Option<*const T> {
    let ptr = xicalloc::<T>(s, 1);
    if ptr.is_null() {
        None
    } else {
        Some(ptr)
    }
}
fn xmalloc(s: usize) -> std::ptr::NonNull<u8> {
    std::ptr::NonNull::new(malloc(s)).expect("malloc returned null")
}
fn xmax(size1: usize, size2: usize) -> usize {
    std::cmp::max(size1, size2)
}
fn xmemdup(p: &[u8]) -> std::ptr::NonNull<u8> {
    let s = p.len();
    let dest = xmalloc(s);
    unsafe {
        std::ptr::copy_nonoverlapping(p.as_ptr(), dest.as_ptr(), s);
    }
    dest
}
fn xnmalloc(n: usize, s: usize) -> Option<*mut u8> {
    let ptr = std::ptr::null_mut();
    let result = xreallocarray(ptr, n, s);
    if result.is_null() {
        None
    } else {
        Some(result)
    }
}
fn xnrealloc(p: *mut u8, n: usize, s: usize) -> Option<*mut u8> {
    let new_ptr = xreallocarray(p, n, s);
    if new_ptr.is_null() {
        None
    } else {
        Some(new_ptr)
    }
}
fn xpalloc(pa: Option<std::ptr::NonNull<u8>>, pn: &mut IdxT, n_incr_min: IdxT, n_max: isize, s: IdxT) -> Result<std::ptr::NonNull<u8>, ()> {
    use std::convert::TryInto;
    let n0 = *pn;
    const DEFAULT_MXFAST: IdxT = 64 * std::mem::size_of::<usize>() as IdxT / 4;
    let mut n = n0.checked_add(n0 >> 1).unwrap_or(9223372036854775807i64 as IdxT);
    if n_max >= 0 && n_max < n {
        n = n_max;
    }
    let mut nbytes = n.checked_mul(s).unwrap_or_else(|| {
        if 9223372036854775807i64 < 18446744073709551615u64 as i64 {
            9223372036854775807isize
        } else {
            (18446744073709551615u64 as i64).try_into().unwrap()
        }
    });
    let adjusted_nbytes = if nbytes < DEFAULT_MXFAST {
        DEFAULT_MXFAST
    } else {
        0
    };
    if adjusted_nbytes != 0 {
        n = adjusted_nbytes / s;
        nbytes = adjusted_nbytes - adjusted_nbytes % s;
    }
    if pa.is_none() {
        *pn = 0;
    }
    if n - n0 < n_incr_min
        && (n0.checked_add(n_incr_min).is_none()
            || (n_max >= 0 && n_max < n)
            || n.checked_mul(s).is_none())
    {
        xalloc_die();
    }
    let new_pa = xrealloc(pa.map_or(std::ptr::null_mut(), |p| p.as_ptr()), nbytes as usize);
    *pn = n;
    Ok(new_pa)
}
fn xrealloc(p: *mut u8, s: usize) -> std::ptr::NonNull<u8> {
    let r = unsafe { realloc(p, s) };
    if r.is_null() && (!p.is_null() || s != 0) {
        xalloc_die();
    }
    std::ptr::NonNull::new(r).unwrap()
}
fn xreallocarray(p: *mut u8, n: usize, s: usize) -> *mut u8 {
    let layout = std::alloc::Layout::array::<u8>(n * s).unwrap();
    let r = unsafe { std::alloc::realloc(p as *mut u8, layout, n * s) };
    if r.is_null() && (p.is_null() || (n != 0 && s != 0)) {
        xalloc_die();
    }
    r
}
fn xstrdup(string: &str) -> Box<[u8]> {
    let bytes = string.as_bytes();
    let mut vec = Vec::with_capacity(bytes.len() + 1);
    vec.extend_from_slice(bytes);
    vec.push(0); // Add null terminator
    vec.into_boxed_slice()
}
fn xsum(size1: usize, size2: usize) -> Result<usize, ()> {
    size1.checked_add(size2).ok_or(())
}
fn xsum3(size1: usize, size2: usize, size3: usize) -> Result<usize, ()> {
    match crate::xsum(size1, size2) {
        Ok(sum1) => crate::xsum(sum1, size3),
        Err(e) => Err(e),
    }
}
fn xsum4(size1: usize, size2: usize, size3: usize, size4: usize) -> Result<usize, ()> {
    xsum(size1, size2)
        .and_then(|sum1| xsum(sum1, size3))
        .and_then(|sum2| xsum(sum2, size4))
}
fn xunlink(filename: &str) -> Result<(), ()> {
    let c_filename = std::ffi::CString::new(filename).map_err(|_| ())?;
    let r = unsafe { libc::unlink(c_filename.as_ptr()) };
    if r == 0 {
        Ok(())
    } else {
        Err(())
    }
}
fn xzalloc<T: Default + std::clone::Clone>(s: usize) -> Vec<T> {
    xcalloc(s)
}
fn yesno() -> Result<bool, ()> {
    use std::io::Read;
    let mut buffer = [0; 1];
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let c = match handle.read_exact(&mut buffer) {
        Ok(_) => buffer[0] as char,
        Err(_) => return Err(()),
    };
    let yes = c == 'y' || c == 'Y';
    while match handle.read_exact(&mut buffer) {
        Ok(_) => buffer[0] as char != '\n',
        Err(_) => false,
    } {}
    Ok(yes)
}
fn zip(in_data: i32, out: i32) -> i32 {
    let mut flags: Uch = 0;
    let mut attr: Ush = 0;
    let mut deflate_flags: Ush = 0;
    let mut stamp: Ulg;
    unsafe {
        IFD = in_data;
        OFD = out;
        OUTCNT = 0;
        METHOD = 8;
        OUTBUF[OUTCNT as usize] = b'\x1f';
        OUTCNT += 1;
        if OUTCNT == 0x40000 {
            flush_outbuf().unwrap();
        }
        OUTBUF[OUTCNT as usize] = b'\x8b';
        OUTCNT += 1;
        if OUTCNT == 0x40000 {
            flush_outbuf().unwrap();
        }
        OUTBUF[OUTCNT as usize] = 8;
        OUTCNT += 1;
        if OUTCNT == 0x40000 {
            flush_outbuf().unwrap();
        }
        if SAVE_ORIG_NAME != 0 {
            flags |= 0x08;
        }
        OUTBUF[OUTCNT as usize] = flags;
        OUTCNT += 1;
        if OUTCNT == 0x40000 {
            flush_outbuf().unwrap();
        }
        if TIME_STAMP.subsec_nanos() < 0 {
            stamp = 0;
        } else if TIME_STAMP.as_secs() > 0 && TIME_STAMP.as_secs() <= 0xffffffff {
            stamp = TIME_STAMP.as_secs();
        } else {
            warning("file timestamp out of range for gzip format");
            stamp = 0;
        }
        if OUTCNT < 0x40000 - 2 {
            OUTBUF[OUTCNT as usize] = (stamp & 0xff) as Uch;
            OUTBUF[OUTCNT as usize + 1] = ((stamp >> 8) & 0xff) as Uch;
            OUTCNT += 2;
        } else {
            OUTBUF[OUTCNT as usize] = (stamp & 0xff) as Uch;
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf().unwrap();
            }
            OUTBUF[OUTCNT as usize] = ((stamp >> 8) & 0xff) as Uch;
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf().unwrap();
            }
        }
        if OUTCNT < 0x40000 - 2 {
            OUTBUF[OUTCNT as usize] = ((stamp >> 16) & 0xff) as Uch;
            OUTBUF[OUTCNT as usize + 1] = ((stamp >> 24) & 0xff) as Uch;
            OUTCNT += 2;
        } else {
            OUTBUF[OUTCNT as usize] = ((stamp >> 16) & 0xff) as Uch;
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf().unwrap();
            }
            OUTBUF[OUTCNT as usize] = ((stamp >> 24) & 0xff) as Uch;
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf().unwrap();
            }
        }
        updcrc(None, 0);
        bi_init(out);
        ct_init(Some(Box::new(attr)), Some(&mut METHOD));
        if LEVEL == 1 {
            deflate_flags |= 0x04; // FAST
        } else if LEVEL == 9 {
            deflate_flags |= 0x02; // SLOW
        }
        OUTBUF[OUTCNT as usize] = deflate_flags as Uch;
        OUTCNT += 1;
        if OUTCNT == 0x40000 {
            flush_outbuf().unwrap();
        }
        OUTBUF[OUTCNT as usize] = 0x03;
        OUTCNT += 1;
        if OUTCNT == 0x40000 {
            flush_outbuf().unwrap();
        }
        if SAVE_ORIG_NAME != 0 {
            if let Some(p) = gzip_base_name(std::str::from_utf8(&IFNAME).unwrap()) {
                for &byte in p.as_bytes() {
                    OUTBUF[OUTCNT as usize] = byte;
                    OUTCNT += 1;
                    if OUTCNT == 0x40000 {
                        flush_outbuf().unwrap();
                    }
                }
                OUTBUF[OUTCNT as usize] = 0;
                OUTCNT += 1;
                if OUTCNT == 0x40000 {
                    flush_outbuf().unwrap();
                }
            }
        }
        HEADER_BYTES = OUTCNT as libc::off_t;
        deflate(LEVEL).unwrap();
        if IFILE_SIZE != -1 && BYTES_IN != IFILE_SIZE {
            // Convert the strings to ArgValue manually or use a different method
            let program_name = PROGRAM_NAME.unwrap();
            let ifname_str = std::str::from_utf8(&IFNAME).unwrap();
            rpl_fprintf(
                &mut std::fs::File::create("/dev/stderr").unwrap(), // Use a File instead of Stderr
                "%s: %s: file size changed while zipping\n",
                &[ArgValue::from(program_name), ArgValue::from(ifname_str)],
            )
            .unwrap();
        }
        if OUTCNT < 0x40000 - 2 {
            OUTBUF[OUTCNT as usize] = (getcrc() & 0xff) as Uch;
            OUTBUF[OUTCNT as usize + 1] = ((getcrc() >> 8) & 0xff) as Uch;
            OUTCNT += 2;
        } else {
            OUTBUF[OUTCNT as usize] = (getcrc() & 0xff) as Uch;
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf().unwrap();
            }
            OUTBUF[OUTCNT as usize] = ((getcrc() >> 8) & 0xff) as Uch;
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf().unwrap();
            }
        }
        if OUTCNT < 0x40000 - 2 {
            OUTBUF[OUTCNT as usize] = ((getcrc() >> 16) & 0xff) as Uch;
            OUTBUF[OUTCNT as usize + 1] = ((getcrc() >> 24) & 0xff) as Uch;
            OUTCNT += 2;
        } else {
            OUTBUF[OUTCNT as usize] = ((getcrc() >> 16) & 0xff) as Uch;
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf().unwrap();
            }
            OUTBUF[OUTCNT as usize] = ((getcrc() >> 24) & 0xff) as Uch;
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf().unwrap();
            }
        }
        if OUTCNT < 0x40000 - 2 {
            OUTBUF[OUTCNT as usize] = (BYTES_IN & 0xff) as Uch;
            OUTBUF[OUTCNT as usize + 1] = ((BYTES_IN >> 8) & 0xff) as Uch;
            OUTCNT += 2;
        } else {
            OUTBUF[OUTCNT as usize] = (BYTES_IN & 0xff) as Uch;
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf().unwrap();
            }
            OUTBUF[OUTCNT as usize] = ((BYTES_IN >> 8) & 0xff) as Uch;
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf().unwrap();
            }
        }
        if OUTCNT < 0x40000 - 2 {
            OUTBUF[OUTCNT as usize] = ((BYTES_IN >> 16) & 0xff) as Uch;
            OUTBUF[OUTCNT as usize + 1] = ((BYTES_IN >> 24) & 0xff) as Uch;
            OUTCNT += 2;
        } else {
            OUTBUF[OUTCNT as usize] = ((BYTES_IN >> 16) & 0xff) as Uch;
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf().unwrap();
            }
            OUTBUF[OUTCNT as usize] = ((BYTES_IN >> 24) & 0xff) as Uch;
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf().unwrap();
            }
        }
        HEADER_BYTES += 2 * 4;
        flush_outbuf().unwrap();
    }
    0
}
fn main() {}