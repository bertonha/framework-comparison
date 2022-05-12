from django.db import models


class Form(models.Model):
    title = models.CharField(max_length=250)

    class Meta:
        db_table = "forms"


class Question(models.Model):
    title = models.CharField(max_length=250)
