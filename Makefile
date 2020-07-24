make:
	cargo build

install:
	cp -f target/debug/gtktranslate /usr/bin/
	cp -f src/gtktranslate.desktop /usr/share/applications/
	cp -f icons/8x8/org.skylinecc.GtkTranslate.png /usr/share/icons/hicolor/8x8/apps
	cp -f icons/16x16/org.skylinecc.GtkTranslate.png /usr/share/icons/hicolor/16x16/apps
	cp -f icons/22x22/org.skylinecc.GtkTranslate.png /usr/share/icons/hicolor/22x22/apps
	cp -f icons/24x24/org.skylinecc.GtkTranslate.png /usr/share/icons/hicolor/24x24/apps
	cp -f icons/32x32/org.skylinecc.GtkTranslate.png /usr/share/icons/hicolor/32x32/apps
	cp -f icons/48x48/org.skylinecc.GtkTranslate.png /usr/share/icons/hicolor/48x48/apps
	cp -f icons/64x64/org.skylinecc.GtkTranslate.png /usr/share/icons/hicolor/64x64/apps
	cp -f icons/96x96/org.skylinecc.GtkTranslate.png /usr/share/icons/hicolor/96x96/apps
	cp -f icons/256x256/org.skylinecc.GtkTranslate.png /usr/share/icons/hicolor/256x256/apps
	cp -f icons/512x512/org.skylinecc.GtkTranslate.png /usr/share/icons/hicolor/512x512/apps
	cp -f icons/scalable/org.skylinecc.GtkTranslate.svg /usr/share/icons/hicolor/scalable/apps

uninstall:
	rm -f /usr/bin/gtktranslate
	rm -f /usr/share/applications/gtktranslate.desktop
	rm -f /usr/share/icons/hicolor/8x8/apps/org.skylinecc.GtkTranslate.png
	rm -f /usr/share/icons/hicolor/16x16/apps/org.skylinecc.GtkTranslate.svg
	rm -f /usr/share/icons/hicolor/22x22/apps/org.skylinecc.GtkTranslate.svg
	rm -f /usr/share/icons/hicolor/24x24/apps/org.skylinecc.GtkTranslate.svg
	rm -f /usr/share/icons/hicolor/32x32/apps/org.skylinecc.GtkTranslate.svg
	rm -f /usr/share/icons/hicolor/48x48/apps/org.skylinecc.GtkTranslate.svg
	rm -f /usr/share/icons/hicolor/64x64/apps/org.skylinecc.GtkTranslate.svg
	rm -f /usr/share/icons/hicolor/96x96/apps/org.skylinecc.GtkTranslate.svg
	rm -f /usr/share/icons/hicolor/256x256/apps/org.skylinecc.GtkTranslate.svg
	rm -f /usr/share/icons/hicolor/512x512/apps/org.skylinecc.GtkTranslate.svg
	rm -f /usr/share/icons/hicolor/scalable/apps/org.skylinecc.GtkTranslate.svg

