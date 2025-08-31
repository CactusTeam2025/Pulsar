// Copyright 2023 Your Name
// Licensed under Apache License, Version 2.0 (см. LICENSE)

use sysinfo::System;

#[derive(Debug)]
pub struct MultiCpuInfo {
    pub cpus: Vec<CpuInfo>,
    pub physical_cores: usize,
    pub logical_cores: usize,
}
#[derive(Debug)]
pub struct CpuInfo {
    pub name: String,
    pub brand: String,
    pub vendor: String,
    pub frequency: u64,
}

#[derive(Debug)]
pub struct CpuUsageInfo {
    pub overall_usage: f32,
    pub per_core_usage: Vec<f32>,
    pub cpu_count: usize,
}

pub fn get_cpu_usage() -> CpuUsageInfo {
    let mut sys = System::new_all();

    // Ждем немного для сбора данных об использовании CPU
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Обновляем информацию о CPU
    sys.refresh_cpu_all();

    // Получаем нагрузку по ядрам
    let per_core_usage: Vec<f32> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    // Рассчитываем общую нагрузку как среднее арифметическое
    let overall_usage = if !per_core_usage.is_empty() {
        per_core_usage.iter().sum::<f32>() / per_core_usage.len() as f32
    } else {
        0.0
    };

    CpuUsageInfo {
        overall_usage,
        per_core_usage,
        cpu_count: sys.cpus().len(),
    }
}

// Дополнительная функция для красивого форматирования
pub fn format_cpu_usage(usage: &CpuUsageInfo) -> String {
    let mut result = String::new();

    result.push_str(&format!(
        "Общая нагрузка CPU: {:.1}%\n",
        usage.overall_usage
    ));
    result.push_str(&format!("Количество ядер: {}\n", usage.cpu_count));
    result.push_str("Нагрузка по ядрам:\n");

    for (i, core_usage) in usage.per_core_usage.iter().enumerate() {
        result.push_str(&format!("  Ядро {}: {:.1}%\n", i + 1, core_usage));
    }

    result
}

// функция для получения данных о процессорах
pub fn cpu_base_information_checker() -> MultiCpuInfo {
    let mut sys: System = System::new();

    sys.refresh_cpu_all(); // Обновляем информацию о CPU

    let cpu = &sys.cpus();
    let mut cpus_info = Vec::new();

    for cpu in sys.cpus() {
        cpus_info.push(CpuInfo {
            name: cpu.name().to_string(),        // Получаем название процессора
            brand: cpu.brand().to_string(),      // Получаем бренд процессора
            frequency: cpu.frequency(),          // Получаем частоту процессора
            vendor: cpu.vendor_id().to_string(), // Получаем ID производителя процессора
        });
    }
    MultiCpuInfo {
        cpus: cpus_info,
        physical_cores: System::physical_core_count().unwrap_or(0),
        logical_cores: sys.cpus().len(),
    }
}

// Дополнительная функция для красивого форматирования
pub fn format_cpu_information(cpui: &MultiCpuInfo) -> String {
    let mut result = String::new();

    result.push_str("💻 Характеристики процессоров:\n\n");
    result.push_str(&format!("Физические ядра: {}\n", cpui.physical_cores));
    result.push_str(&format!("Логические ядра: {}\n", cpui.logical_cores));

    // Группируем по модели для избежания дублирования
    let first_cpu = &cpui.cpus[0];
    result.push_str(&format!("Модель: {}\n", first_cpu.brand));
    result.push_str(&format!("Производитель: {}\n", first_cpu.vendor));
    result.push_str(&format!(
        "Максимальная частота: {} MHz\n",
        first_cpu.frequency
    ));

    result
}
