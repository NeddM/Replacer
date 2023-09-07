# Replacer

Script made with Rust. Let you swap the variable names on a file. It is made to migrate Azure Pipelines with replace tokens to Github Actions.

## How to use

Move the file you want to process inside the repo, and add te variables on the `replace.txt` like this:

```
__Hi__: world
__Hola__: mundo
```

The script will swap the key for the values in any coincidence inside the file to process.

The script will ask you to input a name for the new file processed, if you dont care about it, it will be named `export.yaml`.
