use std::io::{self, Write};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

// ─────────────────────────────────────────
//  RuneOS Shell
// ─────────────────────────────────────────

const VERSION: &str = "0.3.0";
const RUNE_BANNER: &str = r#"
██████╗ ██╗   ██╗███╗   ██╗███████╗ ██████╗ ███████╗
██╔══██╗██║   ██║████╗  ██║██╔════╝██╔═══██╗██╔════╝
██████╔╝██║   ██║██╔██╗ ██║█████╗  ██║   ██║███████╗
██╔══██╗██║   ██║██║╚██╗██║██╔══╝  ██║   ██║╚════██║
██║  ██║╚██████╔╝██║ ╚████║███████╗╚██████╔╝███████║
╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝ ╚═════╝ ╚══════╝
"#;

fn main() {
    print_banner();
    shell_loop();
}

fn print_banner() {
    println!("\x1b[35m{}\x1b[0m", RUNE_BANNER);
    println!("\x1b[36m  RuneOS v{} — Terminal Operating System\x1b[0m", VERSION);
    println!("\x1b[90m  Type 'help' for available commands.\x1b[0m\n");
}

fn shell_loop() {
    loop {
        // Prompt: [|RuneOS|] ~
        print!("\x1b[35m[\x1b[0m\x1b[1m|RuneOS|\x1b[0m\x1b[35m]\x1b[0m \x1b[36m~\x1b[0m \x1b[1m❯\x1b[0m ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {}
            Err(_) => break,
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.splitn(3, ' ').collect();
        let cmd = parts[0];
        let args = &parts[1..];

        match cmd {
            "help"     => cmd_help(),
            "fsee"     => cmd_fsee(args),
            "clear"    => cmd_clear(),
            "whoami"   => cmd_whoami(),
            "uptime"   => cmd_uptime(),
            "neofetch" => cmd_neofetch(),
            "my-apps"  => cmd_my_apps(),
            "start"    => cmd_start(args),
            "echo"     => cmd_echo(args),
            "pwd"      => cmd_pwd(),
            "cd"       => cmd_cd(args),
            "exit"     => {
                println!("\x1b[35mRuneOS shutdown. Goodbye.\x1b[0m");
                std::process::exit(0);
            }
            _ => {
                println!(
                    "\x1b[31mruneos: command not found:\x1b[0m {} \x1b[90m(try 'help')\x1b[0m",
                    cmd
                );
            }
        }
    }
}

// ─────────────────────────────────────────
//  Commands
// ─────────────────────────────────────────

fn cmd_help() {
    let cmds = vec![
        ("help",              "Show this help menu"),
        ("fsee [path]",       "List files in directory (like ls)"),
        ("pwd",               "Print current directory"),
        ("cd <path>",         "Change directory"),
        ("echo <text>",       "Print text to terminal"),
        ("clear",             "Clear the screen"),
        ("whoami",            "Show current user"),
        ("uptime",            "Show system uptime"),
        ("neofetch",          "Show system information"),
        ("my-apps",           "Open the RuneOS app manager"),
        ("start app <name>",  "Launch an app by name"),
        ("exit",              "Shutdown RuneOS shell"),
    ];

    println!();
    println!("\x1b[35m╔══════════════════════════════════════════════╗\x1b[0m");
    println!("\x1b[35m║\x1b[0m  \x1b[1m\x1b[36mRuneOS Shell — Command Reference\x1b[0m             \x1b[35m║\x1b[0m");
    println!("\x1b[35m╠══════════════════════════════════════════════╣\x1b[0m");
    for (name, desc) in &cmds {
        println!(
            "\x1b[35m║\x1b[0m  \x1b[32m{:<22}\x1b[0m \x1b[90m{:<22}\x1b[0m \x1b[35m║\x1b[0m",
            name, desc
        );
    }
    println!("\x1b[35m╚══════════════════════════════════════════════╝\x1b[0m");
    println!();
}

fn cmd_fsee(args: &[&str]) {
    let path = if args.is_empty() { "." } else { args[0] };
    match std::fs::read_dir(path) {
        Ok(entries) => {
            println!("\x1b[90m  fsee: {}\x1b[0m", path);
            println!();
            let mut dirs = vec![];
            let mut files = vec![];
            for entry in entries.flatten() {
                let name = entry.file_name().into_string().unwrap_or_default();
                let meta = entry.metadata();
                if let Ok(m) = meta {
                    if m.is_dir() {
                        dirs.push(name);
                    } else {
                        let size = m.len();
                        files.push((name, size));
                    }
                }
            }
            dirs.sort();
            files.sort();
            for d in &dirs {
                println!("  \x1b[34m📁 {}/\x1b[0m", d);
            }
            for (f, size) in &files {
                println!("  \x1b[37m📄 {:<30}\x1b[90m {:>8} B\x1b[0m", f, size);
            }
            println!();
        }
        Err(e) => println!("\x1b[31mfsee: {}: {}\x1b[0m", path, e),
    }
}

fn cmd_clear() {
    print!("\x1b[2J\x1b[1;1H");
    io::stdout().flush().unwrap();
    print_banner();
}

fn cmd_whoami() {
    let user = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "rune-user".to_string());
    println!("  \x1b[36m{}\x1b[0m", user);
}

