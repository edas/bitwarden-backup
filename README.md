# Bitwarden Backup

A Rust program to backup your Bitwarden vault data. 

It leaves the data encrypted and never requires your master password. It's only as a backup in case you or the admin of your bitwarden server messes something. If ever, you will have to manually decrypt everything with the help of your master password.

## Building the Project

### Prerequisites

- Rust toolchain (latest stable version)
- Cargo (comes with Rust)

### Build

```bash
cargo build --release
```

The executable will be created in `target/release/bitwarden-backup`

## Running the Program

```bash
./target/release/bitwarden-backup --config=config.yaml ./backup
```

### Arguments

- `--config=<FILE>`: Path to the configuration file (required)
- `<DIR>`: Directory path where to export the data (required)

## Configuration File

Create a YAML file (e.g., `config.yaml`) with the following structure:

```yaml
email: "your.email@example.com"
api_url: "https://api.bitwarden.com"
identity_url: "https://identity.bitwarden.com"
client_id: "your_client_id"
client_secret: "your_client_secret"
scope: "api"
grant_type: "client_credentials"
device_type: "21"
device_identifier: "b86dd6ab-4265-4ddf-a7f1-eb28d5677f33"
device_name: "firefox"
```

Most users are in the .com server but you may be in the .eu server if you did opt in for it. You will have a login error if you choose the wrong one.

You can get an API key (client id and secret) by following the documentation on https://bitwarden.com/help/personal-api-key/. It should avoid you to leave your master password in clear somewhere. With the API key you can download the profil and the encrypted vault, but nothing more. 

### Configuration Fields

- `email`: Your Bitwarden account email
- `api_url`: Bitwarden API server URL, probably "https://api.bitwarden.com" or "https://api.bitwarden.eu"
- `identity_url`: Bitwarden Identity server URL, probably "https://identity.bitwarden.com" or "https://identity.bitwarden.eu"
- `client_id`: Your Bitwarden client ID, from an API KEY request. Documentation https://bitwarden.com/help/personal-api-key/
- `client_secret`: Your Bitwarden client secret, from an API KEY request. Documentation https://bitwarden.com/help/personal-api-key/
- `scope`: API scope (usually "api")
- `grant_type`: OAuth grant type (usually "client_credentials")
- `device_type`: Device type identifier (put "21")
- `device_identifier`: Unique identifier for your device (put "b86dd6ab-4265-4ddf-a7f1-eb28d5677f33")
- `device_name`: Name of your device (put "firefox")


## Output Files

The program will create the following files in the specified output directory:

1. `bitwarden.{email}.prelogin.json`: Response from the prelogin request to the identity server
2. `bitwarden.{email}.token.json`: OAuth token response
3. `bitwarden.{email}.profile.json`: Your Bitwarden account profile data
4. `bitwarden.{email}.sync.json`: Your Bitwarden vault sync data

Each file contains the raw JSON response from the respective API endpoint.

## Example

```bash
# Create a backup directory
mkdir -p ./backup

# Run the program
./target/release/bitwarden-backup --config=config.yaml ./backup
```

This will create files like:
```
./backup/
  ├── bitwarden.your.email@example.com.prelogin2.json
  ├── bitwarden.your.email@example.com.token.json
  ├── bitwarden.your.email@example.com.profile.json
  └── bitwarden.your.email@example.com.sync.json
```
