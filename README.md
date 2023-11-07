# Obsidian2Neorg
Transform from Obsidian Markdown to Neorg format.

`obsidian2neorg` takes input from `stdin` and outputs the transformation to `stdout`.

You can transform a file like this:
```bash
cat file.md | obsidian2neorg > file.norg
```

## Installation
You'll need to have [rustup](https://rustup.rs/) installed. Then install with Cargo
```
cargo install obsidian2neorg
```

## Bulk Transformations
You can find in this repository a shell script that will transform all `.md` files
in a folder and copy all other files. I've used this script to port all my notes to
Neorg using obsidian2neorg.
```bash
folder_path="vault"
out_path="vault_neorg"
mkdir -p $out_path
find "$folder_path" -type f -not -path "*/.git/*" -not -name "*.md" | while read -r file; do
    relative_path="${file#$folder_path/}"
    out_file="${out_path}/${relative_path}"
    mkdir -p "$(dirname "$out_file")"
    cp "$file" "$out_file"
done

find "$folder_path" -type f -not -path "*/.git/*" -name "*.md" | while read -r file; do
    lowercase_filename=$(echo "$file" | tr '[:upper:]' '[:lower:]')
    final_filename=${lowercase_filename// /-}
    relative_path="${final_filename#$folder_path/}"
    out_file="${out_path}/${relative_path%.md}.norg"
    mkdir -p "$(dirname "$out_file")"
    cat "$file" | obsidian2neorg > "$out_file"
done
```

The transformations are based on regex, and not every possible Markdown style is implemented,
but it should be enough to get your notes in an acceptable state to start working on Neorg.

## lowercase-links
By default file links are forced into `lowercase-without-spaces` form. This makes it easy to rename all
files so that relations are easily transferred.
If you want to keep them as they are pass the `--literal-links` flag:
```bash
obsidian2neorg --literal-links
```

In this case you'll need to modify the bulk transformation script to avoid the renaming
```bash
folder_path="vault" # change this to the location of your vault/notes folder
out_path="vault_neorg" # this will be the output folder

mkdir -p $out_path
find "$folder_path" -type f -not -path "*/.git/*" -not -name "*.md" | while read -r file; do
    relative_path="${file#$folder_path/}"
    out_file="${out_path}/${relative_path}"
    mkdir -p "$(dirname "$out_file")"
    cp "$file" "$out_file"
done

find "$folder_path" -type f -not -path "*/.git/*" -name "*.md" | while read -r file; do
    relative_path="${file#$folder_path/}"
    out_file="${out_path}/${relative_path%.md}.norg"
    mkdir -p "$(dirname "$out_file")"
    cat "$file" | obsidian2neorg --literal-links > "$out_file"
done
```
