initSidebarItems({"constant":[["DLDataTypeCode_kDLBfloat","\\brief bfloat16"],["DLDataTypeCode_kDLComplex","\\brief complex number (C/C++/Python layout: compact struct per complex number)"],["DLDataTypeCode_kDLFloat","\\brief IEEE floating point"],["DLDataTypeCode_kDLInt","\\brief signed integer"],["DLDataTypeCode_kDLOpaqueHandle","\\brief Opaque handle type, reserved for testing purposes. Frameworks need to agree on the handle data type for the exchange to be well-defined."],["DLDataTypeCode_kDLUInt","\\brief unsigned integer"],["DLDeviceType_kDLCPU","\\brief CPU device"],["DLDeviceType_kDLCUDA","\\brief CUDA GPU device"],["DLDeviceType_kDLCUDAHost","\\brief Pinned CUDA CPU memory by cudaMallocHost"],["DLDeviceType_kDLExtDev","\\brief Reserved extension device type, used for quickly test extension device The semantics can differ depending on the implementation."],["DLDeviceType_kDLMetal","\\brief Metal for Apple GPU."],["DLDeviceType_kDLOpenCL","\\brief OpenCL devices."],["DLDeviceType_kDLROCM","\\brief ROCm GPUs for AMD GPUs"],["DLDeviceType_kDLVPI","\\brief Verilog simulator buffer"],["DLDeviceType_kDLVulkan","\\brief Vulkan buffer for next generation graphics."],["DLPACK_VERSION",""],["INT16_MAX",""],["INT16_MIN",""],["INT32_MAX",""],["INT32_MIN",""],["INT8_MAX",""],["INT8_MIN",""],["INTPTR_MAX",""],["INTPTR_MIN",""],["INT_FAST16_MAX",""],["INT_FAST16_MIN",""],["INT_FAST32_MAX",""],["INT_FAST32_MIN",""],["INT_FAST8_MAX",""],["INT_FAST8_MIN",""],["INT_LEAST16_MAX",""],["INT_LEAST16_MIN",""],["INT_LEAST32_MAX",""],["INT_LEAST32_MIN",""],["INT_LEAST8_MAX",""],["INT_LEAST8_MIN",""],["PTRDIFF_MAX",""],["PTRDIFF_MIN",""],["SIG_ATOMIC_MAX",""],["SIG_ATOMIC_MIN",""],["SIZE_MAX",""],["TVMArgTypeCode_kDLDevice",""],["TVMArgTypeCode_kTVMArgFloat",""],["TVMArgTypeCode_kTVMArgInt",""],["TVMArgTypeCode_kTVMBytes",""],["TVMArgTypeCode_kTVMDLTensorHandle",""],["TVMArgTypeCode_kTVMDataType",""],["TVMArgTypeCode_kTVMExtBegin",""],["TVMArgTypeCode_kTVMExtEnd",""],["TVMArgTypeCode_kTVMExtReserveEnd",""],["TVMArgTypeCode_kTVMModuleHandle",""],["TVMArgTypeCode_kTVMNDArrayHandle",""],["TVMArgTypeCode_kTVMNNVMFirst",""],["TVMArgTypeCode_kTVMNNVMLast",""],["TVMArgTypeCode_kTVMNullptr",""],["TVMArgTypeCode_kTVMObjectHandle",""],["TVMArgTypeCode_kTVMObjectRValueRefArg",""],["TVMArgTypeCode_kTVMOpaqueHandle",""],["TVMArgTypeCode_kTVMPackedFuncHandle",""],["TVMArgTypeCode_kTVMStr",""],["TVMDeviceExtType_kDLAOCL",""],["TVMDeviceExtType_kDLHexagon",""],["TVMDeviceExtType_kDLMicroDev",""],["TVMDeviceExtType_kDLSDAccel",""],["TVMDeviceExtType_kDLWebGPU",""],["TVMDeviceExtType_kOpenGL",""],["TVM_VERSION",""],["UINT16_MAX",""],["UINT32_MAX",""],["UINT8_MAX",""],["UINTPTR_MAX",""],["UINT_FAST16_MAX",""],["UINT_FAST32_MAX",""],["UINT_FAST8_MAX",""],["UINT_LEAST16_MAX",""],["UINT_LEAST32_MAX",""],["UINT_LEAST8_MAX",""],["WINT_MAX",""],["WINT_MIN",""],["_ATFILE_SOURCE",""],["_BITS_STDINT_INTN_H",""],["_BITS_STDINT_UINTN_H",""],["_BITS_TYPESIZES_H",""],["_BITS_TYPES_H",""],["_BITS_WCHAR_H",""],["_DEFAULT_SOURCE",""],["_FEATURES_H",""],["_POSIX_C_SOURCE",""],["_POSIX_SOURCE",""],["_STDC_PREDEF_H",""],["_STDINT_H",""],["_SYS_CDEFS_H",""],["__FD_SETSIZE",""],["__GLIBC_MINOR__",""],["__GLIBC_USE_DEPRECATED_GETS",""],["__GLIBC_USE_IEC_60559_BFP_EXT",""],["__GLIBC_USE_IEC_60559_FUNCS_EXT",""],["__GLIBC_USE_IEC_60559_TYPES_EXT",""],["__GLIBC_USE_LIB_EXT2",""],["__GLIBC__",""],["__GNU_LIBRARY__",""],["__HAVE_GENERIC_SELECTION",""],["__INO_T_MATCHES_INO64_T",""],["__OFF_T_MATCHES_OFF64_T",""],["__RLIM_T_MATCHES_RLIM64_T",""],["__STDC_IEC_559_COMPLEX__",""],["__STDC_IEC_559__",""],["__STDC_ISO_10646__",""],["__STDC_NO_THREADS__",""],["__SYSCALL_WORDSIZE",""],["__USE_ATFILE",""],["__USE_FORTIFY_LEVEL",""],["__USE_ISOC11",""],["__USE_ISOC95",""],["__USE_ISOC99",""],["__USE_MISC",""],["__USE_POSIX",""],["__USE_POSIX199309",""],["__USE_POSIX199506",""],["__USE_POSIX2",""],["__USE_POSIX_IMPLICITLY",""],["__USE_XOPEN2K",""],["__USE_XOPEN2K8",""],["__WORDSIZE",""],["__WORDSIZE_TIME64_COMPAT32",""],["__glibc_c99_flexarr_available",""]],"fn":[["TVMAPISetLastError","\\brief Used for implementing C API function. Set last error message before return. \\param msg The error message to be set."],["TVMArrayAlloc","\\brief Allocate a nd-array’s memory, including space of shape, of given spec."],["TVMArrayCopyFromBytes","\\brief Copy array data from CPU byte array. \\param handle The array handle. \\param data the data pointer \\param nbytes The number of bytes to copy. \\return 0 when success, nonzero when failure happens"],["TVMArrayCopyFromTo","\\brief Copy the array, both from and to must be valid during the copy. \\param from The array to be copied from. \\param to The target space. \\param stream The stream where the copy happens, can be NULL. \\return 0 when success, nonzero when failure happens"],["TVMArrayCopyToBytes","\\brief Copy array data to CPU byte array. \\param handle The array handle. \\param data the data pointer \\param nbytes The number of bytes to copy. \\return 0 when success, nonzero when failure happens"],["TVMArrayFree","\\brief Free the TVM Array. \\param handle The array handle to be freed. \\return 0 when success, nonzero when failure happens"],["TVMArrayFromDLPack","\\brief Produce an array from the DLManagedTensor that shares data memory with the DLManagedTensor. \\param from The source DLManagedTensor. \\param out The output array handle. \\return 0 when success, nonzero when failure happens"],["TVMArrayToDLPack","\\brief Produce a DLMangedTensor from the array that shares data memory with the array. \\param from The source array. \\param out The DLManagedTensor handle. \\return 0 when success, nonzero when failure happens"],["TVMBackendAllocWorkspace","\\brief Backend function to allocate temporal workspace."],["TVMBackendFreeWorkspace","\\brief Backend function to free temporal workspace."],["TVMBackendGetFuncFromEnv","\\brief Backend function for modules to get function from its environment mod_node (its imports and global function). The user do should not call TVMFuncFree on func."],["TVMBackendParallelBarrier","\\brief BSP barrrier between parallel threads \\param task_id the task id of the function. \\param penv The parallel environment backs the execution. \\return 0 when no error is thrown, -1 when failure happens"],["TVMBackendParallelLaunch","\\brief Backend function for running parallel jobs."],["TVMBackendRegisterEnvCAPI","\\brief Backend function to register execution environment(e.g. python) specific C APIs."],["TVMBackendRegisterSystemLibSymbol","\\brief Backend function to register system-wide library symbol."],["TVMBackendRunOnce","\\brief Simple static initialization function. Run f once and set handle to be not null. This function is mainly used for test purpose."],["TVMByteArrayFree","\\brief Free a TVMByteArray returned from TVMFuncCall, and associated memory. \\param arr The TVMByteArray instance. \\return 0 on success, -1 on failure."],["TVMCFuncSetReturn","\\brief Set the return value of TVMPackedCFunc."],["TVMCbArgToReturn","\\brief Inplace translate callback argument value to return value. This is only needed for non-POD arguments."],["TVMDLManagedTensorCallDeleter","\\brief Delete (free) a DLManagedTensor’s data. \\param dltensor Pointer to the DLManagedTensor."],["TVMDeviceAllocDataSpace","\\brief Allocate a data space on device. \\param dev The device to perform operation. \\param nbytes The number of bytes in memory. \\param alignment The alignment of the memory. \\param type_hint The type of elements. Only needed by certain backends such as nbytes & alignment are sufficient for most backends. \\param out_data The allocated device pointer. \\return 0 when success, nonzero when failure happens"],["TVMDeviceAllocDataSpaceWithScope","\\brief Allocate a data space on device with special memory scope. \\note The memory could use a special multi-dimensional memory layout. That is why we pass shape and dtype instead of raw number of bytes. \\param dev The device to perform operation. \\param ndim The number of dimension of the tensor. \\param shape The shape of the tensor. \\param dtype The type of elements. \\param mem_scope The memory scope of the tensor, can be nullptr, which indicate the default global DRAM \\param out_data The allocated device pointer. \\return 0 when success, nonzero when failure happens"],["TVMDeviceCopyDataFromTo","\\brief Copy data from one place to another. \\note This API is designed to support special memory with shape dependent layout. We pass in DLTensor* with shape information to support these cases. \\param from The source tensor. \\param to The target tensor. \\param stream Optional stream object. \\return 0 when success, nonzero when failure happens."],["TVMDeviceFreeDataSpace","\\brief Free a data space on device. \\param dev The device to perform operation. \\param ptr The data space. \\return 0 when success, nonzero when failure happens"],["TVMFuncCall","\\brief Call a Packed TVM Function."],["TVMFuncCreateFromCFunc","\\brief Wrap a TVMPackedCFunc to become a FunctionHandle."],["TVMFuncFree","\\brief Free the function when it is no longer needed. \\param func The function handle \\return 0 when success, nonzero when failure happens"],["TVMFuncGetGlobal","\\brief Get a global function."],["TVMFuncListGlobalNames","\\brief List all the globally registered function name \\param out_size The number of functions \\param out_array The array of function names. \\return 0 when success, nonzero when failure happens"],["TVMFuncRegisterGlobal","\\brief Register the function to runtime’s global table."],["TVMFuncRemoveGlobal","\\brief Remove a global function. \\param name The name of the function."],["TVMGetLastError","\\brief return str message of the last error all function in this file will return 0 when success and nonzero when an error occurred, TVMGetLastError can be called to retrieve the error"],["TVMModFree","\\brief Free the Module \\param mod The module to be freed."],["TVMModGetFunction","\\brief Get function from the module. \\param mod The module handle. \\param func_name The name of the function. \\param query_imports Whether to query imported modules \\param out The result function, can be NULL if it is not available. \\return 0 when no error is thrown, nonzero when failure happens"],["TVMModImport","\\brief Add dep to mod’s dependency. This allows functions in this module to use modules."],["TVMModLoadFromFile","\\brief Load module from file. \\param file_name The file name to load the module from. \\param format The format of the module. \\param out The result module"],["TVMObjectDerivedFrom","\\brief Check that an object is derived from another. \\param child_type_index The type index of the derived type. \\param parent_type_index The type index of the parent type. \\param is_derived A boolean representing whether this predicate holds. \\return 0 when success, nonzero when failure happens."],["TVMObjectFree","\\brief Free the object."],["TVMObjectGetTypeIndex","\\brief Get the type_index from an object."],["TVMObjectRetain","\\brief Increase the reference count of an object."],["TVMObjectTypeIndex2Key","\\brief Convert type index to type key. \\param tindex The type index. \\param out_type_key The output type key. \\return 0 when success, nonzero when failure happens"],["TVMObjectTypeKey2Index","\\brief Convert type key to type index. \\param type_key The key of the type. \\param out_tindex the corresponding type index. \\return 0 when success, nonzero when failure happens"],["TVMSetStream","\\brief Set the runtime stream of current thread to be stream. The subsequent calls to the same device_type will use the setted stream handle. The specific type of stream is runtime device dependent."],["TVMStreamCreate","\\brief Create a new runtime stream."],["TVMStreamFree","\\brief Free a created stream handle."],["TVMStreamStreamSynchronize","\\brief Synchronize two streams of execution."],["TVMSynchronize","\\brief Wait until all computations on stream completes."]],"struct":[["DLDataType","\\brief The data type the tensor can hold."],["DLDevice","\\brief A Device for Tensor and operator."],["DLManagedTensor","\\brief C Tensor object, manage memory of DLTensor. This data structure is intended to facilitate the borrowing of DLTensor by another framework. It is not meant to transfer the tensor. When the borrowing framework doesn’t need the tensor, it should call the deleter to notify the host that the resource is no longer needed."],["DLTensor","\\brief Plain C Tensor object, does not manage memory."],["TVMByteArray","\\brief Byte array type used to pass in byte array When kTVMBytes is used as data type."],["TVMParallelGroupEnv","\\brief Environment for TVM parallel task."],["__fsid_t",""]],"type":[["BackendPackedCFunc",""],["DLDataTypeCode","\\brief The type code options DLDataType."],["DLDeviceType","\\brief The device type in DLDevice."],["FTVMParallelLambda","\\brief The callback function to execute a parallel lambda \\param task_id the task id of the function. \\param penv The parallel environment backs the execution. \\param cdata The supporting closure data."],["TVMArgTypeCode","\\brief The type code in used and only used in TVM FFI for argument passing."],["TVMArrayHandle","\\brief the array handle"],["TVMBackendPackedCFunc","\\brief Signature for backend functions exported as DLL."],["TVMDeviceExtType","\\brief Extension device types in TVM"],["TVMExtensionFuncDeclarer","\\brief Signature for extension function declarer."],["TVMFunctionHandle","\\brief Handle to packed function handle."],["TVMModuleHandle","\\brief Handle to TVM runtime modules."],["TVMObjectHandle","\\brief Handle to Object."],["TVMPackedCFunc","\\brief C type of packed function."],["TVMPackedCFuncFinalizer","\\brief C callback to free the resource handle in C packed function. \\param resource_handle The handle additional resouce handle from fron-end."],["TVMRetValueHandle","\\brief Handle to hold return value."],["TVMStreamHandle","\\brief The stream that is specific to device can be NULL, which indicates the default one."],["__blkcnt64_t",""],["__blkcnt_t",""],["__blksize_t",""],["__caddr_t",""],["__clock_t",""],["__clockid_t",""],["__daddr_t",""],["__dev_t",""],["__fsblkcnt64_t",""],["__fsblkcnt_t",""],["__fsfilcnt64_t",""],["__fsfilcnt_t",""],["__fsword_t",""],["__gid_t",""],["__id_t",""],["__ino64_t",""],["__ino_t",""],["__int16_t",""],["__int32_t",""],["__int64_t",""],["__int8_t",""],["__intmax_t",""],["__intptr_t",""],["__key_t",""],["__loff_t",""],["__mode_t",""],["__nlink_t",""],["__off64_t",""],["__off_t",""],["__pid_t",""],["__quad_t",""],["__rlim64_t",""],["__rlim_t",""],["__sig_atomic_t",""],["__socklen_t",""],["__ssize_t",""],["__suseconds_t",""],["__syscall_slong_t",""],["__syscall_ulong_t",""],["__time_t",""],["__timer_t",""],["__u_char",""],["__u_int",""],["__u_long",""],["__u_quad_t",""],["__u_short",""],["__uid_t",""],["__uint16_t",""],["__uint32_t",""],["__uint64_t",""],["__uint8_t",""],["__uintmax_t",""],["__useconds_t",""],["int_fast16_t",""],["int_fast32_t",""],["int_fast64_t",""],["int_fast8_t",""],["int_least16_t",""],["int_least32_t",""],["int_least64_t",""],["int_least8_t",""],["intmax_t",""],["size_t",""],["tvm_index_t","\\brief type of array index."],["uint_fast16_t",""],["uint_fast32_t",""],["uint_fast64_t",""],["uint_fast8_t",""],["uint_least16_t",""],["uint_least32_t",""],["uint_least64_t",""],["uint_least8_t",""],["uintmax_t",""],["wchar_t",""]],"union":[["TVMValue","\\brief Union type of values being passed through API and function calls."]]});