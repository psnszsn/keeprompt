# KeePrompt

I wrote this in order to use a KeePass database like you whould use pass.

This uses pinentry to prompt you for the master password, unlocks the database, propmts you to select a password using dmenu and finally copies it to clipboard using xclip/wl-copy.

## Installation

* Install from AUR / Cargo
* Create ~/.config/keeprompt/config.toml
  ```toml
  database = "/home/user/Passwords.kdbx"
  dmenu = "bemenu"
  pinentry = "pinentry-gnome3"
  ```
