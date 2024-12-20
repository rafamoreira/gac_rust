# GAC (Git Auto Commit)

A minimalist CLI tool that automates git commits for single-developer repositories.

## Overview

GAC simplifies the git commit process by automatically staging all changes and 
creating commits with sequential numbers as messages. It's designed for personal 
projects where detailed commit messages aren't critical.

## Important Limitations

This tool is **not** recommended for:
- Team projects
- Professional repositories
- Repositories requiring meaningful commit history
- Any repositories that are actually useful
- Any project at all, really

## Why This Exists

This project was primarily developed as a learning exercise for:
- Rust programming
- AUR package development
- CLI tool creation

## Usage

Simply run the command in your git repository:

```bash
gac
```

This will:
1. Stage all changes (`git add .`)
2. Create a commit with an incremental number

## Security Warning

GAC automatically stages **all** files in your repository. Be extremely careful when using this tool, as it may accidentally commit:
- Configuration files with secrets
- Local environment files
- Private credentials
- Other sensitive data

## Best Practices

While this tool exists for convenience, consider these better alternatives:
- Write descriptive commit messages that explain your changes
- Use `.gitignore` to exclude sensitive files
- Stage files deliberately rather than using `git add .`

## Meta

This repository uses GAC for its own commit management as a demonstration of the tool's functionality.
