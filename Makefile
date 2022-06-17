.PHONY: all

all:
	./node_modules/.bin/truffle build

format:
	npx prettier --write contracts/**/*.sol

