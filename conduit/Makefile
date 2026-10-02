TOPIC ?= sensor/1
ADC ?= 300
TIMESTAMP ?= 1

query-cache:
	cargo sqlx prepare --workspace --all -- --all-targets

send-test-data:
	docker compose exec broker mosquitto_pub -u $(USER) -P $(PASSWORD) -t $(TOPIC) -m '{"adc": $(ADC), "timestamp": $(TIMESTAMP)}'
