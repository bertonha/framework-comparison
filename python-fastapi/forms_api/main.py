from fastapi import FastAPI, HTTPException

from . import crud, schemas
from .database import database


app = FastAPI()


@app.on_event("startup")
async def startup():
    await database.connect()


@app.on_event("shutdown")
async def shutdown():
    await database.disconnect()


@app.post("/forms", response_model=schemas.Form)
async def create_form(form: schemas.FormCreate):
    return await crud.create_form(form=form)


@app.get("/forms", response_model=list[schemas.Form])
async def read_forms(skip: int = 0, limit: int = 100):
    forms = await crud.get_forms(skip=skip, limit=limit)
    return forms


@app.get("/forms/{form_id}", response_model=schemas.Form)
async def read_form(form_id: int):
    db_form = await crud.get_form(form_id=form_id)
    if db_form is None:
        raise HTTPException(status_code=404, detail="Form not found")
    return db_form
