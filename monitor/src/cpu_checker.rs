use sysinfo::{System};
use std::{thread, time::Duration};

pub fn cpu_checker() {
    let mut sys = System::new();
    
    loop {
        sys.refresh_cpu_all(); // Обновляем всю информацию о CPU (включая usage)
        
        println!("Number of physical cores: {}", System::physical_core_count().unwrap_or(0));
        println!("Number of logical cores: {}", sys.cpus().len());

        // Выводим использование для каждого ядра
        for (i, cpu) in sys.cpus().iter().enumerate() {
            println!("Core {} ({}): {:.1}% usage", i, cpu.name(), cpu.cpu_usage());
        }

        // Общая загрузка CPU
        println!("Total CPU usage: {:.1}%", sys.global_cpu_usage());
        
        thread::sleep(Duration::from_secs(1));
    }
}

// pub fn temperature_checker()
// {   

// }