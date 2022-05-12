from .database import database
from . import models, schemas


async def get_form(form_id: int):
    query = models.Form.select().where(models.Form.c.id == form_id)
    return await database.fetch_one(query)


async def get_forms(skip: int = 0, limit: int = 100):
    query = models.Form.select().offset(skip).limit(limit)
    return await database.fetch_all(query)


async def create_form(form: schemas.FormCreate):
    query = models.Form.insert().values(title=form.title)
    last_record_id = await database.execute(query)
    return {**form.dict(), "id": last_record_id}
