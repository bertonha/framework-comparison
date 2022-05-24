# Rust actix-web
running (7m00.9s), 000/100 VUs, 35372 complete and 0 interrupted iterations
default ✓ [======================================] 000/100 VUs  7m0s

     data_received..................: 28 MB  66 kB/s
     data_sent......................: 9.2 MB 22 kB/s
     http_req_blocked...............: avg=11.38µs min=0s    med=3µs     max=5.2ms   p(90)=10µs    p(95)=32µs
     http_req_connecting............: avg=2.19µs  min=0s    med=0s      max=4.33ms  p(90)=0s      p(95)=0s
     http_req_duration..............: avg=17.02ms min=957µs med=14.12ms max=91.06ms p(90)=34.56ms p(95)=42.61ms
       { expected_response:true }...: avg=17.02ms min=957µs med=14.12ms max=91.06ms p(90)=34.56ms p(95)=42.61ms
     http_req_failed................: 0.00%  ✓ 0          ✗ 106116
     http_req_receiving.............: avg=61.52µs min=4µs   med=32µs    max=11.77ms p(90)=108µs   p(95)=164µs
     http_req_sending...............: avg=23.86µs min=2µs   med=12µs    max=16.8ms  p(90)=32µs    p(95)=52µs
     http_req_tls_handshaking.......: avg=0s      min=0s    med=0s      max=0s      p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=16.94ms min=926µs med=14.03ms max=90.95ms p(90)=34.47ms p(95)=42.5ms
     http_reqs......................: 106116 252.092607/s
     iteration_duration.............: avg=1.01s   min=1s    med=1.01s   max=1.09s   p(90)=1.03s   p(95)=1.04s
     iterations.....................: 35372  84.030869/s
     vus............................: 100    min=1        max=100
     vus_max........................: 100    min=100      max=100


# Fastapi - gunicorn -k uvicorn.workers.UvicornWorker forms_api.main:app
running (7m00.8s), 000/100 VUs, 34672 complete and 0 interrupted iterations
default ✗ [======================================] 000/100 VUs  7m0s

     data_received..................: 29 MB  69 kB/s
     data_sent......................: 9.0 MB 21 kB/s
     http_req_blocked...............: avg=8µs     min=0s     med=2µs     max=9.37ms   p(90)=6µs     p(95)=10µs
     http_req_connecting............: avg=2.2µs   min=0s     med=0s      max=4.77ms   p(90)=0s      p(95)=0s
     http_req_duration..............: avg=35.02ms min=1.77ms med=33.46ms max=177.7ms  p(90)=59.19ms p(95)=68.1ms
       { expected_response:true }...: avg=35.02ms min=1.77ms med=33.46ms max=177.7ms  p(90)=59.19ms p(95)=68.1ms
     http_req_failed................: 0.00%  ✓ 0          ✗ 104016
     http_req_receiving.............: avg=44.74µs min=4µs    med=22µs    max=14ms     p(90)=74µs    p(95)=119µs
     http_req_sending...............: avg=17.42µs min=2µs    med=9µs     max=6.83ms   p(90)=23µs    p(95)=36µs
     http_req_tls_handshaking.......: avg=0s      min=0s     med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=34.96ms min=1.74ms med=33.39ms max=177.68ms p(90)=59.12ms p(95)=68.04ms
     http_reqs......................: 104016 247.215362/s
     iteration_duration.............: avg=1.03s   min=1s     med=1.03s   max=1.17s    p(90)=1.06s   p(95)=1.07s
     iterations.....................: 34672  82.405121/s
     vus............................: 100    min=1        max=100
     vus_max........................: 100    min=100      max=100


# Fastapi - gunicorn -k uvicorn.workers.UvicornWorker forms_api.main:app --worker-connections=1000 --workers=4
running (7m01.0s), 000/100 VUs, 35002 complete and 0 interrupted iterations
default ✗ [======================================] 000/100 VUs  7m0s

     data_received..................: 29 MB  70 kB/s
     data_sent......................: 9.1 MB 22 kB/s
     http_req_blocked...............: avg=7.37µs  min=0s     med=2µs     max=6.81ms   p(90)=6µs     p(95)=9µs
     http_req_connecting............: avg=2.14µs  min=0s     med=0s      max=4.16ms   p(90)=0s      p(95)=0s
     http_req_duration..............: avg=25ms    min=1.74ms med=24.01ms max=112.11ms p(90)=39.3ms  p(95)=45.38ms
       { expected_response:true }...: avg=25ms    min=1.74ms med=24.01ms max=112.11ms p(90)=39.3ms  p(95)=45.38ms
     http_req_failed................: 0.00%  ✓ 0          ✗ 105006
     http_req_receiving.............: avg=40.18µs min=4µs    med=22µs    max=9.21ms   p(90)=69µs    p(95)=105µs
     http_req_sending...............: avg=18.26µs min=1µs    med=9µs     max=6ms      p(90)=23µs    p(95)=35µs
     http_req_tls_handshaking.......: avg=0s      min=0s     med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=24.94ms min=1.71ms med=23.96ms max=112.09ms p(90)=39.24ms p(95)=45.3ms
     http_reqs......................: 105006 249.403502/s
     iteration_duration.............: avg=1.02s   min=1s     med=1.02s   max=1.11s    p(90)=1.04s   p(95)=1.05s
     iterations.....................: 35002  83.134501/s
     vus............................: 2      min=1        max=100
     vus_max........................: 100    min=100      max=100


