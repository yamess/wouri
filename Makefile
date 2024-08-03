create-net:
	docker network create --driver bridge wouri-net

run:
	cargo watch -x 'run --bin worker'