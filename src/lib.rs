#![allow(non_camel_case_types,non_upper_case_globals)]

pub use ::std::os::raw::{c_void, c_char};

// Timeout value.
/// Wait forever timeout value.
pub const osWaitForever: u32 = 0xffffffff;

// Flags options ([`osThreadFlagsWait`] and [`osEventFlagsWait`]).
/// Wait for any flag (default).
pub const osFlagsWaitAny: i32 = 0;
/// Wait for all flags.
pub const osFlagsWaitAll: i32 = 1;
/// Do not clear flags which have been specified to wait for.
pub const osFlagsNoClear: i32 = 2;

// Thread attributes (attr_bits in [`osThreadAttr_t`]).
/// Thread created in detached state (default)
pub const osThreadDetached: u32 = 0;
/// Thread created in joinable state
pub const osThreadJoinable: u32 = 1;

// Mutex attributes (attr_bits in [`osMutexAttr_t`]).
/// Recursive mutex.
pub const osMutexRecursive:   u32 = 1;
/// Priority inherit protocol.
pub const osMutexPrioInherit: u32 = 2;
/// Robust mutex.
pub const osMutexRobust:      u32 = 8;




/// Version information.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osVersion_t {
    /// API version (major.minor.rev: mmnnnrrrr dec).
    pub api: u32,
    /// Kernel version (major.minor.rev: mmnnnrrrr dec).
    pub kernel: u32,
}

/// Kernel state.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum osKernelState_t {
    osKernelInactive = 0,
    osKernelReady = 1,
    osKernelRunning = 2,
    osKernelLocked = 3,
    osKernelSuspended = 4,
    osKernelError = -1,
}

/// Thread state.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum osThreadState_t {
    osThreadInactive = 0,
    osThreadReady = 1,
    osThreadRunning = 2,
    osThreadBlocked = 3,
    osThreadTerminated = 4,
    osThreadError = -1,
}

/// Priority values.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum osPriority_t {
    osPriorityNone = 0,
    osPriorityIdle = 1,
    osPriorityLow = 8,
    osPriorityLow1 = 9,
    osPriorityLow2 = 10,
    osPriorityLow3 = 11,
    osPriorityLow4 = 12,
    osPriorityLow5 = 13,
    osPriorityLow6 = 14,
    osPriorityLow7 = 15,
    osPriorityBelowNormal = 16,
    osPriorityBelowNormal1 = 17,
    osPriorityBelowNormal2 = 18,
    osPriorityBelowNormal3 = 19,
    osPriorityBelowNormal4 = 20,
    osPriorityBelowNormal5 = 21,
    osPriorityBelowNormal6 = 22,
    osPriorityBelowNormal7 = 23,
    osPriorityNormal = 24,
    osPriorityNormal1 = 25,
    osPriorityNormal2 = 26,
    osPriorityNormal3 = 27,
    osPriorityNormal4 = 28,
    osPriorityNormal5 = 29,
    osPriorityNormal6 = 30,
    osPriorityNormal7 = 31,
    osPriorityAboveNormal = 32,
    osPriorityAboveNormal1 = 33,
    osPriorityAboveNormal2 = 34,
    osPriorityAboveNormal3 = 35,
    osPriorityAboveNormal4 = 36,
    osPriorityAboveNormal5 = 37,
    osPriorityAboveNormal6 = 38,
    osPriorityAboveNormal7 = 39,
    osPriorityHigh = 40,
    osPriorityHigh1 = 41,
    osPriorityHigh2 = 42,
    osPriorityHigh3 = 43,
    osPriorityHigh4 = 44,
    osPriorityHigh5 = 45,
    osPriorityHigh6 = 46,
    osPriorityHigh7 = 47,
    osPriorityRealtime = 48,
    osPriorityRealtime1 = 49,
    osPriorityRealtime2 = 50,
    osPriorityRealtime3 = 51,
    osPriorityRealtime4 = 52,
    osPriorityRealtime5 = 53,
    osPriorityRealtime6 = 54,
    osPriorityRealtime7 = 55,
    osPriorityISR = 56,
    osPriorityError = -1,
}

/// Entry point of a thread.
pub type os_thread_func_t = unsafe extern "C" fn(argument: *mut ::c_void);

/// Entry point of a timer call back function.
pub type os_timer_func_t = unsafe extern "C" fn(argument: *mut ::c_void);


/// Timer type.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum osTimerType_t { osTimerOnce = 0, osTimerPeriodic = 1, }

