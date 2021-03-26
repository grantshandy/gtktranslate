#!/bin/bash

for i in 8x8 16x16 22x22 24x24 32x32 42x42 48x48 64x64 72x72 96x96 128x128 192x192 256x256 512x512; do
    echo removing icon ${i}
    rm -rf /usr/share/icons/hicolor/${i}/apps/gtktranslate.png
done

echo "removing icon scalable"
rm -rf /usr/share/icons/hicolor/scalable/apps/gtktranslate.svg

echo "Installing binary"
rm -rf /usr/bin/gtktranslate

echo "Installing desktop file"
rm /usr/share/applications/gtktranslate.desktop