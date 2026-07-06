use colored::*;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Stdio};

pub const ALL_COMMANDS: &[&str] = &[
    "/tp",
    "/spawn",
    "/whereami",
    "/list",
    "/build",
    "/destroy",
    "/summon",
    "/clone",
    "/move",
    "/read",
    "/enchant",
    "/op",
    "/entities",
    "/smite",
    "/lag",
    "/seed",
    "/say",
    "/kill",
    "/help",
    "/history",
    "/clear",
    "/me",
    "/gamemode",
    "/give",
];

pub fn get_prompt() -> String {
    let cwd = env::current_dir().unwrap_or_default();
    let home = env::var("HOME").unwrap_or_default();
    let path_str = cwd.to_string_lossy();

    // Replace home dir with ~ 
    let display_path = if path_str.starts_with(&home) {
        format!("~{}", &path_str[home.len()..])
    } else {
        path_str.to_string()
    };

    // Pick a biome based on path
    let biome = get_biome(&display_path);
    let user = env::var("USER").unwrap_or_else(|_| "Steve".to_string());

    format!(
        "{} {} {} ",
        format!("[{} @ {}]", user, biome).green().bold(),
        display_path.yellow(),
        ">".white().bold()
    )
}

fn get_biome(path: &str) -> &'static str {
    if path == "~" || path == "/" {
        "Spawn"
    } else if path.contains("tmp") {
        "Nether"
    } else if path.contains("home") {
        "Plains"
    } else if path.contains("etc") {
        "Stronghold"
    } else if path.contains("var") {
        "Cave"
    } else if path.contains("usr") {
        "Village"
    } else if path.contains("dev") {
        "End"
    } else {
        "Overworld"
    }
}

pub fn handle_command(input: &str) -> bool {
    let input = input.trim();

    // Empty input
    if input.is_empty() {
        return true;
    }

    // No slash = Minecraft chat message
    if !input.starts_with('/') {
        let user = env::var("USER").unwrap_or_else(|_| "Steve".to_string());
        println!("{}", format!("<{}> {}", user, input).white());
        return true;
    }

    // Pipeline: /cmd1 arg | /cmd2 arg | /cmd3 arg
    if input.contains('|') {
        return handle_pipeline(input);
    }

    let input = &input[1..]; // strip leading /
    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let cmd = parts[0].to_lowercase();
    let args = if parts.len() > 1 { parts[1] } else { "" };

    match cmd.as_str() {
        // --- Navigation ---
        "tp" => cmd_tp(args),
        "spawn" => cmd_spawn(),
        "whereami" => cmd_whereami(),

        // --- Files & Dirs ---
        "list" => cmd_list(args),
        "build" => cmd_build(args),
        "destroy" => cmd_destroy(args),
        "summon" => cmd_summon(args),
        "clone" => cmd_clone(args),
        "move" => cmd_move(args),
        "read" => cmd_read(args),

        // --- Permissions ---
        "enchant" => cmd_enchant(args),
        "op" => cmd_op(args),

        // --- System ---
        "entities" => cmd_entities(args),
        "smite" => cmd_smite(args),
        "lag" => cmd_lag(),
        "seed" => cmd_seed(),

        // --- Builtins ---
        "say" => cmd_say(args),
        "kill" => {
            println!("{}", "[Server] Goodbye! Returning to title screen...".red().bold());
            return false; // signal exit
        }
        "help" => cmd_help(),
        "clear" => cmd_clear(),
        "me" => cmd_me(args),
        "gamemode" => cmd_gamemode(args),
        "give" => cmd_give(args),

        // --- Unknown /command → try as raw bash ---
        _ => cmd_raw(&cmd, args),
    }

    true
}

// ─── Navigation ────────────────────────────────────────────────────────────

fn cmd_tp(args: &str) {
    if args.is_empty() {
        server_msg("Usage: /tp <directory>");
        return;
    }
    let path = shellexpand::tilde(args).to_string();
    match env::set_current_dir(Path::new(&path)) {
        Ok(_) => server_msg(&format!("Teleported to {}", args)),
        Err(e) => error_msg(&format!("Could not teleport: {}", e)),
    }
}

fn cmd_spawn() {
    let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
    match env::set_current_dir(&home) {
        Ok(_) => server_msg("Teleported to spawn point!"),
        Err(e) => error_msg(&format!("Could not teleport to spawn: {}", e)),
    }
}

