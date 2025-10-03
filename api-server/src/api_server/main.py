from typing import Dict

from fastapi import FastAPI

app = FastAPI()

@app.get("/")
def index() -> Dict[str, str]:
    """Root route."""
    return {"Hello": "World"}
