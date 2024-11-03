extern crate once_cell;extern crate libc;
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum ArgType {
    TYPE_NONE,
    TYPE_SCHAR,
    TYPE_UCHAR,
    TYPE_SHORT,
    TYPE_USHORT,
    TYPE_INT,
    TYPE_UINT,
    TYPE_LONGINT,
    TYPE_ULONGINT,
    TYPE_LONGLONGINT,
    TYPE_ULONGLONGINT,
    TYPE_DOUBLE,
    TYPE_LONGDOUBLE,
    TYPE_CHAR,
    TYPE_WIDE_CHAR,
    TYPE_STRING,
    TYPE_WIDE_STRING,
    TYPE_POINTER,
    TYPE_COUNT_SCHAR_POINTER,
    TYPE_COUNT_SHORT_POINTER,
    TYPE_COUNT_INT_POINTER,
    TYPE_COUNT_LONGINT_POINTER,
    TYPE_COUNT_LONGLONGINT_POINTER,
}
#[derive(Clone, Copy)]
struct Argument {
    type_: ArgType,
    a: ArgumentValue,
}
#[derive(Clone, Copy)]
union ArgumentValue {
    a_schar: std::os::raw::c_char,
    a_uchar: std::os::raw::c_uchar,
    a_short: std::os::raw::c_short,
    a_ushort: std::os::raw::c_ushort,
    a_int: std::os::raw::c_int,
    a_uint: std::os::raw::c_uint,
    a_longint: std::os::raw::c_long,
    a_ulongint: std::os::raw::c_ulong,
    a_longlongint: i64,
    a_ulonglongint: u64,
    a_float: std::os::raw::c_float,
    a_double: std::os::raw::c_double,
    a_longdouble: std::os::raw::c_double, // Rust does not have a separate type for long double
    a_char: std::os::raw::c_int,
    a_wide_char: i32, // Assuming wint_t is equivalent to i32
    a_string: *const std::os::raw::c_char,
    a_wide_string: *const i32, // Assuming wchar_t* is equivalent to *const i32
    a_pointer: *const std::os::raw::c_void,
    a_count_schar_pointer: *mut std::os::raw::c_char,
    a_count_short_pointer: *mut std::os::raw::c_short,
    a_count_int_pointer: *mut std::os::raw::c_int,
    a_count_longint_pointer: *mut std::os::raw::c_long,
    a_count_longlongint_pointer: *mut i64,
}
#[derive(Clone, Copy)]
struct Arguments {
    count: usize,
    arg: *const Argument,
    direct_alloc_arg: [Argument; 7],
}
#[derive(Clone, Copy)]
union Bytes {
    word: i64,
    bytes: std::mem::ManuallyDrop<BytesInner>,
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct BytesInner {
    dummy: i32,
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct CdBuf {
    fd: i32,
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct CharDirective<'a> {
    dir_start: Option<&'a str>,
    dir_end: Option<&'a str>,
    flags: i32,
    width_start: Option<&'a str>,
    width_end: Option<&'a str>,
    width_arg_index: usize,
    precision_start: Option<&'a str>,
    precision_end: Option<&'a str>,
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
type ComparisonFunction = fn(*const std::ffi::c_void, *const std::ffi::c_void) -> std::os::raw::c_int;
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Config {
    good_length: Ush,
    max_lazy: Ush,
    nice_length: Ush,
    max_chain: Ush,
}
type CountInt = u64;
type CountShort = u16;
#[derive(Clone, Copy)]
struct CtData {
    fc: FcUnion,
    dl: DlUnion,
}
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct DirentryT {
    name: Option<String>,
}
#[derive(Clone, Copy)]
union DlUnion {
    dad: Ush,
    len: Ush,
}
type Dummy = i32;
#[derive(Clone, Copy)]
union FcUnion {
    freq: Ush,
    code: Ush,
}
type FileT = i32;
type FpucwT = u32;
type GlFunctionTakingIntReturningVoidT = fn(i32);
type GlLockT = i32;
type GlOnceT = i32;
type GlRecursiveLockT = i32;
type GlRwlockT = i32;
#[derive(Clone, Copy)]
struct Huft {
    e: Uch,
    b: Uch,
    v: HuftUnion,
}
#[derive(Clone, Copy)]
union HuftUnion {
    n: Ush,
    t: *mut Huft,
}
type IPos = u32;
type IdxT = usize;
type Pos = Ush;
type SaHandlerT = fn(i32);
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct SavedCwd {
    desc: i32,
    name: Option<String>,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum SavedirOption {
    SAVEDIR_SORT_NONE = 0,
    SAVEDIR_SORT_NAME = 1,
    SAVEDIR_SORT_FASTREAD = 2,
}
type SmallT = u8;
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct TimeTMustBeIntegral {
    __floating_time_t_unsupported: u32,
}
struct TreeDesc<'a> {
    dyn_tree: &'a mut CtData,
    static_tree: &'a mut CtData,
    extra_bits: &'a mut [i32],
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
static mut BLOCK_MODE: i32 = 0x80;
static mut BLOCK_START: i64 = 0;
static mut BL_COUNT: [Ush; 16] = [0; 16];
static BL_DESC: TreeDesc<'static> = TreeDesc {
    dyn_tree: &mut BL_TREE[0] as &mut CtData,
    static_tree: &mut *mut CtData::default(),
    extra_bits: &mut EXTRA_BLBITS,
    extra_base: 0,
    elems: 19,
    max_length: 7,
    max_code: 0,
};
static BL_ORDER: [Uch; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
const BL_TREE: [CtData; 2*19 + 1] = [CtData {
    fc: FcUnion { freq: 0 },
    dl: DlUnion { dad: 0 },
}; 2*19 + 1];
static BORDER: [u32; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15
];
static mut BYTES_IN: std::os::raw::c_long = 0;
static mut BYTES_OUT: std::os::raw::c_long = 0;
static mut CAUGHT_SIGNALS: libc::sigset_t = libc::sigset_t { __val: [0; 16] };
static COMPARISON_FUNCTION_TABLE: [ComparisonFunction; 2] = [
    std::ptr::null(),
    direntry_cmp_name,
];
static mut COMPRESSED_LEN: std::os::raw::c_long = 0;
const CONFIGURATION_TABLE: [Config; 10] = [
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
static CPDEXT: [Ush; 28] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6,
    7, 7, 8, 8, 9, 9, 10, 10, 11, 11,
    12, 12, 13, 13,
];
static CPDIST: [Ush; 29] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193,
    257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145,
    8193, 12289, 16385,
];
static CPLENS: [Ush; 30] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31,
    35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258, 0
];
static CPLEXT: [Ush; 30] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2,
    3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0, 99
];
static mut CRC: Ulg = 0xffffffff;
const CRC_32_TAB: [Ulg; 256] = [
    0x00000000, 0x77073096, 0xee0e612c, 0x990951ba, 0x076dc419, 0x706af48f, 0xe963a535, 0x9e6495a3,
    0x0edb8832, 0x79dcb8a4, 0xe0d5e91e, 0x97d2d988, 0x09b64c2b, 0x7eb17cbd, 0xe7b82d07, 0x90bf1d91,
    0x1db71064, 0x6ab020f2, 0xf3b97148, 0x84be41de, 0x1adad47d, 0x6ddde4eb, 0xf4d4b551, 0x83d385c7,
    0x136c9856, 0x646ba8c0, 0xfd62f97a, 0x8a65c9ec, 0x14015c4f, 0x63066cd9, 0xfa0f3d63, 0x8d080df5,
    0x3b6e20c8, 0x4c69105e, 0xd56041e4, 0xa2677172, 0x3c03e4d1, 0x4b04d447, 0xd20d85fd, 0xa50ab56b,
    0x35b5a8fa, 0x42b2986c, 0xdbbbc9d6, 0xacbcf940, 0x32d86ce3, 0x45df5c75, 0xdcd60dcf, 0xabd13d59,
    0x26d930ac, 0x51de003a, 0xc8d75180, 0xbfd06116, 0x21b4f4b5, 0x56b3c423, 0xcfba9599, 0xb8bda50f,
    0x2802b89e, 0x5f058808, 0xc60cd9b2, 0xb10be924, 0x2f6f7c87, 0x58684c11, 0xc1611dab, 0xb6662d3d,
    0x76dc4190, 0x01db7106, 0x98d220bc, 0xefd5102a, 0x71b18589, 0x06b6b51f, 0x9fbfe4a5, 0xe8b8d433,
    0x7807c9a2, 0x0f00f934, 0x9609a88e, 0xe10e9818, 0x7f6a0dbb, 0x086d3d2d, 0x91646c97, 0xe6635c01,
    0x6b6b51f4, 0x1c6c6162, 0x856530d8, 0xf262004e, 0x6c0695ed, 0x1b01a57b, 0x8208f4c1, 0xf50fc457,
    0x65b0d9c6, 0x12b7e950, 0x8bbeb8ea, 0xfcb9887c, 0x62dd1ddf, 0x15da2d49, 0x8cd37cf3, 0xfbd44c65,
    0x4db26158, 0x3ab551ce, 0xa3bc0074, 0xd4bb30e2, 0x4adfa541, 0x3dd895d7, 0xa4d1c46d, 0xd3d6f4fb,
    0x4369e96a, 0x346ed9fc, 0xad678846, 0xda60b8d0, 0x44042d73, 0x33031de5, 0xaa0a4c5f, 0xdd0d7cc9,
    0x5005713c, 0x270241aa, 0xbe0b1010, 0xc90c2086, 0x5768b525, 0x206f85b3, 0xb966d409, 0xce61e49f,
    0x5edef90e, 0x29d9c998, 0xb0d09822, 0xc7d7a8b4, 0x59b33d17, 0x2eb40d81, 0xb7bd5c3b, 0xc0ba6cad,
    0xedb88320, 0x9abfb3b6, 0x03b6e20c, 0x74b1d29a, 0xead54739, 0x9dd277af, 0x04db2615, 0x73dc1683,
    0xe3630b12, 0x94643b84, 0x0d6d6a3e, 0x7a6a5aa8, 0xe40ecf0b, 0x9309ff9d, 0x0a00ae27, 0x7d079eb1,
    0xf00f9344, 0x8708a3d2, 0x1e01f268, 0x6906c2fe, 0xf762575d, 0x806567cb, 0x196c3671, 0x6e6b06e7,
    0xfed41b76, 0x89d32be0, 0x10da7a5a, 0x67dd4acc, 0xf9b9df6f, 0x8ebeeff9, 0x17b7be43, 0x60b08ed5,
    0xd6d6a3e8, 0xa1d1937e, 0x38d8c2c4, 0x4fdff252, 0xd1bb67f1, 0xa6bc5767, 0x3fb506dd, 0x48b2364b,
    0xd80d2bda, 0xaf0a1b4c, 0x36034af6, 0x41047a60, 0xdf60efc3, 0xa867df55, 0x316e8eef, 0x4669be79,
    0xcb61b38c, 0xbc66831a, 0x256fd2a0, 0x5268e236, 0xcc0c7795, 0xbb0b4703, 0x220216b9, 0x5505262f,
    0xc5ba3bbe, 0xb2bd0b28, 0x2bb45a92, 0x5cb36a04, 0xc2d7ffa7, 0xb5d0cf31, 0x2cd99e8b, 0x5bdeae1d,
    0x9b64c2b0, 0xec63f226, 0x756aa39c, 0x026d930a, 0x9c0906a9, 0xeb0e363f, 0x72076785, 0x05005713,
    0x95bf4a82, 0xe2b87a14, 0x7bb12bae, 0x0cb61b38, 0x92d28e9b, 0xe5d5be0d, 0x7cdcefb7, 0x0bdbdf21,
    0x86d3d2d4, 0xf1d4e242, 0x68ddb3f8, 0x1fda836e, 0x81be16cd, 0xf6b9265b, 0x6fb077e1, 0x18b74777,
    0x88085ae6, 0xff0f6a70, 0x66063bca, 0x11010b5c, 0x8f659eff, 0xf862ae69, 0x616bffd3, 0x166ccf45,
    0xa00ae278, 0xd70dd2ee, 0x4e048354, 0x3903b3c2, 0xa7672661, 0xd06016f7, 0x4969474d, 0x3e6e77db,
    0xaed16a4a, 0xd9d65adc, 0x40df0b66, 0x37d83bf0, 0xa9bcae53, 0xdebb9ec5, 0x47b2cf7f, 0x30b5ffe9,
    0xbdbdf21c, 0xcabac28a, 0x53b39330, 0x24b4a3a6, 0xbad03605, 0xcdd70693, 0x54de5729, 0x23d967bf,
    0xb3667a2e, 0xc4614ab8, 0x5d681b02, 0x2a6f2b94, 0xb40bbe37, 0xc30c8ea1, 0x5a05df1b, 0x2d02ef8d,
];
static mut DBITS: i32 = 6;
static mut DECOMPRESS: i32 = 0;
static mut DECRYPT: i32 = 0;
static mut DEPTH: [Uch; 2*(256 +1+29)+1] = [0; 2*(256 +1+29)+1];
static mut DFD: i32 = -1;
static mut DFNAME: [u8; 1024] = [0; 1024];
static mut DIST_CODE: [Uch; 512] = [0; 512];
static mut DONE: i32 = 0;
const DOT: char = '.';
const DYN_DTREE: [CtData; 2*30 + 1] = [CtData {
    fc: FcUnion { freq: 0 },
    dl: DlUnion { dad: 0 },
}; 2*30 + 1];
static mut DYN_LTREE: [CtData; (2*(256 +1+29)+1)] = [CtData { fc: FcUnion { freq: 0 }, dl: DlUnion { dad: 0 } }; (2*(256 +1+29)+1)];
static mut D_BUF: [Ush; 0x8000] = [0; 0x8000];
static D_DESC: TreeDesc<'static> = TreeDesc {
    dyn_tree: &mut DYN_DTREE[0],
    static_tree: unsafe { &mut *STATIC_DTREE.as_mut_ptr() },
    extra_bits: &mut EXTRA_DBITS,
    extra_base: 0,
    elems: 30,
    max_length: 15,
    max_code: 0,
};
static mut ENV: *const i8 = std::ptr::null();
static mut EOFILE: i32 = 0;
static mut EXITING_SIGNAL: i32 = 0;
static mut EXIT_CODE: i32 = 0;
static mut EXIT_FAILURE: i32 = 1;
static EXTRA_BLBITS: [i32; 19] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 7];
static EXTRA_DBITS: [i32; 30] = [0,0,0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9,10,10,11,11,12,12,13,13];
static EXTRA_LBITS: [i32; 29] = [0,0,0,0,0,0,0,0,1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4,5,5,5,5,0];
static mut EXT_HEADER: i32 = 0;
static mut FILE_METHOD: *mut i32 = std::ptr::null_mut();
static mut FILE_TYPE: *mut Ush = std::ptr::null_mut();
static mut FLAGS: Uch = 0;
static mut FLAG_BIT: Uch = 0;
static mut FLAG_BUF: [Uch; 0x8000 / 8] = [0; 0x8000 / 8];
static mut FORCE: i32 = 0;
static mut FOREGROUND: i32 = 0;
static mut GOOD_MATCH: u32 = 0;
static HANDLED_SIG: [i32; 6] = [2, 1, 13, 15, 24, 25];
static mut HEADER_BYTES: std::os::raw::c_long = 0;
static mut HEAP: [i32; 2*(256 +1+29)+1] = [0; 2*(256 +1+29)+1];
static mut HEAP_LEN: i32 = 0;
static mut HEAP_MAX: i32 = 0;
static mut HUFTS: u32 = 0;
static mut IFD: i32 = 0;
static mut IFILE_SIZE: std::os::raw::c_long = 0;
static mut IFNAME: [u8; 1024] = [0; 1024];
static mut INBUF: [Uch; 0x40000 + 64] = [0; 0x40000 + 64];
static mut INPTR: u32 = 0;
static mut INPUT_LEN: std::os::raw::c_long = 0;
static mut INSIZE: u32 = 0;
static mut INS_H: u32 = 0;
static mut ISTAT: Option<std::fs::Metadata> = None;
static mut J: i32 = 0;
static mut KEEP: i32 = 0;
static mut LAST_DIST: u32 = 0;
static mut LAST_FLAGS: u32 = 0;
static mut LAST_LIT: u32 = 0;
static mut LAST_MEMBER: i32 = 0;
static mut LBITS: i32 = 9;
static mut LEAVES: [i32; 26] = [0; 26];
static mut LENGTH_CODE: [Uch; 256] = [0; 256];
static mut LEVEL: i32 = 6;
static LICENSE_MSG: [&'static str; 6] = [
    "Copyright (C) 2018 Free Software Foundation, Inc.",
    "Copyright (C) 1993 Jean-loup Gailly.",
    "This is free software.  You may redistribute copies of it under the terms of",
    "the GNU General Public License <https://www.gnu.org/licenses/gpl.html>.",
    "There is NO WARRANTY, to the extent permitted by law.",
    "0",
];
static mut LIST: i32 = 0;
static mut LITERAL: [Uch; 256] = [0; 256];
static mut LIT_BASE: [i32; 25 + 1] = [0; 25 + 1];
const LONGOPTS: [Option<&str>; 27] = [
    Some("ascii"),
    Some("to-stdout"),
    Some("stdout"),
    Some("decompress"),
    Some("uncompress"),
    Some("force"),
    Some("help"),
    Some("keep"),
    Some("list"),
    Some("license"),
    Some("no-name"),
    Some("name"),
    Some("-presume-input-tty"),
    Some("quiet"),
    Some("silent"),
    Some("synchronous"),
    Some("recursive"),
    Some("suffix"),
    Some("test"),
    Some("verbose"),
    Some("version"),
    Some("fast"),
    Some("best"),
    Some("lzw"),
    Some("bits"),
    Some("rsyncable"),
    None,
];
static mut LOOKAHEAD: u32 = 0;
static mut LUTIMENSAT_WORKS_REALLY: i32 = 0;
static L_DESC: TreeDesc = TreeDesc {
    dyn_tree: std::cell::Cell::new(&mut DYN_LTREE[0]),
    static_tree: std::cell::Cell::new(&mut STATIC_LTREE[0]),
    extra_bits: std::cell::Cell::new(&mut EXTRA_LBITS),
    extra_base: 256 + 1,
    elems: 256 + 1 + 29,
    max_length: 15,
    max_code: 0,
};
static MASK_BITS: [Ush; 17] = [
    0x0000,
    0x0001, 0x0003, 0x0007, 0x000f, 0x001f, 0x003f, 0x007f, 0x00ff,
    0x01ff, 0x03ff, 0x07ff, 0x0fff, 0x1fff, 0x3fff, 0x7fff, 0xffff
];
static mut MATCH_START: u32 = 0;
static mut MAXBITS: i32 = 16;
static mut MAX_CHAIN_LENGTH: u32 = 0;
static mut MAX_LAZY_MATCH: u32 = 0;
static mut MAX_LEN: i32 = 0;
static METHOD: i32 = 8;
static mut NICE_MATCH: i32 = 0;
static mut NO_NAME: i32 = -1;
static mut NO_TIME: i32 = -1;
static mut OFD: i32 = 0;
static mut OFNAME: [u8; 1024] = [0; 1024];
static mut OPT_LEN: Ulg = 0;
static mut ORIG_LEN: Ulg = 0;
static mut OUTBUF: [Uch; 0x40000 + 2048] = [0; 0x40000 + 2048];
static mut OUTCNT: u32 = 0;
static mut PARENTS: [i32; 25 + 1] = [0; 25 + 1];
static mut PART_NB: i32 = 0;
static mut PEEK_BITS: i32 = 0;
static mut PKZIP: i32 = 0;
static mut PRESUME_INPUT_TTY: bool = false;
const PREV: [Ush; 1 << 16] = [0; 1 << 16];
static mut PREV_LENGTH: u32 = 0;
static mut PROGRAM_NAME: *const i8 = std::ptr::null();
static mut PT_LEN: [Uch; 1 << 5] = [0; 1 << 5];
static mut PT_TABLE: [Ush; 256] = [0; 256];
static mut QUIET: i32 = 0;
static mut READ_BUF: Option<usize> = None;
static mut RECURSIVE: i32 = 0;
static mut REMOVE_OFNAME: [u8; 1024] = [0; 1024];
static mut REMOVE_OFNAME_FD: i32 = -1;
static mut RSYNC: i32 = 0;
static mut RSYNC_CHUNK_END: Ulg = 0;
static mut RSYNC_SUM: Ulg = 0;
static mut SAVE_ORIG_NAME: i32 = 0;
const SHORTOPTS: &str = "ab:cdfhH?klLmMnNqrS:tvVZ123456789";
static mut STATIC_DTREE: [CtData; 30] = [CtData {
    fc: FcUnion { freq: 0 },
    dl: DlUnion { dad: 0 },
}; 30];
static mut STATIC_LEN: Ulg = 0;
static mut STATIC_LTREE: [CtData; (256 + 1 + 29) + 2] = [CtData { fc: FcUnion { freq: 0 }, dl: DlUnion { dad: 0 } }; (256 + 1 + 29) + 2];
static mut STDIN_WAS_READ: bool = false;
static mut STRSTART: u32 = 0;
static mut SUBBITBUF: u32 = 0;
static mut SYNCHRONOUS: bool = false;
static mut TEST: i32 = 0;
static mut TIME_STAMP: std::time::SystemTime = std::time::SystemTime::UNIX_EPOCH;
static mut TOTAL_IN: std::os::raw::c_long = 0;
static mut TOTAL_OUT: std::os::raw::c_long = 0;
static mut TO_STDOUT: i32 = 0;
static mut UNZIP_CRC: Ulg = 0;
static mut UTIMENSAT_WORKS_REALLY: i32 = 0;
static mut VALID: i32 = 0;
static mut VERBOSE: i32 = 0;
static VERSION: &str = "1.12";
static mut WINDOW: [Uch; 2 * 0x8000] = [0; 2 * 0x8000];
static mut WINDOW_SIZE: Ulg = 2 * 0x8000;
static mut WORK: fn(i32, i32) -> i32 = std::iter::zip;
static mut ZFILE: FileT = 0;
static mut Z_LEN: usize = 0;
const Z_SUFFIX: *const i8 = std::ptr::null();
fn __printf__(format: *const std::os::raw::c_char, ) -> std::os::raw::c_int {
    let retval: std::os::raw::c_int;
    let mut args: *mut std::os::raw::c_void = std::ptr::null_mut();
    unsafe {
        let mut ap: usize = std::mem::zeroed();
        __builtin_va_start(&mut ap, format);
        args = ap.as_va_list();
        retval = rpl_vfprintf(std::ptr::null_mut(), format, args);
        __builtin_va_end(&mut ap);
    }
    retval
}
fn __xpg_strerror_r(errnum: i32, buf: &mut [u8], buflen: usize) -> Result<(), ()>{todo!("proto")}
fn _gl_alloc_nomem() -> *const std::ffi::c_void {
    unsafe {
        *libc::__errno_location() = 12;
    }
    std::ptr::null()
}
fn _gl_utimens_windows(filename: &str, ts: [libc::timespec; 2]) -> Result<(), ()>{todo!("proto")}
fn abort_gzip() -> ! {
    match remove_output_file(false) {
        Ok(_) => do_exit(1),
        Err(_) => panic!("Failed to remove output file"),
    }
}
fn abort_gzip_signal(sig: i32) -> Result<(), ()> {
    remove_output_file(true)?;

    if sig == unsafe { crate::EXITING_SIGNAL } {
        std::process::exit(2);
    }

    unsafe {
        libc::signal(sig, std::mem::transmute(0 as usize));
    }

    unsafe {
        libc::raise(sig);
    }

    Ok(())
}
fn add_envopt(argcp: &mut i32, argvp: &mut *mut *mut std::os::raw::c_char, envvar_name: &str) -> *mut std::os::raw::c_char {
    let env_val = match std::env::var(envvar_name) {
        Ok(val) => val,
        Err(_) => return std::ptr::null_mut(),
    };

    let env_val_c = String::new(env_val).expect("CString::new failed");
    let p = env_val_c.as_ptr();

    let mut nargc = 0;
    let mut p_iter = p;
    while *p_iter != 0 {
        p_iter = p_iter.add(1);
        nargc += 1;
    }

    if nargc == 0 {
        return std::ptr::null_mut();
    }

    *argcp = nargc + 1;
    let nargv = unsafe {
        std::alloc::alloc_array::<*mut std::os::raw::c_char>(*argcp as usize).cast()
    };
    let oargv = *argvp;
    *argvp = nargv;
    *nargv = *oargv;
    let mut nargv_iter = nargv.add(1);

    let mut p_iter = p;
    while nargc > 0 {
        p_iter = p_iter.add(1);
        *nargv_iter = p_iter;
        nargv_iter = nargv_iter.add(1);
        while *p_iter != 0 {
            p_iter = p_iter.add(1);
        }
        nargc -= 1;
    }

    *nargv_iter = std::ptr::null_mut();
    p.cast()
}
fn aligned_alloc(alignment: usize, size: usize) -> *mut std::ffi::c_void{todo!("proto")}
fn asnprintf<'a>(resultbuf: &'a mut str, lengthp: &'a mut usize, format: &'a str) -> Result<&'a str, ()> {
    let mut args: Arguments = Arguments {
        count: 0,
        arg: std::ptr::null(),
        direct_alloc_arg: [Argument {
            // Initialize your Argument struct fields here
        }; 7],
    };

    let mut arg_list: Vec<Argument> = Vec::new();

    let mut ap: *mut libc::c_void = std::ptr::null_mut();
    unsafe {
        // Placeholder implementation for __builtin_va_start
        // __builtin_va_start(&mut ap, format);

        let mut fmt_str = format;
        while !fmt_str.is_empty() {
            let (directive, remaining) = parse_directive(fmt_str);
            let arg = extract_argument(&mut ap, directive);
            arg_list.push(arg);
            fmt_str = remaining;
        }
        // Placeholder implementation for __builtin_va_end
        // __builtin_va_end(&mut ap);
    }

    args.count = arg_list.len();
    args.arg = arg_list.as_ptr();

    vasnprintf(Some(resultbuf), lengthp, format, args.count)
}
fn at_func2(fd1: i32, file1: &str, fd2: i32, file2: &str, func: impl Fn(&str, &str) -> i32) -> i32{todo!("proto")}
fn atdir_eq(dir: &str, dirlen: usize, dfname: &[u8]) -> bool {
    let mut dir_slice = dir.as_bytes();
    if dirlen == 0 {
        dir_slice = &[b'.' as u8];
    }
    dfname.iter().zip(dir_slice.iter()).all(|(&a, &b)| a == b) && dfname.get(dirlen).copied() == Some(0)
}
fn atdir_set(dir: &str, dirlen: usize) -> i32 {
    const TRY_OPENING_DIRECTORIES: bool = true;
    if TRY_OPENING_DIRECTORIES && !atdir_eq(dir, dirlen) {
        if DFD >= 0 {
            libc::close(DFD);
        }
        let mut dir_to_use = dir;
        let mut len_to_use = dirlen;
        if dirlen == 0 {
            dir_to_use = &DOT.to_string();
            len_to_use = 1;
        }
        DFNAME[..len_to_use].copy_from_slice(dir_to_use.as_bytes());
        DFNAME[len_to_use] = DOT as u8;
        DFD = open_safer(std::str::from_utf8(&DFNAME[..len_to_use + 1]).unwrap(), 0 | 0o200000);
    }
    DFD
}
fn base_len(name: &str) -> usize {
    let mut len: usize;
    let prefix_len = { let _ = name; 0 };
    len = name.len();
    while len > 1 && name.chars().nth(len - 1) == Some('/') {
        len -= 1;
    }
    if false && len == 1 && name.chars().nth(0) == Some('/') && name.chars().nth(1) == Some('/') && name.chars().nth(2) == None {
        return 2;
    }
    if false && prefix_len != 0 && len == prefix_len && name.chars().nth(prefix_len) == Some('/') {
        return prefix_len + 1;
    }
    len
}
fn bi_init(zipfile: FileT) -> Option<&'static mut usize> {
    unsafe {
        ZFILE = zipfile;
        BI_BUF = 0;
        BI_VALID = 0;
        if ZFILE != -1 {
            READ_BUF = Some(file_read);
        }
    }
    READ_BUF.as_mut()
}
fn bi_reverse(code: u32, len: i32) -> u32 {
    let mut res: u32 = 0;
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
    if unsafe { BI_VALID } > 8 {
        if unsafe { OUTCNT } < 0x40000 - 2 {
            unsafe {
                OUTBUF[OUTCNT as usize] = (BI_BUF & 0xff) as Uch;
                OUTCNT += 1;
                OUTBUF[OUTCNT as usize] = ((BI_BUF >> 8) & 0xff) as Uch;
                OUTCNT += 1;
            }
        } else {
            unsafe {
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
        }
    } else if unsafe { BI_VALID } > 0 {
        unsafe {
            OUTBUF[OUTCNT as usize] = BI_BUF as Uch;
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf()?;
            }
        }
    }
    unsafe {
        BI_BUF = 0;
        BI_VALID = 0;
    }
    Ok(())
}
fn build_bl_tree() -> Result<i32, ()> {
    let mut max_blindex: usize = 0;
    scan_tree(&mut BL_TREE, L_DESC.max_code);
    scan_tree(&mut DYN_DTREE, D_DESC.max_code);
    build_tree(&mut BL_DESC);
    
    for index in (3..=19).rev() {
        if BL_TREE[BL_ORDER[index]].dl.len != 0 {
            max_blindex = index;
            break;
        }
    }
    
    let opt_len_increment: u64 = 3 * (max_blindex as u64 + 1) + 5 + 5 + 4;
    OPT_LEN += opt_len_increment;
    
    Ok(max_blindex as i32)
}
fn build_tree(desc: &mut TreeDesc) -> Result<(), ()> {
    let tree = &mut desc.dyn_tree;
    let stree = desc.static_tree.as_ref();
    let elems = desc.elems;
    let mut max_code = -1;
    let mut node = elems;

    unsafe {
        HEAP_LEN = 0;
        HEAP_MAX = 2 * (256 + 1 + 29) + 1;
    }

    for n in 0..elems {
        if tree[n].fc.freq != 0 {
            unsafe {
                HEAP_LEN += 1;
                HEAP[HEAP_LEN as usize] = n;
                max_code = n as i32;
                DEPTH[n as usize] = 0;
            }
        } else {
            tree[n].dl.len = 0;
        }
    }

    while unsafe { HEAP_LEN } < 2 {
        let new = unsafe {
            HEAP_LEN += 1;
            if max_code < 2 {
                max_code += 1;
            } else {
                max_code = 0;
            }
            HEAP[HEAP_LEN as usize] = max_code;
            max_code
        };
        tree[new as usize].fc.freq = 1;
        DEPTH[new as usize] = 0;
        unsafe {
            OPT_LEN -= 1;
            if let Some(stree) = stree {
                STATIC_LEN -= stree[new as usize].dl.len;
            }
        }
    }

    desc.max_code = max_code;

    for n in ((unsafe { HEAP_LEN } / 2)..=1).rev() {
        pqdownheap(tree, n as i32);
    }

    loop {
        let n;
        let m;
        unsafe {
            n = HEAP[1];
            HEAP[1] = HEAP[HEAP_LEN as usize];
            HEAP_LEN -= 1;
            pqdownheap(tree, 1);
            m = HEAP[1];
            HEAP[HEAP_MAX as usize] = n;
            HEAP_MAX -= 1;
            HEAP[HEAP_MAX as usize] = m;
            tree[node as usize].fc.freq = tree[n as usize].fc.freq + tree[m as usize].fc.freq;
            DEPTH[node as usize] = if DEPTH[n as usize] >= DEPTH[m as usize] {
                DEPTH[n as usize]
            } else {
                DEPTH[m as usize]
            } + 1;
            tree[n as usize].dl.dad = node as u16;
            tree[m as usize].dl.dad = node as u16;
            HEAP[1] = node;
            node += 1;
            pqdownheap(tree, 1);
        }

        if unsafe { HEAP_LEN } < 2 {
            break;
        }
    }

    unsafe {
        HEAP[HEAP_MAX as usize] = HEAP[1];
    }

    gen_bitlen(desc);
    gen_codes(tree, max_code)?;

    Ok(())
}
fn calloc(nmemb: usize, size: usize) -> *mut std::ffi::c_void{todo!("proto")}
fn canonicalize_file_name(name: &str) -> Option<String>{todo!("proto")}
fn cdb_advance_fd(cdb: &mut CdBuf, dir: &str) -> Result<(), ()> {
    use std::os::fd::AsRawFd;
    use std::os::unix::fs::OpenOptionsExt;
    use std::fs::OpenOptions;

    let new_path = std::path::Path::new(dir);

    let new_fd = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW)
        .open(new_path);

    match new_fd {
        Ok(fd) => {
            cdb_free(cdb);
            cdb.fd = fd.as_raw_fd();
            Ok(())
        }
        Err(_) => Err(()),
    }
}
fn cdb_fchdir(cdb: &CdBuf) -> Result<(), ()> {
    if unsafe { libc::fchdir(cdb.fd) } == -1 {
        Err(())
    } else {
        Ok(())
    }
}
fn cdb_free(cdb: &mut CdBuf) {
    if cdb.fd >= 0 {
        unsafe {
            let close_fail = libc::close(cdb.fd);
            assert_eq!(close_fail, 0, "Failed to close file descriptor");
        }
    }
}
fn cdb_init(cdb: &mut CdBuf) {
    cdb.fd = -100;
}
fn chdir_long(mut dir: &str) -> i32 {
    let e = unsafe { libc::chdir(dir.as_ptr() as *const char) };
    if e == 0 || unsafe { *libc::__errno_location() } != 36 {
        return e;
    }

    let len = dir.len();
    let dir_end = dir.as_ptr().wrapping_add(len);
    let mut cdb = CdBuf { fd: -1 };
    let n_leading_slash = dir.as_bytes().iter().take_while(|&&c| c == b'/').count();

    if n_leading_slash == 2 {
        let mut err = 0;
        let slash = dir[n_leading_slash + 1..].as_ptr().wrapping_add(3).offset(3);
        if let Some(slash_idx) = slash {
            let slash = dir.as_ptr().wrapping_add(n_leading_slash + 3 + slash_idx);
            unsafe { *slash = b'\0' };
            err = cdb_advance_fd(&mut cdb, &dir[..slash_idx]);
            unsafe { *slash = b'/' };
            if err != 0 {
                goto_fail(&mut cdb);
            }
            dir = find_non_slash(&dir[slash_idx + 1..]).unwrap_or("");
        } else {
            unsafe { *libc::__errno_location() = 36 };
            return -1;
        }
    } else if n_leading_slash > 0 {
        if cdb_advance_fd(&mut cdb, "/").is_err() {
            goto_fail(&mut cdb);
        }
        dir = &dir[n_leading_slash..];
    }

    assert!(!dir.starts_with('/'));
    assert!(dir.as_ptr() <= dir_end);

    while dir_end.offset_from(dir.as_ptr()) as usize >= 4096 {
        let mut err = 0;
        let slash = dir.as_bytes().iter().rev().take(4096).position(|&c| c == b'/');
        if let Some(slash_idx) = slash {
            let slash = dir.as_ptr().wrapping_add(slash_idx);
            unsafe { *slash = b'\0' };
            err = cdb_advance_fd(&mut cdb, &dir[..slash_idx]);
            unsafe { *slash = b'/' };
            if err != 0 {
                goto_fail(&mut cdb);
            }
            dir = find_non_slash(&dir[slash_idx + 1..]).unwrap_or("");
        } else {
            unsafe { *libc::__errno_location() = 36 };
            return -1;
        }
    }

    if dir < dir_end {
        if cdb_advance_fd(&mut cdb, dir).is_err() {
            goto_fail(&mut cdb);
        }
    }

    if cdb_fchdir(&cdb).is_err() {
        goto_fail(&mut cdb);
    }

    cdb_free(&mut cdb);
    return 0;

    fn goto_fail(cdb: &mut CdBuf) -> i32 {
        let saved_errno = unsafe { *libc::__errno_location() };
        cdb_free(cdb);
        unsafe { *libc::__errno_location() = saved_errno };
        -1
    }
}
fn check_ofname() -> Result<(), ()> {
    if unsafe { FORCE } == 0 {
        let mut ok = 0;
        rpl_fprintf(&mut std::io::stderr(), "{}: {} already exists;", unsafe { std::ffi::CStr::from_ptr(PROGRAM_NAME).to_str().unwrap() }, unsafe { std::str::from_utf8(&OFNAME).unwrap() });
        if unsafe { FOREGROUND } != 0 && (unsafe { PRESUME_INPUT_TTY } || unsafe { libc::isatty(0) != 0 }) {
            rpl_fprintf(&mut std::io::stderr(), " do you wish to overwrite (y or n)? ");
            rpl_fflush(&mut std::io::stderr());
            ok = yesno() as i32;
        }
        if ok == 0 {
            rpl_fprintf(&mut std::io::stderr(), "\tnot overwritten\n");
            if unsafe { EXIT_CODE } == 0 {
                unsafe { EXIT_CODE = 2 };
            }
            return Err(());
        }
    }
    if let Err(_) = xunlink(unsafe { std::str::from_utf8(&OFNAME).unwrap() }) {
        progerror(unsafe { std::str::from_utf8(&OFNAME).unwrap() });
        return Err(());
    }
    Ok(())
}
fn check_zipfile(in_data: i32) -> Result<(), ()> {
    let h = unsafe { INBUF.as_ptr().offset(INPTR as isize) };
    IFD = in_data;
    INPTR += 30 + ((unsafe { *h.offset(26) } as u16 | (unsafe { *h.offset(27) } as u16) << 8) as u32)
        + ((unsafe { *h.offset(28) } as u16 | (unsafe { *h.offset(29) } as u16) << 8) as u32);

    if INPTR > INSIZE
        || ((unsafe { *h } as u32 | (unsafe { *h.offset(1) } as u32) << 8) as u64
            | ((unsafe { *h.offset(2) } as u64 | (unsafe { *h.offset(3) } as u64) << 8) << 16))
            != 0x04034b50 {
        let program_name = unsafe { std::ffi::CStr::from_ptr(PROGRAM_NAME).to_str().unwrap() };
        let ifname = unsafe { std::str::from_utf8_unchecked(&IFNAME) };
        eprintln!("\n{}: {}: not a valid zip file\n", program_name, ifname);
        EXIT_CODE = 1;
        return Err(());
    }

    METHOD = unsafe { (*h.offset(8)).into() };
    if METHOD != 0 && METHOD != 8 {
        let program_name = unsafe { std::ffi::CStr::from_ptr(PROGRAM_NAME).to_str().unwrap() };
        let ifname = unsafe { std::str::from_utf8_unchecked(&IFNAME) };
        eprintln!("\n{}: {}: first entry not deflated or stored -- use unzip\n", program_name, ifname);
        EXIT_CODE = 1;
        return Err(());
    }

    let decrypt = unsafe { *h.offset(6) } & 1;
    if decrypt != 0 {
        let program_name = unsafe { std::ffi::CStr::from_ptr(PROGRAM_NAME).to_str().unwrap() };
        let ifname = unsafe { std::str::from_utf8_unchecked(&IFNAME) };
        eprintln!("\n{}: {}: encrypted file -- use unzip\n", program_name, ifname);
        EXIT_CODE = 1;
        return Err(());
    }

    EXT_HEADER = (unsafe { *h.offset(6) } & 8) != 0;
    PKZIP = 1;
    Ok(())
}
fn clear_bufs() {
    unsafe {
        OUTCNT = 0;
        INSIZE = 0;
        INPTR = 0;
        BYTES_IN = 0;
        BYTES_OUT = 0;
    }
}
fn clear_ungetc_buffer_preserving_position(fp: &mut std::fs::File) -> Result<(), ()> {
    use std::io::Seek;
    if fp.metadata().map(|meta| meta.permissions().readonly()).unwrap_or(false) {
        fp.seek(std::io::SeekFrom::Start(0)).map_err(|_| ())?;
    }
    Ok(())
}
fn compress_block(ltree: *const CtData, dtree: *const CtData) {
    let mut dist: u16;
    let mut lc: i32;
    let mut lx = 0;
    let mut dx = 0;
    let mut fx = 0;
    let mut flag: Uch = 0;
    let mut code: u16;
    let mut extra: i32;

    if unsafe { LAST_LIT } != 0 {
        loop {
            if (lx & 7) == 0 {
                flag = unsafe { FLAG_BUF[fx] };
                fx += 1;
            }
            lc = unsafe { INBUF[lx as usize] } as i32;
            lx += 1;

            if (flag & 1) == 0 {
                let ltree_entry = unsafe { &*ltree.offset(lc as isize) };
                send_bits(ltree_entry.fc.freq as i32, ltree_entry.dl.len as i32).unwrap();
            } else {
                code = unsafe { LENGTH_CODE[lc as usize] as u16 };
                let ltree_entry = unsafe { &*ltree.offset((code + 256 + 1) as isize) };
                send_bits(ltree_entry.fc.freq as i32, ltree_entry.dl.len as i32).unwrap();

                extra = unsafe { EXTRA_LBITS[code as usize] };
                if extra != 0 {
                    lc -= unsafe { BASE_LENGTH[code as usize] };
                    send_bits(lc, extra).unwrap();
                }

                dist = unsafe { D_BUF[dx as usize] };
                dx += 1;
                code = if dist < 256 {
                    unsafe { DIST_CODE[dist as usize].into() }
                } else {
                    unsafe { DIST_CODE[256 + (dist >> 7) as usize].into() }
                };
                let dtree_entry = unsafe { &*dtree.offset(code as isize) };
                send_bits(dtree_entry.fc.freq as i32, dtree_entry.dl.len as i32).unwrap();

                extra = unsafe { EXTRA_DBITS[code as usize] };
                if extra != 0 {
                    dist -= unsafe { BASE_DIST[code as usize] };
                    send_bits(dist as i32, extra).unwrap();
                }
            }
            flag >>= 1;

            if lx >= unsafe { LAST_LIT } {
                break;
            }
        }
    }
    let ltree_entry = unsafe { &*ltree.offset(256) };
    send_bits(ltree_entry.fc.freq as i32, ltree_entry.dl.len as i32).unwrap();
}
fn copy(in_data: i32, out: i32) -> Result<(), ()> {
    unsafe {
        // Define __errno_location function
        extern "C" {
            fn __errno_location() -> *mut i32;
        }

        *__errno_location() = 0;

        while crate::INSIZE > crate::INPTR {
            let bytes_to_write = (crate::INSIZE - crate::INPTR) as u32;
            write_buf(out, crate::INBUF.as_ptr().add(crate::INPTR as usize) as *mut std::ffi::c_void, bytes_to_write)?;
            let got = read_buffer(in_data, crate::INBUF.as_mut_ptr() as *mut std::ffi::c_void, 0x40000);
            if got == -1 {
                read_error();
            }
            crate::BYTES_IN += got as i64; // Cast got to i64 to match BYTES_IN type
            crate::INSIZE = got as u32;
            crate::INPTR = 0;
        }
    }
    Ok(())
}
fn copy_block(buf: *const Uch, len: usize, header: bool) -> Result<(), ()> {
    bi_windup()?;
    if header {
        if OUTCNT < 0x40000 - 2 {
            OUTBUF[OUTCNT] = (len as Ush & 0xff) as Uch;
            OUTCNT += 1;
            OUTBUF[OUTCNT] = ((len as Ush) >> 8) as Uch;
            OUTCNT += 1;
        } else {
            OUTBUF[OUTCNT] = ((len as Ush & 0xff) as Uch);
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf()?;
            }
            OUTBUF[OUTCNT] = (((len as Ush) >> 8) as Uch);
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf()?;
            }
        }

        if OUTCNT < 0x40000 - 2 {
            OUTBUF[OUTCNT] = (!len as Ush & 0xff) as Uch;
            OUTCNT += 1;
            OUTBUF[OUTCNT] = ((!len as Ush) >> 8) as Uch;
            OUTCNT += 1;
        } else {
            OUTBUF[OUTCNT] = ((!len as Ush & 0xff) as Uch);
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf()?;
            }
            OUTBUF[OUTCNT] = (((!len as Ush) >> 8) as Uch);
            OUTCNT += 1;
            if OUTCNT == 0x40000 {
                flush_outbuf()?;
            }
        }
    }

    let mut index = 0;
    while index < len {
        OUTBUF[OUTCNT] = unsafe { *buf.add(index) };
        OUTCNT += 1;
        if OUTCNT == 0x40000 {
            flush_outbuf()?;
        }
        index += 1;
    }

    Ok(())
}
fn copy_stat(ifstat: &libc::stat) -> Result<(), ()> {
    let mode = ifstat.st_mode & ((0o400 | 0o200 | 0o100) | ((0o400 | 0o200 | 0o100) >> 3) | (((0o400 | 0o200 | 0o100) >> 3) >> 3));
    let mut r;
    let mut restoring;
    let mut timespec = [get_stat_atime(ifstat), get_stat_mtime(ifstat)];
    restoring = unsafe { DECOMPRESS != 0 && TIME_STAMP.tv_nsec >= 0 && !(timespec[1].tv_sec == TIME_STAMP.tv_sec && timespec[1].tv_nsec == TIME_STAMP.tv_nsec) };
    if restoring {
        timespec[1] = unsafe { TIME_STAMP };
    }
    if fdutimens(unsafe { OFD }, unsafe { std::str::from_utf8_unchecked(&OFNAME) }, Some(timespec)) == 0 {
        if restoring && unsafe { VERBOSE } > 1 {
            rpl_fprintf(&mut std::io::stderr(), "{}: timestamp restored\n", unsafe { std::str::from_utf8_unchecked(&OFNAME) }).unwrap();
        }
    } else {
        let e = unsafe { *libc::__errno_location() };
        if !unsafe { QUIET } {
            rpl_fprintf(&mut std::io::stderr(), "{}: ", unsafe { std::ffi::CStr::from_ptr(PROGRAM_NAME).to_str().unwrap() }).unwrap();
            if unsafe { EXIT_CODE } == 0 {
                unsafe { EXIT_CODE = 2 };
            }
        }
        if !unsafe { QUIET } {
            unsafe { *libc::__errno_location() = e };
            libc::perror(unsafe { std::str::from_utf8_unchecked(&OFNAME) });
        }
    }
    do_chown(unsafe { OFD }, unsafe { std::str::from_utf8_unchecked(&OFNAME) }, u32::MAX, ifstat.st_gid);
    r = unsafe { libc::fchmod(unsafe { OFD }, mode) };
    if r != 0 {
        let e = unsafe { *libc::__errno_location() };
        if !unsafe { QUIET } {
            rpl_fprintf(&mut std::io::stderr(), "{}: ", unsafe { std::ffi::CStr::from_ptr(PROGRAM_NAME).to_str().unwrap() }).unwrap();
            if unsafe { EXIT_CODE } == 0 {
                unsafe { EXIT_CODE = 2 };
            }
        }
        if !unsafe { QUIET } {
            unsafe { *libc::__errno_location() = e };
            libc::perror(unsafe { std::str::from_utf8_unchecked(&OFNAME) });
        }
    }
    do_chown(unsafe { OFD }, unsafe { std::str::from_utf8_unchecked(&OFNAME) }, ifstat.st_uid, u32::MAX);
    Ok(())
}
fn creat_safer(file: &str, mode: std::os::unix::raw::mode_t) -> Result<i32, ()> {
    use std::os::fd::AsRawFd;
    let fd = std::fs::File::create(file).map_err(|_| ())?;
    fd_safer(fd.as_raw_fd())
}
fn create_outfile() -> i32 {
    let mut name_shortened = 0;
    let flags = (0o1 | 0o100 | 0o200 | if ASCII != 0 && DECOMPRESS != 0 { 0 } else { 0 });
    let base = unsafe { std::ffi::CStr::from_ptr(OFNAME.as_ptr() as *const i8).to_str().unwrap() };
    let mut atfd = -100;
    if KEEP == 0 {
        if let Some(b) = last_component(base) {
            if let Ok(f) = atdir_set(base, b.as_ptr() as usize - OFNAME.as_ptr() as usize) {
                base = b;
                atfd = f;
            }
        }
    }
    loop {
        let mut open_errno;
        let mut oldset = std::mem::MaybeUninit::<libc::sigset_t>::uninit();
        unsafe {
            volatile_strcpy(REMOVE_OFNAME.as_mut_ptr() as *mut i8, OFNAME.as_ptr() as *const i8);
            libc::sigprocmask(0, &CAUGHT_SIGNALS as *const libc::sigset_t, oldset.as_mut_ptr() as *mut libc::sigset_t);
            match openat_safer(atfd, base, flags, Some(0o400 | 0o200)) {
                Ok(fd) => {
                    REMOVE_OFNAME_FD = fd;
                    OFD = fd;
                }
                Err(_) => {
                    open_errno = *libc::__errno_location();
                    libc::sigprocmask(2, oldset.as_mut_ptr() as *mut libc::sigset_t, std::ptr::null_mut());
                    match open_errno {
                        36 => {
                            shorten_name(&mut std::ffi::CString::new(OFNAME).unwrap());
                            name_shortened = 1;
                        }
                        17 => {
                            if check_ofname().is_err() {
                                libc::close(IFD);
                                return 1;
                            }
                        }
                        _ => {
                            progerror(&std::ffi::CString::new(OFNAME).unwrap().to_string_lossy());
                            libc::close(IFD);
                            return 1;
                        }
                    }
                    continue;
                }
            };
        }
        if OFD >= 0 {
            break;
        }
    }
    if name_shortened != 0 && DECOMPRESS != 0 {
        if QUIET == 0 {
            if rpl_fprintf(&mut std::io::stderr(), "{}: {}: warning, name truncated\n", PROGRAM_NAME, OFNAME).is_ok() {
                if EXIT_CODE == 0 {
                    EXIT_CODE = 2;
                }
            }
        }
    }
    0
}
fn ct_init(attr: *mut Ush, methodp: *mut i32) -> Result<(), ()> {
    let mut n: i32;
    let mut bits: i32;
    let mut length: i32;
    let mut code: i32;
    let mut dist: i32;
    unsafe {
        FILE_TYPE = attr;
        FILE_METHOD = methodp;
        COMPRESSED_LEN = 0;
        INPUT_LEN = 0;
        if STATIC_DTREE[0].dl.len != 0 {
            return Ok(());
        }
    }
    length = 0;
    for code in 0..29 - 1 {
        BASE_LENGTH[code as usize] = length;
        for n in 0..(1 << EXTRA_LBITS[code as usize]) {
            unsafe {
                LENGTH_CODE[length as usize] = code as Uch;
            }
            length += 1;
        }
    }
    LENGTH_CODE[(length - 1) as usize] = code as Uch;
    dist = 0;
    for code in 0..16 {
        BASE_DIST[code as usize] = dist;
        for n in 0..(1 << EXTRA_DBITS[code as usize]) {
            unsafe {
                DIST_CODE[dist as usize] = code as Uch;
            }
            dist += 1;
        }
    }
    dist >>= 7;
    for code in 16..30 {
        BASE_DIST[code as usize] = dist << 7;
        for n in 0..(1 << (EXTRA_DBITS[code as usize] - 7)) {
            unsafe {
                DIST_CODE[(256 + dist) as usize] = code as Uch;
            }
            dist += 1;
        }
    }
    for bits in 0..=15 {
        unsafe {
            BL_COUNT[bits as usize] = 0;
        }
    }
    n = 0;
    while n <= 143 {
        unsafe {
            STATIC_LTREE[n as usize].dl.len = 8;
            BL_COUNT[8] += 1;
        }
        n += 1;
    }
    while n <= 255 {
        unsafe {
            STATIC_LTREE[n as usize].dl.len = 9;
            BL_COUNT[9] += 1;
        }
        n += 1;
    }
    while n <= 279 {
        unsafe {
            STATIC_LTREE[n as usize].dl.len = 7;
            BL_COUNT[7] += 1;
        }
        n += 1;
    }
    while n <= 287 {
        unsafe {
            STATIC_LTREE[n as usize].dl.len = 8;
            BL_COUNT[8] += 1;
        }
        n += 1;
    }
    gen_codes(&mut STATIC_LTREE, (256 + 1 + 29) + 1).map_err(|_| ())?;
    for n in 0..30 {
        unsafe {
            STATIC_DTREE[n as usize].dl.len = 5;
            STATIC_DTREE[n as usize].fc.code = bi_reverse(n as u32, 5);
        }
    }
    init_block();
    Ok(())
}
fn ct_tally(dist: i32, lc: i32) -> i32 {
    unsafe {
        INBUF[LAST_LIT as usize] = lc as Uch;
        LAST_LIT += 1;

        if dist == 0 {
            DYN_LTREE[lc as usize].fc.freq += 1;
        } else {
            let mut dist = dist - 1;
            DYN_LTREE[LENGTH_CODE[lc as usize] + 256 + 1].fc.freq += 1;
            DYN_DTREE[if dist < 256 { DIST_CODE[dist as usize] } else { DIST_CODE[256 + (dist >> 7) as usize] }].fc.freq += 1;
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
            let out_length = (LAST_LIT as Ulg) * 8;
            let in_length = (STRSTART as Ulg - BLOCK_START as Ulg);
            let mut dcode = 0;
            while dcode < 30 {
                out_length += (DYN_DTREE[dcode].fc.freq as Ulg) * (5 + EXTRA_DBITS[dcode] as Ulg);
                dcode += 1;
            }
            out_length >>= 3;

            if LAST_DIST < LAST_LIT / 2 && out_length < in_length / 2 {
                return 1;
            }
        }

        (LAST_LIT == 0x8000 - 1 || LAST_DIST == 0x8000) as i32
    }
}
fn current_timespec() -> Result<libc::timespec, ()> {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    gettime(&mut ts);
    Ok(ts)
}
fn dcnpgettext_expr(domain: &str, msgctxt: &str, msgid: &str, msgid_plural: &str, n: u64, category: i32) -> Option<&'static str> {
    let msgctxt_len = msgctxt.len() + 1;
    let msgid_len = msgid.len() + 1;
    let mut buf = [0; 1024];
    let msg_ctxt_id: &mut [u8];

    let msg_ctxt_id_ptr: *mut u8;

    if msgctxt_len + msgid_len <= buf.len() {
        msg_ctxt_id = &mut buf;
        msg_ctxt_id_ptr = msg_ctxt_id.as_mut_ptr();
    } else {
        let malloc_size = msgctxt_len + msgid_len;
        msg_ctxt_id_ptr = unsafe { libc::malloc(malloc_size) as *mut u8 };
        if msg_ctxt_id_ptr.is_null() {
            return None;
        }
        msg_ctxt_id = unsafe { std::slice::from_raw_parts_mut(msg_ctxt_id_ptr, malloc_size) };
    }

    msg_ctxt_id[..msgctxt_len - 1].copy_from_slice(msgctxt.as_bytes());
    msg_ctxt_id[msgctxt_len - 1] = 4;

    let translation: &'static str = if n == 1 {
        msgid_plural
    } else {
        let c_str = unsafe { String::from_raw_parts(msg_ctxt_id_ptr) };
        c_str.as_str().expect("Failed to convert CString").into()
    };

    let found_translation = !(translation.as_ptr() == msg_ctxt_id_ptr || translation.as_ptr() == msgid_plural.as_ptr());

    if msg_ctxt_id_ptr != buf.as_mut_ptr() {
        unsafe { libc::free(msg_ctxt_id_ptr as *mut libc::c_void) };
    }

    if found_translation {
        Some(unsafe { std::str::from_utf8(std::slice::from_raw_parts(translation.as_ptr(), msgctxt_len + msgid_len - 2)).unwrap() })
    } else {
        None
    }
}
fn dcpgettext_expr<'a>(domain: &'a str, msgctxt: &'a str, msgid: &'a str, category: i32) -> Option<&'a str> {
    let msgctxt_len = msgctxt.len() + 1;
    let msgid_len = msgid.len() + 1;
    let translation: &'a str;
    let mut buf = [0; 1024];
    let msg_ctxt_id: &'a mut [u8] = if msgctxt_len + msgid_len <= buf.len() {
        &mut buf
    } else {
        let ptr = malloc(msgctxt_len + msgid_len) as *mut u8;
        if ptr.is_null() {
            return None;
        }
        unsafe { std::slice::from_raw_parts_mut(ptr, msgctxt_len + msgid_len) }
    };

    let found_translation: bool;
    msg_ctxt_id[..msgctxt_len - 1].copy_from_slice(msgctxt.as_bytes());
    msg_ctxt_id[msgctxt_len - 1] = b"\004";
    msg_ctxt_id[msgctxt_len..].copy_from_slice(msgid.as_bytes());
    translation = unsafe { std::mem::transmute(msg_ctxt_id) };
    found_translation = translation.as_ptr() != msg_ctxt_id.as_ptr();

    if msg_ctxt_id.as_ptr() != buf.as_ptr() {
        rpl_free(msg_ctxt_id.as_mut_ptr() as *mut std::ffi::c_void).ok()?;
    }

    if found_translation {
        Some(translation)
    } else {
        Some(msgid)
    }
}
fn decode(count: u32, buffer: &mut [Uch]) -> Result<u32, ()> {
    let mut r = 0;
    while unsafe { J -= 1; J } >= 0 {
        buffer[r as usize] = buffer[J as usize];
        J = (J + 1) & ((1 << 13) - 1);
        if r + 1 == count {
            return Ok(r + 1);
        }
        r += 1;
    }
    loop {
        let c = decode_c();
        if c == (255 + 256 + 2 - 3) {
            unsafe {
                DONE = 1;
            }
            return Ok(r);
        }
        if c <= 255 {
            buffer[r as usize] = c as Uch;
            if r + 1 == count {
                return Ok(r + 1);
            }
            r += 1;
        } else {
            unsafe {
                J = c as i32 - (255 + 1 - 3);
                J = (r as u64 - decode_p()? as u64 - 1) as u32 & ((1 << 13) - 1);
            }
            while unsafe { J -= 1; J } >= 0 {
                buffer[r as usize] = buffer[J as usize];
                J = (J + 1) & ((1 << 13) - 1);
                if r + 1 == count {
                    return Ok(r + 1);
                }
                r += 1;
            }
        }
    }
}
fn decode_c() -> u32 {
    let mut j: u32;
    let mut mask: u64; // Change mask type to u64 to match BITBUF type
    if unsafe { BLOCKSIZE } == 0 {
        unsafe {
            BLOCKSIZE = getbits(16).unwrap() as u32; // Cast the result of getbits(16) to u32
            if BLOCKSIZE == 0 {
                return 255 + 256 + 2 - 3;
            }
            read_pt_len(16 + 3, 5, 3, &mut [], &mut []).unwrap(); // Provide missing arguments
            read_c_len().unwrap();
            read_pt_len(13 + 1, 4, -1, &mut [], &mut []).unwrap(); // Provide missing arguments
        }
    }
    unsafe {
        BLOCKSIZE -= 1;
        j = D_BUF[(BITBUF >> (8 * 2 * std::mem::size_of::<u8>() as u64 - 12)) as usize] as u32;
        if j >= 255 + 256 + 2 - 3 {
            mask = 1 << (8 * 2 * std::mem::size_of::<u8>() as u64 - 1 - 12); // Change mask type to u64
            loop {
                if BITBUF & mask != 0 {
                    j = PREV[0x8000 + j as usize] as u32;
                } else {
                    j = PREV[j as usize] as u32;
                }
                mask >>= 1;
                if j < 255 + 256 + 2 - 3 {
                    break;
                }
            }
        }
    }
    fillbuf(OUTBUF[j as usize] as i32);
    j
}
fn decode_p() -> Result<Ulg, ()> {
    let j: Ush;
    let mask: Ulg;
    j = PT_TABLE[(BITBUF >> ((8 * 2 * core::mem::size_of::<Uch>()) - 8)) as usize];
    if j >= (13 + 1) {
        mask = (1 as Ulg) << ((8 * 2 * core::mem::size_of::<Uch>()) - 1 - 8);
        while j >= (13 + 1) {
            if BITBUF & mask != 0 {
                j = PREV[(j + 0x8000) as usize];
            } else {
                j = PREV[j as usize];
            }
            mask >>= 1;
        }
    }
    fillbuf(PT_LEN[j as usize] as i32);
    if j != 0 {
        let shift_amount = j - 1;
        let shifted_value = (1 as Ulg) << shift_amount;
        let bits = getbits(shift_amount as i32)?;
        j = shifted_value + bits as Ulg;
    }
    Ok(j as Ulg)
}
fn decode_start() {
    crate::huf_decode_start();
    unsafe {
        crate::J = 0;
        crate::DONE = 0;
    }
}
fn deflate(pack_level: i32) -> Result<Ulg, ()> {
    use std::convert::TryInto;
    let mut hash_head: IPos;
    let mut prev_match: IPos;
    let mut flush = 0;
    let mut match_available = 0;
    let mut match_length: u32 = 3 - 1;
    lm_init(pack_level);
    if pack_level <= 3 {
        return deflate_fast();
    }
    while unsafe { LOOKAHEAD } != 0 {
        unsafe {
            INS_H = ((INS_H << ((15 + 3 - 1) / 3)) ^ WINDOW[STRSTART as usize + 3 - 1]) & ((1 << 15) - 1);
            PREV[STRSTART as usize & (0x8000 - 1)] = hash_head = (PREV as *const _ as *const u16).add(0x8000)[INS_H as usize];
            (PREV as *mut u16).add(0x8000)[INS_H as usize] = STRSTART as u16;
        }
        unsafe {
            PREV_LENGTH = match_length as u32;
            prev_match = MATCH_START;
        }
        match_length = 3 - 1;
        if hash_head != 0 && unsafe { PREV_LENGTH } < MAX_LAZY_MATCH &&
            STRSTART - hash_head <= (0x8000 - (258 + 3 + 1)) &&
            u64::from(STRSTART) <= unsafe { WINDOW_SIZE } - (258 + 3 + 1) {
            match_length = longest_match(hash_head);
            if match_length > unsafe { LOOKAHEAD } {
                match_length = unsafe { LOOKAHEAD };
            }
            if match_length == 3 && STRSTART - MATCH_START > 4096 {
                match_length -= 1;
            }
        }
        if unsafe { PREV_LENGTH } >= 3 && match_length <= unsafe { PREV_LENGTH } {
            flush = ct_tally((STRSTART - 1 - prev_match) as i32, (PREV_LENGTH - 3) as i32);
            unsafe {
                LOOKAHEAD -= PREV_LENGTH - 1;
                PREV_LENGTH -= 2;
            }
            while unsafe { RSYNC } != 0 {
                rsync_roll(STRSTART, PREV_LENGTH + 1);
            }
            while unsafe { PREV_LENGTH } != 0 {
                unsafe {
                    STRSTART += 1;
                    INS_H = ((INS_H << ((15 + 3 - 1) / 3)) ^ WINDOW[STRSTART as usize + 3 - 1]) & ((1 << 15) - 1);
                    PREV[STRSTART as usize & (0x8000 - 1)] = hash_head = (PREV as *const _ as *const u16).add(0x8000)[INS_H as usize];
                    (PREV as *mut u16).add(0x8000)[INS_H as usize] = STRSTART as u16;
                }
                unsafe {
                    PREV_LENGTH -= 1;
                }
            }
            match_available = 0;
            match_length = 3 - 1;
            STRSTART += 1;
            if unsafe { RSYNC } != 0 && STRSTART > unsafe { RSYNC_CHUNK_END.try_into().unwrap() } {
                unsafe {
                    RSYNC_CHUNK_END = 0xFFFFFFFF;
                    flush = 2;
                }
            }
            if flush != 0 {
                flush_block(if unsafe { BLOCK_START } >= 0 {
                    &WINDOW[unsafe { BLOCK_START as usize }]
                } else {
                    std::ptr::null()
                }, (STRSTART - unsafe { BLOCK_START as usize }) as Ulg, flush - 1, 0)?;
                unsafe {
                    BLOCK_START = STRSTART as i64;
                }
            }
        } else if match_available != 0 {
            flush = ct_tally(0, unsafe { WINDOW[STRSTART as usize - 1].into() });
            if unsafe { RSYNC } != 0 && STRSTART > unsafe { RSYNC_CHUNK_END.try_into().unwrap() } {
                unsafe {
                    RSYNC_CHUNK_END = 0xFFFFFFFF;
                    flush = 2;
                }
            }
            if flush != 0 {
                flush_block(if unsafe { BLOCK_START } >= 0 {
                    &WINDOW[unsafe { BLOCK_START as usize }]
                } else {
                    std::ptr::null()
                }, (STRSTART - unsafe { BLOCK_START as usize }) as Ulg, flush - 1, 0)?;
                while unsafe { RSYNC } != 0 {
                    rsync_roll(STRSTART, 1);
                }
                STRSTART += 1;
                unsafe {
                    LOOKAHEAD -= 1;
                }
            }
        } else {
            if unsafe { RSYNC } != 0 && STRSTART > unsafe { RSYNC_CHUNK_END.try_into().unwrap() } {
                unsafe {
                    RSYNC_CHUNK_END = 0xFFFFFFFF;
                    flush = 2;
                    flush_block(if unsafe { BLOCK_START } >= 0 {
                        &WINDOW[unsafe { BLOCK_START as usize }]
                    } else {
                        std::ptr::null()
                    }, (STRSTART - unsafe { BLOCK_START as usize }) as Ulg, flush - 1, 0)?;
                    BLOCK_START = STRSTART as i64;
                }
            }
            match_available = 1;
            while unsafe { RSYNC } != 0 {
                rsync_roll(STRSTART, 1);
            }
            STRSTART += 1;
            unsafe {
                LOOKAHEAD -= 1;
            }
        }
        while unsafe { LOOKAHEAD } < (258 + 3 + 1) && !unsafe { EOFILE != 0 } {
            fill_window();
        }
    }
    if match_available != 0 {
        ct_tally(0, unsafe { WINDOW[STRSTART as usize - 1].into() });
    }
    flush_block(if unsafe { BLOCK_START } >= 0 {
        &WINDOW[unsafe { BLOCK_START as usize }]
    } else {
        std::ptr::null()
    }, (STRSTART - unsafe { BLOCK_START as usize }) as Ulg, flush - 1, 1)
}
fn deflate_fast() -> Result<Ulg, ()> {
    use std::convert::TryInto;
    let mut hash_head: u16;
    let mut flush = 0;
    let mut match_length = 0;
    unsafe {
        PREV_LENGTH = 3 - 1;
    }
    while unsafe { LOOKAHEAD } != 0 {
        unsafe {
            INS_H = ((INS_H << ((15 + 3 - 1) / 3)) ^ WINDOW[STRSTART as usize + 3 - 1] as u32) & ((1 << 15) - 1);
            let prev_index = STRSTART & (0x8000 - 1);
            hash_head = PREV[prev_index as usize];
            PREV[prev_index as usize] = hash_head;
        }
        if hash_head != 0 && STRSTART - hash_head <= (0x8000 - (258 + 3 + 1))
            && u64::from(STRSTART) <= unsafe { WINDOW_SIZE } - (258 + 3 + 1) {
            match_length = longest_match(hash_head.into());
            if match_length > unsafe { LOOKAHEAD } {
                match_length = unsafe { LOOKAHEAD };
            }
        }
        if match_length >= 3 {
            flush = ct_tally((STRSTART - unsafe { MATCH_START }).try_into().unwrap(), (match_length - 3).try_into().unwrap());
            unsafe {
                LOOKAHEAD -= match_length;
            }
            if unsafe { RSYNC != 0 } {
                rsync_roll(STRSTART, match_length);
            }
            if match_length <= unsafe { MAX_LAZY_MATCH } {
                match_length -= 1;
                while match_length != 0 {
                    unsafe {
                        STRSTART += 1;
                        INS_H = ((INS_H << ((15 + 3 - 1) / 3)) ^ WINDOW[STRSTART as usize + 3 - 1] as u32) & ((1 << 15) - 1);
                        let prev_index = STRSTART & (0x8000 - 1);
                        hash_head = PREV[prev_index as usize];
                        PREV[prev_index as usize] = hash_head;
                    }
                    match_length -= 1;
                }
                STRSTART += 1;
            } else {
                STRSTART += match_length;
                match_length = 0;
                INS_H = WINDOW[STRSTART as usize] as u32;
                INS_H = ((INS_H << ((15 + 3 - 1) / 3)) ^ WINDOW[STRSTART as usize + 1] as u32) & ((1 << 15) - 1);
            }
        } else {
            flush = ct_tally(0, unsafe { WINDOW[STRSTART as usize].into() });
            if unsafe { RSYNC != 0 } {
                rsync_roll(STRSTART, 1);
            }
            LOOKAHEAD -= 1;
            STRSTART += 1;
        }
        if unsafe { RSYNC != 0 } && STRSTART > unsafe { RSYNC_CHUNK_END.try_into().unwrap() as u32 } {
            RSYNC_CHUNK_END = 0xFFFFFFFF;
            flush = 2;
        }
        if flush != 0 {
            let block_start = if unsafe { BLOCK_START } >= 0 {
                &WINDOW[unsafe { BLOCK_START as usize }]
            } else {
                std::ptr::null()
            };
            let stored_len = (STRSTART as Ulg - unsafe { BLOCK_START } as Ulg);
            let pad = flush - 1;
            let eof = 0;
            BLOCK_START = STRSTART as i64;
            flush_block(block_start, stored_len, pad, eof)?;
        }
        while unsafe { LOOKAHEAD } < (258 + 3 + 1) && !unsafe { EOFILE != 0 } {
            fill_window();
        }
    }
    let block_start = if unsafe { BLOCK_START } >= 0 {
        &WINDOW[unsafe { BLOCK_START as usize }]
    } else {
        std::ptr::null()
    };
    let stored_len = (STRSTART as Ulg - unsafe { BLOCK_START } as Ulg);
    let pad = flush - 1;
    let eof = 1;
    flush_block(block_start, stored_len, pad, eof)
}
fn dir_len(file: &str) -> usize {
    let prefix_length = file.chars().take_while(|&c| c == '/').count();
    let length = last_component(file).unwrap_or(file).len();
    
    for length in (prefix_length..length).rev() {
        if file.chars().nth(length - 1) != Some('/') {
            break;
        }
    }
    
    length
}
fn direntry_cmp_name(a: &DirentryT, b: &DirentryT) -> std::cmp::Ordering {
    a.name.as_ref().unwrap_or(&String::new()).cmp(b.name.as_ref().unwrap_or(&String::new()))
}
fn discard_input_bytes(mut nbytes: usize, flags: u32) -> Result<(), ()> {
    while nbytes != 0 {
        let c = if unsafe { INPTR < INSIZE } {
            unsafe { INBUF[INPTR as usize] }
        } else {
            fill_inbuf(false) as Uch
        };
        if unsafe { FLAGS } & 0x02 != 0 {
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
fn display_ratio(num: i64, den: i64, file: &mut std::fs::File) {
    use std::io::Write;
    let ratio = if den == 0 {
        0.0
    } else {
        100.0 * num as f64 / den as f64
    };

    write!(file, "{:5.1}%", ratio).unwrap();
}
fn do_chown(fd: i32, name: &str, uid: std::os::unix::raw::uid_t, gid: std::os::unix::raw::gid_t) {
    let _ = std::mem::transmute::<_, ()>(std::os::unix::io::AsRawFd::as_raw_fd(&fd));
}
fn do_exit(exitcode: i32) -> ! {
    thread_local! {
        static IN_EXIT: std::cell::RefCell<bool> = std::cell::RefCell::new(false);
    }

    IN_EXIT.with(|in_exit| {
        if *in_exit.borrow() {
            std::process::exit(exitcode);
        }
        *in_exit.borrow_mut() = true;
    });

    rpl_free(unsafe { std::mem::transmute(ENV) });
    unsafe { ENV = std::ptr::null() };

    std::process::exit(exitcode);
}
fn do_list() {
    use std::convert::TryInto;
    let mut crc: u64 = 0;
    static mut FIRST_TIME: i32 = 1;
    static METHODS: [&str; 9] = [
        "store",
        "compr",
        "pack ",
        "lzh  ",
        "",
        "",
        "",
        "",
        "defla",
    ];

    let positive_off_t_width = (((std::mem::size_of::<i64>() * 8) * 146 + 484) / 485) - 1;

    if unsafe { FIRST_TIME } != 0 && METHOD >= 0 {
        unsafe {
            FIRST_TIME = 0;
        }
        if VERBOSE != 0 {
            print!("method  crc     date  time  ");
        }
        if QUIET == 0 {
            println!("{:width$} {:width$}  ratio uncompressed_name",
                     "compressed", "uncompressed", width = positive_off_t_width as usize);
        }
    } else if METHOD < 0 {
        if TOTAL_IN <= 0 || TOTAL_OUT <= 0 {
            return;
        }
        if VERBOSE != 0 {
            print!("                            ");
        }
        if VERBOSE != 0 || QUIET == 0 {
            fprint_off(&mut i8::stdout(), TOTAL_IN, positive_off_t_width.try_into().unwrap()).unwrap();
            print!(" ");
            fprint_off(&mut i8::stdout(), TOTAL_OUT, positive_off_t_width.try_into().unwrap()).unwrap();
            print!(" ");
        }
        display_ratio(TOTAL_OUT - (TOTAL_IN - HEADER_BYTES), TOTAL_OUT, &mut i8::stdout());
        println!(" (totals)");
        return;
    }

    crc = u64::MAX;
    if METHOD == 8 && LAST_MEMBER == 0 {
        crc = UNZIP_CRC;
    }

    if VERBOSE != 0 {
        let month_abbr: [&str; 12] = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        let tm = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let tm = std::time::UNIX_EPOCH + std::time::Duration::from_secs(tm);
        if let Ok(tm) = tm.tm() {
            print!("{:>5} {:08x} ", METHODS[METHOD as usize], crc);
            print!("{}{:>3} {:02}:{:02} ",
                   month_abbr[tm.tm_mon as usize], tm.tm_mday, tm.tm_hour, tm.tm_min);
        } else {
            print!("??? ?? ??:?? ");
        }
    }

    fprint_off(&mut i8::stdout(), BYTES_IN, positive_off_t_width.try_into().unwrap()).unwrap();
    print!(" ");
    fprint_off(&mut i8::stdout(), BYTES_OUT, positive_off_t_width.try_into().unwrap()).unwrap();
    print!(" ");

    if BYTES_IN == -1 {
        TOTAL_IN = -1;
        BYTES_IN = 0;
        BYTES_OUT = 0;
        HEADER_BYTES = 0;
    } else if TOTAL_IN >= 0 {
        TOTAL_IN += BYTES_IN;
    }

    if BYTES_OUT == -1 {
        TOTAL_OUT = -1;
        BYTES_IN = 0;
        BYTES_OUT = 0;
        HEADER_BYTES = 0;
    } else if TOTAL_OUT >= 0 {
        TOTAL_OUT += BYTES_OUT;
    }

    display_ratio(BYTES_OUT - (BYTES_IN - HEADER_BYTES), BYTES_OUT, &mut i8::stdout());
    println!(" {}", std::str::from_utf8(&OFNAME).unwrap());
}
fn dtotimespec(d: f64) -> Result<usize, ()>{todo!("proto")}
fn dup_cloexec(fd: i32) -> Result<i32, ()> {
    match crate::rpl_fcntl(fd, 1030) {
        -1 => Err(()),
        res => Ok(res),
    }
}
fn dup_safer(fd: i32) -> i32 {
    rpl_fcntl(fd, 0, 2 + 1)
}
fn dup_safer_flag(fd: i32, flag: i32) -> Result<i32, ()> {
    let action = if flag & 0o2000000 != 0 { 1030 } else { 0 };
    let result = rpl_fcntl(fd, action, 2 + 1);
    Ok(result)
}
fn error(status: i32, errnum: i32, format: &str, ) -> (){todo!("proto")}
fn error_at_line(status: i32, errnum: i32, fname: &str, lineno: u32, format: &str, ) -> (){todo!("proto")}
fn fd_safer(fd: i32) -> Result<i32, ()> {
    use std::os::fd::IntoRawFd;
    use std::os::fd::FromRawFd;
    if 0 <= fd && fd <= 2 {
        let f = dup_safer(fd);
        let e = std::io::Error::last_os_error();
        std::fs::File::from_raw_fd(fd).into_raw_fd();
        std::io::Error::last_os_error(e);
        fd = f;
    }
    Ok(fd)
}
fn fd_safer_flag(mut fd: i32, flag: i32) -> Result<i32, ()> {
    if (0 <= fd && fd <= 2) {
        let f = match dup_safer_flag(fd, flag) {
            Ok(val) => val,
            Err(_) => return Err(()),
        };
        let e = unsafe { *libc::__errno_location() };
        unsafe { libc::close(fd) };
        unsafe { *libc::__errno_location() = e };
        fd = f;
    }
    Ok(fd)
}
fn fdopen(fd: i32, mode: &str) -> *mut libc::FILE{todo!("proto")}
fn fdopendir(fd: i32) -> *mut libc::DIR{todo!("proto")}
fn fdutimens(fd: i32, file: &str, timespec: Option<[libc::timespec; 2]>) -> i32 {
    let mut adjusted_timespec = [libc::timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    let ts = if let Some(timespec) = timespec {
        adjusted_timespec[0] = timespec[0];
        adjusted_timespec[1] = timespec[1];
        validate_timespec(&adjusted_timespec)
    } else {
        0
    };

    if ts < 0 {
        return -1;
    }

    if fd < 0 && file.is_empty() {
        unsafe {
            *libc::__errno_location() = 9;
        }
        return -1;
    }

    if UTIMENSAT_WORKS_REALLY >= 0 {
        let result;
        if ts == 2 {
            let mut st = libc::stat { st_dev: 0, st_ino: 0, st_mode: 0, st_nlink: 0, st_uid: 0, st_gid: 0, st_rdev: 0, st_size: 0, st_blksize: 0, st_blocks: 0, st_atime: libc::timespec { tv_sec: 0, tv_nsec: 0 }, st_mtime: libc::timespec { tv_sec: 0, tv_nsec: 0 }, st_ctime: libc::timespec { tv_sec: 0, tv_nsec: 0 }};
            if fd < 0 {
                if libc::stat(file.as_ptr(), &mut st) != 0 {
                    return -1;
                }
                if timespec.unwrap()[0].tv_nsec == ((1 << 30) - 2) {
                    timespec.unwrap()[0] = get_stat_atime(&st);
                } else if timespec.unwrap()[1].tv_nsec == ((1 << 30) - 2) {
                    timespec.unwrap()[1] = get_stat_mtime(&st);
                }
                ts += 1;
            }
            if fd < 0 {
                result = libc::utimensat(-100, file.as_ptr(), adjusted_timespec.as_ptr(), 0);
                if result > 0 {
                    unsafe {
                        *libc::__errno_location() = 38;
                    }
                }
                if result == 0 || unsafe { *libc::__errno_location() } != 38 {
                    UTIMENSAT_WORKS_REALLY = 1;
                    return result;
                }
            }
            if fd >= 0 {
                result = libc::futimens(fd, adjusted_timespec.as_ptr());
                if result > 0 {
                    unsafe {
                        *libc::__errno_location() = 38;
                    }
                }
                if result == 0 || unsafe { *libc::__errno_location() } != 38 {
                    UTIMENSAT_WORKS_REALLY = 1;
                    return result;
                }
            }
        }
    }

    UTIMENSAT_WORKS_REALLY = -1;
    LUTIMENSAT_WORKS_REALLY = -1;

    if ts != 3 && (fd < 0 || libc::fstat(fd, &mut libc::stat { st_dev: 0, st_ino: 0, st_mode: 0, st_nlink: 0, st_uid: 0, st_gid: 0, st_rdev: 0, st_size: 0, st_blksize: 0, st_blocks: 0, st_atime: libc::timespec { tv_sec: 0, tv_nsec: 0 }, st_mtime: libc::timespec { tv_sec: 0, tv_nsec: 0 }, st_ctime: libc::timespec { tv_sec: 0, tv_nsec: 0 }}) != 0) {
        return -1;
    }

    if let Some(ts) = &timespec {
        let mut timeval = [libc::timeval { tv_sec: 0, tv_usec: 0 }; 2];
        timeval[0].tv_sec = ts[0].tv_sec;
        timeval[0].tv_usec = ts[0].tv_nsec / 1000;
        timeval[1].tv_sec = ts[1].tv_sec;
        timeval[1].tv_usec = ts[1].tv_nsec / 1000;

        let t = timeval.as_ptr();

        if fd < 0 {
            return libc::futimes(-100, file.as_ptr(), t);
        } else {
            if libc::futimes(fd, std::ptr::null(), t) == 0 {
                if timeval.iter().any(|tv| tv.tv_usec >= 500000) {
                    let mut st = libc::stat { st_dev: 0, st_ino: 0, st_mode: 0, st_nlink: 0, st_uid: 0, st_gid: 0, st_rdev: 0, st_size: 0, st_blksize: 0, st_blocks: 0, st_atime: libc::timespec { tv_sec: 0, tv_nsec: 0 }, st_mtime: libc::timespec { tv_sec: 0, tv_nsec: 0 }, st_ctime: libc::timespec { tv_sec: 0, tv_nsec: 0 }};
                    if libc::fstat(fd, &mut st) == 0 {
                        let adiff = st.st_atime.tv_sec - timeval[0].tv_sec;
                        let mdiff = st.st_mtime.tv_sec - timeval[1].tv_sec;
                        let mut tt = [libc::timeval { tv_sec: 0, tv_usec: 0 }; 2];
                        let mut truncated_timeval = timeval;
                        if adiff == 1 && get_stat_atime_ns(&st) == 0 {
                            tt = truncated_timeval;
                            tt[0].tv_usec = 0;
                        }
                        if mdiff == 1 && get_stat_mtime_ns(&st) == 0 {
                            tt = truncated_timeval;
                            tt[1].tv_usec = 0;
                        }
                        if tt.iter().any(|tv| tv.tv_usec != 0) {
                            libc::futimes(fd, std::ptr::null(), tt.as_ptr());
                        }
                    }
                }
                return 0;
            }
        }
    }

    if file.is_empty() {
        return -1;
    }

    libc::utimes(file.as_ptr(), std::ptr::null())
}
fn file_read(buf: *mut std::ffi::c_void, size: u32) -> Result<i32, ()> {
    use std::convert::TryInto;
    let len = read_buffer(unsafe { IFD }, buf, size);
    if len == 0 {
        return Ok(len as i32);
    }
    if len == std::u32::MAX.try_into().unwrap() {
        read_error();
    }
    let buf_slice = unsafe { std::slice::from_raw_parts_mut(buf as *mut Uch, len as usize) };
    updcrc(Some(buf_slice), len as usize);
    unsafe {
        BYTES_IN += len as std::os::raw::c_long;
    }
    Ok(len as i32)
}
fn fill_inbuf(eof_ok: bool) -> i32 {
    let mut len: i32;
    unsafe {
        INSIZE = 0;
        loop {
            len = read_buffer(IFD, INBUF.as_mut_ptr().offset(INSIZE as isize) as Voidp, 0x40000 - INSIZE);
            if len == 0 {
                break;
            }
            if len == -1 {
                read_error();
                break;
            }
            INSIZE += len as u32;
            if INSIZE >= 0x40000 {
                break;
            }
        }
        if INSIZE == 0 {
            if eof_ok {
                return -1;
            }
            flush_window();
            std::ptr::write_volatile(&mut *libc::__errno_location(), 0);
            read_error();
        }
        BYTES_IN += INSIZE as std::os::raw::c_long;
        INPTR = 1;
        return INBUF[0] as i32;
    }
}
fn fill_window() {
    use std::convert::TryInto;
    
    let mut n: u32;
    let mut m: u32;
    let more = WINDOW_SIZE - LOOKAHEAD as Ulg - STRSTART as Ulg;
    
    if more == u64::MAX {
        more -= 1;
    } else if STRSTART >= 0x8000 + (0x8000 - (258 + 3 + 1)) {
        WINDOW.copy_within(0x8000.., 0);
        MATCH_START -= 0x8000;
        STRSTART -= 0x8000;
        
        if RSYNC_CHUNK_END != 0xFFFFFFFF {
            RSYNC_CHUNK_END -= 0x8000;
        }
        
        BLOCK_START -= 0x8000;
        
        for n in 0..(1 << 15) {
            m = PREV[0x8000 + n] as u32;
            PREV[0x8000 + n] = if m >= 0x8000 { (m - 0x8000).try_into().unwrap() } else { 0 };
        }
        
        for n in 0..0x8000 {
            m = PREV[n] as u32;
            PREV[n] = if m >= 0x8000 { (m - 0x8000).try_into().unwrap() } else { 0 };
        }
        
        more += 0x8000;
    }
    
    if !EOFILE != 0 {
        n = READ_BUF.unwrap_or(0) as u32;
        
        if n == 0 || n == u32::MAX {
            EOFILE = true;
            WINDOW[STRSTART as usize + LOOKAHEAD as usize..].fill(0);
        } else {
            LOOKAHEAD += n;
        }
    }
}
fn fillbuf(n: i32) {
    unsafe {
        BITBUF <<= n;
        while n > BITCOUNT {
            BITBUF |= (SUBBITBUF as Ulg) << (n - BITCOUNT);
            SUBBITBUF = if INPTR < INSIZE {
                INBUF[INPTR as usize].into()
            } else {
                (fill_inbuf(true) as Uch).into()
            };
            if SUBBITBUF as i32 == -1 {
                SUBBITBUF = 0;
            }
            BITCOUNT = 8;
        }
        BITBUF |= (SUBBITBUF as Ulg) >> (BITCOUNT - n);
    }
}
fn find_non_slash(s: &str) -> Option<&str> {
    let n_slash = s.chars().take_while(|&c| c == '/').count();
    Some(&s[n_slash..])
}
fn finish_out() -> ! {
    use std::os::fd::AsRawFd;
    let stdout_raw_ptr = std::io::stdout().as_raw_fd() as *mut std::fs::File;
    
    if rpl_fclose(stdout_raw_ptr) != 0 {
        write_error();
    }
    do_exit(0);
}
fn flush_block(buf: *const Uch, stored_len: Ulg, pad: i32, eof: i32) -> Result<Ulg, ()> {
    use std::convert::TryInto;
    let mut opt_lenb: Ulg;
    let mut static_lenb: Ulg;
    let mut max_blindex: i32;

    unsafe {
        FLAG_BUF[LAST_FLAGS as usize] = FLAGS;
        if *FILE_TYPE == 0xffff {
            set_file_type();
        }
        build_tree(&mut *L_DESC)?;
        build_tree(&mut *D_DESC)?;
        max_blindex = build_bl_tree()?;
        opt_lenb = (OPT_LEN + 3 + 7) >> 3;
        static_lenb = (STATIC_LEN + 3 + 7) >> 3;
        INPUT_LEN += stored_len as i64;

        if static_lenb <= opt_lenb {
            opt_lenb = static_lenb;
        }

        if stored_len <= opt_lenb && eof != 0 && COMPRESSED_LEN == 0 && false {
            if buf.is_null() {
                gzip_error("block vanished");
            }
            copy_block(buf, stored_len as usize, false)?;
            COMPRESSED_LEN = (stored_len << 3) as i64;
            *FILE_METHOD = 0;
        } else if stored_len + 4 <= opt_lenb && !buf.is_null() {
            send_bits((0 << 1) + eof, 3)?;
            COMPRESSED_LEN = ((COMPRESSED_LEN + 3 + 7) & !7) as i64;
            COMPRESSED_LEN += ((stored_len + 4) << 3) as i64;
            copy_block(buf, stored_len as usize, true)?;
        } else if static_lenb == opt_lenb {
            send_bits((1 << 1) + eof, 3)?;
            compress_block(STATIC_LTREE.as_ptr() as *const CtData, STATIC_DTREE.as_ptr() as *const CtData);
            COMPRESSED_LEN += (3 + STATIC_LEN) as i64;
        } else {
            send_bits((2 << 1) + eof, 3)?;
            send_all_trees((L_DESC.max_code + 1).try_into().unwrap(), (D_DESC.max_code + 1).try_into().unwrap(), (max_blindex + 1).try_into().unwrap());
            compress_block(DYN_LTREE.as_ptr() as *const CtData, DYN_DTREE.as_ptr() as *const CtData);
            COMPRESSED_LEN += (3 + OPT_LEN) as i64;
        }

        init_block();

        if eof != 0 {
            bi_windup()?;
            COMPRESSED_LEN += 7;
        } else if pad != 0 && COMPRESSED_LEN % 8 != 0 {
            send_bits((0 << 1) + eof, 3)?;
            COMPRESSED_LEN = ((COMPRESSED_LEN + 3 + 7) & !7) as i64;
            copy_block(buf, 0, true)?;
        }
    }

    Ok((COMPRESSED_LEN >> 3).try_into().unwrap())
}
fn flush_outbuf() -> Result<(), ()> {
    unsafe {
        if OUTCNT == 0 {
            return Ok(());
        }
        write_buf(OFD, OUTBUF.as_mut_ptr() as *mut std::ffi::c_void, OUTCNT)?;
        OUTCNT = 0;
    }
    Ok(())
}
fn flush_window() {
    if unsafe { OUTCNT } == 0 {
        return;
    }
    let crc = updcrc(Some(&unsafe { WINDOW[..unsafe { OUTCNT as usize }] }), unsafe { OUTCNT as usize });
    if let Err(_) = write_buf(unsafe { OFD }, unsafe { WINDOW.as_mut_ptr() }, unsafe { OUTCNT }) {
        return;
    }
    unsafe {
        OUTCNT = 0;
    }
}
fn fopen(filename: &str, mode: &str) -> *mut std::fs::File{todo!("proto")}
fn fprint_off(file: &mut std::fs::File, mut offset: i64, mut width: i32) -> Result<(), ()> {
    use std::convert::TryInto;
    use std::io::Write;
    let mut buf = [0; 8 * std::mem::size_of::<i64>()];
    let mut p = buf.as_mut_ptr().wrapping_add(buf.len());

    if offset < 0 {
        loop {
            unsafe {
                p = p.wrapping_sub(1);
                *p = ('0' as u8).wrapping_sub((offset % 10).abs() as u8).try_into().unwrap();
            }
            if offset / 10 != 0 {
                offset /= 10;
            } else {
                break;
            }
        }
        unsafe {
            p = p.wrapping_sub(1);
            *p = '-' as u8;
        }
    } else {
        loop {
            unsafe {
                p = p.wrapping_sub(1);
                *p = ('0' as u8 + (offset % 10) as u8).try_into().unwrap();
            }
            if offset / 10 != 0 {
                offset /= 10;
            } else {
                break;
            }
        }
    }

    width -= (buf.as_ptr().wrapping_add(buf.len()) as usize - p as usize) as i32;

    while width > 0 {
        file.write_all(&[b' ']).map_err(|_| ())?;
        width -= 1;
    }

    for i in 0..buf.len() - (p as usize - buf.as_ptr() as usize) {
        unsafe {
            file.write_all(&[*(p.add(i))]).map_err(|_| ())?;
        }
    }

    Ok(())
}
fn fpurge(fp: *mut std::os::raw::c_void) -> i32 {
    unsafe {
        libc::fflush(fp as *mut libc::FILE);
    }
    0
}
fn free_cwd(cwd: &mut SavedCwd) -> Result<(), ()> {
    use std::os::fd::IntoRawFd;
    use std::os::fd::FromRawFd;
    if cwd.desc >= 0 {
        std::fs::File::from_raw_fd(cwd.desc).into_raw_fd();
    }
    rpl_free(cwd.name.as_mut())
}
fn freea(p: *mut std::ffi::c_void) -> Result<(), ()> {
    let sa_alignment_max = std::mem::align_of::<SmallT>();
    let p_val = p as usize;
    
    if p_val & (sa_alignment_max - 1) != 0 {
        return Err(());
    }
    
    if p_val & sa_alignment_max != 0 {
        let mem = (p as *const u8).wrapping_sub(1) as *const SmallT;
        let offset = unsafe { *mem.offset(-1) as isize };
        let mem_to_free = (p as *const u8).wrapping_sub(offset as usize) as *mut std::ffi::c_void;
        return rpl_free(mem_to_free);
    }
    
    Ok(())
}
fn fseterr(fp: &mut std::fs::File) {
    fp.flags_mut().insert(std::fs::FileFlags::F_ERR);
}
fn gen_bitlen(desc: &mut TreeDesc) {
    let tree = desc.dyn_tree;
    let extra = desc.extra_bits;
    let base = desc.extra_base;
    let max_code = desc.max_code;
    let max_length = desc.max_length;
    let stree = desc.static_tree;
    let mut h: i32;
    let mut n: i32;
    let mut m: i32;
    let mut bits: i32;
    let mut xbits: i32;
    let mut f: Ush;
    let mut overflow = 0;

    for bits in 0..=15 {
        unsafe {
            BL_COUNT[bits as usize] = 0;
        }
    }

    tree[HEAP[HEAP_MAX as usize]].dl.len = 0;

    for h in (HEAP_MAX + 1)..(2 * (256 + 1 + 29) + 1) {
        n = unsafe { HEAP[h as usize] };
        bits = tree[tree[n as usize].dl.dad as usize].dl.len as i32 + 1;
        if bits > max_length {
            bits = max_length;
            overflow += 1;
        }
        tree[n as usize].dl.len = bits as Ush;

        if n > max_code {
            continue;
        }

        unsafe {
            BL_COUNT[bits as usize] += 1;
        }

        xbits = if n >= base { extra[(n - base) as usize] } else { 0 };
        f = tree[n as usize].fc.freq;
        OPT_LEN += (f as Ulg) * ((bits + xbits) as Ulg);

        if let Some(stree) = stree {
            STATIC_LEN += (f as Ulg) * ((stree[n as usize].dl.len + xbits) as Ulg);
        }
    }

    if overflow == 0 {
        return;
    }

    loop {
        bits = max_length - 1;
        while unsafe { BL_COUNT[bits as usize] } == 0 {
            bits -= 1;
        }

        unsafe {
            BL_COUNT[bits as usize] -= 1;
            BL_COUNT[(bits + 1) as usize] += 2;
            BL_COUNT[max_length as usize] -= 1;
            overflow -= 2;
        }

        if overflow <= 0 {
            break;
        }
    }

    for bits in (1..=max_length).rev() {
        n = unsafe { BL_COUNT[bits as usize].into() };
        while n > 0 {
            m = unsafe { HEAP[h as usize - 1] };
            if m > max_code {
                continue;
            }

            if tree[m as usize].dl.len != bits as Ush {
                OPT_LEN += ((bits - tree[m as usize].dl.len as i32) as Ulg) * (tree[m as usize].fc.freq as Ulg);
                tree[m as usize].dl.len = bits as Ush;
            }

            n -= 1;
            h -= 1;
        }
    }
}
fn gen_codes(tree: &mut [CtData], max_code: i32) -> Result<(), ()> {
    let mut next_code: [Ush; 16] = [0; 16];
    let mut code: Ush = 0;
    for bits in 1..=15 {
        next_code[bits] = code;
        code = (code + unsafe { BL_COUNT.get_unchecked(bits - 1) }) << 1;
    }

    for n in 0..=max_code {
        let len = tree[n as usize].dl.len;
        if len == 0 {
            continue;
        }
        tree[n as usize].fc.code = bi_reverse(next_code[len as usize].into(), len.into());
        next_code[len as usize] += 1;
    }

    Ok(())
}
fn get_handler(a: &libc::sigaction) -> Option<extern "C" fn(i32)> {
    if a.sa_sigaction == 0 {
        None
    } else {
        Some(unsafe { std::mem::transmute(a.sa_sigaction) })
    }
}
fn get_input_size_and_time() {
    use std::os::linux::fs::MetadataExt;
    unsafe {
        let mut file_size = -1;
        let mut time_stamp = std::time::SystemTime::now();

        if let Some(istat) = ISTAT.as_ref() {
            if istat.st_mode() & 0o170000 == 0o100000 {
                file_size = istat.len() as i64;
                if NO_TIME == 0 || LIST != 0 {
                    time_stamp = get_stat_mtime(istat);
                }
            }
        }
    }
}
fn get_method(in_data: i32) -> i32 {
    use std::convert::TryInto;
    let mut flags: Uch;
    let mut magic: [Uch; 10];
    let mut imagic0: i32;
    let mut imagic1: i32;
    let mut stamp: Ulg;

    if unsafe { FORCE } != 0 && unsafe { TO_STDOUT } != 0 {
        imagic0 = if unsafe { INPTR } < unsafe { INSIZE } {
            unsafe { INBUF[unsafe { INPTR }] }
        } else {
            fill_inbuf(1)
        };
        magic[0] = imagic0;
        imagic1 = if unsafe { INPTR } < unsafe { INSIZE } {
            unsafe { INBUF[unsafe { INPTR }] }
        } else {
            fill_inbuf(1)
        };
        magic[1] = imagic1;
    } else {
        magic[0] = if unsafe { INPTR } < unsafe { INSIZE } {
            unsafe { INBUF[unsafe { INPTR }] }
        } else {
            fill_inbuf(0).try_into().unwrap()
        };
        imagic0 = 0;
        if magic[0] != 0 {
            magic[1] = if unsafe { INPTR } < unsafe { INSIZE } {
                unsafe { INBUF[unsafe { INPTR }] }
            } else {
                fill_inbuf(0).try_into().unwrap()
            };
            imagic1 = 0;
        } else {
            imagic1 = if unsafe { INPTR } < unsafe { INSIZE } {
                unsafe { INBUF[unsafe { INPTR }] }
            } else {
                fill_inbuf(1)
            };
            magic[1] = imagic1;
        }
    }

    unsafe {
        METHOD = -1;
        PART_NB += 1;
        HEADER_BYTES = 0;
        LAST_MEMBER = 0;
    }

    if magic == [0o037, 0o213] || magic == [0o037, 0o236] {
        unsafe {
            METHOD = if unsafe { INPTR } < unsafe { INSIZE } {
                unsafe { INBUF[unsafe { INPTR }] }
            } else {
                fill_inbuf(0)
            };
            if METHOD != 8 {
                rpl_fprintf(std::io::stderr(), "{}: {}: unknown method {} -- not supported\n", unsafe { PROGRAM_NAME }, unsafe { IFNAME }, METHOD).unwrap();
                unsafe {
                    EXIT_CODE = 1;
                }
                return -1;
            }
            unsafe {
                WORK = unzip;
                flags = if unsafe { INPTR } < unsafe { INSIZE } {
                    unsafe { INBUF[unsafe { INPTR }] }
                } else {
                    fill_inbuf(0).try_into().unwrap()
                };
                if (flags & 0x20) != 0 {
                    rpl_fprintf(std::io::stderr(), "{}: {} is encrypted -- not supported\n", unsafe { PROGRAM_NAME }, unsafe { IFNAME }).unwrap();
                    EXIT_CODE = 1;
                    return -1;
                }
                if (flags & 0xC0) != 0 {
                    rpl_fprintf(std::io::stderr(), "{}: {} has flags 0x{:x} -- not supported\n", unsafe { PROGRAM_NAME }, unsafe { IFNAME }, flags).unwrap();
                    EXIT_CODE = 1;
                    if unsafe { FORCE } <= 1 {
                        return -1;
                    }
                }
                stamp = if unsafe { INPTR } < unsafe { INSIZE } {
                    unsafe { INBUF[unsafe { INPTR }] }
                } else {
                    fill_inbuf(0)
                } as Ulg;
                stamp |= (if unsafe { INPTR } < unsafe { INSIZE } {
                    unsafe { INBUF[unsafe { INPTR }] }
                } else {
                    fill_inbuf(0)
                } as Ulg) << 8;
                stamp |= (if unsafe { INPTR } < unsafe { INSIZE } {
                    unsafe { INBUF[unsafe { INPTR }] }
                } else {
                    fill_inbuf(0)
                } as Ulg) << 16;
                stamp |= (if unsafe { INPTR } < unsafe { INSIZE } {
                    unsafe { INBUF[unsafe { INPTR }] }
                } else {
                    fill_inbuf(0)
                } as Ulg) << 24;
                if stamp != 0 && !unsafe { NO_TIME } {
                    if stamp <= std::time::SystemTime::UNIX_EPOCH.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs() {
                        unsafe {
                            TIME_STAMP = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(stamp);
                        }
                    } else {
                        if !unsafe { QUIET } {
                            rpl_fprintf(std::io::stderr(), "{}: {}: MTIME {} out of range for this platform\n", unsafe { PROGRAM_NAME }, unsafe { IFNAME }, stamp).unwrap();
                        }
                        if EXIT_CODE == 0 {
                            EXIT_CODE = 2;
                        }
                        unsafe {
                            TIME_STAMP = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(std::time::SystemTime::UNIX_EPOCH.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs());
                        }
                    }
                }
                magic[8] = if unsafe { INPTR } < unsafe { INSIZE } {
                    unsafe { INBUF[unsafe { INPTR }] }
                } else {
                    fill_inbuf(0).try_into().unwrap()
                };
                magic[9] = if unsafe { INPTR } < unsafe { INSIZE } {
                    unsafe { INBUF[unsafe { INPTR }] }
                } else {
                    fill_inbuf(0).try_into().unwrap()
                };
                if flags & 0x02 != 0 {
                    let mut lenbuf: [Uch; 2];
                    let len = lenbuf[0] = if unsafe { INPTR } < unsafe { INSIZE } {
                        unsafe { INBUF[unsafe { INPTR }] }
                    } else {
                        fill_inbuf(0)
                    } as u16;
                    len |= (lenbuf[1] = if unsafe { INPTR } < unsafe { INSIZE } {
                        unsafe { INBUF[unsafe { INPTR }] }
                    } else {
                        fill_inbuf(0)
                    } as u16) << 8;
                    if unsafe { VERBOSE } {
                        rpl_fprintf(std::io::stderr(), "{}: {}: extra field of {} bytes ignored\n", unsafe { PROGRAM_NAME }, unsafe { IFNAME }, len).unwrap();
                    }
                    if flags & 0x02 != 0 {
                        updcrc(None, 0);
                        updcrc(Some(&lenbuf), 2);
                    }
                    discard_input_bytes(len as usize, flags.into());
                }
                if flags & 0x08 != 0 {
                    if unsafe { NO_NAME } || (unsafe { TO_STDOUT } && !unsafe { LIST }) || unsafe { PART_NB } > 1 {
                        discard_input_bytes(usize::MAX, flags.into());
                    } else {
                        let p = gzip_base_name(unsafe { OFNAME });
                        let base = p;
                        loop {
                            let byte = if unsafe { INPTR } < unsafe { INSIZE } {
                                unsafe { INBUF[unsafe { INPTR }] }
                            } else {
                                fill_inbuf(0)
                            } as u8;
                            if byte == 0 {
                                break;
                            }
                            if p >= unsafe { OFNAME }.as_ptr().offset(unsafe { OFNAME }.len() as isize) {
                                gzip_error("corrupted input -- file name too large");
                            }
                            p = p.offset(1);
                        }
                        if flags & 0x02 != 0 {
                            updcrc(base, (p as usize - base as usize) as usize);
                        }
                        let p = gzip_base_name(base);
                        let len = unsafe { std::ffi::CStr::from_ptr(p).to_bytes().len() };
                        unsafe {
                            std::ptr::copy(p, base, len + 1);
                        }
                        if !unsafe { LIST } {
                            if base.is_null() {
                                unsafe {
                                    LIST = 0;
                                }
                            }
                        }
                    }
                }
                if flags & 0x10 != 0 {
                    discard_input_bytes(usize::MAX, flags.into());
                }
                if flags & 0x02 != 0 {
                    let crc16 = updcrc(Some(&magic), 0) & 0xffff;
                    let header16 = if unsafe { INPTR } < unsafe { INSIZE } {
                        unsafe { INBUF[unsafe { INPTR }] }
                    } else {
                        fill_inbuf(0)
                    } as u16;
                    let header16 = header16 | ((if unsafe { INPTR } < unsafe { INSIZE } {
                        unsafe { INBUF[unsafe { INPTR }] }
                    } else {
                        fill_inbuf(0)
                    } as u16) << 8);
                    if u64::from(header16) != crc16 {
                        rpl_fprintf(std::io::stderr(), "{}: {}: header checksum 0x{:04x} != computed checksum 0x{:04x}\n", unsafe { PROGRAM_NAME }, unsafe { IFNAME }, header16, crc16).unwrap();
                        EXIT_CODE = 1;
                        if unsafe { FORCE } <= 1 {
                            return -1;
                        }
                    }
                }
                if unsafe { PART_NB } == 1 {
                    unsafe {
                        HEADER_BYTES = INPTR + 2 * 4;
                    }
                }
            }
        }
    } else if magic == [0o120, 0o113, 0o003, 0o004] && unsafe { INPTR } == 2
            && unsafe { INBUF }.as_ptr() == [0o120, 0o113, 0o003, 0o004].as_ptr() {
        unsafe {
            INPTR = 0;
            WORK = unzip;
        }
        if check_zipfile(in_data) != Ok(()) {
            return -1;
        }
        unsafe {
            LAST_MEMBER = 1;
        }
    } else if magic == [0o037, 0o036] {
        unsafe {
            WORK = unpack;
            METHOD = 2;
        }
    } else if magic == [0o037, 0o235] {
        unsafe {
            WORK = unlzw;
            METHOD = 1;
            LAST_MEMBER = 1;
        }
    } else if magic == [0o037, 0o240] {
        unsafe {
            WORK = unlzh;
            METHOD = 3;
            LAST_MEMBER = 1;
        }
    } else if unsafe { FORCE } != 0 && unsafe { TO_STDOUT } && !unsafe { LIST } {
        unsafe {
            METHOD = 0;
            WORK = std::fs::copy;
            if imagic1 != -1 {
                INPTR -= 1;
            }
            LAST_MEMBER = 1;
            if imagic0 != -1 {
                write_buf(1, &magic[0] as *const Uch as *mut std::ffi::c_void, 1).unwrap();
            }
        }
    }

    if unsafe { METHOD } >= 0 {
        return unsafe { METHOD };
    }

    if unsafe { PART_NB } == 1 {
        rpl_fprintf(std::io::stderr(), "\n{}: {}: not in gzip format\n", unsafe { PROGRAM_NAME }, unsafe { IFNAME }).unwrap();
        unsafe {
            EXIT_CODE = 1;
        }
        return -1;
    } else {
        if magic[0] == 0 {
            let mut inbyte;
            loop {
                inbyte = if unsafe { INPTR } < unsafe { INSIZE } {
                    unsafe { INBUF[unsafe { INPTR }] }
                } else {
                    fill_inbuf(1)
                };
                if inbyte != 0 {
                    break;
                }
            }
            if inbyte == -1 {
                if unsafe { VERBOSE } {
                    if !unsafe { QUIET } {
                        rpl_fprintf(std::io::stderr(), "\n{}: {}: decompression OK, trailing zero bytes ignored\n", unsafe { PROGRAM_NAME }, unsafe { IFNAME }).unwrap();
                    }
                    if EXIT_CODE == 0 {
                        EXIT_CODE = 2;
                    }
                }
                return -3;
            }
        }
        if !unsafe { QUIET } {
            rpl_fprintf(std::io::stderr(), "\n{}: {}: decompression OK, trailing garbage ignored\n", unsafe { PROGRAM_NAME }, unsafe { IFNAME }).unwrap();
        }
        if EXIT_CODE == 0 {
            EXIT_CODE = 2;
        }
        return -2;
    }
}
fn get_stat_atime(st: &libc::stat) -> libc::timespec {
    libc::timespec {
        tv_sec: st.st_atime,
        tv_nsec: 0,
    }
}
fn get_stat_atime_ns(st: &std::fs::Metadata) -> u32 {
    let accessed_time = st.accessed().unwrap();
    let duration = accessed_time.duration_since(std::time::UNIX_EPOCH).unwrap();
    duration.subsec_nanos()
}
fn get_stat_birthtime(st: &std::os::linux::raw::stat) -> Result<libc::timespec, ()> {
    let mut t = libc::timespec {
        tv_sec: -1,
        tv_nsec: -1,
    };
    Ok(t)
}
fn get_stat_birthtime_ns(st: &libc::stat) -> i64 {
    0
}
fn get_stat_ctime(st: &libc::stat) -> libc::timespec {
    libc::timespec {
        tv_sec: st.st_ctime,
        tv_nsec: 0,
    }
}
fn get_stat_ctime_ns(st: &std::fs::Metadata) -> u32 {
    use std::os::unix::fs::MetadataExt;
    st.ctime().unwrap().subsec_nanos()
}
fn get_stat_mtime(st: &libc::stat) -> libc::timespec {
    libc::timespec {
        tv_sec: st.st_mtime,
        tv_nsec: 0,
    }
}
fn get_stat_mtime_ns(st: &std::fs::Metadata) -> u32 {
    let modified_time = st.modified().unwrap();
    let duration = modified_time.duration_since(std::time::UNIX_EPOCH).unwrap();
    duration.subsec_nanos()
}
fn get_suffix(name: *const i8) -> *const i8 {
    use std::convert::TryInto;
    use std::ffi::CString;
    use std::ptr;

    let mut nlen: usize;
    let mut slen: usize;
    let mut suffix = [0 as i8; 30 + 3];
    let mut known_suffixes: Vec<*const i8> = vec![ptr::null(); 9];
    known_suffixes[1] = ".gz\0".as_ptr();
    known_suffixes[2] = ".z\0".as_ptr();
    known_suffixes[3] = ".taz\0".as_ptr();
    known_suffixes[4] = ".tgz\0".as_ptr();
    known_suffixes[5] = "-gz\0".as_ptr();
    known_suffixes[6] = "-z\0".as_ptr();
    known_suffixes[7] = "_z\0".as_ptr();

    let mut suffix_of_builtin = false;
    for &suf in known_suffixes.iter().skip(1) {
        let suflen = unsafe { libc::strlen(suf) };
        if unsafe { Z_LEN } < suflen && unsafe { libc::strcmp(Z_SUFFIX, suf.offset((suflen - unsafe { Z_LEN }).try_into().unwrap())) == 0 } {
            suffix_of_builtin = true;
            break;
        }
    }

    let z_lower = match xstrdup(unsafe { std::ffi::CStr::from_ptr(Z_SUFFIX).to_str().unwrap() }) {
        Ok(s) => {
            let mut z_lower = s.to_string();
            strlwr(&mut z_lower);
            z_lower
        }
        Err(_) => return std::ptr::null(),
    };

    known_suffixes[(suffix_of_builtin as usize).max(known_suffixes.len() - 2)] = z_lower.as_ptr();
    let mut suf = known_suffixes.as_ptr().offset(suffix_of_builtin as isize);

    nlen = unsafe { libc::strlen(name) };
    if nlen <= 30 + 2 {
        unsafe { libc::strcpy(suffix.as_mut_ptr(), name) };
    } else {
        unsafe { libc::strcpy(suffix.as_mut_ptr(), name.offset((nlen - 30 - 2) as isize)) };
    }

    strlwr(&mut CString::new(suffix.iter().map(|&c| c as u8).collect::<Vec<u8>>()).unwrap());
    slen = unsafe { libc::strlen(suffix.as_ptr()) };

    let mut match_ptr: *const i8 = std::ptr::null();
    while !suf.is_null() {
        let s = unsafe { libc::strlen(*suf) };
        if slen > s && suffix[slen - s - 1] != '/' as i8 && unsafe { libc::strcmp(suffix.as_ptr().offset((slen - s) as isize), *suf) == 0 } {
            match_ptr = name.offset((nlen - s) as isize);
            break;
        }
        suf = suf.offset(1);
    }

    rpl_free(z_lower.as_ptr() as *mut std::ffi::c_void);
    match_ptr
}
fn getbits(n: i32) -> Result<Ulg, ()> {
    let x = unsafe { BITBUF >> ((8 * 2 * std::mem::size_of::<u8>() as i32) - n) };
    fillbuf(n);
    Ok(x)
}
fn getcrc() -> Ulg {
    unsafe {
        CRC ^ 0xffffffffu64
    }
}
fn getprogname() -> Option<&'static str> {
    std::env::current_exe().ok().and_then(|path| path.file_name())
        .and_then(|file_name| file_name.to_str())
}
fn gettime(ts: &mut libc::timespec) {
    unsafe {
        libc::clock_gettime(0, ts);
    }
}
fn gettime_res() -> i64{todo!("proto")}
fn gzip_base_name(fname: &mut String) -> Option<&str> {
    if let Some(base_name) = last_component(fname) {
        if 'A' == 'a' {
            strlwr(&mut base_name.to_string());
        }
        Some(base_name)
    } else {
        None
    }
}
fn gzip_error(m: &str) -> ! {
    let stderr = std::io::stderr();
    let _ = rpl_fprintf(&mut stderr.lock(), "\n%s: %s: %s\n", unsafe { &*PROGRAM_NAME }, unsafe { &IFNAME }, m.len());
    abort_gzip();
}
fn help() {
    static HELP_MSG: [&str; 27] = [
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

    let mut p = HELP_MSG.iter();
    __printf__("Usage: %s [OPTION]... [FILE]...\n", unsafe { PROGRAM_NAME });
    while let Some(msg) = p.next() {
        __printf__("%s\n", msg);
    }
}
fn huf_decode_start() {
    init_getbits();
    unsafe {
        crate::BLOCKSIZE = 0;
    }
}
fn huft_build(b: &[u32], n: u32, s: u32, d: &[u16], e: &[u16], t: &mut *mut Huft, m: &mut i32) -> Result<(), ()> {
    use std::convert::TryInto;
    let mut a: u32;
    let mut c = [0u32; 16 + 1];
    let mut f: u32;
    let mut g: i32;
    let mut h: i32;
    let mut i: u32;
    let mut j: u32;
    let mut k: i32;
    let mut l: i32;
    let mut p: *const u32;
    let mut q: *mut Huft;
    let mut r: Huft;
    let mut u = [std::ptr::null_mut(); 16];
    let mut v = [0u32; 288];
    let mut w: i32;
    let mut x = [0u32; 16 + 1];
    let mut xp: *mut u32;
    let mut y: i32;
    let mut z: u32;

    unsafe {
        libc::memset(c.as_mut_ptr() as *mut std::ffi::c_void, 0, std::mem::size_of_val(&c));
    }

    p = b.as_ptr();
    i = n;
    while i > 0 {
        c[*p as usize] += 1;
        p = p.offset(1);
        i -= 1;
    }

    if c[0] == n {
        let q = unsafe {
            libc::malloc(3 * std::mem::size_of::<Huft>()) as *mut Huft
        };
        if q.is_null() {
            return Err(());
        }
        unsafe {
            HUFTS += 3;
            (*q.offset(0)).v.t = std::ptr::null_mut();
            (*q.offset(1)).e = 99;
            (*q.offset(1)).b = 1;
            (*q.offset(2)).e = 99;
            (*q.offset(2)).b = 1;
            *t = q.offset(1);
            *m = 1;
        }
        return Ok(());
    }

    l = *m;
    for j in 1..=16 {
        if c[j as usize] != 0 {
            break;
        }
    }
    k = j as i32;
    if l < j as i32 {
        l = j as i32;
    }
    for i in (1..=16).rev() {
        if c[i as usize] != 0 {
            break;
        }
    }
    g = i as i32;
    if l > i.try_into().unwrap() {
        l = i;
    }
    *m = l;

    let mut y = 1 << j;
    for j in j..i {
        y -= c[j as usize];
        if y < 0 {
            return Err(());
        }
    }
    y -= c[i as usize];
    if y < 0 {
        return Err(());
    }
    c[i as usize] += y;

    x[1] = j;
    let mut j = 0;
    p = c.as_ptr().offset(1);
    xp = x.as_mut_ptr().offset(2);
    for _ in 0..i {
        j += *p;
        *xp = j;
        xp = xp.offset(1);
        p = p.offset(1);
    }

    p = b.as_ptr();
    i = 0;
    while i < n {
        let j = *p as usize;
        if j != 0 {
            v[x[j] as usize] = i;
        }
        i += 1;
        p = p.offset(1);
    }

    let mut n = x[g as usize];
    x[0] = 0;
    i = 0;
    p = v.as_ptr();
    h = -1;
    w = -l;
    u[0] = std::ptr::null_mut();
    q = std::ptr::null_mut();
    z = 0;

    for k in k..=g {
        let mut a = c[k as usize];
        while a > 0 {
            while k > w + l {
                h += 1;
                w += l;
                z = std::cmp::min((g - w).try_into().unwrap(), l as u32);
                let mut f = 1 << (k - w);
                if f > a + 1 {
                    f -= a + 1;
                    let mut xp = c.as_ptr().offset(k as isize);
                    let mut j = k - w;
                    if j < z.try_into().unwrap() {
                        while j < z.try_into().unwrap() {
                            if f <= *xp {
                                break;
                            }
                            f <<= 1;
                            xp = xp.offset(1);
                            f -= *xp;
                        }
                    }
                }
                let z = 1 << j;
                let q = unsafe {
                    libc::malloc((z + 1) * std::mem::size_of::<Huft>()) as *mut Huft
                };
                if q.is_null() {
                    if h > 0 {
                        huft_free(u[0]);
                    }
                    return Err(());
                }
                unsafe {
                    HUFTS += z + 1;
                    *t = q.offset(1);
                    *t = (*q).v.t;
                    u[h] = q.offset(1);
                    if h > 0 {
                        x[h as usize] = i;
                        r.b = l as u8;
                        r.e = (16 + j) as u8;
                        r.v.t = q;
                        let j = i >> (w - l);
                        (*u[h - 1])[j as usize] = r;
                    }
                }
            }
            r.b = (k - w) as u8;
            if p >= v.as_ptr().offset(n as isize) {
                r.e = 99;
            } else if *p < s {
                r.e = if *p < 256 { 16 } else { 15 };
                r.v.n = *p as u16;
                p = p.offset(1);
            } else {
                r.e = e[*p as usize - s];
                r.v.n = d[*p as usize - s];
                p = p.offset(1);
            }
            let f = 1 << (k - w);
            let mut j = i >> w;
            while j < z {
                q[j as usize] = r;
                j += f;
            }
            let mut j = 1 << (k - 1);
            while i & j != 0 {
                i ^= j;
                j >>= 1;
            }
            i ^= j;
            while i & ((1 << w) - 1) != x[h as usize] {
                h -= 1;
                w -= l;
            }
            a -= 1;
        }
    }

    if y != 0 && g != 1 {
        return Err(());
    }

    Ok(())
}
fn huft_free(t: &mut Huft) {
    let mut p: *mut Huft = t;
    while p != std::ptr::null_mut() {
        let q = unsafe { (*p).v.t };
        rpl_free(p as *mut std::ffi::c_void);
        p = q;
    }
}
fn icalloc(n: IdxT, s: IdxT) -> Option<*mut std::ffi::c_void> {
    use std::convert::TryInto;
    if std::u64::MAX < n.try_into().unwrap() {
        if s != 0 {
            return Some(_gl_alloc_nomem() as *mut std::ffi::c_void);
        }
        return Some(std::ptr::null_mut());
    }
    if std::u64::MAX < s.try_into().unwrap() {
        if n != 0 {
            return Some(_gl_alloc_nomem() as *mut std::ffi::c_void);
        }
        return Some(std::ptr::null_mut());
    }
    Some(calloc(n, s))
}
fn imalloc(s: IdxT) -> Option<*mut std::ffi::c_void> {
    if s <= std::u64::MAX as IdxT {
        Some(malloc(s))
    } else {
        None
    }
}
fn inflate() -> Result<(), ()> {
    let mut e: i32;
    let mut r: i32;
    let mut h: u32;
    OUTCNT = 0;
    BK = 0;
    BB = 0;
    h = 0;
    loop {
        HUFTS = 0;
        if let Err(err) = inflate_block(&mut e) {
            return Err(err);
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
        INPTR = INPTR.wrapping_sub(1);
    }
    (OUTCNT, flush_window())?;
    Ok(())
}
fn inflate_block(e: &mut i32) -> Result<(), ()> {
    use std::convert::TryInto;
    let mut t: u32;
    let mut w: u32;
    let mut b: Ulg;
    let mut k: u32;
    b = unsafe { BB };
    k = unsafe { BK };
    w = unsafe { OUTCNT };
    while k < 1 {
        b |= ((b as Ulg) << k) | ((if unsafe { INPTR } < unsafe { INSIZE } {
            unsafe { INBUF[unsafe { INPTR } as usize] }
        } else {
            unsafe {
                OUTCNT = w;
                fill_inbuf(false).try_into().unwrap()
            }
        }) as Ulg);
        k += 8;
    }
    *e = (b as i32) & 1;
    b >>= 1;
    k -= 1;
    while k < 2 {
        b |= ((b as Ulg) << k) | ((if unsafe { INPTR } < unsafe { INSIZE } {
            unsafe { INBUF[unsafe { INPTR } as usize] }
        } else {
            unsafe {
                OUTCNT = w;
                fill_inbuf(false).try_into().unwrap()
            }
        }) as Ulg);
        k += 8;
    }
    t = (b as u32) & 3;
    b >>= 2;
    k -= 2;
    unsafe {
        BB = b;
        BK = k;
    }
    match t {
        2 => inflate_dynamic(),
        0 => inflate_stored().map_err(|_| ()),
        1 => inflate_fixed().map_err(|_| ()),
        _ => Ok(()),
    }
}
fn inflate_codes(tl: &[Huft], td: &[Huft], bl: usize, bd: usize) -> i32 {
    let mut e: u32;
    let mut n: u32;
    let mut d: u32;
    let mut w: u32;
    let mut t: &Huft;
    let mut ml: u16;
    let mut md: u16;
    let mut b: u64;
    let mut k: u32;

    b = unsafe { BB };
    k = unsafe { BK };
    w = unsafe { OUTCNT };
    ml = unsafe { MASK_BITS[bl] };
    md = unsafe { MASK_BITS[bd] };

    loop {
        while k < bl as u32 {
            b |= (u64::from(INPTR < INSIZE) as u64 * u64::from(INBUF[INPTR as usize])) << k;
            k += 8;
            if INPTR >= INSIZE {
                OUTCNT = w;
                if fill_inbuf(false) != 0 {
                    return 1;
                }
            }
        }

        t = &tl[(b & u64::from(ml)) as usize];
        e = u32::from(t.e);

        if e > 16 {
            loop {
                if e == 99 {
                    return 1;
                }
                b >>= t.b as u64;
                k -= u32::from(t.b);
                e -= 16;

                while k < e {
                    b |= (u64::from(INPTR < INSIZE) as u64 * u64::from(INBUF[INPTR as usize])) << k;
                    k += 8;
                    if INPTR >= INSIZE {
                        OUTCNT = w;
                        if fill_inbuf(false) != 0 {
                            return 1;
                        }
                    }
                }

                t = &t.v.t[(b & u64::from(MASK_BITS[e as usize])) as usize];
                e = u32::from(t.e);

                if e <= 16 {
                    break;
                }
            }
            b >>= t.b as u64;
            k -= u32::from(t.b);
        }

        if e == 16 {
            WINDOW[w as usize] = t.v.n as u8;
            w += 1;
            if w == 0x8000 {
                OUTCNT = w;
                flush_window();
                w = 0;
            }
        } else {
            if e == 15 {
                break;
            }

            while k < e {
                b |= (u64::from(INPTR < INSIZE) as u64 * u64::from(INBUF[INPTR as usize])) << k;
                k += 8;
                if INPTR >= INSIZE {
                    OUTCNT = w;
                    if fill_inbuf(false) != 0 {
                        return 1;
                    }
                }
            }

            n = u32::from(t.v.n) + (b & u64::from(MASK_BITS[e as usize])) as u32;
            b >>= e;
            k -= e;

            while k < bd as u32 {
                b |= (u64::from(INPTR < INSIZE) as u64 * u64::from(INBUF[INPTR as usize])) << k;
                k += 8;
                if INPTR >= INSIZE {
                    OUTCNT = w;
                    if fill_inbuf(false) != 0 {
                        return 1;
                    }
                }
            }

            t = &td[(b & u64::from(md)) as usize];
            e = u32::from(t.e);

            if e > 16 {
                loop {
                    if e == 99 {
                        return 1;
                    }
                    b >>= t.b as u64;
                    k -= u32::from(t.b);
                    e -= 16;

                    while k < e {
                        b |= (u64::from(INPTR < INSIZE) as u64 * u64::from(INBUF[INPTR as usize])) << k;
                        k += 8;
                        if INPTR >= INSIZE {
                            OUTCNT = w;
                            if fill_inbuf(false) != 0 {
                                return 1;
                            }
                        }
                    }

                    t = &t.v.t[(b & u64::from(MASK_BITS[e as usize])) as usize];
                    e = u32::from(t.e);

                    if e <= 16 {
                        break;
                    }
                }
                b >>= t.b as u64;
                k -= u32::from(t.b);
            }

            while k < e {
                b |= (u64::from(INPTR < INSIZE) as u64 * u64::from(INBUF[INPTR as usize])) << k;
                k += 8;
                if INPTR >= INSIZE {
                    OUTCNT = w;
                    if fill_inbuf(false) != 0 {
                        return 1;
                    }
                }
            }

            d = w - u32::from(t.v.n) - (b & u64::from(MASK_BITS[e as usize])) as u32;
            b >>= e;
            k -= e;

            loop {
                let mut temp_e = 0x8000 - if d & 0x8000 > w { d & 0x8000 } else { w };
                let mut min_e = if temp_e > n { n } else { temp_e };
                n -= min_e;

                if min_e <= if d < w { w - d } else { d - w } {
                    WINDOW[w as usize..(w + min_e) as usize].copy_from_slice(&WINDOW[d as usize..(d + min_e) as usize]);
                    w += min_e;
                    d += min_e;
                } else {
                    for _ in 0..min_e {
                        WINDOW[w as usize] = WINDOW[d as usize];
                        w += 1;
                        d += 1;
                    }
                }

                if w == 0x8000 {
                    OUTCNT = w;
                    flush_window();
                    w = 0;
                }

                if n == 0 {
                    break;
                }
            }
        }
    }

    OUTCNT = w;
    unsafe { BB = b };
    unsafe { BK = k };
    0
}
fn inflate_dynamic() -> Result<(), ()> {
    let mut i: i32;
    let mut j: u32;
    let mut l: u32;
    let mut m: u32;
    let mut n: u32;
    let mut w: u32;
    let mut tl: *mut Huft;
    let mut td: *mut Huft;
    let mut bl: i32;
    let mut bd: i32;
    let mut nb: u32;
    let mut nl: u32;
    let mut nd: u32;
    let mut ll: [u32; 316] = [0; 316];
    let mut b: Ulg;
    let mut k: u32;
    unsafe {
        b = BB;
        k = BK as u32;
        w = OUTCNT;
        while k < 5 {
            b |= ((INPTR < INSIZE) as Ulg as Ulg) << k;
            k += 8;
        }
        nl = 257 + (b & 0x1f) as u32;
        b >>= 5;
        k -= 5;
        while k < 5 {
            b |= ((INPTR < INSIZE) as Ulg as Ulg) << k;
            k += 8;
        }
        nd = 1 + (b & 0x1f) as u32;
        b >>= 5;
        k -= 5;
        while k < 4 {
            b |= ((INPTR < INSIZE) as Ulg as Ulg) << k;
            k += 8;
        }
        nb = 4 + (b & 0xf) as u32;
        b >>= 4;
        k -= 4;
    }
    if nl > 286 || nd > 30 {
        return Err(());
    }
    for j in 0..nb {
        unsafe {
            while k < 3 {
                b |= ((INPTR < INSIZE) as Ulg as Ulg) << k;
                k += 8;
            }
            ll[BORDER[j] as usize] = (b & 7) as u32;
            b >>= 3;
            k -= 3;
        }
    }
    for j in nb..19 {
        ll[BORDER[j] as usize] = 0;
    }
    bl = 7;
    if let Err(err) = huft_build(&ll, 19, 19, std::ptr::null_mut(), std::ptr::null_mut(), &mut tl, &mut bl) {
        if err == 1 {
            huft_free(tl);
        }
        return Err(());
    }
    if tl.is_null() {
        return Err(());
    }
    n = nl + nd;
    m = MASK_BITS[bl as usize];
    let mut i = 0;
    let mut l = 0;
    while i < n {
        unsafe {
            while k < bl as u32 {
                b |= ((INPTR < INSIZE) as Ulg as Ulg) << k;
                k += 8;
            }
            j = (td = tl.offset((b & m) as isize)).as_ref().unwrap().b as u32;
            b >>= j;
            k -= j;
            if (*td).e == 99 {
                huft_free(tl);
                return Err(());
            }
            j = (*td).v.n as u32;
            if j < 16 {
                ll[i as usize] = l;
                i += 1;
                l = j;
            } else if j == 16 {
                while k < 2 {
                    b |= ((INPTR < INSIZE) as Ulg as Ulg) << k;
                    k += 8;
                }
                j = 3 + (b & 3) as u32;
                b >>= 2;
                k -= 2;
                if i + j > n {
                    return Err(());
                }
                for _ in 0..j {
                    ll[i as usize] = l;
                    i += 1;
                }
            } else if j == 17 {
                while k < 3 {
                    b |= ((INPTR < INSIZE) as Ulg as Ulg) << k;
                    k += 8;
                }
                j = 3 + (b & 7) as u32;
                b >>= 3;
                k -= 3;
                if i + j > n {
                    return Err(());
                }
                for _ in 0..j {
                    ll[i as usize] = 0;
                    i += 1;
                }
                l = 0;
            } else {
                while k < 7 {
                    b |= ((INPTR < INSIZE) as Ulg as Ulg) << k;
                    k += 8;
                }
                j = 11 + (b & 0x7f) as u32;
                b >>= 7;
                k -= 7;
                if i + j > n {
                    return Err(());
                }
                for _ in 0..j {
                    ll[i as usize] = 0;
                    i += 1;
                }
                l = 0;
            }
        }
    }
    huft_free(tl);
    unsafe {
        BB = b;
        BK = k as u32;
        bl = LBITS;
    }
    if let Err(err) = huft_build(&ll, nl, 257, &CPLENS, &CPLEXT, &mut tl, &mut bl) {
        if err == 1 {
            huft_free(tl);
        }
        return Err(());
    }
    bd = unsafe { DBITS };
    if let Err(err) = huft_build(&ll[nl as usize..], nd, 0, &CPDIST, &CPDEXT, &mut td, &mut bd) {
        if err == 1 {
            huft_free(td);
        }
        huft_free(tl);
        return Err(());
    }
    if inflate_codes(&std::slice::from_raw_parts(tl, nl as usize), &std::slice::from_raw_parts(td, nd as usize), bl as usize, bd as usize) != 0 {
        return Err(());
    }
    huft_free(tl);
    huft_free(td);
    Ok(())
}
fn inflate_fixed() -> Result<(), ()> {
    let mut i: usize = 0;
    let mut tl: *mut Huft = todo!();
    let mut td: *mut Huft = todo!();
    let mut bl: i32;
    let mut bd: i32;
    let mut l: [u32; 288] = [0; 288];

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

    bl = 7;
    if let Err(err) = huft_build(&l, 288, 257, &CPLENS, &CPLEXT, &mut tl, &mut bl) {
        return Err(err);
    }

    for i in 0..30 {
        l[i] = 5;
    }
    bd = 5;
    if let Err(err) = huft_build(&l, 30, 0, &CPDIST, &CPDEXT, &mut td, &mut bd) {
        unsafe {
            huft_free(&mut *tl);
        }
        return Err(err);
    }

    if inflate_codes(&[unsafe { *tl }], &[unsafe { *td }], bl as usize, bd as usize) != 0 {
        unsafe {
            huft_free(&mut *tl);
            huft_free(&mut *td);
        }
        return Err(());
    }

    unsafe {
        huft_free(&mut *tl);
        huft_free(&mut *td);
    }
    Ok(())
}
fn inflate_stored() -> i32 {
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
        unsafe {
            let inptr_lt_insize = if INPTR < INSIZE { 1 } else { 0 };
            let inbuf_val = INBUF.get(INPTR as usize).cloned().unwrap_or(0) as Ulg;
            b |= (inptr_lt_insize as Ulg as Ulg) * inbuf_val << k;
            INPTR += 1;
            if INPTR >= INSIZE {
                OUTCNT = w;
                fill_inbuf(false);
            }
        }
        k += 8;
    }
    n = b as u32 & 0xffff;
    b >>= 16;
    k -= 16;
    while k < 16 {
        unsafe {
            let inptr_lt_insize = if INPTR < INSIZE { 1 } else { 0 };
            let inbuf_val = INBUF.get(INPTR as usize).cloned().unwrap_or(0) as Ulg;
            b |= (inptr_lt_insize as Ulg as Ulg) * inbuf_val << k;
            INPTR += 1;
            if INPTR >= INSIZE {
                OUTCNT = w;
                fill_inbuf(false);
            }
        }
        k += 8;
    }
    if n != (!b as u32 & 0xffff) {
        return 1;
    }
    b >>= 16;
    k -= 16;
    while n > 0 {
        while k < 8 {
            unsafe {
                let inptr_lt_insize = if INPTR < INSIZE { 1 } else { 0 };
                let inbuf_val = INBUF.get(INPTR as usize).cloned().unwrap_or(0) as Ulg;
                b |= (inptr_lt_insize as Ulg as Ulg) * inbuf_val << k;
                INPTR += 1;
                if INPTR >= INSIZE {
                    OUTCNT = w;
                    fill_inbuf(false);
                }
            }
            k += 8;
        }
        unsafe {
            WINDOW[w as usize] = b as Uch;
        }
        w += 1;
        if w == 0x8000 {
            unsafe {
                OUTCNT = w;
                flush_window();
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
    return 0;
}
fn init_block() {
    unsafe {
        for n in 0..(256 + 1 + 29) {
            BL_TREE[n].fc.freq = 0;
        }
        for n in 0..30 {
            DYN_DTREE[n].fc.freq = 0;
        }
        for n in 0..19 {
            BL_TREE[n].fc.freq = 0;
        }
        DYN_LTREE[256].fc.freq = 1;
        OPT_LEN = 0;
        STATIC_LEN = 0;
        LAST_LIT = 0;
        LAST_DIST = 0;
        LAST_FLAGS = 0;
        FLAGS = 0;
        FLAG_BIT = 1;
    }
}
fn init_getbits() {
    use std::convert::TryInto;
    unsafe {
        crate::BITBUF = 0;
        crate::SUBBITBUF = 0;
        crate::BITCOUNT = 0;
        fillbuf(((8 * 2 * std::mem::size_of::<char>())).try_into().unwrap());
    }
}
fn input_eof() -> Result<(), ()> {
    unsafe {
        if crate::DECOMPRESS == 0 || crate::LAST_MEMBER != 0 {
            return Err(());
        }
        if crate::INPTR == crate::INSIZE {
            if crate::INSIZE != 0x40000 || crate::fill_inbuf(true) == -1 {
                return Err(());
            }
            crate::INPTR = 0;
        }
    }
    Ok(())
}
fn install_signal_handlers() {
    let nsigs = HANDLED_SIG.len();
    let mut act: libc::sigaction = unsafe { std::mem::zeroed() };

    unsafe {
        libc::sigemptyset(&mut CAUGHT_SIGNALS);

        for i in 0..nsigs {
            libc::sigaction(HANDLED_SIG[i], std::ptr::null(), &mut act);
            if act.sa_sigaction != libc::SIG_DFL {
                libc::sigaddset(&mut CAUGHT_SIGNALS, HANDLED_SIG[i]);
            }
        }

        act.sa_sigaction = Some(abort_gzip_signal as usize);
        act.sa_mask = CAUGHT_SIGNALS;
        act.sa_flags = 0;

        for i in 0..nsigs {
            if libc::sigismember(&CAUGHT_SIGNALS, HANDLED_SIG[i]) != 0 {
                if i == 0 {
                    unsafe {
                        FOREGROUND = 1;
                    }
                }
                libc::sigaction(HANDLED_SIG[i], &act, std::ptr::null_mut());
            }
        }
    }
}
fn irealloc(p: *mut std::ffi::c_void, s: u64) -> Option<*mut std::ffi::c_void> {
    use std::convert::TryInto;
    if s <= 18446744073709551615 {
        Some(realloc(p, (s | !s).try_into().unwrap()))
    } else {
        Some(unsafe { std::mem::transmute(_gl_alloc_nomem()) })
    }
}
fn ireallocarray(p: *mut std::ffi::c_void, n: IdxT, s: IdxT) -> Option<*mut std::ffi::c_void> {
    let (mut new_n, mut new_s) = (n, s);
    if n == 0 || s == 0 {
        new_n = 1;
        new_s = 1;
    }
    if new_n <= std::u64::MAX as IdxT && new_s <= std::u64::MAX as IdxT {
        Some(unsafe { libc::reallocarray(p, new_n, new_s) as *mut std::ffi::c_void })
    } else {
        None
    }
}
fn last_component(name: &str) -> Option<&str> {
    let mut base = name;
    let mut last_was_slash = false;

    while base.starts_with('/') {
        base = &base[1..];
    }

    for (i, c) in base.char_indices() {
        if c == '/' {
            last_was_slash = true;
        } else if last_was_slash {
            base = &base[i..];
            last_was_slash = false;
        }
    }

    Some(base)
}
fn license() {
    let mut p = LICENSE_MSG.iter();
    __printf__("%s %s\n\0".as_ptr() as *const i8, PROGRAM_NAME);
    __printf__("%s\n\0".as_ptr() as *const i8, VERSION);
    while let Some(msg) = p.next() {
        __printf__("%s\n\0".as_ptr() as *const i8, *msg);
    }
}
fn lm_init(pack_level: i32) {
    use std::convert::TryInto;
    if pack_level < 1 || pack_level > 9 {
        gzip_error("bad pack level");
    }
    let prev_offset = PREV.as_ptr() as usize + 0x8000;
    unsafe {
        std::ptr::write_bytes(prev_offset as *mut Ush, 0, (1 << 15) * std::mem::size_of::<Ush>());
        RSYNC_CHUNK_END = 0xFFFFFFFF;
        RSYNC_SUM = 0;
        MAX_LAZY_MATCH = CONFIGURATION_TABLE[pack_level as usize].max_lazy as u32;
        GOOD_MATCH = CONFIGURATION_TABLE[pack_level as usize].good_length as u32;
        NICE_MATCH = CONFIGURATION_TABLE[pack_level as usize].nice_length as i32;
        MAX_CHAIN_LENGTH = CONFIGURATION_TABLE[pack_level as usize].max_chain as u32;
        STRSTART = 0;
        BLOCK_START = 0;
        LOOKAHEAD = READ_BUF.unwrap_or(0).wrapping_add((WINDOW.as_ptr() as u32 + if std::mem::size_of::<i32>() <= 2 { 0x8000 } else { 2 * 0x8000 }).try_into().unwrap()) as u32;
        if LOOKAHEAD == 0 || LOOKAHEAD == std::u32::MAX {
            EOFILE = 1;
            LOOKAHEAD = 0;
            return;
        }
        EOFILE = 0;
        while LOOKAHEAD < 258 + 3 + 1 && EOFILE == 0 {
            fill_window();
        }
        INS_H = 0;
        for j in 0..3 - 1 {
            INS_H = ((INS_H << ((15 + 3 - 1) / 3)) ^ WINDOW[j] as u32) & ((1 << 15) - 1);
        }
    }
}
fn longest_match(cur_match: IPos) -> u32 {
    use std::convert::TryInto;
    let mut chain_length = unsafe { MAX_CHAIN_LENGTH };
    let mut scan = unsafe { WINDOW.as_ptr().offset(STRSTART as isize) };
    let mut match_ptr: *const Uch;
    let mut len: i32;
    let mut best_len = unsafe { PREV_LENGTH };
    let limit = if STRSTART > (0x8000 - (258 + 3 + 1) as u32) {
        STRSTART - (0x8000 - (258 + 3 + 1) as u32)
    } else {
        0
    };
    let strend = unsafe { WINDOW.as_ptr().offset((STRSTART + 258) as isize) };
    let scan_end1 = unsafe { *scan.offset(best_len as isize - 1) };
    let scan_end = unsafe { *scan.offset(best_len as isize) };

    if unsafe { PREV_LENGTH } >= unsafe { GOOD_MATCH } {
        chain_length >>= 2;
    }

    loop {
        match_ptr = unsafe { WINDOW.as_ptr().offset(cur_match as isize) };

        if unsafe { *match_ptr.offset(best_len as isize) } != scan_end ||
            unsafe { *match_ptr.offset(best_len as isize - 1) } != scan_end1 ||
            unsafe { *match_ptr } != *scan ||
            unsafe { *match_ptr.offset(1) } != *scan.offset(1) {
            continue;
        }

        scan = unsafe { scan.offset(2) };
        match_ptr = unsafe { match_ptr.offset(1) };

        while unsafe { *scan } == unsafe { *match_ptr } &&
              unsafe { *scan.offset(1) } == unsafe { *match_ptr.offset(1) } &&
              unsafe { *scan.offset(2) } == unsafe { *match_ptr.offset(2) } &&
              unsafe { *scan.offset(3) } == unsafe { *match_ptr.offset(3) } &&
              unsafe { *scan.offset(4) } == unsafe { *match_ptr.offset(4) } &&
              unsafe { *scan.offset(5) } == unsafe { *match_ptr.offset(5) } &&
              unsafe { *scan.offset(6) } == unsafe { *match_ptr.offset(6) } &&
              unsafe { *scan.offset(7) } == unsafe { *match_ptr.offset(7) } &&
              scan < strend {
            scan = unsafe { scan.offset(1) };
            match_ptr = unsafe { match_ptr.offset(1) };
        }

        len = 258 - (strend as usize - scan as usize) as i32;
        scan = unsafe { strend.offset(-258) };

        if len > best_len.try_into().unwrap() {
            unsafe {
                MATCH_START = cur_match;
            }
            best_len = len;
            if len >= unsafe { NICE_MATCH } {
                break;
            }
            scan_end1 = unsafe { *scan.offset(best_len as isize - 1) };
            scan_end = unsafe { *scan.offset(best_len as isize) };
        }

        cur_match = unsafe { PREV[(cur_match & (0x8000 - 1)) as usize].into() };
        if cur_match <= limit || chain_length == 0 {
            break;
        }
        chain_length -= 1;
    }

    best_len as u32
}
fn lutimens(file: &str, timespec: Option<[libc::timespec; 2]>) -> Result<i32, ()> {
    let mut adjusted_timespec: [libc::timespec; 2] = [libc::timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    let ts = if let Some(timespec) = timespec {
        adjusted_timespec[0] = timespec[0];
        adjusted_timespec[1] = timespec[1];
        validate_timespec(&adjusted_timespec)?;
        Some(&adjusted_timespec)
    } else {
        None
    };

    let adjustment_needed = if ts.is_some() { 1 } else { 0 };
    let mut st = libc::stat::default();

    if adjustment_needed < 0 {
        return Err(());
    }

    if 0 <= unsafe { LUTIMENSAT_WORKS_REALLY } {
        let result;
        if adjustment_needed == 2 {
            if unsafe { libc::lstat(file.as_ptr() as *const i8, &mut st) } != 0 {
                return Err(());
            }
            if ts.unwrap()[0].tv_nsec == ((1 << 30) - 2) {
                ts.unwrap()[0] = get_stat_atime(&st);
            } else if ts.unwrap()[1].tv_nsec == ((1 << 30) - 2) {
                ts.unwrap()[1] = get_stat_mtime(&st);
            }
            adjustment_needed += 1;
        }
        result = unsafe { libc::utimensat(-100, file.as_ptr() as *const i8, ts.unwrap() as *const libc::timespec, 0x100) };
        if result > 0 {
            unsafe { *libc::__errno_location() = 38 };
        }
        if result == 0 || unsafe { *libc::__errno_location() } != 38 {
            unsafe {
                UTIMENSAT_WORKS_REALLY = 1;
                LUTIMENSAT_WORKS_REALLY = 1;
            }
            return Ok(result);
        }
    }

    unsafe {
        LUTIMENSAT_WORKS_REALLY = -1;
    }

    if adjustment_needed != 0 {
        if adjustment_needed != 3 && unsafe { libc::lstat(file.as_ptr() as *const i8, &mut st) } != 0 {
            return Err(());
        }
        if let Some(ts) = ts {
            if update_timespec(&st, &mut (&mut ts[0] as *mut libc::timespec)) != Ok(()) {
                return Ok(0);
            }
        }
    }

    if adjustment_needed == 0 && unsafe { libc::lstat(file.as_ptr() as *const i8, &mut st) } != 0 {
        return Err(());
    }

    if !((st.st_mode & 0o170000) == 0o120000) {
        return Ok(fdutimens(-1, file, Some(*ts.unwrap())));
    }

    unsafe {
        *libc::__errno_location() = 38;
    }
    Err(())
}
fn make_ofname() -> Result<(), ()> {
    let mut suff: *const i8;
    unsafe {
        std::ptr::copy(IFNAME.as_ptr(), OFNAME.as_mut_ptr(), IFNAME.len());
    }
    suff = get_suffix(OFNAME.as_ptr() as *const i8);
    if unsafe { DECOMPRESS != 0 } {
        if suff.is_null() {
            if !unsafe { RECURSIVE == 0 } && unsafe { TEST != 0 } {
                return Ok(());
            }
            if unsafe { VERBOSE != 0 } || (!unsafe { RECURSIVE == 0 } && !unsafe { QUIET != 0 }) {
                rpl_fprintf(&mut std::io::stderr(), "%s: %s: unknown suffix -- ignored\n", unsafe { PROGRAM_NAME as usize }, unsafe { IFNAME.as_ptr() as *const i8 })?;
                if unsafe { EXIT_CODE == 0 } {
                    unsafe { EXIT_CODE = 2 };
                }
            }
            return Ok(());
        }
        let mut suff_str = unsafe { std::ffi::CStr::from_ptr(suff).to_string_lossy().into_owned() };
        strlwr(&mut suff_str);
        if suff_str == ".tgz" || suff_str == ".taz" {
            unsafe {
                std::ptr::write(suff as *mut i8, b".tar\0" as *const u8 as i8);
            }
        } else {
            unsafe {
                std::ptr::write(suff as *mut i8, b"\0" as *const u8 as i8);
            }
        }
    } else if !suff.is_null() && !unsafe { FORCE != 0 } {
        if unsafe { VERBOSE != 0 } || (!unsafe { RECURSIVE == 0 } && !unsafe { QUIET != 0 }) {
            rpl_fprintf(&mut std::io::stderr(), "%s: %s already has %s suffix -- unchanged\n", unsafe { PROGRAM_NAME as usize }, unsafe { IFNAME.as_ptr() as *const i8 }, unsafe { std::ffi::CStr::from_ptr(suff).to_string_lossy().as_ptr() as i8 })?;
        }
        return Ok(());
    } else {
        unsafe {
            SAVE_ORIG_NAME = 0;
            if OFNAME.len() <= std::ffi::CStr::from_ptr(OFNAME.as_ptr() as *const i8).to_bytes().len() + Z_LEN {
                return Err(());
            }
            let z_suffix_cstr = std::ffi::CStr::from_ptr(Z_SUFFIX);
            let z_suffix_bytes = z_suffix_cstr.to_bytes();
            std::ptr::copy(z_suffix_bytes.as_ptr(), OFNAME.as_mut_ptr().add(std::ffi::CStr::from_ptr(OFNAME.as_ptr() as *const i8).to_bytes().len()), z_suffix_bytes.len());
        }
    }
    Ok(())
}
fn make_simple_name(name: &mut [u8]){todo!("proto")}
fn make_table(nchar: i32, bitlen: &[u8], tablebits: i32, table: &mut [u16]) -> Result<(), ()> {
    let mut count = [0; 17];
    let mut weight = [0; 17];
    let mut start = [0; 18];
    let mut i: usize = 0;
    let mut k: usize = 0;
    let mut len: u16 = 0;
    let mut ch: usize = 0;
    let mut jutbits: i32 = 0;
    let mut avail: usize = 0;
    let mut nextcode: usize = 0;
    let mut mask: usize = 0;

    for i in 1..=16 {
        count[i] = 0;
    }

    for i in 0..nchar as usize {
        count[bitlen[i] as usize] += 1;
    }

    start[1] = 0;
    for i in 1..=16 {
        start[i + 1] = start[i] + (count[i] << (16 - i));
    }

    if start[17] & 0xffff != 0 {
        return Err(());
    }

    jutbits = 16 - tablebits;
    for i in 1..=tablebits as usize {
        start[i] >>= jutbits;
        weight[i] = 1 << (tablebits - i as i32);
    }

    while i <= 16 {
        weight[i] = 1 << (16 - i);
        i += 1;
    }

    i = start[tablebits as usize + 1] >> jutbits;
    if i != 0 {
        k = 1 << tablebits;
        while i != k {
            table[i] = 0;
            i += 1;
        }
    }

    avail = nchar as usize;
    mask = 1 << (15 - tablebits);
    for ch in 0..nchar as usize {
        len = bitlen[ch] as u16;
        if len == 0 {
            continue;
        }
        nextcode = start[len as usize] + weight[len as usize];
        if len <= tablebits as u16 {
            if 1 << tablebits < nextcode {
                return Err(());
            }
            for i in start[len as usize]..nextcode {
                table[i] = ch as u16;
            }
        } else {
            k = start[len as usize];
            let mut p = k >> jutbits;
            i = len as usize - tablebits as usize;
            while i != 0 {
                if table[p] == 0 {
                    table[0x8000 + avail] = 0;
                    table[avail] = 0;
                    table[p] = avail as u16;
                    avail += 1;
                }
                if k & mask != 0 {
                    p = 0x8000 + table[p] as usize;
                } else {
                    p = table[p] as usize;
                }
                k <<= 1;
                i -= 1;
            }
            table[p] = ch as u16;
        }
        start[len as usize] = nextcode;
    }

    Ok(())
}
fn make_timespec(s: std::os::unix::raw::time_t, ns: i64) -> Result<libc::timespec, ()> {
    Ok(libc::timespec {
        tv_sec: s,
        tv_nsec: ns,
    })
}
fn malloc(size: usize) -> *mut std::ffi::c_void{todo!("proto")}
fn mdir_name(file: &str) -> Result<String, ()> {
    let length = dir_len(file);
    let append_dot = length == 0 || (false && length == ((file.len(), 0).1) && file.as_bytes()[2] != b'\0' && !(file.as_bytes()[2] == b'/'));
    let mut dir = malloc(length + append_dot as usize + 1);
    if dir.is_null() {
        return Err(());
    }
    let dir_slice = unsafe { std::slice::from_raw_parts_mut(dir as *mut u8, length) };
    dir_slice.copy_from_slice(file.as_bytes());
    if append_dot {
        dir_slice[length] = b'.';
    }
    dir_slice[length + append_dot as usize] = b'\0';
    let dir_str = String::from_utf8(dir_slice.to_vec()).unwrap();
    std::mem::forget(dir);
    Ok(dir_str)
}
fn mfile_name_concat(dir: &str, base: &str) -> Option<String> {
    let dirbase = last_component(dir)?;
    let dirbaselen = base_len(dirbase);
    let dirlen = dirbase.as_ptr() as usize - dir.as_ptr() as usize + dirbaselen;
    let baselen = base.len();
    let sep = if dirbaselen > 0 {
        if dir.chars().nth(dirlen - 1) != Some('/') && base.chars().next() != Some('/') {
            '/'
        } else {
            '\0'
        }
    } else if base.chars().next() == Some('/') {
        '.'
    } else {
        '\0'
    };

    let mut concat_len = dirlen + if sep != '\0' { 1 } else { 0 } + baselen;
    let mut concat = String::with_capacity(concat_len);
    concat.push_str(&dir[..dirlen]);
    if sep != '\0' {
        concat.push(sep);
    }
    concat.push_str(base);

    Some(concat)
}
fn mmalloca(n: usize, sa_alignment_max: usize) -> Option<*mut std::ffi::c_void> {
    use std::convert::TryInto;
    let alignment2_mask = 2 * sa_alignment_max - 1;
    let plus = std::mem::size_of::<SmallT>() + alignment2_mask;
    let mut nplus: IdxT = 0;
    if !n.checked_add(plus as IdxT).map_or(true, |result| result.checked_mul(1).is_some()) {
        return None;
    }

    let mem = unsafe { std::mem::transmute::<*mut std::ffi::c_void, *mut u8>(malloc(nplus)) };
    if mem.is_null() {
        return None;
    }

    let umem = mem as usize;
    let umemplus = umem + std::mem::size_of::<SmallT>() + sa_alignment_max - 1;
    let offset = ((umemplus & !alignment2_mask) + sa_alignment_max - umem) as IdxT;
    let vp = unsafe { mem.add(offset) };
    let p = vp as *mut SmallT;
    unsafe {
        p.offset(-1).write(offset.try_into().unwrap());
    }

    Some(p as *mut std::ffi::c_void)
}
fn my_main(argc: i32, argv: *mut *mut std::os::raw::c_char) -> Result<(), ()> {
    use std::convert::TryInto;
    let mut file_count: i32;
    let mut proglen: usize;
    let mut argv_copy: *mut *mut std::os::raw::c_char;
    let mut env_argc: i32;
    let mut env_argv: *mut *mut std::os::raw::c_char;

    unsafe {
        PROGRAM_NAME = gzip_base_name(&mut *argv);
        proglen = libc::strlen(PROGRAM_NAME as *const i8);
        if 4 < proglen && libc::strcmp((PROGRAM_NAME as *const i8).add(proglen - 4), ".exe\0".as_ptr() as *const i8) == 0 {
            (PROGRAM_NAME as *mut u8).add(proglen - 4).write(0);
        }
        argv_copy = argv;
        ENV = add_envopt(&mut env_argc, &mut argv_copy, "GZIP");
        env_argv = if !ENV.is_null() { argv_copy } else { std::ptr::null_mut() };
        Z_SUFFIX = ".gz\0".as_ptr() as *const i8;
        Z_LEN = libc::strlen(Z_SUFFIX);
    }

    loop {
        let optc: i32;
        let mut longind = -1;

        unsafe {
            if !env_argv.is_null() {
                if !(*env_argv.add(optind)).is_null() && libc::strcmp(*env_argv.add(optind), "--\0".as_ptr() as *const i8) == 0 {
                    optc = ENV_OPTION + b'-';
                } else {
                    optc = libc::getopt_long(env_argc, env_argv, SHORTOPTS, LONGOPTS, &mut longind);
                    if optc >= 0 {
                        optc += ENV_OPTION;
                    } else {
                        if optind != env_argc {
                            rpl_fprintf(std::io::stderr, "%s: %s: non-option in \"GZIP\" environment variable\n\0", PROGRAM_NAME, *env_argv.add(optind));
                            try_help();
                        }
                        if env_argc != 1 && QUIET == 0 {
                            rpl_fprintf(std::io::stderr, "%s: warning: \"GZIP\" environment variable is deprecated; use an alias or script\n\0", PROGRAM_NAME);
                        }
                        rpl_free(env_argv as *mut std::ffi::c_void);
                        env_argv = std::ptr::null_mut();
                        optind = 1;
                        longind = -1;
                    }
                }
            }
            if env_argv.is_null() {
                optc = libc::getopt_long(argc, argv, SHORTOPTS, LONGOPTS, &mut longind);
            }

            if optc < 0 {
                break;
            }

            match optc {
                // Handle cases for each option
                // ...
                _ => {
                    if ENV_OPTION <= optc && optc != ENV_OPTION + b'?' {
                        rpl_fprintf(std::io::stderr, "%s: ", PROGRAM_NAME);
                        if longind < 0 {
                            rpl_fprintf(std::io::stderr, "-%c: ", (optc - ENV_OPTION).try_into().unwrap());
                        } else {
                            rpl_fprintf(std::io::stderr, "--%s: ", LONGOPTS[longind as usize].unwrap());
                        }
                        rpl_fprintf(std::io::stderr, "option not valid in \"GZIP\" environment variable\n\0");
                    }
                    try_help();
                }
            }
        }
    }

    // Perform remaining operations and return Result
    // ...
}
fn nonnull(p: *const std::os::raw::c_void) -> *const std::os::raw::c_void {
    if p.is_null() {
        xalloc_die();
    }
    p
}
fn npgettext_aux<'a>(domain: &'a str, msg_ctxt_id: &'a str, msgid: &'a str, msgid_plural: &'a str, n: u64, category: i32) -> Option<&'a str> {
    let translation = match n {
        1 => msg_ctxt_id,
        _ => msgid_plural,
    };

    if translation == msg_ctxt_id || translation == msgid_plural {
        Some(if n == 1 { msgid } else { msgid_plural })
    } else {
        Some(translation)
    }
}
fn open_and_stat(name: &str, flags: i32, st: &mut std::fs::Metadata) -> i32 {
    let mut fd: i32;
    let mut atfd = -100;
    let mut base = name;
    
    // Rest of the code remains the same
    
    if unsafe { KEEP == 0 } {
        if let Some(b) = last_component(name) {
            let f = atdir_set(name, b.as_ptr() as usize - name.as_ptr() as usize);
            if f >= 0 {
                base = b;
                atfd = f;
            }
        }
    }
    
    fd = openat_safer(atfd, base, unsafe { FLAGS.into() }, None).unwrap_or(-1);
    
    if fd >= 0 && fd != 0 {
        let mut st_temp = std::mem::MaybeUninit::uninit();
        if libc::fstat(fd, st_temp.as_mut_ptr()) != 0 {
            let e = unsafe { *libc::__errno_location() };
            libc::close(fd);
            unsafe { *libc::__errno_location() = e; }
            return -1;
        } else {
            let st_ptr = st_temp.assume_init();
            *st = st_ptr;
        }
    }
    
    fd
}
fn open_input_file(iname: *const i8, sbuf: &mut std::fs::Metadata) -> i32 {
    let ilen: i32;
    let mut z_suffix_errno = 0;
    static SUFFIXES: [&'static str; 6] = ["", ".gz", ".z", "-z", ".Z", ""];
    let mut suf = SUFFIXES.iter().filter_map(|&x| Some(x));
    let mut s: &str = "";
    let mut fd: i32;
    let open_flags = 0o00 | 0o4000 | 0o400
                      | if unsafe { ASCII } != 0 && unsafe { DECOMPRESS } == 0 { 0 } else { 0 };
    suf.next().unwrap();
    if (1024 - 1) <= unsafe { libc::strlen(iname) } {
        // goto_name_too_long(); // Fix: Handle the case where the name is too long
        return -1;
    }
    unsafe {
        libc::strcpy(IFNAME.as_mut_ptr() as *mut i8, iname);
    }
    fd = open_and_stat(std::str::from_utf8(&unsafe { std::slice::from_raw_parts(IFNAME.as_ptr(), 1024) }).unwrap(), open_flags, sbuf);
    if fd >= 0 {
        return fd;
    }
    if unsafe { DECOMPRESS } == 0 || unsafe { *libc::__errno_location() } != 2 {
        progerror(std::str::from_utf8(&unsafe { std::slice::from_raw_parts(IFNAME.as_ptr(), 1024) }).unwrap());
        return -1;
    }
    // Fix the following lines
    let suffix_ptr = get_suffix(IFNAME as *const i8);
    let suffix_slice = std::slice::from_raw_parts(suffix_ptr, 1024);
    s = std::str::from_utf8(suffix_slice).unwrap();
    
    if !s.is_empty() {
        progerror(std::str::from_utf8(&unsafe { std::slice::from_raw_parts(IFNAME.as_ptr(), 1024) }).unwrap());
        return -1;
    }
    ilen = unsafe { libc::strlen(IFNAME.as_ptr() as *const i8) as i32 };
    if unsafe { libc::strcmp(Z_SUFFIX, ".gz".as_ptr() as *const i8) } == 0 {
        suf.next();
    }
    loop {
        let s0 = s;
        unsafe {
            libc::strcpy(IFNAME.as_mut_ptr() as *mut i8, iname);
        }
        if 1024 <= ilen + s.len() as i32 {
            // goto_name_too_long(); // Fix: Handle the case where the name is too long
            return -1;
        }
        unsafe {
            libc::strcat(IFNAME.as_mut_ptr() as *mut i8, s.as_ptr() as *const i8);
        }
        fd = open_and_stat(std::str::from_utf8(&unsafe { std::slice::from_raw_parts(IFNAME.as_ptr(), 1024) }).unwrap(), open_flags, sbuf);
        if fd >= 0 {
            return fd;
        }
        if unsafe { *libc::__errno_location() } != 2 {
            progerror(std::str::from_utf8(&unsafe { std::slice::from_raw_parts(IFNAME.as_ptr(), 1024) }).unwrap());
            return -1;
        }
        if unsafe { libc::strcmp(s0.as_ptr() as *const i8, Z_SUFFIX) } == 0 {
            z_suffix_errno = unsafe { *libc::__errno_location() };
        }
        if let Some(suffix) = suf.next() {
            s = suffix;
        } else {
            break;
        }
    }
    unsafe {
        libc::strcpy(IFNAME.as_mut_ptr() as *mut i8, iname);
        libc::strcat(IFNAME.as_mut_ptr() as *mut i8, Z_SUFFIX);
        *libc::__errno_location() = z_suffix_errno;
    }
    progerror(std::str::from_utf8(&unsafe { std::slice::from_raw_parts(IFNAME.as_ptr(), 1024) }).unwrap());
    return -1;
}
fn open_safer(file: &str, flags: i32, mode: Option<std::os::unix::raw::mode_t>) -> Result<i32, ()> {
    let mut mode_val = 0;
    if unsafe { FLAGS } & 0o100 != 0 {
        if let Some(mode) = mode {
            mode_val = mode;
        }
    }
    fd_safer(unsafe { libc::open(file.as_ptr() as *const i8, unsafe { FLAGS.into() }, mode_val) })
}
fn openat_proc_name<'a>(buf: &'a mut [u8], fd: i32, file: &'a str) -> &'a str {
    use std::os::fd::AsRawFd;
    use std::os::unix::fs::OpenOptionsExt;
    
    let result: &mut [u8] = buf;
    let mut dirlen: usize;
    
    if file.is_empty() {
        buf[0] = b'\0';
        return std::str::from_utf8(&buf[..1]).unwrap();
    }
    
    {
        const PROC_SELF_FD_DIR_SIZE_BOUND: usize = "/proc/self/fd/%d/".len() + "%d".len() - 1
            + (((std::mem::size_of::<i32>() * 8) - (!0 < !-1) * 146 + 484) / 485) + (!0 < !-1) as usize;
        
        static mut PROC_STATUS: i32 = 0;
        
        if PROC_STATUS == 0 {
            let proc_self_fd = std::fs::File::open("/proc/self/fd").unwrap();
            let mut dotdot_buf = [0u8; PROC_SELF_FD_DIR_SIZE_BOUND + "../fd".len() - 1];
            let _ = std::io::Write::write_fmt(&mut dotdot_buf, format_args!("/proc/self/fd/{}/", proc_self_fd.as_raw_fd()));
            
            PROC_STATUS = if std::fs::metadata(std::str::from_utf8(&dotdot_buf).unwrap()).is_ok() { 1 } else { -1 };
        }
        
        if PROC_STATUS < 0 {
            return "";
        } else {
            let bufsize = PROC_SELF_FD_DIR_SIZE_BOUND + file.len();
            
            if (4096) < (4096 - 64) {
                result = &mut [0u8; bufsize];
            }
            
            dirlen = std::io::Write::write_fmt(&mut result, format_args!("/proc/self/fd/{}/", fd)).unwrap();
        }
    }
    
    std::io::Write::write_fmt(&mut result, format_args!("{}", file)).unwrap();
    
    std::str::from_utf8(&result[..dirlen + file.len()]).unwrap()
}
fn openat_restore_fail(errnum: i32) -> ! {
    error(unsafe { ::std::mem::transmute::<_, i32>(EXIT_FAILURE) }, errnum, "failed to return to initial working directory");
    ::std::process::abort();
}
fn openat_safer(fd: i32, file: &str, flags: i32, mode: Option<std::os::unix::raw::mode_t>) -> Result<i32, ()> {
    let mut mode_val = 0;
    if unsafe { FLAGS } & 0o100 != 0 {
        if let Some(m) = mode {
            mode_val = m;
        }
    }
    let file_ptr = file.as_ptr() as *const i8;
    fd_safer(unsafe { libc::openat(fd, file_ptr, unsafe { FLAGS.into() }, mode_val) })
}
fn openat_save_fail(errnum: i32) -> ! {
    error(unsafe { EXIT_FAILURE }, errnum, "unable to record current working directory");
    std::process::abort();
}
fn opendir(dir_name: &str) -> Option<*mut libc::DIR>{todo!("proto")}
fn opendir_safer(name: &str) -> *mut libc::DIR {
    if let Some(dp) = opendir(name) {
        let fd = unsafe { libc::dirfd(dp) };
        if 0 <= fd && fd <= 2 {
            let newdp: *mut libc::DIR;
            let e: i32;
            let f = rpl_fcntl(fd, 1030);
            if f < 0 {
                e = unsafe { *libc::__errno_location() };
                newdp = std::ptr::null_mut();
            } else {
                newdp = unsafe { fdopendir(f) };
                e = unsafe { *libc::__errno_location() };
                if newdp.is_null() {
                    unsafe { libc::close(f) };
                }
            }
            unsafe { libc::closedir(dp) };
            unsafe { *libc::__errno_location() = e };
            return newdp;
        }
    }
    std::ptr::null_mut()
}
fn pgettext_aux(domain: &str, msg_ctxt_id: &str, msgid: &str, category: i32) -> String {
    let translation = {
        let _ = category;
        let _ = domain;
        msg_ctxt_id
    };

    if translation == msg_ctxt_id {
        msgid.to_string()
    } else {
        translation.to_string()
    }
}
fn pipe_safer(fd: &mut [i32; 2]) -> Result<(), ()> {
    if unsafe { libc::pipe(fd.as_mut_ptr()) } == 0 {
        for i in 0..2 {
            fd[i] = fd_safer(fd[i])?;
            if fd[i] < 0 {
                let saved_errno = unsafe { *libc::__errno_location() };
                unsafe { libc::close(fd[1 - i]) };
                unsafe { *libc::__errno_location() = saved_errno };
                return Err(());
            }
        }
        return Ok(());
    }
    Err(())
}
fn pqdownheap(tree: &mut [CtData], mut k: i32) {
    let mut v = unsafe { HEAP[k as usize] };
    let mut j = k << 1;
    while j <= unsafe { HEAP_LEN } {
        unsafe {
            if j < HEAP_LEN && (tree[HEAP[j as usize + 1] as usize].fc.freq < tree[HEAP[j as usize] as usize].fc.freq || (tree[HEAP[j as usize + 1] as usize].fc.freq == tree[HEAP[j as usize] as usize].fc.freq && DEPTH[HEAP[j as usize + 1] as usize] <= DEPTH[HEAP[j as usize] as usize])) {
                j += 1;
            }
            if (tree[v as usize].fc.freq < tree[HEAP[j as usize] as usize].fc.freq || (tree[v as usize].fc.freq == tree[HEAP[j as usize] as usize].fc.freq && DEPTH[v as usize] <= DEPTH[HEAP[j as usize] as usize])) {
                break;
            }
            HEAP[k as usize] = HEAP[j as usize];
        }
        k = j;
        j <<= 1;
    }
    unsafe { HEAP[k as usize] = v; }
}
fn printf_fetchargs(args: usize, a: *mut Arguments) -> i32 {
    let mut i: usize = 0;
    let mut ap: *mut Argument = &mut (*a).arg[0];

    while i < (*a).count {
        match unsafe { (*ap).type_ } {
            ArgType::TYPE_SCHAR => unsafe { (*ap).a.a_schar = args as i8 },
            ArgType::TYPE_UCHAR => unsafe { (*ap).a.a_uchar = args as u8 },
            ArgType::TYPE_SHORT => unsafe { (*ap).a.a_short = args as i16 },
            ArgType::TYPE_USHORT => unsafe { (*ap).a.a_ushort = args as u16 },
            ArgType::TYPE_INT => unsafe { (*ap).a.a_int = args as i32 },
            ArgType::TYPE_UINT => unsafe { (*ap).a.a_uint = args as u32 },
            ArgType::TYPE_LONGINT => unsafe { (*ap).a.a_longint = args as i64 },
            ArgType::TYPE_ULONGINT => unsafe { (*ap).a.a_ulongint = args as u64 },
            ArgType::TYPE_LONGLONGINT => unsafe { (*ap).a.a_longlongint = args as i64 },
            ArgType::TYPE_ULONGLONGINT => unsafe { (*ap).a.a_ulonglongint = args as u64 },
            ArgType::TYPE_DOUBLE => unsafe { (*ap).a.a_double = args as f64 },
            ArgType::TYPE_LONGDOUBLE => unsafe { (*ap).a.a_longdouble = args as f64 },
            ArgType::TYPE_CHAR => unsafe { (*ap).a.a_char = args as i32 },
            ArgType::TYPE_WIDE_CHAR => unsafe {
                (*ap).a.a_wide_char = if std::mem::size_of::<i32>() < std::mem::size_of::<usize>() {
                    args as i32
                } else {
                    args as i32
                }
            },
            ArgType::TYPE_STRING => unsafe {
                (*ap).a.a_string = args as *const std::os::raw::c_char;
                if (*ap).a.a_string.is_null() {
                    (*ap).a.a_string = "(NULL)".as_ptr() as *const std::os::raw::c_char;
                }
            },
            ArgType::TYPE_WIDE_STRING => unsafe {
                (*ap).a.a_wide_string = args as *const i32;
                if (*ap).a.a_wide_string.is_null() {
                    static WIDE_NULL_STRING: [i32; 7] = [
                        '(' as i32,
                        'N' as i32, 'U' as i32, 'L' as i32, 'L' as i32,
                        ')' as i32,
                        0,
                    ];
                    (*ap).a.a_wide_string = WIDE_NULL_STRING.as_ptr();
                }
            },
            ArgType::TYPE_POINTER => unsafe { (*ap).a.a_pointer = args as *const std::os::raw::c_void },
            ArgType::TYPE_COUNT_SCHAR_POINTER => unsafe { (*ap).a.a_count_schar_pointer = args as *mut std::os::raw::c_char },
            ArgType::TYPE_COUNT_SHORT_POINTER => unsafe { (*ap).a.a_count_short_pointer = args as *mut std::os::raw::c_short },
            ArgType::TYPE_COUNT_INT_POINTER => unsafe { (*ap).a.a_count_int_pointer = args as *mut std::os::raw::c_int },
            ArgType::TYPE_COUNT_LONGINT_POINTER => unsafe { (*ap).a.a_count_longint_pointer = args as *mut std::os::raw::c_long },
            ArgType::TYPE_COUNT_LONGLONGINT_POINTER => unsafe { (*ap).a.a_count_longlongint_pointer = args as *mut i64 },
            _ => return -1,
        }
        i += 1;
        ap = unsafe { ap.offset(1) };
    }
    0
}
fn printf_frexp(x: f64, expptr: &mut i32) -> f64 {
    let mut exponent: i32;

    x = x.exp(&mut exponent);
    x += x;
    exponent -= 1;
    if exponent < (-1021) - 1 {
        x = x.exp(exponent - ((-1021) - 1));
        exponent = (-1021) - 1;
    }
    *expptr = exponent;
    x
}
fn printf_frexpl(mut x: f64, expptr: &mut i32) -> f64 {
    let mut exponent: i32 = 0;

    x = x.exp();
    x = x + x;
    exponent -= 1;
    if exponent < (-16381) - 1 {
        x = x.exp();
        exponent = (-16381) - 1;
    }
    *expptr = exponent;
    x
}
fn printf_parse(format: &str, d: &mut CharDirectives, a: &mut Arguments) -> Result<(), ()> {
    let mut cp = format;
    let mut arg_posn = 0;
    let mut d_allocated;
    let mut a_allocated;
    let mut max_width_length = 0;
    let mut max_precision_length = 0;
    d.count = 0;
    d_allocated = 7;
    d.dir = (&mut d.direct_alloc_dir).to_vec();
    a.count = 0;
    a_allocated = 7;
    a.arg = &mut a.direct_alloc_arg;
    while let Some(c) = cp.chars().next() {
        cp = &cp[c.len_utf8()..];
        if c == '%' {
            let mut arg_index = usize::MAX;
            let dp = &mut d.dir[d.count];
            dp.dir_start = Some(cp);
            dp.flags = 0;
            dp.width_start = None;
            dp.width_end = None;
            dp.width_arg_index = usize::MAX;
            dp.precision_start = None;
            dp.precision_end = None;
            dp.precision_arg_index = usize::MAX;
            dp.arg_index = usize::MAX;
            if let Some(np) = cp.find(|ch: char| ch >= '0' && ch <= '9') {
                let mut np = np;
                while let Some(ch) = cp.chars().nth(np) {
                    if ch >= '0' && ch <= '9' {
                        np += 1;
                    } else {
                        break;
                    }
                }
                if cp.chars().nth(np) == Some('$') {
                    let mut n = 0;
                    for ch in cp.chars().take(np) {
                        n = xsum(n * 10, ch.to_digit(10).unwrap() as usize).ok_or(())?;
                    }
                    if n == 0 || n == usize::MAX {
                        return Err(());
                    }
                    arg_index = n - 1;
                    cp = &cp[np + 1..];
                }
            }
            loop {
                match cp.chars().next() {
                    Some('\'') => {
                        dp.flags |= 1;
                        cp = &cp[1..];
                    }
                    Some('-') => {
                        dp.flags |= 2;
                        cp = &cp[1..];
                    }
                    Some('+') => {
                        dp.flags |= 4;
                        cp = &cp[1..];
                    }
                    Some(' ') => {
                        dp.flags |= 8;
                        cp = &cp[1..];
                    }
                    Some('#') => {
                        dp.flags |= 16;
                        cp = &cp[1..];
                    }
                    Some('0') => {
                        dp.flags |= 32;
                        cp = &cp[1..];
                    }
                    Some('I') => {
                        dp.flags |= 64;
                        cp = &cp[1..];
                    }
                    _ => break,
                }
            }
            if let Some('*') = cp.chars().next() {
                dp.width_start = Some(cp);
                cp = &cp[1..];
                dp.width_end = Some(cp);
                if max_width_length < 1 {
                    max_width_length = 1;
                }
                if let Some(np) = cp.find(|ch: char| ch >= '0' && ch <= '9') {
                    let mut np = np;
                    while let Some(ch) = cp.chars().nth(np) {
                        if ch >= '0' && ch <= '9' {
                            np += 1;
                        } else {
                            break;
                        }
                    }
                    if cp.chars().nth(np) == Some('$') {
                        let mut n = 0;
                        for ch in cp.chars().take(np) {
                            n = xsum(n * 10, ch.to_digit(10).unwrap() as usize).ok_or(())?;
                        }
                        if n == 0 || n == usize::MAX {
                            return Err(());
                        }
                        dp.width_arg_index = n - 1;
                        cp = &cp[np + 1..];
                    }
                }
                if dp.width_arg_index == usize::MAX {
                    dp.width_arg_index = arg_posn;
                    arg_posn = arg_posn.checked_add(1).ok_or(())?;
                }
                let n = dp.width_arg_index;
                if n >= a_allocated {
                    a_allocated = a_allocated.checked_mul(2).ok_or(())?;
                    if a_allocated <= n {
                        a_allocated = n.checked_add(1).ok_or(())?;
                    }
                    let memory_size = a_allocated.checked_mul(std::mem::size_of::<Argument>()).ok_or(())?;
                    let memory = if a.arg != a.direct_alloc_arg {
                        realloc(a.arg, memory_size).ok_or(())?
                    } else {
                        malloc(memory_size).ok_or(())?
                    };
                    if a.arg == a.direct_alloc_arg {
                        a.arg.copy_from_slice(&a.arg[..a.count]);
                    }
                    a.arg = memory;
                }
                while a.count <= n {
                    a.arg[a.count].type_ = ArgType::TYPE_NONE;
                    a.count += 1;
                }
                if a.arg[n].type_ == ArgType::TYPE_NONE {
                    a.arg[n].type_ = ArgType::TYPE_INT;
                } else if a.arg[n].type_ != ArgType::TYPE_INT {
                    return Err(());
                }
            } else if let Some(np) = cp.find(|ch: char| ch >= '0' && ch <= '9') {
                let mut width_length = np;
                dp.width_start = Some(cp);
                cp = &cp[np..];
                while let Some(ch) = cp.chars().next() {
                    if ch >= '0' && ch <= '9' {
                        cp = &cp[ch.len_utf8()..];
                    } else {
                        break;
                    }
                }
                dp.width_end = Some(cp);
                width_length = dp.width_end.unwrap().len() - dp.width_start.unwrap().len();
                if max_width_length < width_length {
                    max_width_length = width_length;
                }
            }
            if let Some('.') = cp.chars().next() {
                cp = &cp[1..];
                if let Some('*') = cp.chars().next() {
                    dp.precision_start = Some(cp);
                    cp = &cp[1..];
                    dp.precision_end = Some(cp);
                    if max_precision_length < 2 {
                        max_precision_length = 2;
                    }
                    if let Some(np) = cp.find(|ch: char| ch >= '0' && ch <= '9') {
                        let mut n = 0;
                        for ch in cp.chars().take(np) {
                            n = xsum(n * 10, ch.to_digit(10).unwrap() as usize).ok_or(())?;
                        }
                        if n == 0 || n == usize::MAX {
                            return Err(());
                        }
                        dp.precision_arg_index = n - 1;
                        cp = &cp[np + 1..];
                    }
                    if dp.precision_arg_index == usize::MAX {
                        dp.precision_arg_index = arg_posn;
                        arg_posn = arg_posn.checked_add(1).ok_or(())?;
                    }
                    let n = dp.precision_arg_index;
                    if n >= a_allocated {
                        a_allocated = a_allocated.checked_mul(2).ok_or(())?;
                        if a_allocated <= n {
                            a_allocated = n.checked_add(1).ok_or(())?;
                        }
                        let memory_size = a_allocated.checked_mul(std::mem::size_of::<Argument>()).ok_or(())?;
                        let memory = if a.arg != a.direct_alloc_arg {
                            realloc(a.arg, memory_size).ok_or(())?
                        } else {
                            malloc(memory_size).ok_or(())?
                        };
                        if a.arg == a.direct_alloc_arg {
                            a.arg.copy_from_slice(&a.arg[..a.count]);
                        }
                        a.arg = memory;
                    }
                    while a.count <= n {
                        a.arg[a.count].type_ = ArgType::TYPE_NONE;
                        a.count += 1;
                    }
                    if a.arg[n].type_ == ArgType::TYPE_NONE {
                        a.arg[n].type_ = ArgType::TYPE_INT;
                    } else if a.arg[n].type_ != ArgType::TYPE_INT {
                        return Err(());
                    }
                } else {
                    let mut precision_length = 0;
                    dp.precision_start = Some(cp);
                    while let Some(ch) = cp.chars().next() {
                        if ch >= '0' && ch <= '9' {
                            cp = &cp[ch.len_utf8()..];
                        } else {
                            break;
                        }
                    }
                    dp.precision_end = Some(cp);
                    precision_length = dp.precision_end.unwrap().len() - dp.precision_start.unwrap().len();
                    if max_precision_length < precision_length {
                        max_precision_length = precision_length;
                    }
                }
            }
            let r#type = {
                let mut flags = 0;
                loop {
                    match cp.chars().next() {
                        Some('h') => {
                            flags |= 1 << (flags & 1);
                            cp = &cp[1..];
                        }
                        Some('L') => {
                            flags |= 4;
                            cp = &cp[1..];
                        }
                        Some('l') => {
                            flags += 8;
                            cp = &cp[1..];
                        }
                        Some('j') => {
                            if std::mem::size_of::<i64>() > std::mem::size_of::<usize>() {
                                flags += 16;
                            } else if std::mem::size_of::<i64>() > std::mem::size_of::<i32>() {
                                flags += 8;
                            }
                            cp = &cp[1..];
                        }
                        Some('z') | Some('Z') => {
                            if std::mem::size_of::<usize>() > std::mem::size_of::<usize>() {
                                flags += 16;
                            } else if std::mem::size_of::<usize>() > std::mem::size_of::<i32>() {
                                flags += 8;
                            }
                            cp = &cp[1..];
                        }
                        Some('t') => {
                            if std::mem::size_of::<libc::ptrdiff_t>() > std::mem::size_of::<usize>() {
                                flags += 16;
                            } else if std::mem::size_of::<libc::ptrdiff_t>() > std::mem::size_of::<i32>() {
                                flags += 8;
                            }
                            cp = &cp[1..];
                        }
                        _ => break,
                    }
                }
                let c = cp.chars().next().ok_or(())?;
                cp = &cp[c.len_utf8()..];
                match c {
                    'd' | 'i' => {
                        if flags >= 16 || (flags & 4 != 0) {
                            ArgType::TYPE_LONGLONGINT
                        } else if flags >= 8 {
                            ArgType::TYPE_LONGINT
                        } else if flags & 2 != 0 {
                            ArgType::TYPE_SCHAR
                        } else if flags & 1 != 0 {
                            ArgType::TYPE_SHORT
                        } else {
                            ArgType::TYPE_INT
                        }
                    }
                    'o' | 'u' | 'x' | 'X' => {
                        if flags >= 16 || (flags & 4 != 0) {
                            ArgType::TYPE_ULONGLONGINT
                        } else if flags >= 8 {
                            ArgType::TYPE_ULONGINT
                        } else if flags & 2 != 0 {
                            ArgType::TYPE_UCHAR
                        } else if flags & 1 != 0 {
                            ArgType::TYPE_USHORT
                        } else {
                            ArgType::TYPE_UINT
                        }
                    }
                    'f' | 'F' | 'e' | 'E' | 'g' | 'G' | 'a' | 'A' => {
                        if flags >= 16 || (flags & 4 != 0) {
                            ArgType::TYPE_LONGDOUBLE
                        } else {
                            ArgType::TYPE_DOUBLE
                        }
                    }
                    'c' => {
                        if flags >= 8 {
                            ArgType::TYPE_WIDE_CHAR
                        } else {
                            ArgType::TYPE_CHAR
                        }
                    }
                    'C' => {
                        ArgType::TYPE_WIDE_CHAR
                    }
                    's' => {
                        if flags >= 8 {
                            ArgType::TYPE_WIDE_STRING
                        } else {
                            ArgType::TYPE_STRING
                        }
                    }
                    'S' => {
                        ArgType::TYPE_WIDE_STRING
                    }
                    'p' => ArgType::TYPE_POINTER,
                    'n' => {
                        if flags >= 16 || (flags & 4 != 0) {
                            ArgType::TYPE_COUNT_LONGLONGINT_POINTER
                        } else if flags >= 8 {
                            ArgType::TYPE_COUNT_LONGINT_POINTER
                        } else if flags & 2 != 0 {
                            ArgType::TYPE_COUNT_SCHAR_POINTER
                        } else if flags & 1 != 0 {
                            ArgType::TYPE_COUNT_SHORT_POINTER
                        } else {
                            ArgType::TYPE_COUNT_INT_POINTER
                        }
                    }
                    '%' => ArgType::TYPE_NONE,
                    _ => return Err(()),
                }
            };
            if type != ArgType::TYPE_NONE {
                dp.arg_index = arg_index;
                if dp.arg_index == usize::MAX {
                    dp.arg_index = arg_posn;
                    arg_posn = arg_posn.checked_add(1).ok_or(())?;
                }
                let n = dp.arg_index;
                if n >= a_allocated {
                    a_allocated = a_allocated.checked_mul(2).ok_or(())?;
                    if a_allocated <= n {
                        a_allocated = n.checked_add(1).ok_or(())?;
                    }
                    let memory_size = a_allocated.checked_mul(std::mem::size_of::<Argument>()).ok_or(())?;
                    let memory = if a.arg != a.direct_alloc_arg {
                        realloc(a.arg, memory_size).ok_or(())?
                    } else {
                        malloc(memory_size).ok_or(())?
                    };
                    if a.arg == a.direct_alloc_arg {
                        a.arg.copy_from_slice(&a.arg[..a.count]);
                    }
                    a.arg = memory;
                }
                while a.count <= n {
                    a.arg[a.count].type_ = ArgType::TYPE_NONE;
                    a.count += 1;
                }
                if a.arg[n].type_ == ArgType::TYPE_NONE {
                    a.arg[n].type_ = type;
                } else if a.arg[n].type_ != type {
                    return Err(());
                }
            }
            dp.conversion = c;
            dp.dir_end = cp;
            d.count += 1;
            if d.count >= d_allocated {
                d_allocated = d_allocated.checked_mul(2).ok_or(())?;
                let memory_size = d_allocated.checked_mul(std::mem::size_of::<CharDirective>()).ok_or(())?;
                let memory = if d.dir != d.direct_alloc_dir {
                    realloc(d.dir, memory_size).ok_or(())?
                } else {
                    malloc(memory_size).ok_or(())?
                };
                if d.dir == d.direct_alloc_dir {
                    d.dir.copy_from_slice(&d.dir[..d.count]);
                }
                d.dir = memory;
            }
        }
    }
    d.dir[d.count].dir_start = Some(cp);
    d.max_width_length = max_width_length;
    d.max_precision_length = max_precision_length;
    Ok(())
}
fn progerror(string: &str) {
    let e = unsafe { *libc::__errno_location() };
    let stderr = std::io::stderr();
    let _ = rpl_fprintf(&mut stderr.lock(), &format!("{}: ", unsafe { std::ffi::CStr::from_ptr(PROGRAM_NAME).to_str().unwrap() }), 0);
    unsafe { *libc::__errno_location() = e };
    libc::perror(string);
    unsafe { EXIT_CODE = 1 };
}
fn read_buffer(fd: i32, buf: Voidp, cnt: u32) -> i32 {
    use std::convert::TryInto;
    let mut cnt = cnt;
    if 0x7fffffff < cnt {
        cnt = 0x7fffffff;
    }
    let mut len = unsafe { libc::read(fd, buf as *mut std::ffi::c_void, cnt as usize) };
    if len < 0 && unsafe { *libc::__errno_location() } == 11 {
        let flags = rpl_fcntl(fd, 3);
        if 0 <= flags {
            if (flags & 04000) == 0 {
                unsafe { *libc::__errno_location() = 11 };
            } else if rpl_fcntl(fd, 4, flags & !04000) != -1 {
                len = unsafe { libc::read(fd, buf as *mut std::ffi::c_void, cnt as usize) };
            }
        }
    }
    len.try_into().unwrap()
}
fn read_byte() -> Result<u8, ()> {
    use std::convert::TryInto;

    let b = if unsafe { INPTR < INSIZE } {
        let byte = unsafe { INBUF[INPTR as usize] };
        unsafe { INPTR += 1 };
        byte
    } else {
        fill_inbuf(false).try_into().unwrap()
    };

    if b < 0 {
        gzip_error("invalid compressed data -- unexpected end of file");
    }

    Ok(b)
}
fn read_c_len() -> Result<(), ()> {
    let mut i: i32;
    let mut c: i32;
    let mut n: i32;
    let mut mask: u64;

    n = getbits(9)? as i32;
    if n == 0 {
        c = getbits(9)? as i32;
        for i in 0..(255 + 256 + 2 - 3) {
            unsafe { OUTBUF[i as usize] = 0; }
        }
        for i in 0..4096 {
            unsafe { D_BUF[i as usize] = c as u16; }
        }
    } else {
        i = 0;
        while i < n {
            c = PT_TABLE[(unsafe { BITBUF >> ((8 * 2 * std::mem::size_of::<u8>()) - 8) }) as usize] as i32;
            if c >= (16 + 3) {
                mask = 1 << ((8 * 2 * std::mem::size_of::<u8>()) - 1 - 8);
                loop {
                    if BITBUF & mask != 0 {
                        c = unsafe { PREV[0x8000 + c] as i32 };
                    } else {
                        c = unsafe { PREV[c as usize] as i32 };
                    }
                    mask >>= 1;
                    if c < (16 + 3) {
                        break;
                    }
                }
            }
            fillbuf(PT_LEN[c as usize] as i32);
            if c <= 2 {
                if c == 0 {
                    c = 1;
                } else if c == 1 {
                    c = getbits(4)? as i32 + 3;
                } else {
                    c = getbits(9)? as i32 + 20;
                }
                while c > 0 {
                    unsafe { OUTBUF[i as usize] = 0; }
                    i += 1;
                    c -= 1;
                }
            } else {
                unsafe { OUTBUF[i as usize] = (c - 2) as u8; }
                i += 1;
            }
        }
        while i < (255 + 256 + 2 - 3) {
            unsafe { OUTBUF[i as usize] = 0; }
            i += 1;
        }
        make_table((255 + 256 + 2 - 3), &OUTBUF, 12, &mut D_BUF)?;
    }

    Ok(())
}
fn read_error() -> ! {
    let e = unsafe { *libc::__errno_location() };
    rpl_fprintf(&mut std::io::stderr(), "\n{}: ", unsafe { std::ffi::CStr::from_ptr(PROGRAM_NAME).to_str().unwrap() }).unwrap();
    if e != 0 {
        unsafe { *libc::__errno_location() = e };
        unsafe { libc::perror(IFNAME.as_ptr() as *const i8) };
    } else {
        rpl_fprintf(&mut std::io::stderr(), "{}: unexpected end of file\n", unsafe { std::ffi::CStr::from_ptr(IFNAME.as_ptr() as *const i8).to_str().unwrap() }).unwrap();
    }
    abort_gzip();
}
fn read_pt_len(nn: i32, nbit: i32, i_special: i32, mut pt_len: &mut [u8], mut pt_table: &mut [u16]) -> Result<(), ()> {
    let mut i: i32;
    let mut c: i32;
    let mut n: i32;
    let mut mask: u64;

    n = match getbits(nbit) {
        Ok(val) => val as i32,
        Err(_) => return Err(()),
    };

    if n == 0 {
        c = match getbits(nbit) {
            Ok(val) => val as i32,
            Err(_) => return Err(()),
        };

        for i in 0..nn {
            pt_len[i as usize] = 0;
        }

        for i in 0..256 {
            pt_table[i] = c as u16;
        }
    } else {
        i = 0;
        while i < n {
            c = (unsafe { BITBUF >> ((8 * 2 * std::mem::size_of::<u8>()) - 3) }) as i32;

            if c == 7 {
                mask = 1 << ((8 * 2 * std::mem::size_of::<u8>()) - 1 - 3);
                while mask & unsafe { BITBUF } != 0 {
                    mask >>= 1;
                    c += 1;
                }

                if c > 16 {
                    gzip_error("Bad table\n");
                }
            }

            let fill_amount = if c < 7 { 3 } else { c - 3 };
            fillbuf(fill_amount);

            pt_len[i as usize] = c as u8;

            i += 1;

            if i == i_special {
                c = match getbits(2) {
                    Ok(val) => val as i32,
                    Err(_) => return Err(()),
                };

                while c > 0 {
                    pt_len[i as usize] = 0;
                    i += 1;
                    c -= 1;
                }
            }
        }

        while i < nn {
            pt_len[i as usize] = 0;
            i += 1;
        }

        match make_table(nn, &pt_len, 8, &mut pt_table) {
            Ok(_) => (),
            Err(_) => return Err(()),
        };
    }

    Ok(())
}
fn read_tree() {
    let mut current_len: i32;
    let mut current_base: i32;
    let mut n: i32;
    let mut max_leaves = 1;
    let mut orig_len: i32 = 0;
    for _ in 1..=4 {
        orig_len = (orig_len << 8) | read_byte().unwrap() as i32;
    }
    let max_len: i32 = read_byte().unwrap() as i32;
    if !(0 < max_len && max_len <= 25) {
        gzip_error("invalid compressed data -- Huffman code bit length out of range");
    }
    n = 0;
    for current_len in 1..=max_len {
        LEAVES[current_len as usize] = read_byte().unwrap() as i32;
        if max_leaves - (current_len == max_len) < LEAVES[current_len as usize] {
            gzip_error("too many leaves in Huffman tree");
        }
        max_leaves = (max_leaves - LEAVES[current_len as usize] + 1) * 2 - 1;
        n += LEAVES[current_len as usize];
    }
    if 256 <= n {
        gzip_error("too many leaves in Huffman tree");
    }
    LEAVES[max_len as usize] += 1;
    current_base = 0;
    for current_len in 1..=max_len {
        LIT_BASE[current_len as usize] = current_base;
        for _ in 0..LEAVES[current_len as usize] {
            LITERAL[current_base as usize] = read_byte().unwrap();
            current_base += 1;
        }
    }
    LEAVES[max_len as usize] += 1;
}
fn realloc(ptr: *mut std::ffi::c_void, size: usize) -> *mut std::ffi::c_void{todo!("proto")}
fn remove_output_file(signals_already_blocked: bool) -> Result<(), ()> {
    let fd = unsafe { REMOVE_OFNAME_FD };
    if fd >= 0 {
        let mut fname = [0; 1024];
        unsafe {
            REMOVE_OFNAME_FD = -1;
            libc::close(fd);
            volatile_strcpy(fname.as_mut_ptr() as *mut i8, REMOVE_OFNAME.as_ptr() as *const i8);
            if let Err(_) = xunlink(std::str::from_utf8(&fname).unwrap()) {
                return Err(());
            }
        }
    }

    if !signals_already_blocked {
        let mut oldset = std::mem::MaybeUninit::uninit();
        unsafe {
            libc::sigprocmask(0, &CAUGHT_SIGNALS as *const libc::sigset_t, oldset.as_mut_ptr());
        }
        unsafe {
            libc::sigprocmask(2, oldset.as_ptr(), std::ptr::null_mut());
        }
    }

    Ok(())
}
fn restore_cwd(cwd: &SavedCwd) -> i32 {
    if 0 <= cwd.desc {
        unsafe {
            return libc::fchdir(cwd.desc);
        }
    } else {
        return chdir_long(cwd.name.as_ref().unwrap());
    }
}
fn rpl_fclose(fp: *mut std::fs::File) -> i32 {
    let mut saved_errno = 0;
    let fd: i32;
    let mut result = 0;

    unsafe {
        fd = libc::fileno(fp as *mut libc::FILE);
    }

    if fd < 0 {
        return libc::fclose(fp as *mut libc::FILE);
    }

    if ((!(__freading(fp) != 0) || libc::lseek(fd, 0, 1) != -1) && rpl_fflush(fp).is_ok()) {
        unsafe {
            saved_errno = *libc::__errno_location();
        }
    }

    result = libc::fclose(fp as *mut libc::FILE);

    if saved_errno != 0 {
        unsafe {
            *libc::__errno_location() = saved_errno;
        }
        result = -1;
    }

    result
}
fn rpl_fcntl(fd: i32, action: i32, ) -> i32 {
    let mut result = -1;
    let mut arg: core::ffi::VaListImpl;
    unsafe {
        core::ffi::VaListImpl::new(&mut arg);
    }

    match action {
        0 => {
            let target = unsafe { core::intrinsics::va_arg(arg.as_va_list(), core::intrinsics::TyDesc::<i32>::new()) };
            result = rpl_fcntl_dupfd(fd, target);
        }
        1030 => {
            let target = unsafe { core::intrinsics::va_arg(arg.as_va_list(), core::intrinsics::TyDesc::<i32>::new()) };
            result = rpl_fcntl_dupfd_cloexec(fd, target);
        }
        _ => {
            match action {
                1 | 3 | 1025 | 9 | 1032 | 1034 | 11 => {
                    result = libc::fcntl(fd, action);
                }
                1033 | 0 | 1030 | 1026 | 2 | 4 | 1024 | 8 | 1031 | 10 => {
                    let x = unsafe { core::intrinsics::va_arg(arg.as_va_list(), core::intrinsics::TyDesc::<i32>::new()) };
                    result = libc::fcntl(fd, action, x);
                }
                _ => {
                    let p = unsafe { core::intrinsics::va_arg(arg.as_va_list(), core::intrinsics::TyDesc::<*mut core::ffi::c_void>::new()) };
                    result = libc::fcntl(fd, action, p);
                }
            }
        }
    }

    unsafe {
        core::ffi::VaListImpl::end(&mut arg);
    }

    result
}
fn rpl_fcntl_dupfd(fd: i32, target: i32) -> Result<i32, ()> {
    let result = nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_DUPFD(target)).map_err(|_| ())?;
    Ok(result)
}
fn rpl_fcntl_dupfd_cloexec(fd: i32, target: i32) -> i32 {
    let result: i32;
    static mut HAVE_DUPFD_CLOEXEC: i32 = 0;

    if 0 <= unsafe { HAVE_DUPFD_CLOEXEC } {
        result = unsafe { libc::fcntl(fd, 1030, target) };
        if 0 <= result || unsafe { *libc::__errno_location() } != 22 {
            unsafe {
                HAVE_DUPFD_CLOEXEC = 1;
            }
        } else {
            result = rpl_fcntl_dupfd(fd, target);
            if result >= 0 {
                unsafe {
                    HAVE_DUPFD_CLOEXEC = -1;
                }
            }
        }
    } else {
        result = rpl_fcntl_dupfd(fd, target);
    }

    if 0 <= result && unsafe { HAVE_DUPFD_CLOEXEC } == -1 {
        let flags = unsafe { libc::fcntl(result, 1) };
        if flags < 0 || libc::fcntl(result, 2, flags | 1) == -1 {
            let saved_errno = unsafe { *libc::__errno_location() };
            unsafe {
                libc::close(result);
                *libc::__errno_location() = saved_errno;
            }
            result = -1;
        }
    }

    result
}
fn rpl_fflush(stream: *mut std::fs::File) -> Result<(), ()> {
    use std::io::Write;
    if stream.is_null() || !__freading(stream) != 0 {
        return std::io::stdout().flush();
    }
    clear_ungetc_buffer_preserving_position(stream)?;
    std::io::stdout().flush()
}
fn rpl_fprintf(fp: &mut std::fs::File, format: &str, args: usize) -> Result<usize, ()> {
    let mut buf = [0; 2000];
    let mut output;
    let mut len;
    let mut lenbuf = buf.len();
    let mut args = args as usize;
    let result = vasnprintf(Some(&mut buf), &mut lenbuf, format, args);
    output = match result {
        Ok(val) => val,
        Err(_) => {
            fseterr(fp);
            return Err(());
        }
    };
    len = lenbuf;
    if std::fs::write(fp, output).unwrap_or(0) < len {
        if output.as_ptr() != buf.as_ptr() {
            rpl_free(output);
        }
        return Err(());
    }
    if output.as_ptr() != buf.as_ptr() {
        rpl_free(output);
    }
    if len > 0x7fffffff {
        (*std::io::Error::last_os_error().raw_os_error().unwrap()) = 75;
        fseterr(fp);
        return Err(());
    }
    Ok(len)
}
fn rpl_free(p: *mut std::ffi::c_void) -> Result<(), ()> {
    let mut err = [0; 2];
    err[0] = unsafe { *libc::__errno_location() };
    err[1] = unsafe { *libc::__errno_location() };
    unsafe { *libc::__errno_location() = 0 };
    unsafe { libc::free(p as *mut std::ffi::c_void) };
    unsafe { *libc::__errno_location() = err[(*libc::__errno_location() == 0) as usize] };
    Ok(())
}
fn rpl_fseek(fp: &mut std::fs::File, offset: i64, whence: i32) -> Result<(), ()> {
    let offset = offset as std::os::raw::c_long;
    rpl_fseeko(fp, offset as i64, whence)
}
fn rpl_fseeko(fp: &mut std::fs::File, offset: i64, whence: i32) -> Result<(), ()> {
    use std::os::unix::io::AsRawFd;
    use std::io::{Seek, SeekFrom};

    let metadata = fp.metadata().map_err(|_| ())?;
    if metadata.len() == fp.seek(std::io::SeekFrom::Current(0)).map_err(|_| ())? as u64 {
        let pos = fp.seek(std::io::SeekFrom::Start(offset as u64)).map_err(|_| ())?;
        fp.set_len(pos).map_err(|_| ())?;
        Ok(())
    } else {
        fp.seek(std::io::SeekFrom::Start(offset as u64)).map_err(|_| ()).map(|_| ())
    }
}
fn rpl_strerror_r(errnum: i32, buf: &mut [u8], buflen: usize) -> Result<(), i32> {
    use std::io::Write;
    if buflen <= 1 {
        if buflen > 0 {
            buf[0] = b'\0';
        }
        return Ok(());
    }

    buf[0] = b'\0';

    let msg: Option<&str> = None;
    if let Some(msg) = msg {
        // Assuming safe_copy is a function that returns a Result
        return safe_copy(buf, buflen, msg).map_err(|_| *libc::__errno_location());
    }

    let ret: i32;
    let saved_errno = unsafe { *libc::__errno_location() };

    {
        ret = 0;
        unsafe {
            ret = libc::strerror_r(errnum, buf.as_mut_ptr() as *mut i8, buflen as _);
            if ret < 0 {
                ret = *libc::__errno_location();
            }
            if buf[0] == 0 {
                let mut errstring = [0u8; 256];
                let errstr_ptr = libc::strerror_r(errnum, errstring.as_mut_ptr() as *mut i8, buflen);
                let errstr = if errstr_ptr == 0 {
                    std::str::from_utf8(&errstring).unwrap_or_default()
                } else {
                    ""
                };
                ret = safe_copy(buf, buflen, errstr).unwrap_or(*libc::__errno_location());
            }
        }
    }

    if ret == 22 && buf[0] == 0 {
        let _ = write!(&mut buf[..], "Unknown error {}", errnum);
    }

    unsafe {
        *libc::__errno_location() = saved_errno;
    }

    Ok(ret)
}
fn rpl_vfprintf(fp: &mut std::fs::File, format: &str, args: usize) -> i32 {
    use std::io::Write;
    let mut buf = String::with_capacity(2000);
    let mut lenbuf = buf.len();

    let output = vasnprintf(Some(&mut buf), &mut lenbuf, format, args).unwrap_or_else(|_| {
        fseterr(fp);
        return -1;
    });

    let len = lenbuf;

    if let Err(_) = fp.write_all(output.as_bytes()) {
        if output.as_ptr() != buf.as_ptr() {
            // Assuming rpl_free is a function to free memory
            rpl_free(output.as_ptr() as *mut std::ffi::c_void);
        }
        return -1;
    }

    if len > 0x7fffffff {
        // Assuming fseterr is a function to set error
        std::io::Error::last_os_error().raw_os_error().map_or((), |err| {
            unsafe {
                *(err as *mut i32) = 75;
            }
        });
        fseterr(fp);
        return -1;
    }

    len as i32
}
fn rsync_roll(start: u32, mut num: u32) {
    let mut i: u32;
    if start < 4096 {
        for i in start..4096 {
            if i == start + num {
                return;
            }
            unsafe {
                RSYNC_SUM += WINDOW[i as usize] as Ulg;
            }
        }
        num -= 4096 - start;
        start = 4096;
    }
    for i in start..start + num {
        unsafe {
            RSYNC_SUM += WINDOW[i as usize] as Ulg;
            RSYNC_SUM -= WINDOW[(i - 4096) as usize] as Ulg;
            if RSYNC_CHUNK_END == 0xFFFFFFFF && RSYNC_SUM % 4096 == 0 {
                RSYNC_CHUNK_END = i;
            }
        }
    }
}
fn safe_copy(buf: &mut [u8], buflen: usize, msg: &str) -> Result<(), ()> {
    let len = msg.len();
    let moved = len.min(buflen).min(buflen - 1);
    buf[..moved].copy_from_slice(msg.as_bytes());
    if len < buflen {
        buf[moved] = b'\0';
        Ok(())
    } else {
        Err(())
    }
}
fn save_cwd(cwd: &mut SavedCwd) -> Result<(), ()> {
    cwd.name = None;
    cwd.desc = open_safer(".", 0 | 0o200000, None)?;
    if cwd.desc < 0 {
        cwd.desc = fd_safer_flag(cwd.desc, 0o200000)?;
    }
    if cwd.desc < 0 {
        cwd.name = std::fs::canonicalize(".") // This is equivalent to getcwd in C
            .ok()
            .and_then(|path| path.to_str().map(|s| s.to_string()));
        return if cwd.name.is_some() { Ok(()) } else { Err(()) };
    }
    Ok(())
}
fn savedir(dir: &str, option: SavedirOption) -> Result<String, ()> {
    let dirp = opendir_safer(dir);
    if dirp.is_null() {
        return Err(());
    } else {
        let name_space = streamsavedir(dirp, option);
        if unsafe { libc::closedir(dirp) } != 0 {
            if let Some(name_space_ptr) = name_space.as_ref().map(|s| s.as_ptr() as *mut std::ffi::c_void) {
                rpl_free(name_space_ptr);
            }
            return Err(());
        }
        return Ok(name_space.unwrap_or_default());
    }
}
fn scan_tree(tree: &mut [CtData], max_code: i32) {
    let mut n: i32;
    let mut prevlen = -1;
    let mut curlen: i32;
    let mut nextlen = tree[0].dl.len;
    let mut count = 0;
    let mut max_count = 7;
    let mut min_count = 4;
    if nextlen == 0 {
        max_count = 138;
        min_count = 3;
    }
    BL_TREE[max_code as usize + 1].dl.len = 0xffff;
    for n in 0..=max_code {
        curlen = nextlen;
        nextlen = tree[(n + 1) as usize].dl.len;
        if count < max_count - 1 && curlen == nextlen.into() {
            continue;
        } else if count < min_count {
            BL_TREE[curlen as usize].fc.freq += count as u16;
        } else if curlen != 0 {
            if curlen != prevlen {
                BL_TREE[curlen as usize].fc.freq += 1;
            }
            BL_TREE[16].fc.freq += 1;
        } else if count <= 10 {
            BL_TREE[17].fc.freq += 1;
        } else {
            BL_TREE[18].fc.freq += 1;
        }
        count = 0;
        prevlen = curlen;
        if nextlen == 0 {
            max_count = 138;
            min_count = 3;
        } else if curlen == nextlen.into() {
            max_count = 6;
            min_count = 3;
        } else {
            max_count = 7;
            min_count = 4;
        }
    }
}
fn send_all_trees(lcodes: usize, dcodes: usize, blcodes: usize) {
    use std::convert::TryInto;

    for rank in 0..blcodes {
        unsafe {
            send_bits((BL_TREE[BL_ORDER[rank] as usize].dl.len as usize).try_into().unwrap(), 3).unwrap();
        }
    }

    unsafe {
        send_tree(&mut DYN_LTREE, lcodes - 1).unwrap();
        send_tree(&mut DYN_DTREE, dcodes - 1).unwrap();
    }
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
            BI_BUF = (value >> ((8 * 2 * std::mem::size_of::<u8>() as i32) - BI_VALID)) as Ush;
            BI_VALID += length - (8 * 2 * std::mem::size_of::<u8>() as i32);
        } else {
            BI_BUF |= (value << BI_VALID) as u16;
            BI_VALID += length;
        }
    }
    Ok(())
}
fn send_tree(tree: &mut [CtData], max_code: usize) -> Result<(), ()> {
    use std::convert::TryInto;
    let mut n = 0;
    let mut prevlen = -1i16;
    let mut curlen;
    let mut nextlen = tree[0].dl.len;
    let mut count = 0;
    let mut max_count = 7;
    let mut min_count = 4;
    if nextlen == 0 {
        max_count = 138;
        min_count = 3;
    }
    while n <= max_code {
        curlen = nextlen;
        nextlen = tree[n + 1].dl.len;
        if count < max_count && curlen == nextlen {
            count += 1;
            continue;
        } else if count < min_count {
            while count != 0 {
                send_bits(BL_TREE[curlen as usize].fc.code as i32, BL_TREE[curlen as usize].dl.len as i32)?;
                count -= 1;
            }
        } else if curlen != 0 {
            if curlen != prevlen.try_into().unwrap() {
                send_bits(BL_TREE[curlen as usize].fc.code as i32, BL_TREE[curlen as usize].dl.len as i32)?;
                count -= 1;
            }
            send_bits(BL_TREE[16].fc.code as i32, BL_TREE[16].dl.len as i32)?;
            send_bits(count - 3, 2)?;
        } else if count <= 10 {
            send_bits(BL_TREE[17].fc.code as i32, BL_TREE[17].dl.len as i32)?;
            send_bits(count - 3, 3)?;
        } else {
            send_bits(BL_TREE[18].fc.code as i32, BL_TREE[18].dl.len as i32)?;
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
        n += 1;
    }
    Ok(())
}
fn set_cloexec_flag(desc: i32, value: bool) -> Result<(), ()> {
    let flags = crate::rpl_fcntl(desc, 1);
    if flags >= 0 {
        let newflags = if value { flags | 1 } else { flags & !1 };
        if flags == newflags || crate::rpl_fcntl(desc, 2, newflags) >= 0 {
            return Ok(());
        }
    }
    Err(())
}
fn set_file_type() {
    let mut n = 0;
    let mut ascii_freq = 0;
    let mut bin_freq = 0;

    while n < 7 {
        bin_freq += unsafe { DYN_LTREE.get(n).unwrap().fc.freq };
        n += 1;
    }

    while n < 128 {
        ascii_freq += unsafe { DYN_LTREE.get(n).unwrap().fc.freq };
        n += 1;
    }

    while n < 256 {
        bin_freq += unsafe { DYN_LTREE.get(n).unwrap().fc.freq };
        n += 1;
    }

    unsafe {
        *FILE_TYPE = if bin_freq > (ascii_freq >> 2) { 0 } else { 1 };
    }

    if unsafe { *FILE_TYPE } == 0 && false {
        warning("-l used on binary file");
    }
}
fn setcrc(c: Ulg){todo!("proto")}
fn settime(time: &libc::timespec) -> Result<(), ()>{todo!("proto")}
fn shorten_name(name: &mut std::ffi::CStr) {
    let name = name.to_bytes();
    let mut len = name.len();
    let mut trunc: Option<usize> = None;
    let mut plen: usize;
    let mut min_part = 3;
    let mut p: usize;
    
    if unsafe { DECOMPRESS } != 0 {
        if len <= 1 {
            gzip_error("name too short");
        }
        name[len-1] = b'\0';
        return;
    }
    
    let p_ptr = get_suffix(name.as_ptr() as *const i8);
    if p_ptr.is_null() {
        gzip_error("can't recover suffix\n");
    }
    let p = p_ptr as usize;
    unsafe { *p_ptr = b'\0'; }
    
    unsafe { SAVE_ORIG_NAME = 1; }
    
    if len > 4 && &name[(len-4)..] == b".tar" {
        name[(len-4)..].copy_from_slice(b".tgz");
        return;
    }
    
    loop {
        let p_ptr = last_component(std::str::from_utf8_unchecked(name));
        let p = p_ptr.as_ptr() as usize;
        while name[p] != 0 {
            plen = name[p..].iter().position(|&c| c == b'.').unwrap_or(name.len());
            p += plen;
            if plen > min_part {
                trunc = Some(p-1);
            }
            if name[p] != 0 {
                p += 1;
            }
        }
        if trunc.is_some() || min_part == 0 {
            break;
        }
        min_part -= 1;
    }
    
    if let Some(trunc) = trunc {
        loop {
            name[trunc] = name[trunc + 1];
            trunc += 1;
            if name[trunc] == 0 {
                break;
            }
        }
        trunc -= 1;
    } else {
        let mut trunc = name.iter().rposition(|&c| c == b'.').unwrap_or(0);
        if trunc == 0 {
            gzip_error("internal error in shorten_name");
        }
        if name[trunc + 1] == 0 {
            trunc -= 1;
        }
    }
    
    let suffix = unsafe { std::ffi::CStr::from_ptr(Z_SUFFIX) };
    let suffix = suffix.to_bytes();
    name[trunc..].copy_from_slice(suffix);
}
fn stat_time_normalize(result: i32, st: &std::os::linux::raw::stat) -> i32 {
    result
}
fn strdup(s: &str) -> Option<String>{todo!("proto")}
fn streamsavedir(dirp: *mut libc::DIR, option: SavedirOption) -> Option<String> {
    let mut name_space: Option<Vec<u8>> = None;
    let mut allocated: IdxT = 0;
    let mut entries: Option<Vec<DirentryT>> = None;
    let mut entries_allocated: IdxT = 0;
    let mut entries_used: IdxT = 0;
    let mut used: IdxT = 0;
    let cmp = COMPARISON_FUNCTION_TABLE[option as usize];

    if dirp.is_null() {
        return None;
    }

    loop {
        let dp = unsafe {
            let mut dp: *const libc::dirent = str::null();
            *libc::__errno_location() = 0;
            dp = libc::readdir(dirp);
            dp
        };

        if dp.is_null() {
            break;
        }

        let entry = unsafe { (*dp).d_name };

        if entry[0] != b'.' || entry[1] != b'.' || entry[2] != 0 {
            let entry_size = unsafe { libc::strlen((*dp).d_name) + 1 };

            if let Some(cmp_fn) = cmp {
                if entries_allocated == entries_used {
                    entries = Some(Vec::with_capacity(entries_allocated as usize));
                }

                let entry_name = String::from(entry).into_bytes();
                entries.as_mut().unwrap().push(DirentryT { name: Some(entry_name) });
                entries_used += 1;
            } else {
                if allocated - used <= entry_size {
                    name_space = Some(Vec::with_capacity(allocated as usize));
                }

                unsafe {
                    libc::memcpy(name_space.as_mut().unwrap().as_mut_ptr().offset(used as isize) as *mut libc::c_void, entry.as_ptr() as *const libc::c_void, entry_size);
                }
            }

            used += entry_size;
        }
    }

    if unsafe { *libc::__errno_location() } != 0 {
        rpl_free(entries.as_mut().unwrap().as_mut_ptr() as *mut std::ffi::c_void);
        rpl_free(name_space.as_mut().unwrap().as_mut_ptr() as *mut std::ffi::c_void);
        return None;
    }

    if let Some(cmp_fn) = cmp {
        if entries_used > 0 {
            entries.as_mut().unwrap().sort_by(cmp_fn);
        }

        let mut name_space_vec = Vec::with_capacity(used as usize + 1);
        let mut used_idx = 0;

        for i in 0..entries_used {
            let entry_name = entries.as_ref().unwrap()[i].name.as_ref().unwrap();
            name_space_vec.extend_from_slice(entry_name);
            name_space_vec.push(b'\0');
            rpl_free(entries.as_mut().unwrap()[i].name.as_mut().unwrap().as_mut_ptr() as *mut std::ffi::c_void);
        }

        rpl_free(entries.as_mut().unwrap().as_mut_ptr() as *mut std::ffi::c_void);
        Some(String::from_utf8(name_space_vec).ok()?)
    } else if used == allocated {
        let name_space_vec = xirealloc(name_space.as_mut().unwrap().as_mut_ptr() as *mut std::ffi::c_void, used + 1) as *mut u8;
        name_space_vec.add(used as usize).write(b'\0');
        Some(String::from_utf8_lossy(std::slice::from_raw_parts(name_space_vec, used as usize + 1)).to_string())
    } else {
        None
    }
}
fn strip_trailing_slashes(file: &mut String) -> bool {
    let base = last_component(file).unwrap_or(file);
    let base_lim = base.len() + base_len(base);
    let had_slash = file.chars().nth(base_lim) == Some('/');
    file.truncate(base_lim);
    had_slash
}
fn strlwr(s: &mut String) {
    unsafe {
        let bytes = s.as_bytes_mut();
        for byte in bytes {
            let c = *byte as char;
            if c.is_ascii_uppercase() {
                *byte = c.to_ascii_lowercase() as u8;
            }
        }
    }
}
fn strndup(s: &str, n: usize) -> Option<String>{todo!("proto")}
fn timespec_add(a: &libc::timespec, b: &libc::timespec) -> libc::timespec{todo!("proto")}
fn timespec_cmp(a: &libc::timespec, b: &libc::timespec) -> i32 {
    return 2 * ((a.tv_sec > b.tv_sec) as i32 - (a.tv_sec < b.tv_sec) as i32) + ((a.tv_nsec > b.tv_nsec) as i32 - (a.tv_nsec < b.tv_nsec) as i32);
}
fn timespec_sign(a: libc::timespec) -> i32 {
    ((a.tv_sec > 0) as i32 - (a.tv_sec < 0) as i32) + ((!a.tv_sec) as i32 & (a.tv_nsec != 0) as i32)
}
fn timespec_sub(ts1: &libc::timespec, ts2: &libc::timespec) -> libc::timespec{todo!("proto")}
fn timespectod(a: &libc::timespec) -> f64 {
    a.tv_sec as f64 + a.tv_nsec as f64 / 1e9
}
fn tmpfile() -> *mut std::fs::File{todo!("proto")}
fn treat_dir(fd: i32, dir: &str) -> Result<(), ()> {
    use std::convert::TryInto;
    let dirp = unsafe { libc::fdopendir(fd) };
    if dirp.is_null() {
        progerror(dir);
        unsafe { libc::close(fd) };
        return Err(());
    }

    let entries = streamsavedir(dirp, SavedirOption::SAVEDIR_SORT_NONE);
    if entries.is_none() {
        progerror(dir);
    }

    if unsafe { libc::closedir(dirp) } != 0 {
        progerror(dir);
    }

    if entries.is_none() {
        return Ok(());
    }

    let mut nbuf = [0; 1024];
    let mut entry = entries.unwrap();
    let mut entrylen;

    while let Some((current_entry, next_entry)) = entry.split_once('\0') {
        entry = next_entry.to_string();
        entrylen = current_entry.len();

        let len = dir.len();
        if current_entry == "." || current_entry == ".." {
            continue;
        }

        if len + entrylen < 1024 - 2 {
            nbuf[..len].copy_from_slice(dir.as_bytes());
            if let Some(last_comp) = last_component(std::str::from_utf8(&nbuf).unwrap()) {
                if !last_comp.ends_with('/') {
                    nbuf[len] = b'/';
                }
            }
            nbuf[len + 1..len + 1 + entrylen].copy_from_slice(current_entry.as_bytes());
            treat_file(std::str::from_utf8(&nbuf).unwrap());
        } else {
            rpl_fprintf(&mut std::io::stderr(), "{}: {}/{}: pathname too long\n", unsafe { (*PROGRAM_NAME).try_into().unwrap() }, dir, current_entry).unwrap();
            unsafe { EXIT_CODE = 1 };
        }
    }

    rpl_free(entries.as_mut());
    Ok(())
}
fn treat_file() {todo!()}
fn treat_stdin() {
    if (FORCE == 0) as i32 != 0 && (LIST == 0) as i32 != 0 && (PRESUME_INPUT_TTY || unsafe { libc::isatty(if DECOMPRESS != 0 { 0 } else { 1 }) } != 0) {
        if QUIET == 0 {
            rpl_fprintf(&mut std::io::stderr(),
                        "{}: compressed data not {} a terminal. Use -f to force {}compression.\nFor help, type: {} -h\n",
                        unsafe { std::ffi::CStr::from_ptr(PROGRAM_NAME).to_str().unwrap() },
                        if DECOMPRESS != 0 { "read from" } else { "written to" },
                        if DECOMPRESS != 0 { "de" } else { "" }).unwrap();
        }
        do_exit(1);
    }
    if DECOMPRESS != 0 || ASCII == 0 {
        // Do nothing
    }
    if TEST == 0 && (DECOMPRESS == 0 || ASCII == 0) {
        // Do nothing
    }
    unsafe {
        std::ptr::copy_nonoverlapping("stdin\0".as_ptr(), IFNAME.as_mut_ptr(), 6);
        std::ptr::copy_nonoverlapping("stdout\0".as_ptr(), OFNAME.as_mut_ptr(), 7);
        if libc::fstat(0, &mut ISTAT as *mut Option<std::fs::Metadata> as *mut libc::stat) != 0 {
            progerror("standard input");
            do_exit(1);
        }
    }
    get_input_size_and_time();
    clear_bufs();
    TO_STDOUT = 1;
    PART_NB = 0;
    IFD = 0;
    STDIN_WAS_READ = true;
    if DECOMPRESS != 0 {
        METHOD = get_method(IFD);
        if METHOD < 0 {
            do_exit(EXIT_CODE);
        }
    }
    loop {
        if WORK(0, 1) != 0 {
            return;
        }
        if input_eof().is_ok() {
            break;
        }
        METHOD = get_method(IFD);
        if METHOD < 0 {
            return;
        }
        BYTES_OUT = 0;
    }
    if LIST != 0 {
        do_list();
        return;
    }
    if VERBOSE != 0 {
        if TEST != 0 {
            rpl_fprintf(&mut std::io::stderr(), " OK\n", 0).unwrap();
        } else if DECOMPRESS == 0 {
            display_ratio(BYTES_IN - (BYTES_OUT - HEADER_BYTES), BYTES_IN, &mut std::fs::File::create("output.txt").unwrap());
            rpl_fprintf(&mut std::fs::File::create("output.txt").unwrap(), "\n", 0).unwrap();
        }
    }
}
fn try_help() -> ! {
    use std::convert::TryInto;
    let stderr = std::io::stderr();
    let mut stderr_lock = stderr.lock();
    let _ = rpl_fprintf(&mut stderr_lock, "Try `{}` --help' for more information.\n", unsafe { (*PROGRAM_NAME).try_into().unwrap() });
    do_exit(1);
}
fn unlzh(in_data: i32, out: i32) -> Result<u32, ()> {
    unsafe {
        IFD = in_data;
        OFD = out;
        decode_start();
        while DONE == 0 {
            let n = decode((1 << 13) as u32, &mut WINDOW)?;
            if n > 0 {
                write_buf(out, WINDOW.as_mut_ptr() as *mut std::ffi::c_void, n)?;
            }
        }
    }
    Ok(0)
}
fn unlzw(in_data: i32, out: i32) -> i32 {
    let mut stackp: *mut CharType;
    let mut code: CodeInt;
    let mut finchar: i32;
    let mut oldcode: CodeInt;
    let mut incode: CodeInt;
    let mut inbits: i64;
    let mut posbits: i64;
    let mut outpos: i32;
    let mut bitmask: u32;
    let mut free_ent: CodeInt;
    let mut maxcode: CodeInt;
    let mut maxmaxcode: CodeInt;
    let mut n_bits: i32;
    let mut rsize: i32;
    unsafe {
        MAXBITS = if INPTR < INSIZE { INBUF[INPTR as usize].into() } else { fill_inbuf(false) };
        BLOCK_MODE = MAXBITS & 0x80;
        if (MAXBITS & 0x60) != 0 {
            if !QUIET {
                rpl_fprintf(&mut std::io::stderr(), "\n%s: %s: warning, unknown flags 0x%x\n", PROGRAM_NAME, IFNAME, MAXBITS & 0x60).unwrap();
                if EXIT_CODE == 0 {
                    EXIT_CODE = 2;
                }
            }
        }
        MAXBITS &= 0x1f;
        maxmaxcode = 1 << MAXBITS;
        if MAXBITS > 16 {
            rpl_fprintf(&mut std::io::stderr(),
                        "\n%s: %s: compressed with %d bits, can only handle %d bits\n",
                        PROGRAM_NAME, IFNAME, MAXBITS, 16).unwrap();
            EXIT_CODE = 1;
            return 1;
        }
        rsize = INSIZE;
        maxcode = (1 << (n_bits = 9)) - 1;
        bitmask = (1 << n_bits) - 1;
        oldcode = -1;
        finchar = 0;
        outpos = 0;
        posbits = INPTR as i64 * 3;
        free_ent = if BLOCK_MODE != 0 { 256 + 1 } else { 256 };
        std::ptr::write_bytes(PREV.as_mut_ptr(), 0, 256);
        for code in (0..=255).rev() {
            WINDOW[code as usize] = code as CharType;
        }
    }
    loop {
        let mut i: i32;
        let mut e: i32;
        let mut o: i32;
    resetbuf;
        rsize = (posbits >> 3) as i32;
        e = if o <= INSIZE as i32 { INSIZE as i32 - o } else { 0 };
        unsafe {
            for i in 0..e {
                INBUF[i as usize] = INBUF[(i + o) as usize];
            }
            INSIZE = e as u32;
            posbits = 0;
            if INSIZE < 64 {
                rsize = read_buffer(in_data, INBUF.as_mut_ptr() as Voidp, 0x40000);
                if rsize == -1 {
                    read_error();
                }
                INSIZE += rsize as u32;
                BYTES_IN += rsize as std::os::raw::c_long;
            }
            inbits = if rsize != 0 {
                (INSIZE as i64 - INSIZE as i64 % n_bits as i64) << 3
            } else {
                ((INSIZE as i64) << 3) - (n_bits - 1) as i64
            };
            while inbits > posbits {
                unsafe {
                    if free_ent > maxcode {
                        posbits = (posbits - 1) + ((n_bits << 3) - (posbits - 1 + (n_bits << 3)) % (n_bits << 3));
                        n_bits += 1;
                        if n_bits == MAXBITS {
                            maxcode = maxmaxcode;
                        } else {
                            maxcode = 1 << n_bits - 1;
                        }
                        bitmask = (1 << n_bits) - 1;
                        goto!(resetbuf);
                    }
                    let p = &INBUF[(posbits >> 3) as usize];
                    code = (((p[0] as i64) | (p[1] as i64) << 8 | (p[2] as i64) << 16) >> (posbits & 0x7)) & bitmask as CodeInt;
                    posbits += n_bits as i64;
                }
                if oldcode == -1 {
                    if 256 <= code {
                        gzip_error("corrupt input.");
                    }
                    OUTBUF[outpos as usize] = finchar as CharType;
                    finchar = oldcode = code as i32;
                    outpos += 1;
                    continue;
                }
                if code == 256 && BLOCK_MODE != 0 {
                    unsafe {
                        std::ptr::write_bytes(PREV.as_mut_ptr(), 0, 256);
                        free_ent = 256 + 1 - 1;
                        posbits = (posbits - 1) + ((n_bits << 3) - (posbits - 1 + (n_bits << 3)) % (n_bits << 3));
                        maxcode = (1 << (n_bits = 9)) - 1;
                        bitmask = (1 << n_bits) - 1;
                        goto!(resetbuf);
                    }
                }
                incode = code;
                stackp = &mut D_BUF[0x8000 - 1];
                if code >= free_ent {
                    unsafe {
                        if code > free_ent {
                            if outpos > 0 {
                                write_buf(out, OUTBUF.as_mut_ptr() as *mut std::ffi::c_void, outpos as u32).unwrap();
                            }
                            gzip_error(if TO_STDOUT { "corrupt input." } else { "corrupt input. Use zcat to recover some data." });
                        }
                        stackp = stackp.offset(-1);
                        *stackp = finchar as CharType;
                        code = oldcode;
                    }
                }
                while code as CmpCodeInt >= 256 {
                    unsafe {
                        stackp = stackp.offset(-1);
                        *stackp = WINDOW[code as usize];
                        code = PREV[code as usize];
                    }
                }
                unsafe {
                    stackp = stackp.offset(-1);
                    *stackp = finchar as CharType;
                    let mut i = (stackp as usize - D_BUF[0x8000 - 1] as usize) as i32;
                    if outpos + i >= 0x40000 {
                        while i > 0 {
                            let copy_len = if i > 0x40000 - outpos { 0x40000 - outpos } else { i };
                            if copy_len > 0 {
                                OUTBUF[outpos as usize..(outpos + copy_len) as usize].copy_from_slice(std::slice::from_raw_parts(stackp, copy_len as usize));
                                outpos += copy_len;
                            }
                            if outpos >= 0x40000 {
                                write_buf(out, OUTBUF.as_mut_ptr() as *mut std::ffi::c_void, outpos as u32).unwrap();
                                outpos = 0;
                            }
                            stackp = stackp.offset(copy_len as isize);
                            i -= copy_len;
                        }
                    } else {
                        OUTBUF[outpos as usize..(outpos + i) as usize].copy_from_slice(std::slice::from_raw_parts(stackp, i as usize));
                        outpos += i;
                    }
                }
                if code < free_ent {
                    unsafe {
                        PREV[code as usize] = oldcode as u16;
                        WINDOW[code as usize] = finchar as CharType;
                        free_ent = code + 1;
                    }
                    oldcode = incode;
                }
            }
        }
        if outpos > 0 {
            unsafe {
                write_buf(out, OUTBUF.as_mut_ptr() as *mut std::ffi::c_void, outpos as u32).unwrap();
            }
        }
        if rsize == 0 {
            break;
        }
    }
    0
}
fn unpack(in_data: i32, out: i32) -> Result<(), ()> {
    let mut len: i32;
    let mut eob: u32;
    let mut peek: u32;
    let mut peek_mask: u32;
    
    unsafe {
        IFD = in_data;
        OFD = out;
        read_tree();
        build_tree(&mut TreeDesc); // Pass the required argument
        VALID = 0;
        BITBUF = 0;
        peek_mask = (1 << PEEK_BITS) - 1;
        eob = LEAVES[MAX_LEN as usize] as u32 - 1;
    }
    
    loop {
        while unsafe { VALID < PEEK_BITS } {
            unsafe {
                BITBUF = (BITBUF << 8) | read_byte()? as u64;
                VALID += 8;
            }
            peek = (unsafe { BITBUF >> (VALID - PEEK_BITS) }) as u32 & peek_mask;
        }
        
        len = unsafe { OUTBUF[peek as usize] as i32 };
        
        if len > 0 {
            peek >>= PEEK_BITS - len;
        } else {
            let mut mask = peek_mask as u64;
            len = PEEK_BITS;
            while peek < unsafe { PARENTS[len as usize] as u32 } {
                len += 1;
                mask = (mask << 1) + 1;
                while unsafe { VALID < len } {
                    unsafe {
                        BITBUF = (BITBUF << 8) | read_byte()? as u64;
                        VALID += 8;
                    }
                    peek = (unsafe { BITBUF >> (VALID - len) }) as u32 & mask as u32;
                }
            }
        }
        
        if peek == eob && len == MAX_LEN {
            break;
        }
        
        unsafe {
            WINDOW[OUTCNT as usize] = LITERAL[(peek as u32 + LIT_BASE[len as usize] as u32) as usize];
            OUTCNT += 1;
            if OUTCNT == 0x8000 {
                flush_window();
            }
        }
        
        unsafe {
            VALID -= len;
        }
    }
    
    flush_window();
    
    unsafe {
        if ORIG_LEN != (BYTES_OUT as u64 & 0xffffffff) {
            gzip_error("invalid compressed data--length error");
        }
    }
    
    Ok(())
}
fn unzip(in_data: i32, out: i32) -> Result<(), ()> {
    use std::convert::TryInto;
    let mut orig_crc: Ulg = 0;
    let mut orig_len: Ulg = 0;
    let mut n: i32;
    let mut buf: [Uch; 16] = [0; 16];
    let mut err = 0;
    
    unsafe {
        IFD = in_data;
        OFD = out;
        updcrc(None, 0);
        
        if PKZIP != 0 && EXT_HEADER == 0 {
            orig_crc = ((INBUF[14] as Ush as Uch | (INBUF[14 + 1] as Ush as Uch) << 8) as Ulg) |
                       ((INBUF[14 + 2] as Ush as Uch | (INBUF[14 + 2 + 1] as Ush as Uch) << 8) as Ulg) << 16;
            orig_len = ((INBUF[22] as Ush as Uch | (INBUF[22 + 1] as Ush as Uch) << 8) as Ulg) |
                       ((INBUF[22 + 2] as Ush as Uch | (INBUF[22 + 2 + 1] as Ush as Uch) << 8) as Ulg) << 16;
        }
        
        if METHOD == 8 {
            let res = inflate();
            if res == 3 {
                xalloc_die();
            } else if res != 0 {
                gzip_error("invalid compressed data--format violated");
            }
        } else if PKZIP != 0 && METHOD == 0 {
            let mut n = ((INBUF[22] as Ush as Uch | (INBUF[22 + 1] as Ush as Uch) << 8) as Ulg) |
                        ((INBUF[22 + 2] as Ush as Uch | (INBUF[22 + 2 + 1] as Ush as Uch) << 8) as Ulg) << 16;
            if n != ((INBUF[18] as Ush as Uch | (INBUF[18 + 1] as Ush as Uch) << 8) as Ulg) |
                     ((INBUF[18 + 2] as Ush as Uch | (INBUF[18 + 2 + 1] as Ush as Uch) << 8) as Ulg) - if DECRYPT != 0 { 12 } else { 0 } {
                rpl_fprintf(&mut std::io::stderr(), "len %lu, siz %lu\n", n.try_into().unwrap(), ((INBUF[18] as Ush as Uch | (INBUF[18 + 1] as Ush as Uch) << 8) as Ulg) |
                                                                            ((INBUF[18 + 2] as Ush as Uch | (INBUF[18 + 2 + 1] as Ush as Uch) << 8) as Ulg));
                gzip_error("invalid compressed data--length mismatch");
            }
            while n > 0 {
                let c = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).try_into().unwrap() };
                WINDOW[OUTCNT as usize] = c;
                OUTCNT += 1;
                if OUTCNT == 0x8000 {
                    flush_window();
                }
                n -= 1;
            }
            flush_window();
        } else {
            gzip_error("internal error, invalid method");
        }
        
        if PKZIP == 0 {
            for n in 0..8 {
                buf[n as usize] = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).try_into().unwrap() };
            }
            orig_crc = ((buf[0] as Ush as Uch | (buf[1] as Ush as Uch) << 8) as Ulg) |
                       ((buf[2] as Ush as Uch | (buf[3] as Ush as Uch) << 8) as Ulg) << 16;
            orig_len = ((buf[4] as Ush as Uch | (buf[5] as Ush as Uch) << 8) as Ulg) |
                       ((buf[6] as Ush as Uch | (buf[7] as Ush as Uch) << 8) as Ulg) << 16;
        } else if EXT_HEADER != 0 {
            for n in 0..16 {
                buf[n as usize] = if INPTR < INSIZE { INBUF[INPTR as usize] } else { fill_inbuf(false).try_into().unwrap() };
            }
            orig_crc = ((buf[4] as Ush as Uch | (buf[5] as Ush as Uch) << 8) as Ulg) |
                       ((buf[6] as Ush as Uch | (buf[7] as Ush as Uch) << 8) as Ulg) << 16;
            orig_len = ((buf[12] as Ush as Uch | (buf[13] as Ush as Uch) << 8) as Ulg) |
                       ((buf[14] as Ush as Uch | (buf[15] as Ush as Uch) << 8) as Ulg) << 16;
        }
        
        if orig_crc != updcrc(Some(&OUTBUF), 0) {
            rpl_fprintf(&mut std::io::stderr(), "\n%s: %s: invalid compressed data--crc error\n", PROGRAM_NAME, IFNAME);
            err = 1;
        }
        if orig_len != (BYTES_OUT & 0xffffffff) as Ulg {
            rpl_fprintf(&mut std::io::stderr(), "\n%s: %s: invalid compressed data--length error\n", PROGRAM_NAME, IFNAME);
            err = 1;
        }
        
        if PKZIP != 0 && INPTR + 4 < INSIZE && ((INBUF[INPTR as usize] as Ush as Uch | (INBUF[INPTR as usize + 1] as Ush as Uch) << 8) as Ulg) |
                                                  ((INBUF[INPTR as usize + 2] as Ush as Uch | (INBUF[INPTR as usize + 3] as Ush as Uch) << 8) as Ulg) == 0x04034b50 {
            if TO_STDOUT != 0 {
                if QUIET == 0 {
                    rpl_fprintf(&mut std::io::stderr(), "%s: %s has more than one entry--rest ignored\n", PROGRAM_NAME, IFNAME);
                }
                if EXIT_CODE == 0 {
                    EXIT_CODE = 2;
                }
            } else {
                rpl_fprintf(&mut std::io::stderr(), "%s: %s has more than one entry -- unchanged\n", PROGRAM_NAME, IFNAME);
                err = 1;
            }
        }
        
        EXT_HEADER = 0;
        PKZIP = 0;
        UNZIP_CRC = orig_crc;
    }
    
    if err == 0 {
        Ok(())
    } else {
        EXIT_CODE = 1;
        if TEST == 0 {
            abort_gzip();
        }
        Err(())
    }
}
fn update_timespec(statbuf: &libc::stat, ts: &mut *mut libc::timespec) -> Result<(), ()> {
    let timespec = unsafe { **ts };
    if timespec.tv_nsec == ((1i64 << 30) - 2i64) && timespec.tv_nsec == ((1i64 << 30) - 2i64) {
        return Ok(());
    }
    if timespec.tv_nsec == ((1i64 << 30) - 1i64) && timespec.tv_nsec == ((1i64 << 30) - 1i64) {
        *ts = std::ptr::null_mut();
        return Err(());
    }
    if timespec.tv_nsec == ((1i64 << 30) - 2i64) {
        let atime = get_stat_atime(statbuf);
        unsafe {
            let ts_mut = *ts;
            (*ts_mut).tv_sec = atime.tv_sec;
            (*ts_mut).tv_nsec = atime.tv_nsec;
        }
    } else if timespec.tv_nsec == ((1i64 << 30) - 1i64) {
        unsafe {
            gettime(&mut **ts);
        }
    }
    Ok(())
}
fn updcrc(s: Option<&[Uch]>, n: usize) -> Ulg {
    let mut c: Ulg;
    if let Some(s) = s {
        c = unsafe { CRC };
        if n > 0 {
            for &byte in s.iter().take(n) {
                c = unsafe { CRC_32_TAB[((c as usize ^ byte as usize) & 0xff)] } ^ (c >> 8);
            }
        }
    } else {
        c = 0xffffffff;
    }
    unsafe { CRC = c };
    c ^ 0xffffffff
}
fn utimens(file: &str, timespec: Option<[libc::timespec; 2]>) -> Result<i32, ()> {
    Ok(fdutimens(-1, file, timespec))
}
fn validate_timespec() {todo!()}
fn vasnprintf<'a>(resultbuf: Option<&'a mut str>, lengthp: &'a mut usize, format: &'a str, args: usize) -> Result<&'a str, ()> {
    // TODO: Implement the logic here
    // For demonstration purposes, returning an Ok result with an empty string
    Ok("")
}
fn version() {
    license();
    println!("\n");
    println!("Written by Jean-loup Gailly.");
}
fn volatile_strcpy(dst: *mut i8, src: *const i8) {
    let mut dst_ptr = dst;
    let mut src_ptr = src;

    while unsafe { *src_ptr } != 0 {
        unsafe {
            *dst_ptr = *src_ptr;
            dst_ptr = dst_ptr.offset(1);
            src_ptr = src_ptr.offset(1);
        }
    }
    unsafe {
        *dst_ptr = 0;
    }
}
fn warning(m: &str) {
    use std::convert::TryInto;
    if unsafe { crate::QUIET } == 0 {
        if let Err(_) = crate::rpl_fprintf(&mut std::io::stderr(), "%s: %s: warning: %s\n", unsafe { (*crate::PROGRAM_NAME).try_into().unwrap() }, unsafe { *crate::IFNAME }, m) {}
    }
    if unsafe { crate::EXIT_CODE } == 0 {
        unsafe { crate::EXIT_CODE = 2 };
    }
}
fn wcsdup(s: *const libc::wchar_t) -> *mut libc::wchar_t{todo!("proto")}
fn write_buf(fd: i32, mut buf: *mut std::ffi::c_void, mut cnt: u32) -> Result<(), ()> {
    use std::convert::TryInto;
    let mut n: u32;
    unsafe {
        BYTES_OUT += cnt as std::os::raw::c_long;
        if TEST != 0 {
            return Ok(());
        }
        'write_loop: while {
            n = match write_buffer(fd, buf, cnt) {
                x if x == cnt.try_into().unwrap() => break 'write_loop,
                x if x == std::u32::MAX.try_into().unwrap() => {
                    write_error();
                    return Err(());
                    x.try_into().unwrap()
                }
                x => x.try_into().unwrap(),
            };
            cnt -= n;
            buf = (buf as *mut u8).wrapping_add(n as usize) as *mut std::ffi::c_void;
            true
        } {}
    }
    Ok(())
}
fn write_buffer(fd: i32, buf: *mut std::ffi::c_void, cnt: u32) -> i32 {
    use std::os::unix::io::FromRawFd;
    use std::io::Write;

    let cnt = if 0x7fffffff < cnt { 0x7fffffff } else { cnt };
    
    let mut file = unsafe { std::fs::File::from_raw_fd(fd) };
    
    file.write(unsafe {
        std::slice::from_raw_parts(buf as *const u8, cnt as usize)
    }).unwrap_or(std::usize::MAX as usize) as i32
}
fn write_error() {
    use std::io::Write;
    let e = unsafe { *libc::__errno_location() };

    // Fixing the first error
    write!(&mut std::io::stderr(), "\n{}: ", unsafe { std::ffi::CStr::from_ptr(PROGRAM_NAME as *const i8).to_str().unwrap() }).unwrap();

    unsafe { *libc::__errno_location() = e };

    std::io::stderr().write_all(b": ").unwrap();

    // Fixing the second error
    let err_msg = unsafe { std::ffi::CStr::from_ptr(OFNAME.as_ptr() as *const i8).to_string_lossy() };
    eprintln!("{}", err_msg);

    abort_gzip();
}
fn x2nrealloc(p: *mut u8, pn: &mut usize, s: usize) -> Result<*mut u8, ()> {
    let n = *pn;
    if p.is_null() {
        if n == 0 {
            let default_mxfast = 64 * std::mem::size_of::<usize>() / 4;
            let mut new_n = default_mxfast / s;
            new_n += (new_n == 0) as usize;
            *pn = new_n;
        }
    } else {
        if let Some(new_n) = n.checked_add((n >> 1) + 1) {
            *pn = new_n;
        } else {
            return Err(());
        }
    }
    let new_p = xreallocarray(p, *pn, s);
    *pn = n;
    Ok(new_p)
}
fn x2realloc(p: *mut u8, ps: &mut usize) -> Result<*mut u8, ()> {
    x2nrealloc(p, ps, 1)
}
fn xalloc_die() -> ! {
    unsafe {
        error(::EXIT_FAILURE, 0, "{}", "memory exhausted");
    }
    std::process::abort();
}
fn xcalloc(n: usize, s: usize) -> Option<*const std::os::raw::c_void> {
    let ptr = crate::nonnull(crate::calloc(n, s));
    if ptr.is_null() {
        None
    } else {
        Some(ptr)
    }
}
fn xcharalloc(n: usize) -> Option<*mut u8> {
    if std::mem::size_of::<u8>() == 1 {
        xmalloc(n).map(|ptr| ptr as *mut u8)
    } else {
        xnmalloc(n, std::mem::size_of::<u8>())
    }
}
fn xicalloc(n: IdxT, s: IdxT) -> Option<*const std::ffi::c_void> {
    match icalloc(n, s) {
        Some(ptr) => Some(nonnull(ptr as *const std::os::raw::c_void)),
        None => None,
    }
}
fn ximalloc(s: IdxT) -> Option<*const std::os::raw::c_void> {
    Some(nonnull(imalloc(s)?))
}
fn ximemdup(p: *const std::os::raw::c_void, s: IdxT) -> Result<*const std::os::raw::c_void, ()> {
    let new_mem = match ximalloc(s) {
        Some(mem) => mem,
        None => return Err(()),
    };

    unsafe {
        std::ptr::copy_nonoverlapping(p, new_mem as *mut std::os::raw::c_void, s);
    }

    Ok(new_mem)
}
fn ximemdup0(p: *const std::os::raw::c_void, s: IdxT) -> Result<*const std::os::raw::c_void, ()> {
    let result = ximalloc(s + 1).ok_or(())?;
    let result_slice = unsafe { std::slice::from_raw_parts_mut(result as *mut u8, s + 1) };
    result_slice[s as usize] = 0;
    let p_bytes = unsafe { std::slice::from_raw_parts(p as *const u8, s) };
    result_slice[..s as usize].copy_from_slice(p_bytes);
    Ok(result)
}
fn xinmalloc(n: IdxT, s: IdxT) -> Option<*const std::os::raw::c_void> {
    xireallocarray(std::ptr::null_mut(), n, s)
}
fn xirealloc(p: *mut std::ffi::c_void, s: IdxT) -> *const std::os::raw::c_void {
    use std::convert::TryInto;
    nonnull(irealloc(p, s.try_into().unwrap()).unwrap_or(std::ptr::null_mut()))
}
fn xireallocarray(p: *mut std::ffi::c_void, n: IdxT, s: IdxT) -> Option<*const std::os::raw::c_void> {
    Some(ireallocarray(p, n, s)?)
}
fn xizalloc(s: IdxT) -> Option<*const std::ffi::c_void> {
    xicalloc(s, 1)
}
fn xmalloc(s: usize) -> Option<*const std::os::raw::c_void> {
    let ptr = malloc(s);
    if ptr.is_null() {
        None
    } else {
        Some(nonnull(ptr))
    }
}
fn xmax(size1: usize, size2: usize) -> usize {
    if size1 >= size2 {
        size1
    } else {
        size2
    }
}
fn xmemdup(p: *const std::os::raw::c_void, s: usize) -> Result<*const std::os::raw::c_void, ()> {
    let new_ptr = match xmalloc(s) {
        Some(ptr) => ptr,
        None => return Err(()),
    };

    unsafe {
        std::ptr::copy_nonoverlapping(p, new_ptr as *mut std::os::raw::c_void, s);
    }

    Ok(new_ptr)
}
fn xnmalloc(n: usize, s: usize) -> Option<*mut u8> {
    Some(crate::xreallocarray(std::ptr::null_mut(), n, s))
}
fn xnrealloc(p: *mut u8, n: usize, s: usize) -> *mut u8 {
    xreallocarray(p, n, s)
}
fn xpalloc(pa: *mut std::ffi::c_void, pn: &mut IdxT, n_incr_min: IdxT, n_max: libc::ptrdiff_t, s: IdxT) -> Result<*mut std::ffi::c_void, ()> {
    use std::convert::TryInto;
    let n0 = *pn;
    const DEFAULT_MXFAST: IdxT = 64 * std::mem::size_of::<usize>() / 4;
    let mut n: IdxT = 0;
    if n0.checked_add(n0 >> 1).map_or(true, |result| {
        n = result;
        false
    }) {
        n = std::i64::MAX as IdxT;
    }
    if n_max >= 0 && n_max < n.try_into().unwrap() {
        n = n_max as IdxT;
    }
    let mut nbytes: IdxT = 0;
    let adjusted_nbytes = if n.checked_mul(s).map_or(true, |result| {
        nbytes = result;
        true
    }) {
        if n < DEFAULT_MXFAST {
            DEFAULT_MXFAST
        } else {
            0
        }
    } else {
        nbytes
    };
    if adjusted_nbytes != 0 {
        n = adjusted_nbytes / s;
        nbytes = adjusted_nbytes - adjusted_nbytes % s;
    }
    if pa.is_null() {
        *pn = 0;
    }
    if n - n0 < n_incr_min
        && (n0.checked_add(n_incr_min).is_none()
            || (n_max >= 0 && n_max < n.try_into().unwrap())
            || n.checked_mul(s).is_none())
    {
        return Err(());
    }
    pa = xrealloc(pa, nbytes);
    *pn = n;
    Ok(pa)
}
fn xrealloc(p: *mut std::ffi::c_void, s: usize) -> ! {
    let r = std::alloc::realloc(p, std::alloc::Layout::from_size_align(s, std::mem::align_of::<std::ffi::c_void>()).unwrap());
    if r.is_null() && (p.is_null() || s != 0) {
        xalloc_die();
    }
    r
}
fn xreallocarray(p: *mut u8, n: usize, s: usize) -> *mut u8 {
    let r = unsafe {
        std::alloc::realloc(
            p,
            std::alloc::Layout::from_size_align(n * s, std::mem::align_of::<u8>()).unwrap(),
            n * s, // Original size of the memory block
        )
    };
    if r.is_null() && (p.is_null() || (n > 0 && s > 0)) {
        xalloc_die();
    }
    r
}
fn xstrdup(string: &str) -> Result<String, ()> {
    match std::ffi::CString::new(string) {
        Ok(c_string) => {
            match xmemdup(c_string.as_ptr() as *const std::os::raw::c_void, c_string.as_bytes().len()) {
                Ok(ptr) => {
                    let c_str = unsafe { std::ffi::CStr::from_ptr(ptr as *const i8) };
                    match c_str.to_str() {
                        Ok(s) => Ok(s.to_string()),
                        Err(_) => Err(()),
                    }
                }
                Err(_) => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}
fn xsum(size1: usize, size2: usize) -> Option<usize> {
    let sum = size1.checked_add(size2)?;
    Some(if sum >= size1 { sum } else { usize::MAX })
}
fn xsum3(size1: usize, size2: usize, size3: usize) -> Option<usize> {
    xsum(xsum(size1, size2)?, size3)
}
fn xsum4(size1: usize, size2: usize, size3: usize, size4: usize) -> Option<usize> {
    xsum(xsum(xsum(size1, size2)?, size3)?, size4)
}
fn xunlink(filename: &str) -> Result<(), std::io::Error> {
    std::fs::remove_file(filename)
}
fn xzalloc(s: usize) -> Option<*const std::os::raw::c_void> {
    xcalloc(s, 1)
}
fn yesno() -> bool {
    use std::io::Read;
    let mut yes = false;
    let mut c = std::io::stdin().bytes().next().and_then(|result| result.ok()).unwrap_or(u8::MAX) as char;
    yes = c == 'y' || c == 'Y';
    while c != '\n' && c != (u8::MAX as char) {
        c = std::io::stdin().bytes().next().and_then(|result| result.ok()).unwrap_or(u8::MAX) as char;
    }
    yes
}
fn zip(in_data: i32, out: i32) -> i32 {
    use std::convert::TryInto;
    
    let mut flags: u8 = 0;
    let mut attr: u16 = 0;
    let mut deflate_flags: u16 = 0;
    let mut stamp: u32 = 0;

    unsafe {
        static mut IFD: i32 = 0;
        static mut OFD: i32 = 0;
        static mut OUTCNT: usize = 0;
        static mut METHOD: i32 = 8;
        static mut OUTBUF: [u8; 0x40000] = [0; 0x40000];

        IFD = in_data;
        OFD = out;
        OUTCNT = 0;
        OUTBUF[OUTCNT] = b'\x1f'; OUTCNT += 1; if OUTCNT == 0x40000 { flush_outbuf(); }
        OUTBUF[OUTCNT] = b'\x8b'; OUTCNT += 1; if OUTCNT == 0x40000 { flush_outbuf(); }
        OUTBUF[OUTCNT] = 8; OUTCNT += 1; if OUTCNT == 0x40000 { flush_outbuf(); }
    }

    if SAVE_ORIG_NAME != 0 {
        flags |= 0x08;
    }

    unsafe {
        let time_stamp = TIME_STAMP.duration_since(std::time::UNIX_EPOCH).unwrap();
        if let (Ok(nsec), Ok(sec)) = (time_stamp.subsec_nanos(), time_stamp.as_secs().try_into()) {
            if nsec < 0 {
                stamp = 0;
            } else if sec > 0 && sec <= 0xffffffff {
                stamp = sec as u32;
            } else {
                warning("file timestamp out of range for gzip format");
                stamp = 0;
            }

            if OUTCNT < 0x40000 - 2 {
                OUTBUF[OUTCNT] = (stamp & 0xffff) as u8; OUTCNT += 1;
                OUTBUF[OUTCNT] = ((stamp >> 8) & 0xff) as u8; OUTCNT += 1;
            } else {
                OUTBUF[OUTCNT] = (stamp & 0xff) as u8; OUTCNT += 1; if OUTCNT == 0x40000 { flush_outbuf(); }
                OUTBUF[OUTCNT] = ((stamp >> 8) & 0xff) as u8; OUTCNT += 1; if OUTCNT == 0x40000 { flush_outbuf(); }
            }
        }
    }

    updcrc(None, 0);
    bi_init(out.try_into().unwrap());
    ct_init(&mut attr, &mut METHOD);

    if LEVEL == 1 {
        deflate_flags |= 1; // Assuming FAST is represented by 1
    } else if LEVEL == 9 {
        deflate_flags |= 2; // Assuming SLOW is represented by 2
    }

    unsafe {
        OUTBUF[OUTCNT] = deflate_flags as u8; OUTCNT += 1; if OUTCNT == 0x40000 { flush_outbuf(); }
        OUTBUF[OUTCNT] = 0x03; OUTCNT += 1; if OUTCNT == 0x40000 { flush_outbuf(); }
    }

    if SAVE_ORIG_NAME != 0 {
        let mut p = gzip_base_name(&mut IFNAME.to_string());
        if let Some(p) = p {
            for &byte in p.as_bytes() {
                unsafe {
                    OUTBUF[OUTCNT] = byte; OUTCNT += 1; if OUTCNT == 0x40000 { flush_outbuf(); }
                }
            }
        }
    }

    unsafe {
        HEADER_BYTES = OUTCNT as std::os::raw::c_long;
    }

    deflate(LEVEL);

    if IFILE_SIZE != -1 && BYTES_IN != IFILE_SIZE {
        rpl_fprintf(&mut std::io::stderr(), "{}: {}: file size changed while zipping\n", PROGRAM_NAME, IFNAME.to_string());
    }

    unsafe {
        let crc = getcrc();
        OUTBUF[OUTCNT] = (crc & 0xffff) as u8; OUTCNT += 1; if OUTCNT == 0x40000 { flush_outbuf(); }
        OUTBUF[OUTCNT] = ((crc >> 8) & 0xff) as u8; OUTCNT += 1; if OUTCNT == 0x40000 { flush_outbuf(); }

        OUTBUF[OUTCNT] = (BYTES_IN & 0xffff) as u8; OUTCNT += 1; if OUTCNT == 0x40000 { flush_outbuf(); }
        OUTBUF[OUTCNT] = ((BYTES_IN >> 8) & 0xff) as u8; OUTCNT += 1; if OUTCNT == 0x40000 { flush_outbuf(); }
        OUTBUF[OUTCNT] = ((BYTES_IN >> 16) & 0xff) as u8; OUTCNT += 1; if OUTCNT == 0x40000 { flush_outbuf(); }
        OUTBUF[OUTCNT] = ((BYTES_IN >> 24) & 0xff) as u8; OUTCNT += 1; if OUTCNT == 0x40000 { flush_outbuf(); }
    }

    unsafe {
        HEADER_BYTES += 2 * 4;
        flush_outbuf();
    }

    0
}
fn main() {}