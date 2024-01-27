![Forks](https://img.shields.io/github/forks/Da4ndo/projectstructure?label=Forks&color=lime&logo=githubactions&logoColor=lime)
![Stars](https://img.shields.io/github/stars/Da4ndo/projectstructure?label=Stars&color=yellow&logo=reverbnation&logoColor=yellow)
![License](https://img.shields.io/github/license/Da4ndo/projectstructure?label=License&color=808080&logo=gitbook&logoColor=808080)
![Issues](https://img.shields.io/github/issues/Da4ndo/projectstructure?label=Issues&color=red&logo=ifixit&logoColor=red)

# ProjectStructure

ProjectStructure is a sophisticated utility designed for analyzing and preserving your project's structure. It offers an intuitive and efficient methodology for initializing and updating your project structures. Leveraging the speed and efficiency of Rust, it delivers rapid scanning capabilities. In a benchmark test, a 17GB project folder was scanned in just 1.7 seconds. The full command execution typically takes from around 0.1 to 0.5 seconds, although this can vary based on hardware specifications and project size.
Project made with ❤ by Da4ndo.

You can click on the star (⭐️) button above this repository if you liked this project! Thank you all. 🙏

## 🚀 Features

- Rapid Scanning Capabilities
- Auto-fill Details
- .projectstructureignore Support

## 📜 Flags
```
    -h, --help       Prints help information
    -V, --version    Prints version information
```
## 📚 Subcommands
```
    help      Prints this message or the help of the given subcommand(s)
    init      Initializes a new project structure.
    update    Updates an existing project structure.
```

## 🛠️ Getting Started

### 🔧 Installation

You can install ProjectStructure by running the following command: 
```bash
curl https://cdn.da4ndo.com/projectstructure/setup.sh | sh
```
OR
```bash
curl https://raw.githubusercontent.com/Da4ndo/projectstructure/main/setup.sh | sh
```
Alternatively, you can clone the repository and build the project using `cargo build`, then use [`install.sh`](https://github.com/Da4ndo/projectstructure/blob/main/install.sh). 

> The installation creates a folder `~/.da4ndo/`.

## 📖 Documentation

ProjectStructure is a highly efficient tool that leverages the power of Tokio's multi-threading to swiftly traverse through your project's directories, subdirectories, and files. It's designed to automatically extract and set the name, description, and version of your project from files like `Cargo.toml`, `package.json`, or similar. In case it fails to find these details, it will resort to default values.

The application also recognizes a `.projectstructureignore` file, which you can use to specify files and directories that should be ignored during the scanning process. However, the presence of this file is not mandatory, and the application will function normally without it.

## 📝 License

This project is licensed under the MIT License. See the **LICENSE** file for details.

## 🤝 Contributing
Contributions are welcome. Feel free to fix problems, report bugs, or propose new features. 