fn cmd_uptime() {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Simplified — real uptime needs /proc/uptime on Linux
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/uptime") {
            let up: f64 = content.split_whitespace().next()
                .and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let h = (up as u64) / 3600;
            let m = ((up as u64) % 3600) / 60;
            let s = (up as u64) % 60;
            println!("  \x1b[36mUptime:\x1b[0m {}h {}m {}s", h, m, s);
            return;
        }
    }
    println!("  \x1b[36mSystem time (unix):\x1b[0m {}s", secs);
}

fn cmd_neofetch() {
    let user = std::env::var("USER").unwrap_or_else(|_| "rune-user".to_string());
    let host = hostname();
    println!();
    println!("  \x1b[35m██████╗\x1b[0m   \x1b[1mUser:\x1b[0m  \x1b[36m{}@{}\x1b[0m", user, host);
    println!("  \x1b[35m██╔══██╗\x1b[0m  \x1b[1mOS:\x1b[0m    RuneOS v{}", VERSION);
    println!("  \x1b[35m██████╔╝\x1b[0m  \x1b[1mShell:\x1b[0m RuneShell");
    println!("  \x1b[35m██╔══██╗\x1b[0m  \x1b[1mArch:\x1b[0m  {}", std::env::consts::ARCH);
    println!("  \x1b[35m██║  ██║\x1b[0m  \x1b[1mLang:\x1b[0m  Rust");
    println!("  \x1b[35m╚═╝  ╚═╝\x1b[0m  \x1b[1mTheme:\x1b[0m Purple/Cyan");
    println!();
}

fn cmd_my_apps() {
    // Simulated app registry — in a real OS this would scan /apps or a manifest
    let apps: Vec<(&str, &str, &str, u64)> = vec![
        ("rune-shell",   "System Shell",         "v0.3.0",  124_000),
        ("rune-edit",    "Text Editor",           "v0.1.2",   88_000),
        ("rune-fetch",   "Network Tool",          "v0.2.0",  210_000),
        ("rune-calc",    "Calculator",            "v0.1.0",   45_000),
        ("rune-log",     "System Log Viewer",     "v0.1.1",   67_000),
        ("rune-crypto",  "Crypto Utilities",      "v0.2.3",  190_000),
    ];

    let width = 64usize;
    let border = "═".repeat(width - 2);

    println!();
    println!("  \x1b[35m╔{}╗\x1b[0m", border);
    println!("  \x1b[35m║\x1b[0m{:^62}\x1b[35m║\x1b[0m", "\x1b[1m\x1b[36m RuneOS App Manager \x1b[0m");
    println!("  \x1b[35m╠{}╣\x1b[0m", border);
    println!(
        "  \x1b[35m║\x1b[0m  \x1b[1m{:<18} {:<22} {:<8} {:>8}\x1b[0m  \x1b[35m║\x1b[0m",
        "Name", "Description", "Version", "Size"
    );
    println!("  \x1b[35m╠{}╣\x1b[0m", border);

    for (name, desc, ver, size) in &apps {
        let size_str = format!("{} KB", size / 1024);
        println!(
            "  \x1b[35m║\x1b[0m  \x1b[32m{:<18}\x1b[0m \x1b[37m{:<22}\x1b[0m \x1b[90m{:<8}\x1b[0m \x1b[36m{:>8}\x1b[0m  \x1b[35m║\x1b[0m",
            name, desc, ver, size_str
        );
    }

    println!("  \x1b[35m╠{}╣\x1b[0m", border);
    println!(
        "  \x1b[35m║\x1b[0m  \x1b[90mUse: \x1b[0m\x1b[33mstart app <name>\x1b[0m\x1b[90m  to launch an app{:>18}\x1b[0m\x1b[35m║\x1b[0m",
        ""
    );
    println!("  \x1b[35m╚{}╝\x1b[0m", border);
    println!();
}

fn cmd_start(args: &[&str]) {
    if args.len() < 2 || args[0] != "app" {
        println!("\x1b[31mUsage: start app <name>\x1b[0m");
        return;
    }
    let name = args[1];
    // In a real OS: look up binary in /apps/<name>
    println!("\x1b[35m[\x1b[0m\x1b[36mRuneOS Launcher\x1b[35m]\x1b[0m Starting '\x1b[32m{}\x1b[0m'...", name);
    match Command::new(name).spawn() {
        Ok(mut child) => {
            let _ = child.wait();
        }
        Err(_) => {
            println!(
                "\x1b[31m  Error:\x1b[0m App '{}' not found or not executable.",
                name
            );
            println!("\x1b[90m  Tip: Check 'my-apps' for installed apps.\x1b[0m");
        }
    }
}

fn cmd_echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

fn cmd_pwd() {
    match std::env::current_dir() {
        Ok(p) => println!("  \x1b[36m{}\x1b[0m", p.display()),
        Err(e) => println!("\x1b[31mpwd: {}\x1b[0m", e),
    }
}

fn cmd_cd(args: &[&str]) {
    let path = args.first().copied().unwrap_or(".");
    if let Err(e) = std::env::set_current_dir(path) {
        println!("\x1b[31mcd: {}: {}\x1b[0m", path, e);
    }
}

fn hostname() -> String {
    std::fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| "runeos".to_string())
        .trim()
        .to_string()
            }
