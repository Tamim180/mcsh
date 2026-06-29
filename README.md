```
  ███╗   ███╗ ██████╗███████╗██╗  ██╗
  ████╗ ████║██╔════╝██╔════╝██║  ██║
  ██╔████╔██║██║     ███████╗███████║
  ██║╚██╔╝██║██║     ╚════██║██╔══██║
  ██║ ╚═╝ ██║╚██████╗███████║██║  ██║
  ╚═╝     ╚═╝ ╚═════╝╚══════╝╚═╝  ╚═╝
```

# ⛏️ Minecraft Shell (mcsh)

> A Minecraft-inspired Linux shell built in Rust where every command starts with `/`.

`mcsh` transforms your Linux terminal into a Minecraft-like experience. Teleport through directories, build folders instead of creating them, smite processes, and chat simply by typing without a `/`.

---

## ✨ Features

* 🎮 Minecraft-inspired command system
* 🦀 Written in Rust
* 🌈 Built-in syntax highlighting
* 💡 Built-in autosuggestions
* ⚡ Built-in tab completion
* 💬 Chat-style input for non-command text
* 📁 File & directory management
* ⚙️ Process management
* 🖥️ Execute any shell command by prefixing it with `/`

---

## 📦 Installation

### Arch Linux (AUR)

Using `yay`:

```bash
yay -S mcsh
```

Using `paru`:

```bash
paru -S mcsh
```

### Build from source

```bash
git clone https://github.com/Tamim180/mcsh.git
cd mcsh
cargo build --release
./target/release/mcsh
```

---

## 🎮 Command Examples

| Minecraft   | Linux Equivalent |
| ----------- | ---------------- |
| `/tp`       | `cd`             |
| `/spawn`    | `cd ~`           |
| `/whereami` | `pwd`            |
| `/list`     | `ls`             |
| `/build`    | `mkdir`          |
| `/destroy`  | `rm`             |
| `/summon`   | `touch`          |
| `/clone`    | `cp`             |
| `/move`     | `mv`             |
| `/read`     | `cat`            |
| `/entities` | `ps`             |
| `/smite`    | `kill`           |
| `/lag`      | `top`            |
| `/seed`     | `env`            |
| `/clear`    | `clear`          |
| `/kill`     | `exit`           |

> Unknown `/commands` are automatically executed as normal shell commands.

---

## 📸 Example

```text
[tamim @ Overworld] ~/projects > /list
📄 Cargo.toml
📦 src

[tamim @ Overworld] ~/projects > ls
<tamim> ls

[tamim @ Overworld] ~/projects > /tp src

[tamim @ Overworld] ~/projects/src > /read main.rs
```

---

## 🚀 Roadmap

* [ ] Config file support
* [ ] Themes
* [ ] More Minecraft-inspired commands
* [ ] Plugin system
* [ ] Windows support

Ideas and pull requests are always welcome!

---

## 🤝 Contributing

Found a bug? Have an idea for a new command? Feel free to open an issue or submit a pull request.

---

## 📄 License

This project is licensed under the MIT License.
