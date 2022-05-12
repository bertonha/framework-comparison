import http from 'k6/http';
import { sleep } from 'k6';

export const options = {
  stages: [
    { duration: '2m', target: 100 },
    { duration: '5m', target: 100 },
  ],
};

export default function () {
  const BASE_URL = 'http://localhost:8000';
  const responses = http.batch([
    ['GET', `${BASE_URL}/forms`, null, {}],
    ['GET', `${BASE_URL}/forms/1`, null, {}],
    ['GET', `${BASE_URL}/forms/2`, null, {}],
  ]);

  sleep(1);
}