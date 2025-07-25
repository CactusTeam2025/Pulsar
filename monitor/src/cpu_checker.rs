// Copyright 2023 Your Name
// Licensed under Apache License, Version 2.0 (см. LICENSE)

use sysinfo::{System};

// функция для получения данных о процессоре
pub fn cpu_base_information_checker() -> cpu_base_information{
    let mut sys: System = System::new();

    sys.refresh_cpu_all(); // Обновляем информацию о CPU
    
    let cpu = &sys.cpus()[0]; // Получаем информацию о первом процессоре

    cpu_base_information {
        cpu_name: cpu.name().to_string(), // Получаем название процессора   
        cpu_brand: cpu.brand().to_string(), // Получаем бренд процессора
        cpu_maximum_frequency: cpu.frequency(), // Получаем частоту процессора
        physical_cores: System::physical_core_count().unwrap_or(0), // Получаем количество физических ядер
        cpu_vendor: cpu.vendor_id().to_string() // Получаем ID производителя процессора
    }
}

#[derive(Debug)]
struct cpu_base_information {
    pub cpu_name: String,
    pub cpu_brand: String,
    pub cpu_maximum_frequency: u64,
    pub physical_cores: usize,
    pub cpu_vendor: String
}