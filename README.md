### Llama Model Selector (LMS)

An interactive and minimalist launcher, developed in Rust, to manage and run Artificial Intelligence models locally using `llama.cpp`.

The goal of this project is to eliminate the need to manage dozens of scripts (like `.bat` or `.sh` files) or memorize complex command-line parameters. Upon startup, LMS offers an interactive terminal menu where you can choose the execution environment (Web or CLI) and select which GGUF model to load into memory.

---

### Features

* **Interactive Menu:** Dynamically select the interface and the model directly from the terminal.
* **Web Mode:** Starts `llama-server` and exposes a graphical interface in your browser via a local port (`localhost:8080`).
* **CLI Mode:** Starts `llama-cli` for fast, direct interactions purely via the terminal.
* **Dynamic Paths:** Support for command-line flags to point to custom directories, allowing you to keep `llama.cpp` binaries separate from your heavy model storage.

---

### Default Directory Structure

If you do not provide any command-line parameters, the executable will look for the following structure in the same directory where it is launched:

```text
📁 Your_Directory
 ├── 📄 lms.exe (The compiled executable of this project)
 ├── 📁 llama/ 
 │    ├── 📄 llama-server.exe
 │    └── 📄 llama-cli.exe
 └── 📁 models/ 
      ├── 📄 model-v1-q4.gguf
      └── 📄 model-v2-q5.gguf

```

> **Note:** The program automatically filters the models directory to list only files with the `.gguf` extension.

---

### Usage

#### 1. Default Execution

Simply run the program. It will assume the `.\llama` directory for binaries and the `models` directory for AI files.

```bash
.\lms.exe

```

#### 2. Using Custom Directories

You can pass parameters (flags) to specify where the binaries and models are located, regardless of the order. This is ideal for keeping your heavy models on a secondary HDD and the executables on an SSD.

* **`-m`** or **`--models`**: Sets the path to the folder containing the `.gguf` files.
* **`-l`** or **`--llama`**: Sets the path to the folder containing the `llama.cpp` executables.

**Examples:**
Changing only the models folder:

```bash
.\lms.exe -m "D:\AIs\GGUF_Models"

```

Changing both directories:

```bash
.\lms.exe --llama "C:\Tools\llama-bin" --models "E:\My_Models"

```

---

###  How to Build from Source

Make sure you have [Rust and the Cargo package manager](https://rustup.rs/) installed on your system.

1. Clone the repository and navigate to the project folder.


2. Compile the optimized release build:
```bash
cargo build --release

```

3. The ready-to-use executable will be located in the `target/release/` folder.