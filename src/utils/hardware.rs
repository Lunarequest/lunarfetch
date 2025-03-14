use byte_unit::{Byte, Unit};
use owo_colors::OwoColorize;
use pci_ids::Device;
use pci_info::PciInfo;
use procfs::{Current, Meminfo};
use sysinfo::System;

fn print_size_in_gb(size_in_bytes: u64) -> String {
    let byte = Byte::from_u64(size_in_bytes);
    let adjusted_byte = byte.get_adjusted_unit(Unit::GiB);
    format!("{:.2}", adjusted_byte)
}

pub fn ram() -> String {
    let mem = Meminfo::current().unwrap();
    let total_mem = mem.mem_total;

    let used_mem = mem.mem_total + mem.shmem.unwrap_or(0)
        - mem.mem_free
        - mem.buffers
        - mem.cached
        - mem.s_reclaimable.unwrap_or(0);

    let mut percent = used_mem as f64 / total_mem as f64;
    percent *= 100.0;

    format!(
        "{} / {} ({:.0}%)",
        print_size_in_gb(used_mem),
        print_size_in_gb(total_mem),
        percent
    )
}

pub fn cpu() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    let cpus = sys.cpus();
    cpus[0].brand().into()
}

pub fn gpu() -> String {
    let info = PciInfo::enumerate_pci().unwrap();
    let gpus: Vec<String> = info
        .iter()
        .filter(|d| match d {
            Ok(device) => {
                if device.device_class_code().unwrap() == 0x03 {
                    match Device::from_vid_pid(device.vendor_id(), device.device_id()) {
                        Some(_) => {
                            return true;
                        }
                        None => {
                            return false;
                        }
                    }
                }
                return false;
            }
            Err(_error) => {
                return false;
            }
        })
        .map(|d| {
            if let Ok(device) = d {
                if let Some(dev) = Device::from_vid_pid(device.vendor_id(), device.device_id()) {
                    return format!(
                        "{}  {} -> {}",
                        "                                          ",
                        "GPU".purple(),
                        dev.name()
                    );
                };
            }
            return "this shouldn't be possible".to_string();
        })
        .collect();

    gpus.join("\n")
}