/// Status code values returned by CMSIS-RTOS functions.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum osStatus_t {
    osOK = 0,
    osError = -1,
    osErrorTimeout = -2,
    osErrorResource = -3,
    osErrorParameter = -4,
    osErrorNoMemory = -5,
    osErrorISR = -6,
}

/// Thread ID identifies the thread.
pub type osThreadId_t = *mut ::c_void;

/// Timer ID identifies the timer.
pub type osTimerId_t = *mut ::c_void;

/// Event Flags ID identifies the event flags.
pub type osEventFlagsId_t = *mut ::c_void;

/// Mutex ID identifies the mutex.
pub type osMutexId_t = *mut ::c_void;

/// Semaphore ID identifies the semaphore.
pub type osSemaphoreId_t = *mut ::c_void;

/// Memory Pool ID identifies the memory pool.
pub type osMemoryPoolId_t = *mut ::c_void;

/// Message Queue ID identifies the message queue.
pub type osMessageQueueId_t = *mut ::c_void;

/// Data type that identifies secure software modules called by a process.
pub type TZ_ModuleId_t = u32;

/// Attributes structure for thread.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osThreadAttr_t {
    /// name of the thread
    pub name: *const ::c_char,
    /// attribute bits
    pub attr_bits: u32,
    /// memory for control block
    pub cb_mem: *mut ::c_void,
    /// size of provided memory for control block
    pub cb_size: u32,
    /// memory for stack
    pub stack_mem: *mut ::c_void,
    /// size of stack
    pub stack_size: u32,
    /// initial thread priority (default: osPriorityNormal)
    pub priority: osPriority_t,
    /// TrustZone module identifier
    pub tz_module: TZ_ModuleId_t,
    /// reserved (must be 0)
    pub reserved: u32,
}

/// Attributes structure for timer.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osTimerAttr_t {
    /// name of the timer
    pub name: *const ::c_char,
    /// attribute bits
    pub attr_bits: u32,
    /// memory for control block
    pub cb_mem: *mut ::c_void,
    /// size of provided memory for control block
    pub cb_size: u32,
}

/// Attributes structure for event flags.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osEventFlagsAttr_t {
    /// name of the event flags
    pub name: *const ::c_char,
    /// attribute bits
    pub attr_bits: u32,
    /// memory for control block
    pub cb_mem: *mut ::c_void,
    /// size of provided memory for control block
    pub cb_size: u32,
}

/// Attributes structure for mutex.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osMutexAttr_t {
    /// name of the mutex
    pub name: *const ::c_char,
    /// attribute bits
    pub attr_bits: u32,
    /// memory for control block
    pub cb_mem: *mut ::c_void,
    /// size of provided memory for control block
    pub cb_size: u32,
}

/// Attributes structure for semaphore.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osSemaphoreAttr_t {
    /// name of the semaphore
    pub name: *const ::c_char,
    /// attribute bits
    pub attr_bits: u32,
    /// memory for control block
    pub cb_mem: *mut ::c_void,
    /// size of provided memory for control block
    pub cb_size: u32,
}

/// Attributes structure for memory pool.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osMemoryPoolAttr_t {
    /// name of the memory pool
    pub name: *const ::c_char,
    /// attribute bits
    pub attr_bits: u32,
    /// memory for control block
    pub cb_mem: *mut ::c_void,
    /// size of provided memory for control block
    pub cb_size: u32,
    /// memory for data storage
    pub mp_mem: *mut ::c_void,
    /// size of provided memory for data storage 
    pub mp_size: u32,
}

/// Attributes structure for message queue.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct osMessageQueueAttr_t {
    /// name of the message queue
    pub name: *const ::c_char,
    /// attribute bits
    pub attr_bits: u32,
    /// memory for control block
    pub cb_mem: *mut ::c_void,
    /// size of provided memory for control block
    pub cb_size: u32,
    /// memory for data storage
    pub mq_mem: *mut ::c_void,
    /// size of provided memory for data storage 
    pub mq_size: u32,
}