fn cmd_whereami() {
    let cwd = env::current_dir().unwrap_or_default();
    server_msg(&format!("Your coordinates: {}", cwd.display()));
}

// ─── Files & Dirs ──────────────────────────────────────────────────────────

fn cmd_list(args: &str) {
    let path = if args.is_empty() {
        ".".to_string()
    } else {
        args.to_string()
    };

    match fs::read_dir(&path) {
        Ok(entries) => {
            server_msg(&format!("Blocks in {}:", path));
            let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            items.sort_by_key(|e| e.file_name());
            for entry in items {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                let meta = entry.metadata();
                if let Ok(m) = meta {
                    if m.is_dir() {
                        println!("  {} {}", "📦".to_string(), name_str.cyan().bold());
                    } else if m.permissions().mode() & 0o111 != 0 {
                        println!("  {} {}", "⚔️".to_string(), name_str.green());
                    } else {
                        println!("  {} {}", "📄".to_string(), name_str);
                    }
                }
            }
        }
        Err(e) => error_msg(&format!("Could not list blocks: {}", e)),
    }
}

fn cmd_build(args: &str) {
    if args.is_empty() {
        server_msg("Usage: /build <directory>");
        return;
    }
    match fs::create_dir_all(args) {
        Ok(_) => server_msg(&format!("Built structure: {}", args)),
        Err(e) => error_msg(&format!("Could not build: {}", e)),
    }
}

fn cmd_destroy(args: &str) {
    if args.is_empty() {
        server_msg("Usage: /destroy <path>");
        return;
    }
    let path = Path::new(args);
    let result = if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    };
    match result {
        Ok(_) => server_msg(&format!("Destroyed: {}", args)),
        Err(e) => error_msg(&format!("Could not destroy: {}", e)),
    }
}

fn cmd_summon(args: &str) {
    if args.is_empty() {
        server_msg("Usage: /summon <filename>");
        return;
    }
    match fs::File::create(args) {
        Ok(_) => server_msg(&format!("Summoned: {}", args)),
        Err(e) => error_msg(&format!("Could not summon: {}", e)),
    }
}

fn cmd_clone(args: &str) {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.len() < 2 {
        server_msg("Usage: /clone <source> <destination>");
        return;
    }
    match fs::copy(parts[0], parts[1]) {
        Ok(_) => server_msg(&format!("Cloned {} → {}", parts[0], parts[1])),
        Err(e) => error_msg(&format!("Could not clone: {}", e)),
    }
}

fn cmd_move(args: &str) {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.len() < 2 {
        server_msg("Usage: /move <source> <destination>");
        return;
    }
    match fs::rename(parts[0], parts[1]) {
        Ok(_) => server_msg(&format!("Moved {} → {}", parts[0], parts[1])),
        Err(e) => error_msg(&format!("Could not move: {}", e)),
    }
}

fn cmd_read(args: &str) {
    if args.is_empty() {
        server_msg("Usage: /read <filename>");
        return;
    }
    match fs::read_to_string(args) {
        Ok(content) => {
            server_msg(&format!("Contents of {}:", args));
            println!("{}", content);
        }
        Err(e) => error_msg(&format!("Could not read: {}", e)),
    }
}

// ─── Permissions ───────────────────────────────────────────────────────────

fn cmd_enchant(args: &str) {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.len() < 2 {
        server_msg("Usage: /enchant <file> <mode> (e.g. /enchant script.sh 755)");
        return;
    }
    run_raw_silent("chmod", &[parts[1], parts[0]]);
    server_msg(&format!("Enchanted {} with mode {}", parts[0], parts[1]));
}

fn cmd_op(args: &str) {
    if args.is_empty() {
        server_msg("Usage: /op <file>");
        return;
    }
    run_raw_silent("chown", &[&format!("root:root"), args]);
    server_msg(&format!("Granted OP to: {}", args));
}

// ─── System ────────────────────────────────────────────────────────────────

fn cmd_entities(args: &str) {
    let mut cmd = Command::new("ps");
    if args.is_empty() {
        cmd.args(["aux"]);
    } else {
        cmd.args(["aux"]);
        // grep by name if provided
        let _ = args; // used below via pipe
    }
    server_msg("Entities loaded in this world:");
    cmd.status().ok();
}

fn cmd_smite(args: &str) {
    if args.is_empty() {
        server_msg("Usage: /smite <pid>");
        return;
    }
    run_raw_silent("kill", &[args]);
    server_msg(&format!("⚡ Smote process {}!", args));
}

