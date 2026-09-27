## Naive

bench_mult    fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ local_acc  845.2 ms      │ 1.098 s       │ 871 ms        │ 882 ms        │ 100     │ 100
╰─ naive      883.8 ms      │ 1.28 s        │ 925.4 ms      │ 925.5 ms      │ 100     │ 100

## Tiled

bench_mult    fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ local_acc  (ignored)     │               │               │               │         │
├─ naive      (ignored)     │               │               │               │         │
╰─ tiled                    │               │               │               │         │
   ├─ 8       560.1 ms      │ 761.2 ms      │ 564.1 ms      │ 578.9 ms      │ 100     │ 100
   ├─ 16      533.4 ms      │ 573.2 ms      │ 534.7 ms      │ 535.7 ms      │ 100     │ 100
   ├─ 32      530.4 ms      │ 771.1 ms      │ 536.5 ms      │ 555.7 ms      │ 100     │ 100
   ├─ 64      615.1 ms      │ 1.024 s       │ 639.3 ms      │ 648.9 ms      │ 100     │ 100
   ├─ 128     714.4 ms      │ 876 ms        │ 729.1 ms      │ 739.7 ms      │ 100     │ 100
   ├─ 256     805.5 ms      │ 883.3 ms      │ 817.3 ms      │ 819.4 ms      │ 100     │ 100
   ╰─ 512     857.8 ms      │ 912.1 ms      │ 872.2 ms      │ 874.6 ms      │ 100     │
