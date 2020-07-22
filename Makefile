make:
	cargo build

install:
	cp -f target/debug/gtktranslate /usr/bin/
	cp -f src/gtktranslate.desktop /usr/share/applications/
	cp -f icons/8x8/gtktranslate.png /usr/share/icons/Adwaita/8x8/apps
	cp -f icons/16x16/gtktranslate.png /usr/share/icons/Adwaita/16x16/apps
	cp -f icons/22x22/gtktranslate.png /usr/share/icons/Adwaita/22x22/apps
	cp -f icons/24x24/gtktranslate.png /usr/share/icons/Adwaita/24x24/apps
	cp -f icons/32x32/gtktranslate.png /usr/share/icons/Adwaita/32x32/apps
	cp -f icons/48x48/gtktranslate.png /usr/share/icons/Adwaita/48x48/apps
	cp -f icons/64x64/gtktranslate.png /usr/share/icons/Adwaita/64x64/apps
	cp -f icons/96x96/gtktranslate.png /usr/share/icons/Adwaita/96x96/apps
	cp -f icons/256x256/gtktranslate.png /usr/share/icons/Adwaita/256x256/apps
	cp -f icons/512x512/gtktranslate.png /usr/share/icons/Adwaita/512x512/apps
	cp -f icons/scalable/gtktranslate.svg /usr/share/icons/Adwaita/scalable/apps
	cp -f icons/8x8/gtktranslate.png /usr/share/icons/Adwaita/8x8/apps
	cp -f icons/16x16/gtktranslate.png /usr/share/icons/hicolor/16x16/apps
	cp -f icons/22x22/gtktranslate.png /usr/share/icons/hicolor/22x22/apps
	cp -f icons/24x24/gtktranslate.png /usr/share/icons/hicolor/24x24/apps
	cp -f icons/32x32/gtktranslate.png /usr/share/icons/hicolor/32x32/apps
	cp -f icons/48x48/gtktranslate.png /usr/share/icons/hicolor/48x48/apps
	cp -f icons/64x64/gtktranslate.png /usr/share/icons/hicolor/64x64/apps
	cp -f icons/96x96/gtktranslate.png /usr/share/icons/hicolor/96x96/apps
	cp -f icons/256x256/gtktranslate.png /usr/share/icons/hicolor/256x256/apps
	cp -f icons/512x512/gtktranslate.png /usr/share/icons/hicolor/512x512/apps
	cp -f icons/scalable/gtktranslate.svg /usr/share/icons/hicolor/scalable/apps

uninstall:
	rm -f /usr/bin/gtktranslate
	rm -f /usr/share/applications/gtktranslate.desktop
	rm -f /usr/share/icons/Adwaita/8x8/apps/gtktranslate.png
	rm -f /usr/share/icons/Adwaita/16x16/apps/gtktranslate.png
	rm -f /usr/share/icons/Adwaita/22x22/apps/gtktranslate.png
	rm -f /usr/share/icons/Adwaita/24x24/apps/gtktranslate.png
	rm -f /usr/share/icons/Adwaita/32x32/apps/gtktranslate.png
	rm -f /usr/share/icons/Adwaita/36x36/apps/gtktranslate.png
	rm -f /usr/share/icons/Adwaita/42x42/apps/gtktranslate.png
	rm -f /usr/share/icons/Adwaita/48x48/apps/gtktranslate.png
	rm -f /usr/share/icons/Adwaita/64x64/apps/gtktranslate.png
	rm -f /usr/share/icons/Adwaita/72x72/apps/gtktranslate.png
	rm -f /usr/share/icons/Adwaita/96x96/apps/gtktranslate.png
	rm -f /usr/share/icons/Adwaita/128x128/apps/gtktranslate.png
	rm -f /usr/share/icons/Adwaita/192x192/apps/gtktranslate.png
	rm -f /usr/share/icons/Adwaita/256x256/apps/gtktranslate.png
	rm -f /usr/share/icons/Adwaita/512x512/apps/gtktranslate.png
	rm -f /usr/share/icons/Adwaita/scalable/apps/gtktranslate.svg
