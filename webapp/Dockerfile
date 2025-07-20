FROM python:3.12-slim-bookworm
COPY --from=ghcr.io/astral-sh/uv:0.7.16 /uv /uvx /bin/

WORKDIR /app

ENV PYTHONPATH="/app:/app/src:${PYTHONPATH}" \
    PATH="/usr/local/bin:${PATH}" \
    PYTHONUNBUFFERED=1 \
    PYTHONDONTWRITEBYTECODE=1

# Clears previous environemnt cache
RUN rm -rf /root/.cache/uv

COPY pyproject.toml* .
RUN if [ -f pyproject.toml ]; then \
      uv pip install --system --no-cache .; \
    else \
      echo "No pyproject.toml file found. Skipping installation..."; \
    fi

COPY src/ .

# CMD ["streamlit", "run", "1_homepage.py", "--server.port=8501"]