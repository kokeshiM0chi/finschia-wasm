contracts-test:
	cd scripts && bash -xe integration_test_contracts.sh

docker-start:
	./scripts/finschia/template/setup.sh && ./scripts/finschia/start.sh
