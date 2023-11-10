# Uptime Checker

[![Rust](https://img.shields.io/badge/Language-Rust-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue)](LICENSE)

Uptime Checker is a Rust application that periodically checks the accessibility of specified URLs and logs the results to a CSV or JSON file.

## Features

- Periodically checks URLs for accessibility
- Logs timestamp, URL, HTTP status code, and other relevant data
- Supports output in CSV or JSON format

## Installation

### Prerequisites

Make sure you have the following tools installed on your system:

- [Rust](https://www.rust-lang.org/tools/install): The programming language used to build the application.
- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git): Version control system to clone the repository.

1. **Clone the Repository:**
   ```sh
   git clone https://github.com/your-username/url-checker.git
   cd url-checker
   ```

2. **Navigate to the Project Directory:**
    ```sh
    cd url-checker
    ```

2. **Build the Application:**
   ```sh
   cargo build --release
   ```

3. **Configure URLs:**
   Update the `config.yaml` file with the URLs you want to check, file paths, and other configurations.

## Usage

**Run the Application:**

```sh
cargo run
```

The program will start checking the specified URLs and logging the results to the configured file.

## Configuration

Edit the `config.yaml` file to customize the application's behavior. Example configuration:

```yaml
urls_to_check:
  - https://example.com
  - https://another-url.com
csv_file_path: "output.csv"
json_file_path: "output.json"
ping_interval_seconds: 900
output_format: "csv"
```

## Customization

Feel free to modify the code or create additional features according to your needs.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.