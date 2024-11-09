import uvicorn

from fastapi import FastAPI
import httpx
from pydantic import BaseModel

app = FastAPI()

ACTIX_SERVER_URL = "http://127.0.0.1:8081"  # Actix Web Server URL

# Define the Pydantic models for validation
class Item(BaseModel):
    id: int
    name: str


@app.post("/create_item/")
async def create_item(item: Item):
    async with httpx.AsyncClient() as client:
        response = await client.post(f"{ACTIX_SERVER_URL}/items", json=item.dict())
        if response.status_code == 201:
            return {"message": "Item created successfully"}
        return {"message": "Failed to create item"}


@app.get("/get_items/")
async def get_items():
    async with httpx.AsyncClient() as client:
        response = await client.get(f"{ACTIX_SERVER_URL}/items")
        if response.status_code == 200:
            return response.json()
        return {"message": "Failed to fetch items"}


@app.get("/get_item/{item_id}")
async def get_item(item_id: int):
    async with httpx.AsyncClient() as client:
        response = await client.get(f"{ACTIX_SERVER_URL}/items/{item_id}")
        if response.status_code == 200:
            return response.json()
        return {"message": "Item not found"}


@app.put("/update_item/{item_id}")
async def update_item(item_id: int, item: Item):
    async with httpx.AsyncClient() as client:
        response = await client.put(f"{ACTIX_SERVER_URL}/items/{item_id}", json=item.dict())
        if response.status_code == 200:
            return {"message": "Item updated successfully"}
        return {"message": "Failed to update item"}


@app.delete("/delete_item/{item_id}")
async def delete_item(item_id: int):
    async with httpx.AsyncClient() as client:
        response = await client.delete(f"{ACTIX_SERVER_URL}/items/{item_id}")
        if response.status_code == 200:
            return {"message": "Item deleted successfully"}
        return {"message": "Failed to delete item"}


if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=8000)
