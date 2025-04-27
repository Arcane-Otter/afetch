mod collect;

/*fn get_first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}*/ 

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let s_art: String = [
        "       _,met$$$$$gg.          ",
        "    ,g$$$$$$$$$$$$$$$P.       ",
        "  ,g$$P\"        \"\"\"Y$$.\".",
        " ,$$P'              `$$$.     ",
        "',$$P       ,ggs.     `$$b:   ",
        "`d$$'     ,$P\"'   .    $$$   ",
        " $$P      d$'     ,    $$P    ",
        " $$:      $$.   -    ,d$$'    ",
        " $$;      Y$b._   _,d$P'      ",
        " Y$$.    `.`\"Y$$$$P\"'       ",
        " `$$b      \"-.__             ",
        "  `Y$$                        ",
        "   `Y$$.                      ",
        "     `$$b.                    ",
        "       `Y$$b.                 ",
        "          `\"Y$b._            ",
        "              `\"\"\"         ",
        ].join("\n");


    let s_distro: String = collect::get_distro();
    //let s_name: &str = get_first_word(&s_distro);
    let s_uptime = collect::get_uptime();
    let s_shell = collect::get_shell_name();
    let l_username = collect::get_username();
    let l_hostname = collect::get_hostname();
    let l_host = collect::get_host();
    let l_k_version = collect::get_kernel_version();
    let l_uptime = collect::get_formatted_uptime(s_uptime);
    let l_packages = collect::get_formatted_package_count();
    let l_s_version = collect::get_shell_version(&s_shell);
    let l_terminal = collect::get_terminal_name();
    let l_motherb = collect::get_motherboard_info();
    let l_cpu = collect::get_cpu_model();
    let l_mem = collect::get_memory_info();

    let mut statlines: Vec<String> = Vec::new();
    statlines.push(format!("\x1b[32;1m{}@{}\x1b[0m", l_username, l_hostname).to_string());
    statlines.push(format!("\x1b[32m____________________\x1b[0m").to_string());
    statlines.push(format!("").to_string());
    statlines.push(format!("\x1b[32;1mOS:\x1b[0m {}", s_distro).to_string());
    statlines.push(format!("\x1b[32;1mHost:\x1b[0m {}", l_host).to_string());
    statlines.push(format!("\x1b[32;1mKernel:\x1b[0m {}", l_k_version).to_string());
    statlines.push(format!("\x1b[32;1mUptime:\x1b[0m {}", l_uptime).to_string());
    statlines.push(format!("\x1b[32;1mPackages:\x1b[0m {}", l_packages).to_string());
    statlines.push(format!("\x1b[32;1mShell:\x1b[0m {}", l_s_version).to_string());
    statlines.push(format!("\x1b[32;1mTerminal:\x1b[0m {}", l_terminal).to_string());
    statlines.push(format!("\x1b[32;1mMotherboard:\x1b[0m {}", l_motherb).to_string());
    statlines.push(format!("\x1b[32;1mCPU:\x1b[0m {}", l_cpu).to_string());
    statlines.push(format!("\x1b[32;1mMemory:\x1b[0m {}", l_mem).to_string());

    let s_gpus = collect::get_gpu_info();
    for gpu in 0..s_gpus.len() {
        statlines.push(
            format!(
                "\x1b[32;1mGPU {}:\x1b[0m {}", 
                gpu + 1, 
                s_gpus.get(gpu)
                    .expect("[gpu]")
                    .split(":").nth(2).unwrap()
                    .split("[").next().unwrap()
                ).to_string());
    }

    let ascii_lines: Vec<&str> = s_art.lines().collect();

    let max_lines = ascii_lines.len().max(statlines.len());
    for i in 0..max_lines {
        let art_part = ascii_lines.get(i).unwrap_or(&" ");
        let info_prt = statlines.get(i).map_or(" ", |v| v);

        println!("\x1b[31m  {:<45}\x1b[0m| {}", art_part, info_prt);
    }

    Ok(())
}
