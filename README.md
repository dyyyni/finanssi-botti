# Telegram Bot with Access Control

## Project Overview

This project is a Telegram bot implemented in Rust using the `teloxide` framework.

## Features

- **Access Control**: Only users with whitelisted user IDs can access specific commands.
- **Commands**:
  - `/help`: Displays a list of available commands.
  - `/saldo`: Requests entry, accessible only to whitelisted users.

## Getting Started

### Prerequisites

- Rust programming language
- Telegram bot token

### Installation

1. Clone the repository:
    ```sh
    git clone git@github.com:dyyyni/finanssi-botti.git
    ```

2. Set up environment variables:
    ```sh
    export TELOXIDE_TOKEN=your_telegram_bot_token
    export WHITELIST="123456789,987654321"
    ```

3. Run the bot:
    ```sh
    cargo run
    ```

## Usage

- Add your Telegram bot to a chat or group.
- Use the `/help` command to see the available commands.
- Use the `/saldo` command to request entry (only accessible to whitelisted users).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [teloxide](https://github.com/teloxide/teloxide) for the Telegram bot framework.