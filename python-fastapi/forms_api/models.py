import sqlalchemy

from .database import metadata


Form = sqlalchemy.Table(
    "forms",
    metadata,
    sqlalchemy.Column("id", sqlalchemy.Integer, primary_key=True),
    sqlalchemy.Column("title", sqlalchemy.String),
)
