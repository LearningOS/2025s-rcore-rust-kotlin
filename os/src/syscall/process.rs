#![allow(unused)]
//! Process management syscalls
use crate::{
    config::PAGE_SIZE,
    mm::{get_frame_space, translated_byte_buffer, MapPermission, PageTable, VPNRange, VirtAddr},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_syscall_count,
        map_memory_area, suspend_current_and_run_next, unmap_memory_area,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let sec = us / 1_000_000;
    let usec = us % 1_000_000;
    let new = TimeVal { sec, usec };
    let token = current_user_token();
    let ptr = ts as usize as *const u8;
    let len = core::mem::size_of::<TimeVal>();
    let buffer = translated_byte_buffer(token, ptr, len);
    let slice = unsafe { core::slice::from_raw_parts(&new as *const _ as *const u8, len) };
    let mut temp = 0;
    for i in buffer {
        i.copy_from_slice(&slice[temp..temp + i.len()]);
        temp += i.len();
    }
    debug!("kernel: sys_get_time: sec = {}, usec = {}", sec, usec);
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    if trace_request == 2 {
        return get_syscall_count(id) as isize;
    }
    let token = current_user_token();
    let page_table = PageTable::from_token(token);
    let va: VirtAddr = id.into();
    let vpn = va.floor();
    let entry = page_table.translate(vpn);
    if let Some(entry) = entry {
        if !entry.user_mode() {
            return -1;
        }
        if trace_request == 0 {
            if !entry.readable() {
                return -1;
            }
            entry.ppn().get_bytes_array()[va.page_offset()] as isize
        } else if trace_request == 1 {
            if !entry.writable() {
                return -1;
            }
            entry.ppn().get_bytes_array()[va.page_offset()] = data as u8;
            0
        } else {
            return -1;
        }
    } else {
        return -1;
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    trace!("kernel: sys_mmap");
    if start % PAGE_SIZE != 0 || prot & (!0x7) != 0 || prot & (0x7) == 0 {
        return -1;
    }
    let start_va = VirtAddr::from(start);
    let end_va = VirtAddr::from(start + len);
    let start_vpn = start_va.floor();
    let end_vpn = end_va.ceil();
    if get_frame_space() < end_vpn.0 - start_vpn.0 {
        return -1;
    }
    let vpn_range = VPNRange::new(start_vpn, end_vpn);
    let token = current_user_token();
    let page_table = PageTable::from_token(token);
    for vpn in vpn_range {
        let pte = page_table.translate(vpn);
        if pte.is_some() && pte.unwrap().is_valid() {
            return -1;
        }
    }
    let mut flags = MapPermission::U;
    if prot & 0x1 != 0 {
        flags |= MapPermission::R;
    }
    if prot & 0x2 != 0 {
        flags |= MapPermission::W;
    }
    if prot & 0x4 != 0 {
        flags |= MapPermission::X;
    }
    map_memory_area(vpn_range, flags);
    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    if start % PAGE_SIZE != 0 || len % PAGE_SIZE != 0 {
        return -1;
    }
    let start_va = VirtAddr::from(start);
    let end_va = VirtAddr::from(start + len);
    let start_vpn = start_va.floor();
    let end_vpn = end_va.ceil();
    let vpn_range = VPNRange::new(start_vpn, end_vpn);
    unmap_memory_area(vpn_range)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
