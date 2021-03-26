#!/bin/bash

for i in 8x8 16x16 22x22 24x24 32x32 42x42 48x48 64x64 72x72 96x96 128x128 192x192 256x256 512x512; do
    echo copying icon ${i}
    install -Dm0644 -t "/usr/share/icons/hicolor/$i/apps/gtktranslate.png" "icons/${i}/gtktranslate.png"
done

echo "copying icon scalable"
install -Dm0644 -t "/usr/share/icons/hicolor/scalable/apps/gtktranslate.svg" "icons/scalable/gtktranslate.svg"


echo "Installing binary"
install -Dm755 "target/release/gtktranslate" "/usr/bin/gtktranslate"

echo "Installing desktop file"
install -Dm0644 -t "/usr/share/applications" "src/gtktranslate.desktop"