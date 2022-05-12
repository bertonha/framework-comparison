from pydantic import BaseModel


class FormBase(BaseModel):
    title: str


class FormCreate(FormBase):
    pass


class Form(FormBase):
    id: int
