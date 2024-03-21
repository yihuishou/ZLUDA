/* automatically generated by rust-bindgen 0.69.4 */

impl nvrtcResult {
    pub const NVRTC_SUCCESS: nvrtcResult = nvrtcResult(0);
}
impl nvrtcResult {
    pub const NVRTC_ERROR_OUT_OF_MEMORY: nvrtcResult = nvrtcResult(1);
}
impl nvrtcResult {
    pub const NVRTC_ERROR_PROGRAM_CREATION_FAILURE: nvrtcResult = nvrtcResult(2);
}
impl nvrtcResult {
    pub const NVRTC_ERROR_INVALID_INPUT: nvrtcResult = nvrtcResult(3);
}
impl nvrtcResult {
    pub const NVRTC_ERROR_INVALID_PROGRAM: nvrtcResult = nvrtcResult(4);
}
impl nvrtcResult {
    pub const NVRTC_ERROR_INVALID_OPTION: nvrtcResult = nvrtcResult(5);
}
impl nvrtcResult {
    pub const NVRTC_ERROR_COMPILATION: nvrtcResult = nvrtcResult(6);
}
impl nvrtcResult {
    pub const NVRTC_ERROR_BUILTIN_OPERATION_FAILURE: nvrtcResult = nvrtcResult(7);
}
impl nvrtcResult {
    pub const NVRTC_ERROR_NO_NAME_EXPRESSIONS_AFTER_COMPILATION: nvrtcResult = nvrtcResult(8);
}
impl nvrtcResult {
    pub const NVRTC_ERROR_NO_LOWERED_NAMES_BEFORE_COMPILATION: nvrtcResult = nvrtcResult(9);
}
impl nvrtcResult {
    pub const NVRTC_ERROR_NAME_EXPRESSION_NOT_VALID: nvrtcResult = nvrtcResult(10);
}
impl nvrtcResult {
    pub const NVRTC_ERROR_INTERNAL_ERROR: nvrtcResult = nvrtcResult(11);
}
#[repr(transparent)]
#[doc = " \\ingroup error\n \\brief   The enumerated type nvrtcResult defines API call result codes.\n          NVRTC API functions return nvrtcResult to indicate the call\n          result."]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvrtcResult(pub ::std::os::raw::c_int);

#[doc = " \\ingroup error\n \\brief   nvrtcGetErrorString is a helper function that returns a string\n          describing the given nvrtcResult code, e.g., NVRTC_SUCCESS to\n          \\c \"NVRTC_SUCCESS\".\n          For unrecognized enumeration values, it returns\n          \\c \"NVRTC_ERROR unknown\".\n\n \\param   [in] result CUDA Runtime Compilation API result code.\n \\return  Message string for the given #nvrtcResult code."]
#[no_mangle]
pub unsafe extern "system" fn nvrtcGetErrorString(result: nvrtcResult) -> *const ::std::os::raw::c_char {
    crate::get_error_string(result)
}

#[doc = " \\ingroup query\n \\brief   nvrtcVersion sets the output parameters \\p major and \\p minor\n          with the CUDA Runtime Compilation version number.\n\n \\param   [out] major CUDA Runtime Compilation major version number.\n \\param   [out] minor CUDA Runtime Compilation minor version number.\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_INPUT \\endlink\n"]
#[no_mangle]
pub unsafe extern "system" fn nvrtcVersion(
    major: *mut ::std::os::raw::c_int,
    minor: *mut ::std::os::raw::c_int,
) -> nvrtcResult {
    *major = 11;
    *minor = 8;
    nvrtcResult::NVRTC_SUCCESS
}

#[doc = " \\ingroup query\n \\brief   nvrtcGetNumSupportedArchs sets the output parameter \\p numArchs\n          with the number of architectures supported by NVRTC. This can\n          then be used to pass an array to ::nvrtcGetSupportedArchs to\n          get the supported architectures.\n\n \\param   [out] numArchs number of supported architectures.\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_INPUT \\endlink\n\n see    ::nvrtcGetSupportedArchs"]
#[no_mangle]
pub extern "system" fn nvrtcGetNumSupportedArchs(
    numArchs: *mut ::std::os::raw::c_int,
) -> nvrtcResult {
    crate::unsupported()
}

