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
