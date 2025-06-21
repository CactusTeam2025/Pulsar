mod cpu_checker;
use cpu_checker::cpu_checker;

fn main() {
    println!("Starting CPU monitor...");
    println!("Press Ctrl+C to exit\n");
    
    cpu_checker(); // Вызов нашей функции мониторинга CPU

}
