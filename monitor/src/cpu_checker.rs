use sysinfo::{System};
use std::{thread, time::Duration};

pub fn cpu_checker() {
    let mut sys = System::new();
    
    loop {
// Очищаем консоль перед началом
    print!("\x1B[2J\x1B[1;1H");

        sys.refresh_cpu_all(); // Обновляем всю информацию о CPU (включая usage)
        
        let physical_cores = System::physical_core_count().unwrap_or(0); // Получаем количество физических ядер
        let logical_cores = sys.cpus().len(); // Получаем количество логических ядер
        let total_usage = sys.global_cpu_usage(); // Получаем общую загрузку CPU
        let usage_for_each_core: Vec<f32> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(); // Получаем использование для каждого ядра

        print!("\x1B[1;1H"); // Перемещаем курсор в начало
        println!("Number of physical cores: {}", physical_cores);
        println!("Number of logical cores: {}", logical_cores);
        println!("Per core usage: {:.1?}", usage_for_each_core);
        println!("Total CPU usage: {:.1}%", total_usage);
        
        thread::sleep(Duration::from_secs(1));
    }
}

// pub fn temperature_checker()
// {   
    
// }