# 🕵️‍♂️ Domain Grabber CLI

**Domain Grabber CLI** is a simple command-line tool to fetch all registered domains for a specific date. Built with Rust for speed and efficiency.

## 🚀 Features

- Retrieve lists of registered domains from a specified date
- Interactive date selection or CLI argument input ⌨️
- Save results to a `.txt` file 💾
- Interactive CLI with an ASCII banner 🎨

## 📦 Installation

Download the latest release from:

🔗 [**Domain Grabber CLI Releases**](https://github.com/decryptable/domain-grabber-cli/releases/latest)

1. Select the archive file that matches your OS architecture.
2. Extract the archive.
3. Inside the extracted folder, you will find the `domain-grabber-cli` binary.

## 🛠 Usage

Run the tool using:

```sh
./domain-grabber-cli --date 01.02.2025 --output domains.txt
```

Or run it without arguments for interactive date selection:

```sh
./domain-grabber-cli
```

**CLI Arguments:**
- `-d`, `--date <dd.mm.yyyy>` : Date to fetch domains from.
- `-o`, `--output <PATH>` : Output file path (default: `domains_<date>.txt`).

## 📝 Example Output

```
📅 Fetching domains for date: 01.02.2025
✅ Successfully retrieved 120 domains!
💾 Domains saved to: domains_01.02.2025.txt
```

## 🔧 Contributing

1. Fork this repository 🍴
2. Create a new branch: `git checkout -b feature-xyz` ✨
3. Commit your changes: `git commit -m "Add feature XYZ"` ✅
4. Push to the branch: `git push origin feature-xyz` 🚀
5. Open a Pull Request on GitHub 🔥

## 📜 License

This project is licensed under the [**MIT License**](./LICENSE).