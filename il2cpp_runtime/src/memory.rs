use anyhow::Result;

#[cfg(unix)]
use std::fs::File;
#[cfg(unix)]
use std::io::{Read, Seek, SeekFrom};

#[cfg(windows)]
use anyhow::anyhow;

pub struct ProcessMemory {
    pub pid: u32,
    #[cfg(unix)]
    mem_file: File,
    #[cfg(windows)]
    process_handle: windows::Win32::Foundation::HANDLE,
}

impl ProcessMemory {
    pub fn new(pid: u32) -> Result<Self> {
        #[cfg(unix)]
        {
            let mem_path = format!("/proc/{}/mem", pid);
            let mem_file = File::open(mem_path)?;
            Ok(Self { pid, mem_file })
        }

        #[cfg(windows)]
        {
            use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};

            unsafe {
                let process_handle = OpenProcess(PROCESS_ALL_ACCESS, false, pid)?;
                if process_handle.is_invalid() {
                    return Err(anyhow!(
                        "Failed to open process {}. Try running as Administrator.",
                        pid
                    ));
                }
                Ok(Self {
                    pid,
                    process_handle,
                })
            }
        }

        #[cfg(not(any(unix, windows)))]
        {
            let _ = pid;
            anyhow::bail!("ProcessMemory is not supported on this target")
        }
    }

    pub fn read(&mut self, addr: u64, buf: &mut [u8]) -> Result<()> {
        #[cfg(unix)]
        {
            self.mem_file.seek(SeekFrom::Start(addr))?;
            self.mem_file.read_exact(buf)?;
            Ok(())
        }

        #[cfg(windows)]
        {
            use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;

            unsafe {
                let mut bytes_read = 0usize;
                ReadProcessMemory(
                    self.process_handle,
                    addr as *const _,
                    buf.as_mut_ptr() as *mut _,
                    buf.len(),
                    Some(&mut bytes_read),
                )?;

                if bytes_read != buf.len() {
                    return Err(anyhow!(
                        "Incomplete read from process memory at address 0x{:X}: requested {} bytes, got {} bytes",
                        addr,
                        buf.len(),
                        bytes_read
                    ));
                }
                Ok(())
            }
        }

        #[cfg(not(any(unix, windows)))]
        {
            let _ = (addr, buf);
            anyhow::bail!("Reading process memory is not supported on this target")
        }
    }

    pub fn read_bytes(&mut self, addr: u64, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; size];
        self.read(addr, &mut buf)?;
        Ok(buf)
    }

    pub fn read_pointer(&mut self, addr: u64) -> Result<u64> {
        let bytes = self.read_bytes(addr, 8)?;
        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_i8(&mut self, addr: u64) -> Result<i8> {
        let bytes = self.read_bytes(addr, 1)?;
        Ok(i8::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_u8(&mut self, addr: u64) -> Result<u8> {
        let bytes = self.read_bytes(addr, 1)?;
        Ok(bytes[0])
    }

    pub fn read_i16(&mut self, addr: u64) -> Result<i16> {
        let bytes = self.read_bytes(addr, 2)?;
        Ok(i16::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_u16(&mut self, addr: u64) -> Result<u16> {
        let bytes = self.read_bytes(addr, 2)?;
        Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_i32(&mut self, addr: u64) -> Result<i32> {
        let bytes = self.read_bytes(addr, 4)?;
        Ok(i32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_f32(&mut self, addr: u64) -> Result<f32> {
        let bytes = self.read_bytes(addr, 4)?;
        Ok(f32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_i64(&mut self, addr: u64) -> Result<i64> {
        let bytes = self.read_bytes(addr, 8)?;
        Ok(i64::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_f64(&mut self, addr: u64) -> Result<f64> {
        let bytes = self.read_bytes(addr, 8)?;
        Ok(f64::from_le_bytes(bytes.try_into().unwrap()))
    }

    #[cfg(windows)]
    pub fn process_handle(&self) -> windows::Win32::Foundation::HANDLE {
        self.process_handle
    }

    #[cfg(windows)]
    pub fn get_process_handle(&self) -> windows::Win32::Foundation::HANDLE {
        self.process_handle
    }
}

#[cfg(windows)]
impl Drop for ProcessMemory {
    fn drop(&mut self) {
        unsafe {
            use windows::Win32::Foundation::CloseHandle;
            let _ = CloseHandle(self.process_handle);
        }
    }
}
