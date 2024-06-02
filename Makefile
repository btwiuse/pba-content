slides:
	yarn build
	curl -sL https://vara.network/favicon.ico > build/favicon.ico
	cp -r ./build/* ../basics/slides/
