FROM python:3.12-slim-bookworm
COPY --from=ghcr.io/astral-sh/uv:0.7.16 /uv /uvx /bin/

WORKDIR /app

ENV PYTHONPATH="/app:${PYTHONPATH}" \
    PYTHONUNBUFFERED=1 \
    PYTHONDONTWRITEBYTECODE=1

# Clears previous environemnt cache
RUN rm -rf /root/.cache/uv

COPY pyproject.toml* .
RUN if [ -f pyproject.toml ]; then \
      uv pip install --system --no-cache -e .; \
    else \
      echo "No pyproject.toml file found. Skipping installation..."; \
    fi

COPY . .

RUN useradd -m appuser
USER appuser
