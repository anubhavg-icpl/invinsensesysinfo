use colored::*;
use sysinfo::{System, SystemExt, CpuExt};
use std::env;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    print_logo();
    print_system_info(&sys);
}

fn print_logo() {
    let logo = r#"
    @@: %@@=   @@ :+++=  .+***:-@% :@@%:  =@* -%@%%%@#: #@@%%%%%:=@@#   *@= +%@%%@%*..@@%%%%%%
    @@- @@%@* .@@   =**+=***=  -@@ :@@%@- =@# #@#-:.:=- #@+::::  +@%@%: *@=.@@+-:.-+..@@-:::: 
    @@- @@-=@%:@@    :+****:   -@@ :@@ #@*=@#  :=*#%@%= #@%****- +@*:%@=*@= .-+*#%@#:.@@#****.
    @@- @@- :@@@@     .+*=     -@@ :@@  +@@@# %@+===#@% #@*=====.+@*  #@@@=.@@====%@+.@@+=====
    -=. -=.  .==-     .:=:.    .=-  =-   :==:  :-===-:  :======= .=:   -==.  -====-:  -======-
                    :=+---+=:                                                                 
                    ++- - =+=                     INVINSENSE                                  
                    =+. . :+=                 SYSTEM SHIELD v1.0                              
                     -=====:                                                                  
    "#;
    println!("{}", logo.green());
}

fn print_system_info(sys: &System) {
    println!("{}", "┌─────────────────────────────────────────────────────┐".green());
    println!("{} INVINSENSE SYSTEM SHIELD {}", "│".green(), "│".green());
    println!("{}", "├─────────────────────────────────────────────────────┤".green());
    println!("{} {:<15} {:<35} {}", "│".green(), "SYSTEM".yellow(), env::consts::OS, "│".green());
    println!("{} {:<15} {:<35} {}", "│".green(), "KERNEL".yellow(), sys.kernel_version().unwrap_or_default(), "│".green());
    println!("{} {:<15} {:<35} {}", "│".green(), "MEMORY".yellow(), format!("{:.2}GB", sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0), "│".green());
    
    let cpu_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;
    println!("{} {:<15} {:<35} {}", "│".green(), "CPU USAGE".yellow(), format!("{:.2}%", cpu_usage), "│".green());
    
    // Add more system information here
    
    println!("{}", "└─────────────────────────────────────────────────────┘".green());
}