# Fastapi - uvicorn forms_api.main:app
running (7m01.0s), 000/100 VUs, 34792 complete and 0 interrupted iterations
default ✓ [======================================] 000/100 VUs  7m0s

     data_received..................: 29 MB  70 kB/s
     data_sent......................: 9.0 MB 21 kB/s
     http_req_blocked...............: avg=9.53µs  min=0s     med=2µs     max=16.42ms  p(90)=6µs     p(95)=11µs
     http_req_connecting............: avg=2.75µs  min=0s     med=0s      max=15.31ms  p(90)=0s      p(95)=0s
     http_req_duration..............: avg=32.69ms min=2.06ms med=30.31ms max=222.5ms  p(90)=58.99ms p(95)=71.14ms
       { expected_response:true }...: avg=32.69ms min=2.06ms med=30.31ms max=222.5ms  p(90)=58.99ms p(95)=71.14ms
     http_req_failed................: 0.00%  ✓ 0          ✗ 104376
     http_req_receiving.............: avg=55.84µs min=4µs    med=22µs    max=25.24ms  p(90)=80µs    p(95)=138µs
     http_req_sending...............: avg=23.05µs min=2µs    med=9µs     max=22.16ms  p(90)=25µs    p(95)=42µs
     http_req_tls_handshaking.......: avg=0s      min=0s     med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=32.61ms min=2.02ms med=30.22ms max=222.47ms p(90)=58.89ms p(95)=71.06ms
     http_reqs......................: 104376 247.947356/s
     iteration_duration.............: avg=1.03s   min=1s     med=1.03s   max=1.22s    p(90)=1.06s   p(95)=1.07s
     iterations.....................: 34792  82.649119/s
     vus............................: 100    min=1        max=100
     vus_max........................: 100    min=100      max=100

# Django - gunicorn --workers=4 form_api.wsgi -- conn_max_age=0
running (7m01.3s), 000/100 VUs, 29917 complete and 0 interrupted iterations
default ✓ [======================================] 000/100 VUs  7m0s

     data_received..................: 43 MB  102 kB/s
     data_sent......................: 7.7 MB 18 kB/s
     http_req_blocked...............: avg=4.24ms   min=0s     med=351µs    max=703.55ms p(90)=1.16ms   p(95)=2.14ms
     http_req_connecting............: avg=4.07ms   min=0s     med=278µs    max=703.52ms p(90)=1.01ms   p(95)=1.88ms
     http_req_duration..............: avg=184.23ms min=0s     med=195.72ms max=1.37s    p(90)=268.6ms  p(95)=295.44ms
       { expected_response:true }...: avg=184.39ms min=8.06ms med=195.8ms  max=1.37s    p(90)=268.62ms p(95)=295.45ms
     http_req_failed................: 0.08%  ✓ 80         ✗ 89671
     http_req_receiving.............: avg=147.01µs min=0s     med=58µs     max=19.64ms  p(90)=211µs    p(95)=504µs
     http_req_sending...............: avg=102.05µs min=0s     med=40µs     max=19.61ms  p(90)=145µs    p(95)=347µs
     http_req_tls_handshaking.......: avg=0s       min=0s     med=0s       max=0s       p(90)=0s       p(95)=0s
     http_req_waiting...............: avg=183.98ms min=0s     med=195.49ms max=1.37s    p(90)=268.41ms p(95)=295.24ms
     http_reqs......................: 89751  213.058185/s
     iteration_duration.............: avg=1.2s     min=1.01s  med=1.21s    max=2.74s    p(90)=1.28s    p(95)=1.31s
     iterations.....................: 29917  71.019395/s
     vus............................: 31     min=1        max=100
     vus_max........................: 100    min=100      max=100