extern "C" {
    /// Initialize the RTOS Kernel.
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osKernelInitialize() -> osStatus_t;

    /// Get RTOS Kernel Information.
    ///
    /// # Parameters
    ///
    /// * [out]    `version`       Pointer to buffer for retrieving version information.
    /// * [out]    `id_buf`        Pointer to buffer for retrieving kernel identification string.
    /// * [in]     `id_size`       Size of buffer for kernel identification string.
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osKernelGetInfo(version: *mut osVersion_t, id_buf: *mut ::c_char, id_size: u32) -> osStatus_t;

    /// Get the current RTOS Kernel state.
    ///
    /// # Returns
    ///
    /// Current RTOS Kernel state.
    pub fn osKernelGetState() -> osKernelState_t;

    /// Start the RTOS Kernel scheduler.
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osKernelStart() -> osStatus_t;

    /// Lock the RTOS Kernel scheduler.
    ///
    /// # Returns
    ///
    /// 0 already locked, 1 locked.
    pub fn osKernelLock() -> u32;

    /// Unlock the RTOS Kernel scheduler.
    pub fn osKernelUnlock();

    /// Suspend the RTOS Kernel scheduler.
    ///
    /// # Returns
    ///
    /// Time in ticks, for how long the system can sleep or power-down.
    pub fn osKernelSuspend() -> u32;

    /// Resume the RTOS Kernel scheduler.
    ///
    /// # Parameters
    ///
    /// * [in]     `sleep_ticks`   Time in ticks for how long the system was in sleep or power-down mode.
    pub fn osKernelResume(sleep_ticks: u32);

    /// Get the RTOS kernel tick count.
    ///
    /// # Returns
    ///
    /// RTOS kernel current tick count.
    pub fn osKernelGetTickCount() -> u64;

    /// Get the RTOS kernel tick frequency.
    ///
    /// # Returns
    ///
    /// Frequency of the kernel tick.
    pub fn osKernelGetTickFreq() -> u32;

    /// Get the RTOS kernel system timer count.
    ///
    /// # Returns
    ///
    /// RTOS kernel current system timer count as 32-bit value.
    pub fn osKernelGetSysTimerCount() -> u32;

    /// Get the RTOS kernel system timer frequency.
    ///
    /// # Returns
    ///
    /// Frequency of the system timer.
    pub fn osKernelGetSysTimerFreq() -> u32;

    /// Create a thread and add it to Active Threads.
    ///
    /// # Parameters
    ///
    /// * [in]     `func`          Thread function.
    /// * [in]     `argument`      Pointer that is passed to the thread function as start argument.
    /// * [in]     `attr`          Thread attributes; NULL: default values.
    ///
    /// # Returns
    ///
    /// Thread ID for reference by other functions or NULL in case of error.
    pub fn osThreadNew(func: os_thread_func_t, argument: *mut ::c_void, attr: *const osThreadAttr_t) -> osThreadId_t;

    /// Get name of a thread.
    ///
    /// # Parameters
    ///
    /// * [in]     `thread_id`     Thread ID obtained by [`osThreadNew`] or [`osThreadGetId`].
    ///
    /// # Returns
    ///
    /// Name as NULL terminated string.
    pub fn osThreadGetName(thread_id: osThreadId_t) -> *const ::c_char;

    /// Return the thread ID of the current running thread.
    ///
    /// # Returns
    ///
    /// Thread ID for reference by other functions or NULL in case of error.
    pub fn osThreadGetId() -> osThreadId_t;

    /// Get current thread state of a thread.
    ///
    /// # Parameters
    ///
    /// * [in]     `thread_id`     Thread ID obtained by [`osThreadNew`] or [`osThreadGetId`].
    ///
    /// # Returns
    ///
    /// Current thread state of the specified thread.
    pub fn osThreadGetState(thread_id: osThreadId_t) -> osThreadState_t;

    /// Get stack size of a thread.
    ///
    /// # Parameters
    ///
    /// * [in]     `thread_id`     Thread ID obtained by [`osThreadNew`] or [`osThreadGetId`].
    ///
    /// # Returns
    ///
    /// Stack size in bytes.
    pub fn osThreadGetStackSize(thread_id: osThreadId_t) -> u32;

    /// Get available stack space of a thread based on stack watermark recording during execution.
    ///
    /// # Parameters
    ///
    /// * [in]     `thread_id`     Thread ID obtained by [`osThreadNew`] or [`osThreadGetId`].
    ///
    /// # Returns
    ///
    /// Remaining stack space in bytes.
    pub fn osThreadGetStackSpace(thread_id: osThreadId_t) -> u32;

    /// Change priority of a thread.
    ///
    /// # Parameters
    ///
    /// * [in]     `thread_id`     Thread ID obtained by [`osThreadNew`] or [`osThreadGetId`].
    /// * [in]     `priority`      New priority value for the thread function.
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osThreadSetPriority(thread_id: osThreadId_t, priority: osPriority_t) -> osStatus_t;

    /// Get current priority of a thread.
    ///
    /// # Parameters
    ///
    /// * [in]     `thread_id`     Thread ID obtained by [`osThreadNew`] or [`osThreadGetId`].
    ///
    /// # Returns
    ///
    /// Current priority value of the specified thread.
    pub fn osThreadGetPriority(thread_id: osThreadId_t) -> osPriority_t;

    /// Pass control to next thread that is in state **READY**.
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osThreadYield() -> osStatus_t;

    /// Suspend execution of a thread.
    ///
    /// # Parameters
    ///
    /// * [in]     `thread_id`     Thread ID obtained by [`osThreadNew`] or [`osThreadGetId`].
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osThreadSuspend(thread_id: osThreadId_t) -> osStatus_t;

    /// Resume execution of a thread.
    ///
    /// # Parameters
    ///
    /// * [in]     `thread_id`     Thread ID obtained by [`osThreadNew`] or [`osThreadGetId`].
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osThreadResume(thread_id: osThreadId_t) -> osStatus_t;

    /// Detach a thread (thread storage can be reclaimed when thread terminates).
    ///
    /// # Parameters
    ///
    /// * [in]     `thread_id`     Thread ID obtained by [`osThreadNew`] or [`osThreadGetId`].
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osThreadDetach(thread_id: osThreadId_t) -> osStatus_t;

    /// Wait for specified thread to terminate.
    ///
    /// # Parameters
    ///
    /// * [in]     `thread_id`     Thread ID obtained by [`osThreadNew`] or [`osThreadGetId`].
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osThreadJoin(thread_id: osThreadId_t) -> osStatus_t;

    /// Terminate execution of current running thread.
    pub fn osThreadExit();

    /// Terminate execution of a thread.
    ///
    /// # Parameters
    ///
    /// * [in]     `thread_id`     Thread ID obtained by [`osThreadNew`] or [`osThreadGetId`].
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osThreadTerminate(thread_id: osThreadId_t) -> osStatus_t;

    /// Get number of active threads.
    ///
    /// # Returns
    ///
    /// Number of active threads.
    pub fn osThreadGetCount() -> u32;

    /// Enumerate active threads.
    ///
    /// # Parameters
    ///
    /// * [out]    `thread_array`  Pointer to array for retrieving thread IDs.
    /// * [in]     `array_items`   Maximum number of items in array for retrieving thread IDs.
    ///
    /// # Returns
    ///
    /// Number of enumerated threads.
    pub fn osThreadEnumerate(thread_array: *mut osThreadId_t, array_items: u32) -> u32;

    /// Set the specified Thread Flags of a thread.
    ///
    /// # Parameters
    ///
    /// * [in]     `thread_id`     Thread ID obtained by [`osThreadNew`] or [`osThreadGetId`].
    /// * [in]     `flags`         Specifies the flags of the thread that shall be set.
    ///
    /// # Returns
    ///
    /// Thread flags after setting or error code if negative.
    pub fn osThreadFlagsSet(thread_id: osThreadId_t, flags: i32) -> i32;

    /// Clear the specified Thread Flags of current running thread.
    ///
    /// # Parameters
    ///
    /// * [in]     `flags`         Specifies the flags of the thread that shall be cleared.
    ///
    /// # Returns
    ///
    /// Thread flags before clearing or error code if negative.
    pub fn osThreadFlagsClear(flags: i32) -> i32;

    /// Get the current Thread Flags of current running thread.
    ///
    /// # Returns
    ///
    /// Current thread flags.
    pub fn osThreadFlagsGet() -> i32;

    /// Wait for one or more Thread Flags of the current running thread to become signaled.
    ///
    /// # Parameters
    ///
    /// * [in]     `flags`         Specifies the flags to wait for.
    /// * [in]     `options`       Specifies flags options (osFlagsXxxx).
    /// * [in]     `timeout`       [`CMSIS_RTOS_TimeOutValue`] or 0 in case of no time-out.
    ///
    /// # Returns
    ///
    /// Thread flags before clearing or error code if negative.
    pub fn osThreadFlagsWait(flags: i32, options: u32, timeout: u32) -> i32;

    /// Wait for Timeout (Time Delay).
    ///
    /// # Parameters
    ///
    /// * [in]     `ticks`         [`CMSIS_RTOS_TimeOutValue`] "time ticks" value
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osDelay(ticks: u32) -> osStatus_t;

    /// Wait until specified time.
    ///
    /// # Parameters
    ///
    /// * [in]     `ticks`         Absolute time in ticks
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osDelayUntil(ticks: u64) -> osStatus_t;

    /// Create and Initialize a timer.
    ///
    /// # Parameters
    ///
    /// * [in]     `func`          Start address of a timer call back function.
    /// * [in]     `type`          OsTimerOnce for one-shot or osTimerPeriodic for periodic behavior.
    /// * [in]     `argument`      Argument to the timer call back function.
    /// * [in]     `attr`          Timer attributes; NULL: default values.
    ///
    /// # Returns
    ///
    /// Timer ID for reference by other functions or NULL in case of error.
    pub fn osTimerNew(func: os_timer_func_t, type_: osTimerType_t, argument: *mut ::c_void, attr: *const osTimerAttr_t) -> osTimerId_t;

    /// Get name of a timer.
    ///
    /// # Parameters
    ///
    /// * [in]     `timer_id`      Timer ID obtained by [`osTimerNew`].
    ///
    /// # Returns
    ///
    /// Name as NULL terminated string.
    pub fn osTimerGetName(timer_id: osTimerId_t) -> *const ::c_char;

    /// Start or restart a timer.
    ///
    /// # Parameters
    ///
    /// * [in]     `timer_id`      Timer ID obtained by [`osTimerNew`].
    /// * [in]     `ticks`         [`CMSIS_RTOS_TimeOutValue`] "time ticks" value of the timer.
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osTimerStart(timer_id: osTimerId_t, ticks: u32) -> osStatus_t;

    /// Stop a timer.
    ///
    /// # Parameters
    ///
    /// * [in]     `timer_id`      Timer ID obtained by [`osTimerNew`].
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osTimerStop(timer_id: osTimerId_t) -> osStatus_t;

    /// Check if a timer is running.
    ///
    /// # Parameters
    ///
    /// * [in]     `timer_id`      Timer ID obtained by [`osTimerNew`].
    ///
    /// # Returns
    ///
    /// 0 not running, 1 running.
    pub fn osTimerIsRunning(timer_id: osTimerId_t) -> u32;

    /// Delete a timer.
    ///
    /// # Parameters
    ///
    /// * [in]     `timer_id`      Timer ID obtained by [`osTimerNew`].
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osTimerDelete(timer_id: osTimerId_t) -> osStatus_t;

    /// Create and Initialize an Event Flags object.
    ///
    /// # Parameters
    ///
    /// * [in]     `attr`          Event flags attributes; NULL: default values.
    ///
    /// # Returns
    ///
    /// Event flags ID for reference by other functions or NULL in case of error.
    pub fn osEventFlagsNew(attr: *const osEventFlagsAttr_t) -> osEventFlagsId_t;

    /// Get name of an Event Flags object.
    ///
    /// # Parameters
    ///
    /// * [in]     `ef_id`         Event flags ID obtained by [`osEventFlagsNew`].
    ///
    /// # Returns
    ///
    /// Name as NULL terminated string.
    pub fn osEventFlagsGetName(ef_id: osEventFlagsId_t) -> *const ::c_char;

    /// Set the specified Event Flags.
    ///
    /// # Parameters
    ///
    /// * [in]     `ef_id`         Event flags ID obtained by [`osEventFlagsNew`].
    /// * [in]     `flags`         Specifies the flags that shall be set.
    ///
    /// # Returns
    ///
    /// Event flags after setting or error code if negative.
    pub fn osEventFlagsSet(ef_id: osEventFlagsId_t, flags: i32) -> i32;

    /// Clear the specified Event Flags.
    ///
    /// # Parameters
    ///
    /// * [in]     `ef_id`         Event flags ID obtained by [`osEventFlagsNew`].
    /// * [in]     `flags`         Specifies the flags that shall be cleared.
    ///
    /// # Returns
    ///
    /// Event flags before clearing or error code if negative.
    pub fn osEventFlagsClear(ef_id: osEventFlagsId_t, flags: i32) -> i32;

    /// Get the current Event Flags.
    ///
    /// # Parameters
    ///
    /// * [in]     `ef_id`         Event flags ID obtained by [`osEventFlagsNew`].
    ///
    /// # Returns
    ///
    /// Current event flags.
    pub fn osEventFlagsGet(ef_id: osEventFlagsId_t) -> i32;

    /// Wait for one or more Event Flags to become signaled.
    ///
    /// # Parameters
    ///
    /// * [in]     `ef_id`         Event flags ID obtained by [`osEventFlagsNew`].
    /// * [in]     `flags`         Specifies the flags to wait for.
    /// * [in]     `options`       Specifies flags options (osFlagsXxxx).
    /// * [in]     `timeout`       [`CMSIS_RTOS_TimeOutValue`] or 0 in case of no time-out.
    ///
    /// # Returns
    ///
    /// Event flags before clearing or error code if negative.
    pub fn osEventFlagsWait(ef_id: osEventFlagsId_t, flags: i32, options: u32, timeout: u32) -> i32;

    /// Delete an Event Flags object.
    ///
    /// # Parameters
    ///
    /// * [in]     `ef_id`         Event flags ID obtained by [`osEventFlagsNew`].
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osEventFlagsDelete(ef_id: osEventFlagsId_t) -> osStatus_t;

    /// Create and Initialize a Mutex object.
    ///
    /// # Parameters
    ///
    /// * [in]     `attr`          Mutex attributes; NULL: default values.
    ///
    /// # Returns
    ///
    /// Mutex ID for reference by other functions or NULL in case of error.
    pub fn osMutexNew(attr: *const osMutexAttr_t) -> osMutexId_t;

    /// Get name of a Mutex object.
    ///
    /// # Parameters
    ///
    /// * [in]     `mutex_id`      Mutex ID obtained by [`osMutexNew`].
    ///
    /// # Returns
    ///
    /// Name as NULL terminated string.
    pub fn osMutexGetName(mutex_id: osMutexId_t) -> *const ::c_char;

    /// Acquire a Mutex or timeout if it is locked.
    ///
    /// # Parameters
    ///
    /// * [in]     `mutex_id`      Mutex ID obtained by [`osMutexNew`].
    /// * [in]     `timeout`       [`CMSIS_RTOS_TimeOutValue`] or 0 in case of no time-out.
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osMutexAcquire(mutex_id: osMutexId_t, timeout: u32) -> osStatus_t;

    /// Release a Mutex that was acquired by [`osMutexAcquire`].
    ///
    /// # Parameters
    ///
    /// * [in]     `mutex_id`      Mutex ID obtained by [`osMutexNew`].
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osMutexRelease(mutex_id: osMutexId_t) -> osStatus_t;

    /// Get Thread which owns a Mutex object.
    ///
    /// # Parameters
    ///
    /// * [in]     `mutex_id`      Mutex ID obtained by [`osMutexNew`].
    ///
    /// # Returns
    ///
    /// Thread ID of owner thread or NULL when mutex was not acquired.
    pub fn osMutexGetOwner(mutex_id: osMutexId_t) -> osThreadId_t;

    /// Delete a Mutex object.
    ///
    /// # Parameters
    ///
    /// * [in]     `mutex_id`      Mutex ID obtained by [`osMutexNew`].
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osMutexDelete(mutex_id: osMutexId_t) -> osStatus_t;

    /// Create and Initialize a Semaphore object.
    ///
    /// # Parameters
    ///
    /// * [in]     `max_count`     Maximum number of available tokens.
    /// * [in]     `initial_count` Initial number of available tokens.
    /// * [in]     `attr`          Semaphore attributes; NULL: default values.
    ///
    /// # Returns
    ///
    /// Semaphore ID for reference by other functions or NULL in case of error.
    pub fn osSemaphoreNew(max_count: u32, initial_count: u32, attr: *const osSemaphoreAttr_t) -> osSemaphoreId_t;

    /// Get name of a Semaphore object.
    ///
    /// # Parameters
    ///
    /// * [in]     `semaphore_id`  Semaphore ID obtained by [`osSemaphoreNew`].
    ///
    /// # Returns
    ///
    /// Name as NULL terminated string.
    pub fn osSemaphoreGetName(semaphore_id: osSemaphoreId_t) -> *const ::c_char;

    /// Acquire a Semaphore token or timeout if no tokens are available.
    ///
    /// # Parameters
    ///
    /// * [in]     `semaphore_id`  Semaphore ID obtained by [`osSemaphoreNew`].
    /// * [in]     `timeout`       [`CMSIS_RTOS_TimeOutValue`] or 0 in case of no time-out.
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osSemaphoreAcquire(semaphore_id: osSemaphoreId_t, timeout: u32) -> osStatus_t;

    /// Release a Semaphore token that was acquired by [`osSemaphoreAcquire`].
    ///
    /// # Parameters
    ///
    /// * [in]     `semaphore_id`  Semaphore ID obtained by [`osSemaphoreNew`].
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osSemaphoreRelease(semaphore_id: osSemaphoreId_t) -> osStatus_t;

    /// Get current Semaphore token count.
    ///
    /// # Parameters
    ///
    /// * [in]     `semaphore_id`  Semaphore ID obtained by [`osSemaphoreNew`].
    ///
    /// # Returns
    ///
    /// Number of tokens available.
    pub fn osSemaphoreGetCount(semaphore_id: osSemaphoreId_t) -> u32;

    /// Delete a Semaphore object.
    ///
    /// # Parameters
    ///
    /// * [in]     `semaphore_id`  Semaphore ID obtained by [`osSemaphoreNew`].
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osSemaphoreDelete(semaphore_id: osSemaphoreId_t) -> osStatus_t;

    /// Create and Initialize a Memory Pool object.
    ///
    /// # Parameters
    ///
    /// * [in]     `block_count`   Maximum number of memory blocks in memory pool.
    /// * [in]     `block_size`    Memory block size in bytes.
    /// * [in]     `attr`          Memory pool attributes; NULL: default values.
    ///
    /// # Returns
    ///
    /// Memory pool ID for reference by other functions or NULL in case of error.
    pub fn osMemoryPoolNew(block_count: u32, block_size: u32, attr: *const osMemoryPoolAttr_t) -> osMemoryPoolId_t;

    /// Get name of a Memory Pool object.
    ///
    /// # Parameters
    ///
    /// * [in]     `mp_id`         Memory pool ID obtained by [`osMemoryPoolNew`].
    ///
    /// # Returns
    ///
    /// Name as NULL terminated string.
    pub fn osMemoryPoolGetName(mp_id: osMemoryPoolId_t) -> *const ::c_char;

    /// Allocate a memory block from a Memory Pool.
    ///
    /// # Parameters
    ///
    /// * [in]     `mp_id`         Memory pool ID obtained by [`osMemoryPoolNew`].
    /// * [in]     `timeout`       [`CMSIS_RTOS_TimeOutValue`] or 0 in case of no time-out.
    ///
    /// # Returns
    ///
    /// Address of the allocated memory block or NULL in case of no memory is available.
    pub fn osMemoryPoolAlloc(mp_id: osMemoryPoolId_t, timeout: u32) -> *mut ::c_void;

    /// Return an allocated memory block back to a Memory Pool.
    ///
    /// # Parameters
    ///
    /// * [in]     `mp_id`         Memory pool ID obtained by [`osMemoryPoolNew`].
    /// * [in]     `block`         Address of the allocated memory block to be returned to the memory pool.
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osMemoryPoolFree(mp_id: osMemoryPoolId_t, block: *mut ::c_void) -> osStatus_t;

    /// Get maximum number of memory blocks in a Memory Pool.
    ///
    /// # Parameters
    ///
    /// * [in]     `mp_id`         Memory pool ID obtained by [`osMemoryPoolNew`].
    ///
    /// # Returns
    ///
    /// Maximum number of memory blocks.
    pub fn osMemoryPoolGetCapacity(mp_id: osMemoryPoolId_t) -> u32;

    /// Get memory block size in a Memory Pool.
    ///
    /// # Parameters
    ///
    /// * [in]     `mp_id`         Memory pool ID obtained by [`osMemoryPoolNew`].
    ///
    /// # Returns
    ///
    /// Memory block size in bytes.
    pub fn osMemoryPoolGetBlockSize(mp_id: osMemoryPoolId_t) -> u32;

    /// Get number of memory blocks used in a Memory Pool.
    ///
    /// # Parameters
    ///
    /// * [in]     `mp_id`         Memory pool ID obtained by [`osMemoryPoolNew`].
    ///
    /// # Returns
    ///
    /// Number of memory blocks used.
    pub fn osMemoryPoolGetCount(mp_id: osMemoryPoolId_t) -> u32;

    /// Get number of memory blocks available in a Memory Pool.
    ///
    /// # Parameters
    ///
    /// * [in]     `mp_id`         Memory pool ID obtained by [`osMemoryPoolNew`].
    ///
    /// # Returns
    ///
    /// Number of memory blocks available.
    pub fn osMemoryPoolGetSpace(mp_id: osMemoryPoolId_t) -> u32;

    /// Delete a Memory Pool object.
    ///
    /// # Parameters
    ///
    /// * [in]     `mp_id`         Memory pool ID obtained by [`osMemoryPoolNew`].
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osMemoryPoolDelete(mp_id: osMemoryPoolId_t) -> osStatus_t;

    /// Create and Initialize a Message Queue object.
    ///
    /// # Parameters
    ///
    /// * [in]     `msg_count`     Maximum number of messages in queue.
    /// * [in]     `msg_size`      Maximum message size in bytes.
    /// * [in]     `attr`          Message queue attributes; NULL: default values.
    ///
    /// # Returns
    ///
    /// Message queue ID for reference by other functions or NULL in case of error.
    pub fn osMessageQueueNew(msg_count: u32, msg_size: u32, attr: *const osMessageQueueAttr_t) -> osMessageQueueId_t;

    /// Get name of a Message Queue object.
    ///
    /// # Parameters
    ///
    /// * [in]     `mq_id`         Message queue ID obtained by [`osMessageQueueNew`].
    ///
    /// # Returns
    ///
    /// Name as NULL terminated string.
    pub fn osMessageQueueGetName(mq_id: osMessageQueueId_t) -> *const ::c_char;

    /// Put a Message into a Queue or timeout if Queue is full.
    ///
    /// # Parameters
    ///
    /// * [in]     `mq_id`         Message queue ID obtained by [`osMessageQueueNew`].
    /// * [in]     `msg_ptr`       Pointer to buffer with message to put into a queue.
    /// * [in]     `msg_prio`      Message priority.
    /// * [in]     `timeout`       [`CMSIS_RTOS_TimeOutValue`] or 0 in case of no time-out.
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osMessageQueuePut(mq_id: osMessageQueueId_t, msg_ptr: *const ::c_void, msg_prio: u8, timeout: u32) -> osStatus_t;

    /// Get a Message from a Queue or timeout if Queue is empty.
    ///
    /// # Parameters
    ///
    /// * [in]     `mq_id`         Message queue ID obtained by [`osMessageQueueNew`].
    /// * [out]    `msg_ptr`       Pointer to buffer for message to get from a queue.
    /// * [out]    `msg_prio`      Pointer to buffer for message priority or NULL.
    /// * [in]     `timeout`       [`CMSIS_RTOS_TimeOutValue`] or 0 in case of no time-out.
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osMessageQueueGet(mq_id: osMessageQueueId_t, msg_ptr: *mut ::c_void, msg_prio: *mut u8, timeout: u32) -> osStatus_t;

    /// Get maximum number of messages in a Message Queue.
    ///
    /// # Parameters
    ///
    /// * [in]     `mq_id`         Message queue ID obtained by [`osMessageQueueNew`].
    ///
    /// # Returns
    ///
    /// Maximum number of messages.
    pub fn osMessageQueueGetCapacity(mq_id: osMessageQueueId_t) -> u32;

    /// Get maximum message size in a Memory Pool.
    ///
    /// # Parameters
    ///
    /// * [in]     `mq_id`         Message queue ID obtained by [`osMessageQueueNew`].
    ///
    /// # Returns
    ///
    /// Maximum message size in bytes.
    pub fn osMessageQueueGetMsgSize(mq_id: osMessageQueueId_t) -> u32;

    /// Get number of queued messages in a Message Queue.
    ///
    /// # Parameters
    ///
    /// * [in]     `mq_id`         Message queue ID obtained by [`osMessageQueueNew`].
    ///
    /// # Returns
    ///
    /// Number of queued messages.
    pub fn osMessageQueueGetCount(mq_id: osMessageQueueId_t) -> u32;

    /// Get number of available slots for messages in a Message Queue.
    ///
    /// # Parameters
    ///
    /// * [in]     `mq_id`         Message queue ID obtained by [`osMessageQueueNew`].
    ///
    /// # Returns
    ///
    /// Number of available slots for messages.
    pub fn osMessageQueueGetSpace(mq_id: osMessageQueueId_t) -> u32;

    /// Reset a Message Queue to initial empty state.
    ///
    /// # Parameters
    ///
    /// * [in]     `mq_id`         Message queue ID obtained by [`osMessageQueueNew`].
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osMessageQueueReset(mq_id: osMessageQueueId_t) -> osStatus_t;

    /// Delete a Message Queue object.
    ///
    /// # Parameters
    ///
    /// * [in]     `mq_id`         Message queue ID obtained by [`osMessageQueueNew`].
    ///
    /// # Returns
    ///
    /// Status code that indicates the execution status of the function.
    pub fn osMessageQueueDelete(mq_id: osMessageQueueId_t) -> osStatus_t;
}