fn cmd_lag() {
    server_msg("Checking server TPS (top)...");
    Command::new("top").args(["-bn1"]).status().ok();
}

fn cmd_seed() {
    server_msg("World seed (environment variables):");
    Command::new("env").status().ok();
}

// ─── Builtins ──────────────────────────────────────────────────────────────

fn cmd_say(args: &str) {
    println!("{}", format!("[Server] {}", args).yellow().bold());
}

fn cmd_me(args: &str) {
    let user = env::var("USER").unwrap_or_else(|_| "Steve".to_string());
    println!("{}", format!("* {} {}", user, args).italic().white());
}

fn cmd_gamemode(args: &str) {
    match args.to_lowercase().as_str() {
        "creative" | "1" => {
            server_msg("Game mode updated to Creative Mode. sudo who? 🧱");
        }
        "survival" | "0" => {
            server_msg("Game mode updated to Survival Mode. Stay safe out there! 🌲");
        }
        "hardcore" | "3" => {
            server_msg("Game mode updated to Hardcore Mode. One mistake and it's over. 💀");
        }
        "spectator" | "2" => {
            server_msg("Game mode updated to Spectator Mode. Just watching... 👻");
        }
        _ => {
            server_msg("Usage: /gamemode <creative|survival|hardcore|spectator>");
        }
    }
}

fn cmd_give(args: &str) {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    if parts.len() < 2 {
        server_msg("Usage: /give <player> <file>");
        return;
    }
    let target_user = parts[0];
    let file = parts[1];
    let dest = format!("/home/{}/", target_user);
    match fs::copy(file, format!("{}{}", dest, file)) {
        Ok(_) => server_msg(&format!("Gave {} to {}!", file, target_user)),
        Err(e) => error_msg(&format!("Could not give item: {}", e)),
    }
}

fn cmd_clear() {
    print!("\x1B[2J\x1B[1;1H");
    server_msg("Screen cleared.");
}

fn cmd_help() {
    println!("{}", "\n[Server] Available Commands:\n".green().bold());

    let sections = vec![
        ("🗺️  Navigation", vec![
            ("/tp <dir>", "Teleport to a directory"),
            ("/spawn", "Go to home directory"),
            ("/whereami", "Show current location"),
        ]),
        ("📦  Files & Directories", vec![
            ("/list [dir]", "List files"),
            ("/build <dir>", "Create a directory"),
            ("/destroy <path>", "Delete file or folder"),
            ("/summon <file>", "Create empty file"),
            ("/clone <src> <dst>", "Copy a file"),
            ("/move <src> <dst>", "Move or rename a file"),
            ("/read <file>", "Print file contents"),
        ]),
        ("🔐  Permissions", vec![
            ("/enchant <file> <mode>", "Change permissions (chmod)"),
            ("/op <file>", "Change ownership (chown)"),
        ]),
        ("⚙️  System", vec![
            ("/entities", "List running processes"),
            ("/smite <pid>", "Kill a process"),
            ("/lag", "Show resource usage (top)"),
            ("/seed", "Show environment variables"),
        ]),
        ("💬  Builtins", vec![
            ("/say <msg>", "Print a message"),
            ("/me <action>", "Narrate an action"),
            ("/gamemode <mode>", "creative / survival / hardcore / spectator"),
            ("/give <player> <file>", "Copy file to another user"),
            ("/help", "Show this menu"),
            ("/clear", "Clear the screen"),
            ("/kill", "Exit the shell"),
        ]),
    ];

    for (section, cmds) in sections {
        println!("{}", section.cyan().bold());
        for (cmd, desc) in cmds {
            println!("  {:30} {}", cmd.yellow(), desc);
        }
        println!();
    }

    println!("{}", "💡 Tip: Any /command not listed above runs as a raw bash command.".dimmed());
    println!("{}", "🔧 Tip: Chain raw commands with | e.g. /cat f.txt | /grep hi | /wc -l".dimmed());
    println!("{}", "💬 Tip: Text without / is sent as chat.\n".dimmed());
}

// ─── Raw bash fallback ──────────────────────────────────────────────────────

