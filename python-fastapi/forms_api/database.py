import os

from databases import Database
import sqlalchemy


DATABASE_URL = os.getenv("DATABASE_URL", "")

database = Database(DATABASE_URL)

metadata = sqlalchemy.MetaData()
