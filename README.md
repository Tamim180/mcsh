# mcsh — Minecraft Shell

```
  ███╗   ███╗ ██████╗███████╗██╗  ██╗
  ████╗ ████║██╔════╝██╔════╝██║  ██║
  ██╔████╔██║██║     ███████╗███████║
  ██║╚██╔╝██║██║     ╚════██║██╔══██║
  ██║ ╚═╝ ██║╚██████╗███████║██║  ██║
  ╚═╝     ╚═╝ ╚═════╝╚══════╝╚═╝  ╚═╝
```

A Linux shell where every command starts with `/` — just like Minecraft.
Text without a `/` is sent as chat. Written in Rust.

---

## Install

### AUR (Arch / EndeavourOS / Manjaro)

```bash
yay -S mcsh
```

### Build from source

```bash
git clone https://github.com/Tamim180/mcsh.git
cd mcsh
cargo build --release
./target/release/mcsh
```

---

## How it works

| Input | What happens |
|---|---|
| `/list` | Runs the mcsh command |
| `/python3 script.py` | Falls back to raw bash |
| `hello everyone` | Prints `<Steve> hello everyone` |

Everything with a `/` runs. Everything without it is chat.

---

## Commands

### Navigation
| Command | Alias | Does |
|---|---|---|
| `/tp <dir>` | `/cd` | Teleport to a directory |
| `/spawn` | | Go to home directory |
| `/whereami` | `/pwd` | Show current location |

### Files & Directories
| Command | Alias | Does |
|---|---|---|
| `/list [dir]` | `/ls` | List files with icons |
| `/build <dir>` | `/mkdir` | Create a directory |
| `/destroy <path>` | `/rm` | Delete a file or folder |
| `/summon <file>` | `/touch` | Create an empty file |
| `/clone <src> <dst>` | `/cp` | Copy a file |
| `/move <src> <dst>` | `/mv` | Move or rename |
| `/read <file>` | `/cat` | Print file contents |

### Permissions
| Command | Alias | Does |
|---|---|---|
| `/enchant <file> <mode>` | | chmod |
| `/op <file>` | | chown to root |

### System
| Command | Alias | Does |
|---|---|---|
| `/entities` | `/ps` | List running processes |
| `/smite <pid>` | | Kill a process |
| `/lag` | `/top` | Show resource usage |
| `/seed` | `/env` | Show environment variables |

### Builtins
| Command | Does |
|---|---|
| `/help` | Show all commands |
| `/say <msg>` | Print a message (yellow) |
| `/me <action>` | Narrate an action |
| `/gamemode <mode>` | creative / survival / hardcore / spectator |
| `/give <player> <file>` | Copy a file to another user's home |
| `/history` | Show command history |
| `/clear` | Clear the screen |
| `/kill` | Exit the shell |

### Any other `/command`
Falls back to raw bash — so `/git status`, `/python3 app.py`, `/vim file.txt` all just work.

---

## Features

- **Tab completion** — completes `/commands` and file paths
- **Syntax highlighting** — known commands in gold, unknown in red, args in cyan
- **Autosuggestions** — fish-style ghost text as you type
- **Biome-aware prompt** — prompt changes based on your current directory

| Directory | Biome |
|---|---|
| `~` | Spawn |
| `/tmp` | Nether |
| `/etc` | Stronghold |
| `/dev` | The End |
| `/usr` | Village |
| `/var` | Cave |
| `/home` | Plains |
| anywhere else | Overworld |

- **Chat** — anything typed without `/` shows as `<username> message`
- **History** — saved to `~/.mcsh_history` between sessions

---

## Built with

- [Rust](https://www.rust-lang.org/)
- [rustyline](https://github.com/kkawakam/rustyline) — REPL, completion, highlighting, hints
- [colored](https://github.com/colored-rs/colored) — terminal colors

---

## Roadmap

- ✅ Pipe support (`/cat file.txt | /grep something`)
- [ ] Config file (`~/.mcshrc`) for aliases and settings
- [ ] More biomes
- [ ] Set as login shell support
- [ ] Plugin system

---

## License

MIT