fn cmd_raw(cmd: &str, args: &str) {
    let full_args: Vec<&str> = if args.is_empty() {
        vec![]
    } else {
        args.split_whitespace().collect()
    };

    reset_terminal();

    let status = Command::new(cmd)
        .args(&full_args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    reset_terminal();

    match status {
        Ok(s) if s.success() => {}
        Ok(_) => {} // command ran but returned non-zero, output already shown
        Err(_) => {
            println!(
                "{}",
                format!(
                    "Unknown command. Type \"/help\" for a list of commands."
                )
                .red()
            );
        }
    }
}

fn run_raw_silent(cmd: &str, args: &[&str]) {
    Command::new(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .ok();
}

// rustyline puts the terminal into raw mode while reading a line (so it can
// handle keystrokes for tab-completion, history, etc.) and restores it when
// readline() returns. In practice some termios flags — OPOST in particular,
// which controls automatic \n -> \r\n translation — can come back in a
// slightly different state than a normal login shell's terminal. Full-screen
// ANSI programs (sl, lolcat, anything curses-ish) are sensitive to that and
// render garbled as a result. Forcing the terminal back to sane defaults
// right before/after handing control to a child fixes it. Silently a no-op
// if `stty` isn't available or stdin isn't a real tty.
fn reset_terminal() {
    let _ = Command::new("stty")
        .arg("sane")
        .stdin(Stdio::inherit())
        .status();
}

// ─── Pipelines ─────────────────────────────────────────────────────────────
//
// Supports chaining raw/bash commands together, Minecraft-style:
//   /cat file.txt | /grep hello | /wc -l
//
// Each stage is spawned as its own process; stdout of one is wired directly
// into stdin of the next via OS pipes (no buffering through Rust). Only the
// final stage's stdout/stderr are inherited by the terminal — everything in
// between is piped. Built-in commands (/list, /say, etc.) aren't part of the
// pipeline system since they print directly rather than producing a child
// process stdout to hook into; pipeline stages are treated as raw commands.

fn handle_pipeline(input: &str) -> bool {
    let stages: Vec<&str> = input.split('|').map(|s| s.trim()).collect();

    if stages.iter().any(|s| s.is_empty()) {
        error_msg("Invalid pipeline: empty command between pipes");
        return true;
    }

    let mut parsed: Vec<(String, Vec<String>)> = Vec::with_capacity(stages.len());
    for stage in &stages {
        let stripped = stage.strip_prefix('/').unwrap_or(stage);
        let mut words = stripped.split_whitespace();
        let cmd = match words.next() {
            Some(c) => c.to_string(),
            None => {
                error_msg("Invalid pipeline: empty command between pipes");
                return true;
            }
        };
        let args: Vec<String> = words.map(|s| s.to_string()).collect();
        parsed.push((cmd, args));
    }

    if let Err(e) = run_pipeline(&parsed) {
        error_msg(&format!("Pipeline failed: {}", e));
    }

    true
}

fn run_pipeline(stages: &[(String, Vec<String>)]) -> std::io::Result<()> {
    use std::process::Child;

    reset_terminal();

    let n = stages.len();
    let mut children: Vec<Child> = Vec::with_capacity(n);

    for (i, (cmd, args)) in stages.iter().enumerate() {
        let mut command = Command::new(cmd);
        command.args(args);

        // Wire stdin: first stage reads from the terminal, every other
        // stage reads from the previous child's stdout pipe.
        if i == 0 {
            command.stdin(Stdio::inherit());
        } else {
            let prev_stdout = children[i - 1]
                .stdout
                .take()
                .expect("previous pipeline stage had no stdout pipe");
            command.stdin(Stdio::from(prev_stdout));
        }

        // Wire stdout: last stage prints to the terminal, every other
        // stage's output gets piped into the next stage's stdin above.
        command.stdout(if i == n - 1 { Stdio::inherit() } else { Stdio::piped() });
        command.stderr(Stdio::inherit());

        match command.spawn() {
            Ok(child) => children.push(child),
            Err(e) => {
                // A stage failed to launch (e.g. command not found) —
                // tear down anything already running so we don't leak
                // orphaned processes mid-pipe.
                for mut c in children {
                    let _ = c.kill();
                }
                return Err(std::io::Error::new(
                    e.kind(),
                    format!("'/{}' — {}", cmd, e),
                ));
            }
        }
    }

    // Wait left-to-right; earlier stages naturally finish once their
    // output has been fully consumed downstream.
    for mut child in children {
        let _ = child.wait();
    }

    reset_terminal();

    Ok(())
}

// ─── Helpers ───────────────────────────────────────────────────────────────

fn server_msg(msg: &str) {
    println!("{}", format!("[Server] {}", msg).green());
}

fn error_msg(msg: &str) {
    println!("{}", format!("[Server] ❌ {}", msg).red());
}
