use ffi_utils::*;
use snips_nlu_utils_ffi_macros::export_nlu_utils_c_symbols;

generate_error_handling!(snips_nlu_utils_get_last_error);

export_nlu_utils_c_symbols!();
