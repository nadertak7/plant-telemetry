SECRET_FILEPATH = /secrets.enc.yaml

decrypt:
	sops decrypt --in-place ${SECRET_FILEPATH}

encrypt:
	sops encrypt --in-place ${SECRET_FILEPATH}
	python3 scripts/decrypt_secrets.py
