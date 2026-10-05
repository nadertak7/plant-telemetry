"""
Decrypt SOPS secrets into individual files.
Requires AWS credentials with permissions to use the configured KMS key.
"""

import json
import subprocess
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent.parent
ENCRYPTED_SECRETS_FILE = PROJECT_ROOT / "secrets.enc.yaml"
DECRYPTED_SECRETS_DIR = PROJECT_ROOT / "secrets"


def decrypt_secrets_file() -> dict[str, str]:
    """
    Decrypt a secrets file with sops and return its secret names and values.
    """
    return json.loads(
        subprocess.check_output(
            ["sops", "decrypt", ENCRYPTED_SECRETS_FILE, "--output-type", "json"]
        )
    )


def remove_unused_secrets(secrets: dict[str, str]) -> None:
    """
    Remove existing secrets files whose names are not in the sops file.
    """
    for file in DECRYPTED_SECRETS_DIR.iterdir():
        if file.name not in secrets:
            file.unlink()


def write_decrypted_secret_to_file(secrets: dict[str, str]) -> None:
    """
    Write each secret to a file named after its key. Assign read permissions only to the owner.
    """
    for key, value in secrets.items():
        output_path = DECRYPTED_SECRETS_DIR / key
        if output_path.exists():
            output_path.chmod(0o600)
        output_path.write_text(value, encoding="utf-8")
        output_path.chmod(0o400)


def main() -> None:
    DECRYPTED_SECRETS_DIR.mkdir(0o700, exist_ok=True)
    DECRYPTED_SECRETS_DIR.chmod(0o700)
    json_secrets = decrypt_secrets_file()
    remove_unused_secrets(json_secrets)
    write_decrypted_secret_to_file(json_secrets)
    DECRYPTED_SECRETS_DIR.chmod(0o500)


if __name__ == "__main__":
    main()
