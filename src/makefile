release:
	sed -i -e 's/const DEBUG = true/const DEBUG = false/' resources/tis/constants.tis
	packfolder resources resources.rc -binary
	@echo "Successfully compiled resources under release mode!"

debug:
	sed -i -e 's/const DEBUG = false/const DEBUG = true/' resources/tis/constants.tis
	packfolder resources resources.rc -binary
	@echo "Successfully compiled resources under debug mode!"
