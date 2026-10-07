SECRET_FILEPATH = ./secrets.enc.yaml

generate:
	python3 ./scripts/generate_secrets.py

decrypt:
	sops decrypt ${SECRET_FILEPATH}

encrypt:
	sops encrypt --in-place ${SECRET_FILEPATH}
	$(MAKE) generate
