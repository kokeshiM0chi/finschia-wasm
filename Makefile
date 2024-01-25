docker-test-contracts:
	./scripts/finschia/template/setup.sh && ./scripts/finschia/start.sh && sleep 30 && docker exec -it finschia-app /root/scripts/integration_test_contracts.sh /bin/sh
