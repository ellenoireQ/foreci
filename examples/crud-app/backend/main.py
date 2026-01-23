from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List, Optional
from uuid import uuid4

app = FastAPI()

class Item(BaseModel):
    id: Optional[str] = None
    name: str
    description: str

items = []

@app.get("/")
def read_root():
    return {"message": "Hello World"}

@app.get("/items", response_model=List[Item])
def get_items():
    return items

@app.post("/items", response_model=Item)
def create_item(item: Item):
    item.id = str(uuid4())
    items.append(item)
    return item

@app.delete("/items/{item_id}")
def delete_item(item_id: str):
    global items
    items = [i for i in items if i.id != item_id]
    return {"message": "Item deleted"}
