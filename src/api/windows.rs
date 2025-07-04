use std::error::Error;

use windows::Win32::System::Power::{
    ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED, SetThreadExecutionState,
};
use windows::Win32::System::ProcessStatus::EnumProcesses;

pub fn disable_sleep(display: bool) {
    unsafe {
        if display {
            SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED);
        } else {
            SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED);
        }
    }
}

pub fn enable_sleep() {
    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS);
    }
}

pub fn find_process(id: u32) -> Result<bool, Box<dyn Error>> {
    let mut id_arr: Vec<u32> = vec![0, 1024];
    let mut witten_bytes: Vec<u32> = vec![0, 1];
    loop {
        let buf_bytes = u32::try_from(id_arr.len() * 4)?;
        unsafe {
            EnumProcesses(id_arr.as_mut_ptr(), buf_bytes, witten_bytes.as_mut_ptr())?;
        }
        if witten_bytes[0] >= buf_bytes {
            id_arr.resize(id_arr.len() * 2, 0);
        } else {
            break;
        }
    }
    Ok(id_arr.contains(&id))
}
