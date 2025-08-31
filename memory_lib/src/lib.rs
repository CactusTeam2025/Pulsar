// Copyright 2023 Your Name
// Licensed under Apache License, Version 2.0 (см. LICENSE)

use sysinfo::System;

#[derive(Debug)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub available: u64,
    pub swap_total: u64,
    pub swap_used: u64,
    pub swap_free: u64,
}

#[derive(Debug)]
pub struct MemoryUsageInfo {
    pub total_usage: f32,
    pub swap_usage: f32,
    pub used_memory: u64,
    pub available_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
}

pub fn get_memory_usage() -> MemoryUsageInfo {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let available_memory = sys.available_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();

    let total_usage = if total_memory > 0 {
        (used_memory as f32 / total_memory as f32) * 100.0
    } else {
        0.0
    };

    let swap_usage = if total_swap > 0 {
        (used_swap as f32 / total_swap as f32) * 100.0
    } else {
        0.0
    };

    MemoryUsageInfo {
        total_usage,
        swap_usage,
        used_memory,
        available_memory,
        total_swap,
        used_swap,
    }
}

pub fn get_detailed_memory_info() -> MemoryInfo {
    let mut sys = System::new_all();
    sys.refresh_memory();

    MemoryInfo {
        total: sys.total_memory(),
        used: sys.used_memory(),
        free: sys.free_memory(),
        available: sys.available_memory(),
        swap_total: sys.total_swap(),
        swap_used: sys.used_swap(),
        swap_free: sys.free_swap(),
    }
}

// Дополнительная функция для красивого форматирования
pub fn format_memory_usage(usage: &MemoryUsageInfo) -> String {
    let mut result = String::new();

    result.push_str("💾 Использование памяти:\n\n");
    result.push_str(&format!("Общая нагрузка RAM: {:.1}%\n", usage.total_usage));
    result.push_str(&format!("Использовано RAM: {} MB\n", bytes_to_mb(usage.used_memory)));
    result.push_str(&format!("Доступно RAM: {} MB\n", bytes_to_mb(usage.available_memory)));
    result.push_str(&format!("Всего RAM: {} MB\n", bytes_to_mb(usage.used_memory + usage.available_memory)));
    
    result.push_str("\n");
    result.push_str(&format!("Нагрузка SWAP: {:.1}%\n", usage.swap_usage));
    result.push_str(&format!("Использовано SWAP: {} MB\n", bytes_to_mb(usage.used_swap)));
    result.push_str(&format!("Всего SWAP: {} MB\n", bytes_to_mb(usage.total_swap)));

    result
}

// Дополнительная функция для детальной информации о памяти
pub fn format_detailed_memory_info(memory: &MemoryInfo) -> String {
    let mut result = String::new();

    result.push_str("💾 Детальная информация о памяти:\n\n");
    
    result.push_str("Оперативная память (RAM):\n");
    result.push_str(&format!("  Всего: {} MB\n", bytes_to_mb(memory.total)));
    result.push_str(&format!("  Используемая: {} MB\n", bytes_to_mb(memory.used)));
    result.push_str(&format!("  Свободно(не используется): {} MB\n", bytes_to_mb(memory.free)));
    result.push_str(&format!("  Доступно(Для программ): {} MB\n", bytes_to_mb(memory.available)));
    
    result.push_str("\n");
    result.push_str("Файл подкачки (SWAP):\n");
    result.push_str(&format!("  Всего: {} MB\n", bytes_to_mb(memory.swap_total)));
    result.push_str(&format!("  Использовано: {} MB\n", bytes_to_mb(memory.swap_used)));
    result.push_str(&format!("  Свободно: {} MB\n", bytes_to_mb(memory.swap_free)));

    result
}

// Функция для получения информации о памяти в реальном времени
pub fn memory_base_information_checker() -> MemoryInfo {
    let mut sys = System::new_all();
    sys.refresh_memory();

    MemoryInfo {
        total: sys.total_memory(),
        used: sys.used_memory(),
        free: sys.free_memory(),
        available: sys.available_memory(),
        swap_total: sys.total_swap(),
        swap_used: sys.used_swap(),
        swap_free: sys.free_swap(),
    }
}

// Вспомогательная функция для конвертации байтов в мегабайты
fn bytes_to_mb(bytes: u64) -> u64 {
    bytes / 1024 / 1024
}

// Функция для получения использования памяти в процентах
pub fn get_memory_usage_percentage() -> f32 {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let total = sys.total_memory();
    let used = sys.used_memory();

    if total > 0 {
        (used as f32 / total as f32) * 100.0
    } else {
        0.0
    }
}

// Функция для получения использования swap в процентах
pub fn get_swap_usage_percentage() -> f32 {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let total = sys.total_swap();
    let used = sys.used_swap();

    if total > 0 {
        (used as f32 / total as f32) * 100.0
    } else {
        0.0
    }
}