#[doc = " \\ingroup query\n \\brief   nvrtcGetSupportedArchs populates the array passed via the output parameter\n          \\p supportedArchs with the architectures supported by NVRTC. The array is\n          sorted in the ascending order. The size of the array to be passed can be\n          determined using ::nvrtcGetNumSupportedArchs.\n\n \\param   [out] supportedArchs sorted array of supported architectures.\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_INPUT \\endlink\n\n see    ::nvrtcGetNumSupportedArchs"]
#[no_mangle]
pub extern "system" fn nvrtcGetSupportedArchs(
    supportedArchs: *mut ::std::os::raw::c_int,
) -> nvrtcResult {
    crate::unsupported()
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _nvrtcProgram {
    _unused: [u8; 0],
}
#[doc = " \\ingroup compilation\n \\brief   nvrtcProgram is the unit of compilation, and an opaque handle for\n          a program.\n\n To compile a CUDA program string, an instance of nvrtcProgram must be\n created first with ::nvrtcCreateProgram, then compiled with\n ::nvrtcCompileProgram."]
pub type nvrtcProgram = *mut _nvrtcProgram;

#[doc = " \\ingroup compilation\n \\brief   nvrtcCreateProgram creates an instance of nvrtcProgram with the\n          given input parameters, and sets the output parameter \\p prog with\n          it.\n\n \\param   [out] prog         CUDA Runtime Compilation program.\n \\param   [in]  src          CUDA program source.\n \\param   [in]  name         CUDA program name.\\n\n                             \\p name can be \\c NULL; \\c \"default_program\" is\n                             used when \\p name is \\c NULL or \"\".\n \\param   [in]  numHeaders   Number of headers used.\\n\n                             \\p numHeaders must be greater than or equal to 0.\n \\param   [in]  headers      Sources of the headers.\\n\n                             \\p headers can be \\c NULL when \\p numHeaders is\n                             0.\n \\param   [in]  includeNames Name of each header by which they can be\n                             included in the CUDA program source.\\n\n                             \\p includeNames can be \\c NULL when \\p numHeaders\n                             is 0.\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_OUT_OF_MEMORY \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_PROGRAM_CREATION_FAILURE \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_INPUT \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_PROGRAM \\endlink\n\n \\see     ::nvrtcDestroyProgram"]
#[no_mangle]
pub unsafe extern "system" fn nvrtcCreateProgram(
    prog: *mut nvrtcProgram,
    src: *const ::std::os::raw::c_char,
    name: *const ::std::os::raw::c_char,
    numHeaders: ::std::os::raw::c_int,
    headers: *const *const ::std::os::raw::c_char,
    includeNames: *const *const ::std::os::raw::c_char,
) -> nvrtcResult {
    crate::create_program(prog, src, name, numHeaders, headers, includeNames)
}

#[doc = " \\ingroup compilation\n \\brief   nvrtcDestroyProgram destroys the given program.\n\n \\param    [in] prog CUDA Runtime Compilation program.\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_PROGRAM \\endlink\n\n \\see     ::nvrtcCreateProgram"]
#[no_mangle]
pub unsafe extern "system" fn nvrtcDestroyProgram(prog: *mut nvrtcProgram) -> nvrtcResult {
    crate::destroy_program(prog)
}

#[doc = " \\ingroup compilation\n \\brief   nvrtcCompileProgram compiles the given program.\n\n \\param   [in] prog       CUDA Runtime Compilation program.\n \\param   [in] numOptions Number of compiler options passed.\n \\param   [in] options    Compiler options in the form of C string array.\\n\n                          \\p options can be \\c NULL when \\p numOptions is 0.\n\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_OUT_OF_MEMORY \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_INPUT \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_PROGRAM \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_OPTION \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_COMPILATION \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_BUILTIN_OPERATION_FAILURE \\endlink\n\n It supports compile options listed in \\ref options."]
#[no_mangle]
pub unsafe extern "system" fn nvrtcCompileProgram(
    prog: nvrtcProgram,
    numOptions: ::std::os::raw::c_int,
    options: *const *const ::std::os::raw::c_char,
) -> nvrtcResult {
    crate::compile_program(prog, numOptions, options)
}

#[doc = " \\ingroup compilation\n \\brief   nvrtcGetPTXSize sets \\p ptxSizeRet with the size of the PTX\n          generated by the previous compilation of \\p prog (including the\n          trailing \\c NULL).\n\n \\param   [in]  prog       CUDA Runtime Compilation program.\n \\param   [out] ptxSizeRet Size of the generated PTX (including the trailing\n                           \\c NULL).\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_INPUT \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_PROGRAM \\endlink\n\n \\see     ::nvrtcGetPTX"]
#[no_mangle]
pub unsafe extern "system" fn nvrtcGetPTXSize(prog: nvrtcProgram, ptxSizeRet: *mut usize) -> nvrtcResult {
    crate::get_code_size(prog, ptxSizeRet)
}

#[doc = " \\ingroup compilation\n \\brief   nvrtcGetPTX stores the PTX generated by the previous compilation\n          of \\p prog in the memory pointed by \\p ptx.\n\n \\param   [in]  prog CUDA Runtime Compilation program.\n \\param   [out] ptx  Compiled result.\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_INPUT \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_PROGRAM \\endlink\n\n \\see     ::nvrtcGetPTXSize"]
#[no_mangle]
pub unsafe extern "system" fn nvrtcGetPTX(
    prog: nvrtcProgram,
    ptx: *mut ::std::os::raw::c_char,
) -> nvrtcResult {
    crate::get_code(prog, ptx)
}

#[doc = " \\ingroup compilation\n \\brief   nvrtcGetCUBINSize sets \\p cubinSizeRet with the size of the cubin\n          generated by the previous compilation of \\p prog. The value of\n          cubinSizeRet is set to 0 if the value specified to \\c -arch is a\n          virtual architecture instead of an actual architecture.\n\n \\param   [in]  prog       CUDA Runtime Compilation program.\n \\param   [out] cubinSizeRet Size of the generated cubin.\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_INPUT \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_PROGRAM \\endlink\n\n \\see     ::nvrtcGetCUBIN"]
#[no_mangle]
pub unsafe extern "system" fn nvrtcGetCUBINSize(
    prog: nvrtcProgram,
    cubinSizeRet: *mut usize,
) -> nvrtcResult {
    crate::get_code_size(prog, cubinSizeRet)
}

#[doc = " \\ingroup compilation\n \\brief   nvrtcGetCUBIN stores the cubin generated by the previous compilation\n          of \\p prog in the memory pointed by \\p cubin. No cubin is available\n          if the value specified to \\c -arch is a virtual architecture instead\n          of an actual architecture.\n\n \\param   [in]  prog CUDA Runtime Compilation program.\n \\param   [out] cubin  Compiled and assembled result.\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_INPUT \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_PROGRAM \\endlink\n\n \\see     ::nvrtcGetCUBINSize"]
#[no_mangle]
pub unsafe extern "system" fn nvrtcGetCUBIN(
    prog: nvrtcProgram,
    cubin: *mut ::std::os::raw::c_char,
) -> nvrtcResult {
    crate::get_code(prog, cubin)
}

#[doc = " \\ingroup compilation\n \\brief   nvrtcGetNVVMSize sets \\p nvvmSizeRet with the size of the NVVM\n          generated by the previous compilation of \\p prog. The value of\n          nvvmSizeRet is set to 0 if the program was not compiled with\n          \\c -dlto.\n\n \\param   [in]  prog       CUDA Runtime Compilation program.\n \\param   [out] nvvmSizeRet Size of the generated NVVM.\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_INPUT \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_PROGRAM \\endlink\n\n \\see     ::nvrtcGetNVVM"]
#[no_mangle]
pub extern "system" fn nvrtcGetNVVMSize(
    prog: nvrtcProgram,
    nvvmSizeRet: *mut usize,
) -> nvrtcResult {
    crate::unsupported()
}

#[doc = " \\ingroup compilation\n \\brief   nvrtcGetNVVM stores the NVVM generated by the previous compilation\n          of \\p prog in the memory pointed by \\p nvvm.\n          The program must have been compiled with -dlto,\n          otherwise will return an error.\n\n \\param   [in]  prog CUDA Runtime Compilation program.\n \\param   [out] nvvm Compiled result.\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_INPUT \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_PROGRAM \\endlink\n\n \\see     ::nvrtcGetNVVMSize"]
#[no_mangle]
pub extern "system" fn nvrtcGetNVVM(
    prog: nvrtcProgram,
    nvvm: *mut ::std::os::raw::c_char,
) -> nvrtcResult {
    crate::unsupported()
}

#[doc = " \\ingroup compilation\n \\brief   nvrtcGetProgramLogSize sets \\p logSizeRet with the size of the\n          log generated by the previous compilation of \\p prog (including the\n          trailing \\c NULL).\n\n Note that compilation log may be generated with warnings and informative\n messages, even when the compilation of \\p prog succeeds.\n\n \\param   [in]  prog       CUDA Runtime Compilation program.\n \\param   [out] logSizeRet Size of the compilation log\n                           (including the trailing \\c NULL).\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_INPUT \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_PROGRAM \\endlink\n\n \\see     ::nvrtcGetProgramLog"]
#[no_mangle]
pub extern "system" fn nvrtcGetProgramLogSize(
    prog: nvrtcProgram,
    logSizeRet: *mut usize,
) -> nvrtcResult {
    crate::unsupported()
}

#[doc = " \\ingroup compilation\n \\brief   nvrtcGetProgramLog stores the log generated by the previous\n          compilation of \\p prog in the memory pointed by \\p log.\n\n \\param   [in]  prog CUDA Runtime Compilation program.\n \\param   [out] log  Compilation log.\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_INPUT \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_INVALID_PROGRAM \\endlink\n\n \\see     ::nvrtcGetProgramLogSize"]
#[no_mangle]
pub extern "system" fn nvrtcGetProgramLog(
    prog: nvrtcProgram,
    log: *mut ::std::os::raw::c_char,
) -> nvrtcResult {
    crate::unsupported()
}

#[doc = " \\ingroup compilation\n \\brief   nvrtcAddNameExpression notes the given name expression\n          denoting the address of a __global__ function\n          or __device__/__constant__ variable.\n\n The identical name expression string must be provided on a subsequent\n call to nvrtcGetLoweredName to extract the lowered name.\n \\param   [in]  prog CUDA Runtime Compilation program.\n \\param   [in] name_expression constant expression denoting the address of\n               a __global__ function or __device__/__constant__ variable.\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_NO_NAME_EXPRESSIONS_AFTER_COMPILATION \\endlink\n\n \\see     ::nvrtcGetLoweredName"]
#[no_mangle]
pub extern "system" fn nvrtcAddNameExpression(
    prog: nvrtcProgram,
    name_expression: *const ::std::os::raw::c_char,
) -> nvrtcResult {
    crate::unsupported()
}

#[doc = " \\ingroup compilation\n \\brief   nvrtcGetLoweredName extracts the lowered (mangled) name\n          for a __global__ function or __device__/__constant__ variable,\n          and updates *lowered_name to point to it. The memory containing\n          the name is released when the NVRTC program is destroyed by\n          nvrtcDestroyProgram.\n          The identical name expression must have been previously\n          provided to nvrtcAddNameExpression.\n\n \\param   [in]  prog CUDA Runtime Compilation program.\n \\param   [in] name_expression constant expression denoting the address of\n               a __global__ function or __device__/__constant__ variable.\n \\param   [out] lowered_name initialized by the function to point to a\n               C string containing the lowered (mangled)\n               name corresponding to the provided name expression.\n \\return\n   - \\link #nvrtcResult NVRTC_SUCCESS \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_NO_LOWERED_NAMES_BEFORE_COMPILATION \\endlink\n   - \\link #nvrtcResult NVRTC_ERROR_NAME_EXPRESSION_NOT_VALID \\endlink\n\n \\see     ::nvrtcAddNameExpression"]
#[no_mangle]
pub unsafe extern "system" fn nvrtcGetLoweredName(
    prog: nvrtcProgram,
    name_expression: *const ::std::os::raw::c_char,
    lowered_name: *mut *const ::std::os::raw::c_char,
) -> nvrtcResult {
    crate::get_lowered_name(prog, name_expression, lowered_name)
}