# Django - gunicorn --workers=4 form_api.wsgi -- conn_max_age=500
running (7m00.3s), 000/100 VUs, 34032 complete and 0 interrupted iterations
default ✗ [======================================] 000/100 VUs  7m0s

     data_received..................: 49 MB  115 kB/s
     data_sent......................: 8.8 MB 21 kB/s
     http_req_blocked...............: avg=3.47ms   min=0s     med=333µs   max=200.62ms p(90)=1.72ms  p(95)=8.35ms
     http_req_connecting............: avg=3.39ms   min=0s     med=287µs   max=200.58ms p(90)=1.52ms  p(95)=7.34ms
     http_req_duration..............: avg=48.72ms  min=0s     med=44.69ms max=359.63ms p(90)=91.76ms p(95)=103.44ms
       { expected_response:true }...: avg=49.03ms  min=1.49ms med=44.94ms max=359.63ms p(90)=91.87ms p(95)=103.56ms
     http_req_failed................: 0.62%  ✓ 635        ✗ 101461
     http_req_receiving.............: avg=168.1µs  min=0s     med=45µs    max=42.76ms  p(90)=227µs   p(95)=527µs
     http_req_sending...............: avg=100.98µs min=0s     med=32µs    max=37.99ms  p(90)=138µs   p(95)=279µs
     http_req_tls_handshaking.......: avg=0s       min=0s     med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=48.46ms  min=0s     med=44.44ms max=359.15ms p(90)=91.48ms p(95)=103.14ms
     http_reqs......................: 102096 242.920813/s
     iteration_duration.............: avg=1.05s    min=1s     med=1.05s   max=1.36s    p(90)=1.1s    p(95)=1.11s
     iterations.....................: 34032  80.973604/s
     vus............................: 100    min=1        max=100
     vus_max........................: 100    min=100      max=100


# Django - gunicorn --workers=8 --threads=1 form_api.wsgi
running (7m01.1s), 000/100 VUs, 34061 complete and 0 interrupted iterations
default ✓ [======================================] 000/100 VUs  7m0s

     data_received..................: 48 MB  115 kB/s
     data_sent......................: 8.7 MB 21 kB/s
     http_req_blocked...............: avg=6.22ms  min=0s     med=303µs   max=117.69ms p(90)=2.11ms  p(95)=53.72ms
     http_req_connecting............: avg=6.14ms  min=0s     med=263µs   max=117.66ms p(90)=1.85ms  p(95)=53.58ms
     http_req_duration..............: avg=44.38ms min=0s     med=44.6ms  max=356.16ms p(90)=75.98ms p(95)=83.43ms
       { expected_response:true }...: avg=44.9ms  min=1.49ms med=45.07ms max=356.16ms p(90)=76.11ms p(95)=83.56ms
     http_req_failed................: 1.15%  ✓ 1179      ✗ 101004
     http_req_receiving.............: avg=91.1µs  min=0s     med=39µs    max=14.37ms  p(90)=125µs   p(95)=257µs
     http_req_sending...............: avg=79.21µs min=0s     med=29µs    max=14.57ms  p(90)=116µs   p(95)=246µs
     http_req_tls_handshaking.......: avg=0s      min=0s     med=0s      max=0s       p(90)=0s      p(95)=0s
     http_req_waiting...............: avg=44.21ms min=0s     med=44.45ms max=355.84ms p(90)=75.86ms p(95)=83.29ms
     http_reqs......................: 102183 242.65981/s
     iteration_duration.............: avg=1.05s   min=1s     med=1.05s   max=1.36s    p(90)=1.09s   p(95)=1.11s
     iterations.....................: 34061  80.886603/s
     vus............................: 27     min=1       max=100
     vus_max........................: 100    min=100     max=100

# Go chi
running (7m00.5s), 000/100 VUs, 33602 complete and 0 interrupted iterations
default ✓ [======================================] 000/100 VUs  7m0s

     data_received..................: 30 MB  72 kB/s
     data_sent......................: 8.7 MB 21 kB/s
     http_req_blocked...............: avg=8.86µs  min=0s    med=2µs     max=15.02ms  p(90)=5µs      p(95)=15µs
     http_req_connecting............: avg=2.41µs  min=0s    med=0s      max=11.3ms   p(90)=0s       p(95)=0s
     http_req_duration..............: avg=50.9ms  min=463µs med=46.51ms max=288.41ms p(90)=107.24ms p(95)=122.41ms
       { expected_response:true }...: avg=50.89ms min=463µs med=46.5ms  max=288.41ms p(90)=107.25ms p(95)=122.43ms
     http_req_failed................: 0.09%  ✓ 91         ✗ 100715
     http_req_receiving.............: avg=39.56µs min=4µs   med=24µs    max=17.26ms  p(90)=58µs     p(95)=88µs
     http_req_sending...............: avg=14.74µs min=2µs   med=9µs     max=13.49ms  p(90)=18µs     p(95)=26µs
     http_req_tls_handshaking.......: avg=0s      min=0s    med=0s      max=0s       p(90)=0s       p(95)=0s
     http_req_waiting...............: avg=50.84ms min=442µs med=46.45ms max=288.38ms p(90)=107.19ms p(95)=122.35ms
     http_reqs......................: 100806 239.748152/s
     iteration_duration.............: avg=1.07s   min=1s    med=1.07s   max=1.28s    p(90)=1.12s    p(95)=1.13s
     iterations.....................: 33602  79.916051/s
     vus............................: 100    min=1        max=100
     vus_max........................: 100    min=100      max=100
