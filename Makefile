.PHONY: all

all:
	./node_modules/.bin/truffle build

deploy:
	./node_modules/.bin/truffle deploy

format:
	npx prettier --write contracts/**/*.sol

