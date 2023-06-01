use sysinfo::*;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("=> disks:");
    for disk in sys.disks() {
        println!(
            "{:.2?} GB / {:.2?} GB",
            disk.available_space() as f64 / (((1024 as u32).pow(3)) as f64),
            disk.total_space() as f64 / (((1024 as u32).pow(3)) as f64)
        );
    }
    println!("=> networks:");
    for (interface_name, data) in sys.networks() {
        println!(
            "{}: {}/{} B",
            interface_name,
            data.received(),
            data.transmitted()
        );
    }
    println!("=> components:");
    for component in sys.components() {
        println!("{:?}", component);
    }

    println!("=> system:");
    println!(
        "used memory : {:.2} Gigabytes",
        sys.used_memory() as f64 / ((1024 as u64).pow(3)) as f64
    );
    println!(
        "total swap  : {:.2} Gigabytes",
        sys.total_swap() as f64 / ((1024 as u64).pow(3)) as f64
    );
    println!(
        "used swap   : {:.2} Gigabytes",
        sys.used_swap() as f64 / ((1024 as u64).pow(3)) as f64
    );
    println!(
        "total memory: {:.2} Gigabytes",
        sys.total_memory() as f64 / ((1024 as u64).pow(3)) as f64
    );
    println!("System name:             {:?}", sys.name().unwrap());
    println!(
        "System kernel version:   {:?}",
        sys.kernel_version().unwrap()
    );
    println!("System OS version:       {:?}", sys.os_version().unwrap());
    println!("System host name:        {:?}", sys.host_name().unwrap());
    println!(
        "Number of CPU Cores (Including Vertual cores): {}",
        sys.cpus().len()
    );
    println!(
        "Number of physical cores: {}",
        sys.physical_core_count().unwrap()
    );
    println!("CPU name: {:?}", sys.cpus()[0].brand());
}
