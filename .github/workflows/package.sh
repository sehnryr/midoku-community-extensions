#!/bin/env bash

set -e

workspace_dir=$(dirname $(cargo locate-project --workspace --message-format=plain))
gh_pages_dir="$workspace_dir/gh-pages"

# Create the gh-pages directory if it doesn't exist
mkdir -p $gh_pages_dir
mkdir -p $gh_pages_dir/icons
mkdir -p $gh_pages_dir/extensions

# Clean the index.json file
rm -f $gh_pages_dir/index.json
rm -f $gh_pages_dir/index.min.json

# For all packages in src/*/*, copy the wasm file to the package directory
for package_dir in "$workspace_dir/src/*/*"; do
    package_dir=$(realpath $package_dir)

    # Get infos from the Cargo.toml file
    package_name=$(grep -oP -m 1 'name = "\K[^"]+' $package_dir/Cargo.toml)
    package_version=$(grep -oP -m 1 'version = "\K[^"]+' $package_dir/Cargo.toml)

    # Get the path to the wasm file
    target_file=$(sed 's/-/_/g' <<< $package_name).wasm
    target="$workspace_dir/target/wasm32-unknown-unknown/release/$target_file"

    # Copy the wasm file to the package directory
    cp $target $package_dir/res/extension.wasm

    packaged_name=${package_name:7}
    packaged_name=${packaged_name/-/.}

    # Zip the extension and manifest
    tar -czf "$gh_pages_dir/extensions/$packaged_name-v$package_version.mix" -C $package_dir/res .

    # Copy the icon
    cp $package_dir/res/icon.png "$gh_pages_dir/icons/$packaged_name.png"

    # Clean up
    rm $package_dir/res/extension.wasm

    # Update the index.json file
    extension_name=$(grep -oP -m 1 '"name": "\K[^"]+' $package_dir/res/source.json)
    extension_version=$(grep -oP -m 1 '"version": "\K[^"]+' $package_dir/res/source.json)
    extension_language=$(grep -oP -m 1 '"language": "\K[^"]+' $package_dir/res/source.json)
    extension_nsfw=$(grep -oP -m 1 '"nsfw": \K[^,]+' $package_dir/res/source.json)

    if [ -f "$gh_pages_dir/index.json" ]; then
        # Add a comma right after the previous entry, after the "}" character
        sed -i 's/}$/},/' "$gh_pages_dir/index.json"
        echo -n "," >> "$gh_pages_dir/index.min.json"
    else
        echo "[" > "$gh_pages_dir/index.json"
        echo -n "[" > "$gh_pages_dir/index.min.json"
    fi

    cat <<EOF >> "$gh_pages_dir/index.json"
    {
        "id": "$packaged_name",
        "name": "$extension_name",
        "extension": "$packaged_name-v$package_version.mix",
        "icon": "$packaged_name.png",
        "version": "$extension_version",
        "language": "$extension_language",
        "nsfw": $extension_nsfw
    }
EOF
    echo -n "{\"id\":\"$packaged_name\",\"name\":\"$extension_name\",\"extension\":\"$packaged_name-v$package_version.mix\",\"icon\":\"$packaged_name.png\",\"version\":\"$extension_version\",\"language\":\"$extension_language\",\"nsfw\":$extension_nsfw}" >> "$gh_pages_dir/index.min.json"
done

# Close the index.json file
echo -n "]" >> "$gh_pages_dir/index.json"
echo -n "]" >> "$gh_pages_dir/index.min.json"
