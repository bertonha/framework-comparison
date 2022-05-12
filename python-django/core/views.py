from rest_framework import mixins, viewsets

from core.models import Form, Question
from core.serializers import FormSerializer, QuestionSerializer


class FormViewSet(mixins.ListModelMixin, mixins.CreateModelMixin, mixins.RetrieveModelMixin, viewsets.GenericViewSet):
    queryset = Form.objects.all()
    serializer_class = FormSerializer


class QuestionViewSet(mixins.ListModelMixin, mixins.CreateModelMixin, mixins.RetrieveModelMixin, viewsets.GenericViewSet):
    queryset = Question.objects.all()
    serializer_class = QuestionSerializer
