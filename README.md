# Vessel (vsl)

A lightweight, serverless command-line tool for managing data with SQLite storage.

## Features

- Serverless architecture
- SQLite-based storage
- Simple command-line interface
- Zero configuration required
- Portable data storage

## Installation

```bash
# Installation instructions will be added
```

## Usage

The basic command syntax is:

```bash
vsl [command] [options]
```

Example commands:
```bash
vsl --help           # Show help information
vsl --version        # Show version information
```

## Storage

All data is stored locally in SQLite database files, requiring no external server setup or configuration.

## Development

To contribute to Vessel:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
#!/usr/bin/env python3
"""
Vessel (vsl) - A lightweight, serverless command-line tool
"""
import click
import os
from .db import init_db

@click.group()
@click.version_option(version='0.1.0')
def cli():
    """Vessel - Local data management tool"""
    pass

@cli.command()
def init():
    """Initialize a new Vessel database"""
    try:
        init_db()
        click.echo("Initialized empty Vessel database")
    except Exception as e:
        click.echo(f"Error: {str(e)}", err=True)

if __name__ == '__main__':
    cli()
import sqlite3
import os

DEFAULT_DB_PATH = os.path.expanduser("~/.vessel/data.db")

def init_db():
    """Initialize the SQLite database"""
    os.makedirs(os.path.dirname(DEFAULT_DB_PATH), exist_ok=True)
    
    conn = sqlite3.connect(DEFAULT_DB_PATH)
    cursor = conn.cursor()
    
    # Add initial schema here as needed
    
    conn.commit()
    conn.close()
from .cli import cli

__version__ = '0.1.0'
from setuptools import setup, find_packages

setup(
    name='vessel',
    version='0.1.0',
    packages=find_packages(),
    include_package_data=True,
    install_requires=[
        'Click',
    ],
    entry_points={
        'console_scripts': [
            'vsl=vsl.cli:cli',
        ],
    },
)
