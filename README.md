# jgetset

A simple command-line tool for getting and setting first-level keys in JSON dictionary files.

## Overview

`jgetset` provides a straightforward way to read and write top-level key-value pairs in JSON files that contain dictionaries (objects). It's designed for simple configuration files and quick JSON manipulation.

**Important:** This tool only works with JSON files containing a dictionary at the root level, and only operates on first-level keys. Nested paths are not supported.

## Installation

```bash
cargo build --release
```

## Usage

### Get a value

```bash
jgetset <file> <key>
```

Example:
```bash
jgetset config.json username
# Output: "alice"
```

### Set a value

```bash
jgetset <file> <key>=<value>
```

Example:
```bash
jgetset config.json username=alice
```

Values are stored as strings in the JSON file.

## Example

Given `config.json`:
```json
{
  "host": "localhost",
  "port": "8080"
}
```

```bash
# Get a value
$ jgetset config.json host
"localhost"

# Set a value
$ jgetset config.json timeout=30
```

Result:
```json
{
  "host": "localhost",
  "port": "8080",
  "timeout": "30"
}
```

## Limitations

- Only works with JSON dictionaries (not arrays or primitives)
- Only accesses first-level keys (no nested path support like `user.name`)
- Values are always stored as strings
- No support for deleting keys
