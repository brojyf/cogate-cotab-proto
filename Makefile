.PHONY: proto test

proto:
	mkdir -p gen/go
	PATH="$$PATH:$$HOME/go/bin" buf generate

test:
	go test ./...
	cargo test --all-targets
	buf lint
