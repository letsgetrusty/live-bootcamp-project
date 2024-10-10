SHELL := /usr/bin/env bash

create-tag:
	git tag -a v${sprint}.${task} -m "sprint${sprint}-task${task}"
	git push origin v${sprint}.${task}
