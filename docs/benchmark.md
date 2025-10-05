# Benchmark

Demonstrate simple benchmark test of hypernbd on raw block device or on filesystem.

All tests below running on a [c6i.4xlarge](https://aws.amazon.com/ec2/instance-types/c6i/).

## Fio

### Backend on S3 Standard, WAL disabled

```
# start nbd server
nbdkit -f --log=stderr --ipaddr=127.0.0.1 --port=7788 ./target/release/libhypernbd.so backend_uri=s3://MYS3GPBUCKET/nbdroot
```

```
# connect nbd deivce
$ nbd-client 127.0.0.1 7788 /dev/nbd0

# run test
$ fio -group_reporting -numjobs=16 -iodepth=1 -direct=1 -ioengine=libaio -rw=randrw -bsrange=4k-512k -size=8T -time_based -runtime=20 -name=Fio -eta-newline=1 -filename=/dev/nbd0
Fio: (g=0): rw=randrw, bs=(R) 4096B-512KiB, (W) 4096B-512KiB, (T) 4096B-512KiB, ioengine=libaio, iodepth=1
...
fio-3.32
Starting 16 processes
Jobs: 16 (f=16): [m(16)][15.0%][r=271MiB/s,w=257MiB/s][r=1074,w=988 IOPS][eta 00m:17s]
Jobs: 16 (f=16): [m(16)][25.0%][r=118MiB/s,w=128MiB/s][r=495,w=507 IOPS][eta 00m:15s]
Jobs: 16 (f=16): [m(16)][35.0%][r=266MiB/s,w=257MiB/s][r=1011,w=1012 IOPS][eta 00m:13s]
Jobs: 16 (f=16): [m(16)][45.0%][r=116MiB/s,w=128MiB/s][r=468,w=497 IOPS][eta 00m:11s]
Jobs: 16 (f=16): [m(16)][55.0%][r=127MiB/s,w=128MiB/s][r=485,w=496 IOPS][eta 00m:09s]
Jobs: 16 (f=16): [m(16)][65.0%][r=128MiB/s,w=128MiB/s][r=515,w=509 IOPS][eta 00m:07s]
Jobs: 16 (f=16): [m(16)][75.0%][r=117MiB/s,w=128MiB/s][r=458,w=499 IOPS][eta 00m:05s]
Jobs: 16 (f=16): [m(16)][85.0%][r=122MiB/s,w=128MiB/s][r=486,w=517 IOPS][eta 00m:03s]
Jobs: 16 (f=16): [m(16)][95.0%][r=131MiB/s,w=128MiB/s][r=491,w=483 IOPS][eta 00m:01s]
Jobs: 16 (f=16): [m(16)][100.0%][r=138MiB/s,w=128MiB/s][r=534,w=519 IOPS][eta 00m:00s]
Fio: (groupid=0, jobs=16): err= 0: pid=227401: Sat Oct  4 06:05:17 2025
  read: IOPS=639, BW=161MiB/s (169MB/s)(3243MiB/20126msec)
    slat (usec): min=5, max=3271, avg=737.66, stdev=603.84
    clat (nsec): min=1205, max=789492k, avg=11103523.82, stdev=83829810.06
     lat (usec): min=80, max=789517, avg=11841.18, stdev=83836.92
    clat percentiles (usec):
     |  1.00th=[   215],  5.00th=[   326], 10.00th=[   396], 20.00th=[   490],
     | 30.00th=[   570], 40.00th=[   644], 50.00th=[   725], 60.00th=[   824],
     | 70.00th=[   930], 80.00th=[  1090], 90.00th=[  1401], 95.00th=[  2024],
     | 99.00th=[683672], 99.50th=[708838], 99.90th=[725615], 99.95th=[784335],
     | 99.99th=[784335]
   bw (  KiB/s): min=40320, max=348392, per=100.00%, avg=227598.29, stdev=4743.46, samples=451
   iops        : min=  156, max= 1270, avg=880.52, stdev=17.26, samples=451
  write: IOPS=651, BW=165MiB/s (173MB/s)(3327MiB/20126msec); 0 zone resets
    slat (usec): min=7, max=3262, avg=859.84, stdev=611.38
    clat (nsec): min=869, max=789137k, avg=11857221.16, stdev=87595467.80
     lat (usec): min=85, max=789937, avg=12717.07, stdev=87600.50
    clat percentiles (usec):
     |  1.00th=[   165],  5.00th=[   241], 10.00th=[   277], 20.00th=[   338],
     | 30.00th=[   396], 40.00th=[   461], 50.00th=[   537], 60.00th=[   627],
     | 70.00th=[   742], 80.00th=[   898], 90.00th=[  1221], 95.00th=[  1893],
     | 99.00th=[683672], 99.50th=[708838], 99.90th=[725615], 99.95th=[784335],
     | 99.99th=[792724]
   bw (  KiB/s): min=38984, max=356400, per=100.00%, avg=233064.75, stdev=4731.77, samples=451
   iops        : min=  154, max= 1308, avg=897.71, stdev=17.23, samples=451
  lat (nsec)   : 1000=0.01%
  lat (usec)   : 2=0.05%, 4=0.03%, 10=0.02%, 50=0.01%, 100=0.07%
  lat (usec)   : 250=3.75%, 500=29.58%, 750=28.25%, 1000=17.83%
  lat (msec)   : 2=15.50%, 4=3.33%, 10=0.01%, 50=0.01%, 100=0.01%
  lat (msec)   : 250=0.02%, 750=1.48%, 1000=0.06%
  cpu          : usr=0.41%, sys=0.93%, ctx=152036, majf=0, minf=225
  IO depths    : 1=100.0%, 2=0.0%, 4=0.0%, 8=0.0%, 16=0.0%, 32=0.0%, >=64=0.0%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     issued rwts: total=12861,13113,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=1

Run status group 0 (all jobs):
   READ: bw=161MiB/s (169MB/s), 161MiB/s-161MiB/s (169MB/s-169MB/s), io=3243MiB (3401MB), run=20126-20126msec
  WRITE: bw=165MiB/s (173MB/s), 165MiB/s-165MiB/s (173MB/s-173MB/s), io=3327MiB (3488MB), run=20126-20126msec

Disk stats (read/write):
  nbd0: ios=12784/12954, merge=0/0, ticks=151797/166343, in_queue=318139, util=98.71%
```

### Backend on S3 Express One Zone, WAL disabled

```
# start nbd server
nbdkit -f --log=stderr --ipaddr=127.0.0.1 --port=7788 ./target/release/libhypernbd.so backend_uri=s3://MYS3DIRBUCKET/nbdroot
```

```
# connect nbd deivce
$ nbd-client 127.0.0.1 7788 /dev/nbd0

# run test
$ fio -group_reporting -numjobs=16 -iodepth=1 -direct=1 -ioengine=libaio -rw=randrw -bsrange=4k-512k -size=8T -time_based -runtime=20 -name=Fio -eta-newline=1 -filename=/dev/nbd0
Fio: (g=0): rw=randrw, bs=(R) 4096B-512KiB, (W) 4096B-512KiB, (T) 4096B-512KiB, ioengine=libaio, iodepth=1
...
fio-3.32
Starting 16 processes
Jobs: 16 (f=16): [m(16)][19.0%][r=250MiB/s,w=257MiB/s][r=1017,w=1020 IOPS][eta 00m:17s]
Jobs: 16 (f=16): [m(16)][28.6%][r=254MiB/s,w=256MiB/s][r=993,w=1014 IOPS][eta 00m:15s]
Jobs: 16 (f=16): [m(16)][38.1%][r=257MiB/s,w=256MiB/s][r=986,w=1020 IOPS][eta 00m:13s]
Jobs: 16 (f=16): [m(16)][47.6%][r=248MiB/s,w=257MiB/s][r=971,w=1034 IOPS][eta 00m:11s]
Jobs: 16 (f=16): [m(16)][57.1%][r=282MiB/s,w=285MiB/s][r=1093,w=1071 IOPS][eta 00m:09s]
Jobs: 16 (f=16): [m(16)][66.7%][r=253MiB/s,w=257MiB/s][r=997,w=1013 IOPS][eta 00m:07s]
Jobs: 16 (f=16): [m(16)][76.2%][r=248MiB/s,w=256MiB/s][r=981,w=1013 IOPS][eta 00m:05s]
Jobs: 16 (f=16): [m(16)][85.7%][r=261MiB/s,w=256MiB/s][r=1026,w=998 IOPS][eta 00m:03s]
Jobs: 16 (f=16): [m(16)][95.2%][r=301MiB/s,w=291MiB/s][r=1208,w=1197 IOPS][eta 00m:01s]
Jobs: 16 (f=16): [m(16)][100.0%][r=300MiB/s,w=297MiB/s][r=1181,w=1190 IOPS][eta 00m:00s]
Fio: (groupid=0, jobs=16): err= 0: pid=228389: Sat Oct  4 06:13:18 2025
  read: IOPS=1080, BW=272MiB/s (285MB/s)(5480MiB/20171msec)
    slat (usec): min=5, max=3279, avg=721.56, stdev=608.17
    clat (nsec): min=1086, max=398174k, avg=6289226.42, stdev=44255534.22
     lat (usec): min=113, max=398674, avg=7010.79, stdev=44258.58
    clat percentiles (usec):
     |  1.00th=[   225],  5.00th=[   334], 10.00th=[   404], 20.00th=[   506],
     | 30.00th=[   586], 40.00th=[   676], 50.00th=[   758], 60.00th=[   873],
     | 70.00th=[  1004], 80.00th=[  1188], 90.00th=[  1532], 95.00th=[  1909],
     | 99.00th=[358613], 99.50th=[375391], 99.90th=[392168], 99.95th=[392168],
     | 99.99th=[396362]
   bw (  KiB/s): min=169761, max=438282, per=100.00%, avg=280534.17, stdev=3813.96, samples=640
   iops        : min=  724, max= 1686, avg=1089.55, stdev=13.54, samples=640
  write: IOPS=1084, BW=273MiB/s (287MB/s)(5513MiB/20171msec); 0 zone resets
    slat (usec): min=3, max=3774, avg=838.32, stdev=615.05
    clat (nsec): min=913, max=397854k, avg=6867598.71, stdev=47186085.12
     lat (usec): min=153, max=398499, avg=7705.92, stdev=47185.87
    clat percentiles (usec):
     |  1.00th=[   167],  5.00th=[   241], 10.00th=[   281], 20.00th=[   347],
     | 30.00th=[   412], 40.00th=[   490], 50.00th=[   570], 60.00th=[   685],
     | 70.00th=[   824], 80.00th=[  1020], 90.00th=[  1352], 95.00th=[  1778],
     | 99.00th=[367002], 99.50th=[375391], 99.90th=[392168], 99.95th=[392168],
     | 99.99th=[396362]
   bw (  KiB/s): min=178860, max=461144, per=100.00%, avg=282197.00, stdev=3928.75, samples=640
   iops        : min=  728, max= 1724, avg=1093.15, stdev=14.00, samples=640
  lat (nsec)   : 1000=0.01%
  lat (usec)   : 2=0.06%, 4=0.02%, 50=0.01%, 100=0.04%, 250=3.58%
  lat (usec)   : 500=26.81%, 750=26.35%, 1000=17.67%
  lat (msec)   : 2=21.17%, 4=2.48%, 10=0.20%, 20=0.02%, 50=0.01%
  lat (msec)   : 100=0.01%, 500=1.58%
  cpu          : usr=0.52%, sys=1.51%, ctx=261029, majf=0, minf=247
  IO depths    : 1=100.0%, 2=0.0%, 4=0.0%, 8=0.0%, 16=0.0%, 32=0.0%, >=64=0.0%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     issued rwts: total=21800,21870,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=1

Run status group 0 (all jobs):
   READ: bw=272MiB/s (285MB/s), 272MiB/s-272MiB/s (285MB/s-285MB/s), io=5480MiB (5747MB), run=20171-20171msec
  WRITE: bw=273MiB/s (287MB/s), 273MiB/s-273MiB/s (287MB/s-287MB/s), io=5513MiB (5781MB), run=20171-20171msec

Disk stats (read/write):
  nbd0: ios=21865/21863, merge=0/0, ticks=148943/165294, in_queue=314236, util=98.54%
```

### Backend on S3 General Purpose Bucket, WAL on S3 Express One Zone

```
# start nbd server
nbdkit -f --log=stderr --ipaddr=127.0.0.1 --port=7788 ./target/release/libhypernbd.so backend_uri=s3://MYS3GPBUCKET/nbdroot backend_wal_uri=s3://MYS3DIRBUCKET/nbdwalroot
```

```
# connect nbd deivce
$ nbd-client 127.0.0.1 7788 /dev/nbd0

# run test
$ fio -group_reporting -numjobs=16 -iodepth=1 -direct=1 -ioengine=libaio -rw=randrw -bsrange=4k-512k -size=8T -time_based -runtime=20 -name=Fio -eta-newline=1 -filename=/dev/nbd0
Fio: (g=0): rw=randrw, bs=(R) 4096B-512KiB, (W) 4096B-512KiB, (T) 4096B-512KiB, ioengine=libaio, iodepth=1
...
fio-3.32
Starting 16 processes
Jobs: 16 (f=16): [m(16)][15.0%][r=99.7MiB/s,w=95.5MiB/s][r=396,w=381 IOPS][eta 00m:17s]
Jobs: 16 (f=16): [m(16)][25.0%][r=130MiB/s,w=124MiB/s][r=517,w=469 IOPS][eta 00m:15s]
Jobs: 16 (f=16): [m(16)][35.0%][r=79.3MiB/s,w=85.3MiB/s][r=337,w=329 IOPS][eta 00m:13s]
Jobs: 16 (f=16): [m(16)][45.0%][r=108MiB/s,w=126MiB/s][r=451,w=508 IOPS][eta 00m:11s]
Jobs: 16 (f=16): [m(16)][55.0%][r=82.3MiB/s,w=90.1MiB/s][r=323,w=353 IOPS][eta 00m:09s]
Jobs: 16 (f=16): [m(16)][65.0%][r=115MiB/s,w=122MiB/s][r=463,w=471 IOPS][eta 00m:07s]
Jobs: 16 (f=16): [m(16)][75.0%][r=77.3MiB/s,w=89.9MiB/s][r=316,w=357 IOPS][eta 00m:05s]
Jobs: 16 (f=16): [m(16)][85.0%][r=111MiB/s,w=113MiB/s][r=446,w=419 IOPS][eta 00m:03s]
Jobs: 16 (f=16): [m(16)][95.0%][r=76.8MiB/s,w=74.9MiB/s][r=293,w=317 IOPS][eta 00m:01s]
Jobs: 16 (f=16): [m(16)][100.0%][r=92.6MiB/s,w=100MiB/s][r=359,w=400 IOPS][eta 00m:00s]
Fio: (groupid=0, jobs=16): err= 0: pid=226497: Sat Oct  4 05:54:19 2025
  read: IOPS=385, BW=97.1MiB/s (102MB/s)(1948MiB/20054msec)
    slat (usec): min=5, max=1381, avg=35.16, stdev=84.71
    clat (usec): min=9, max=222013, avg=2142.31, stdev=13370.19
     lat (usec): min=55, max=222041, avg=2177.47, stdev=13369.95
    clat percentiles (usec):
     |  1.00th=[    61],  5.00th=[    76], 10.00th=[    90], 20.00th=[   111],
     | 30.00th=[   130], 40.00th=[   149], 50.00th=[   172], 60.00th=[   192],
     | 70.00th=[   215], 80.00th=[   241], 90.00th=[   359], 95.00th=[  1369],
     | 99.00th=[100140], 99.50th=[112722], 99.90th=[123208], 99.95th=[128451],
     | 99.99th=[221250]
   bw (  KiB/s): min=17072, max=250976, per=100.00%, avg=99723.91, stdev=3448.02, samples=630
   iops        : min=   66, max=  946, avg=386.35, stdev=12.75, samples=630
  write: IOPS=393, BW=99.9MiB/s (105MB/s)(2003MiB/20054msec); 0 zone resets
    slat (usec): min=6, max=1714, avg=125.41, stdev=119.48
    clat (msec): min=4, max=363, avg=38.22, stdev=40.77
     lat (msec): min=4, max=363, avg=38.34, stdev=40.78
    clat percentiles (msec):
     |  1.00th=[    6],  5.00th=[    7], 10.00th=[    7], 20.00th=[    8],
     | 30.00th=[    9], 40.00th=[   23], 50.00th=[   33], 60.00th=[   41],
     | 70.00th=[   47], 80.00th=[   56], 90.00th=[   69], 95.00th=[   88],
     | 99.00th=[  228], 99.50th=[  236], 99.90th=[  334], 99.95th=[  351],
     | 99.99th=[  363]
   bw (  KiB/s): min=36288, max=200000, per=100.00%, avg=102324.52, stdev=2483.60, samples=631
   iops        : min=  160, max=  744, avg=393.75, stdev= 8.91, samples=631
  lat (usec)   : 10=0.01%, 50=0.01%, 100=7.28%, 250=33.34%, 500=4.94%
  lat (usec)   : 750=0.61%, 1000=0.35%
  lat (msec)   : 2=1.01%, 4=0.36%, 10=16.89%, 20=2.92%, 50=18.22%
  lat (msec)   : 100=11.39%, 250=2.54%, 500=0.13%
  cpu          : usr=0.37%, sys=0.36%, ctx=21852, majf=0, minf=181
  IO depths    : 1=100.0%, 2=0.0%, 4=0.0%, 8=0.0%, 16=0.0%, 32=0.0%, >=64=0.0%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     issued rwts: total=7728,7889,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=1

Run status group 0 (all jobs):
   READ: bw=97.1MiB/s (102MB/s), 97.1MiB/s-97.1MiB/s (102MB/s-102MB/s), io=1948MiB (2043MB), run=20054-20054msec
  WRITE: bw=99.9MiB/s (105MB/s), 99.9MiB/s-99.9MiB/s (105MB/s-105MB/s), io=2003MiB (2100MB), run=20054-20054msec

Disk stats (read/write):
  nbd0: ios=7812/7866, merge=0/0, ticks=16699/301325, in_queue=318023, util=98.77%
```

### Backend on S3 Express One Zone, WAL on S3 Express One Zone

```
# start nbd server
nbdkit -f --log=stderr --ipaddr=127.0.0.1 --port=7788 ./target/release/libhypernbd.so backend_uri=s3://MYS3DIRBUCKET/nbdroot backend_wal_uri=s3://MYS3DIRBUCKET/nbdwalroot
```

```
# connect nbd deivce
$ nbd-client 127.0.0.1 7788 /dev/nbd0

# run test
$ fio -group_reporting -numjobs=16 -iodepth=1 -direct=1 -ioengine=libaio -rw=randrw -bsrange=4k-512k -size=8T -time_based -runtime=20 -name=Fio -eta-newline=1 -filename=/dev/nbd0
Fio: (g=0): rw=randrw, bs=(R) 4096B-512KiB, (W) 4096B-512KiB, (T) 4096B-512KiB, ioengine=libaio, iodepth=1
...
fio-3.32
Starting 16 processes
Jobs: 16 (f=16): [m(16)][15.0%][r=116MiB/s,w=113MiB/s][r=456,w=447 IOPS][eta 00m:17s]
Jobs: 16 (f=16): [m(16)][25.0%][r=81.5MiB/s,w=89.6MiB/s][r=338,w=339 IOPS][eta 00m:15s]
Jobs: 16 (f=16): [m(16)][35.0%][r=101MiB/s,w=107MiB/s][r=419,w=421 IOPS][eta 00m:13s]
Jobs: 16 (f=16): [m(16)][45.0%][r=98.4MiB/s,w=104MiB/s][r=392,w=410 IOPS][eta 00m:11s]
Jobs: 16 (f=16): [m(16)][55.0%][r=106MiB/s,w=94.2MiB/s][r=403,w=381 IOPS][eta 00m:09s]
Jobs: 16 (f=16): [m(16)][65.0%][r=66.4MiB/s,w=72.9MiB/s][r=265,w=290 IOPS][eta 00m:07s]
Jobs: 16 (f=16): [m(16)][75.0%][r=98.8MiB/s,w=117MiB/s][r=404,w=459 IOPS][eta 00m:05s]
Jobs: 16 (f=16): [m(16)][85.0%][r=73.9MiB/s,w=88.6MiB/s][r=302,w=334 IOPS][eta 00m:03s]
Jobs: 16 (f=16): [m(16)][95.0%][r=90.0MiB/s,w=100MiB/s][r=359,w=403 IOPS][eta 00m:01s]
Jobs: 16 (f=16): [m(16)][100.0%][r=129MiB/s,w=128MiB/s][r=496,w=510 IOPS][eta 00m:00s]
Fio: (groupid=0, jobs=16): err= 0: pid=239158: Sat Oct  4 07:08:49 2025
  read: IOPS=386, BW=97.3MiB/s (102MB/s)(1947MiB/20018msec)
    slat (usec): min=5, max=1373, avg=30.67, stdev=65.05
    clat (usec): min=46, max=131838, avg=1929.34, stdev=13337.42
     lat (usec): min=56, max=131866, avg=1960.01, stdev=13337.93
    clat percentiles (usec):
     |  1.00th=[    61],  5.00th=[    75], 10.00th=[    88], 20.00th=[   111],
     | 30.00th=[   129], 40.00th=[   147], 50.00th=[   169], 60.00th=[   192],
     | 70.00th=[   212], 80.00th=[   241], 90.00th=[   314], 95.00th=[   734],
     | 99.00th=[104334], 99.50th=[114820], 99.90th=[123208], 99.95th=[126354],
     | 99.99th=[131597]
   bw (  KiB/s): min=18112, max=243248, per=100.00%, avg=100353.94, stdev=3413.12, samples=623
   iops        : min=   88, max=  946, avg=389.73, stdev=12.73, samples=623
  write: IOPS=393, BW=99.9MiB/s (105MB/s)(2000MiB/20018msec); 0 zone resets
    slat (usec): min=9, max=1414, avg=121.73, stdev=103.33
    clat (msec): min=4, max=335, avg=38.42, stdev=45.69
     lat (msec): min=4, max=335, avg=38.54, stdev=45.71
    clat percentiles (msec):
     |  1.00th=[    6],  5.00th=[    7], 10.00th=[    7], 20.00th=[    8],
     | 30.00th=[    9], 40.00th=[   22], 50.00th=[   33], 60.00th=[   41],
     | 70.00th=[   47], 80.00th=[   55], 90.00th=[   67], 95.00th=[   80],
     | 99.00th=[  305], 99.50th=[  317], 99.90th=[  330], 99.95th=[  330],
     | 99.99th=[  334]
   bw (  KiB/s): min=37496, max=191776, per=100.00%, avg=103039.85, stdev=2475.51, samples=624
   iops        : min=  174, max=  736, avg=396.21, stdev= 8.99, samples=624
  lat (usec)   : 50=0.03%, 100=7.51%, 250=33.48%, 500=5.46%, 750=0.60%
  lat (usec)   : 1000=0.24%
  lat (msec)   : 2=0.68%, 4=0.48%, 10=17.62%, 20=2.37%, 50=18.02%
  lat (msec)   : 100=11.32%, 250=1.42%, 500=0.78%
  cpu          : usr=0.44%, sys=0.34%, ctx=20797, majf=0, minf=196
  IO depths    : 1=100.0%, 2=0.0%, 4=0.0%, 8=0.0%, 16=0.0%, 32=0.0%, >=64=0.0%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     issued rwts: total=7733,7879,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=1

Run status group 0 (all jobs):
   READ: bw=97.3MiB/s (102MB/s), 97.3MiB/s-97.3MiB/s (102MB/s-102MB/s), io=1947MiB (2042MB), run=20018-20018msec
  WRITE: bw=99.9MiB/s (105MB/s), 99.9MiB/s-99.9MiB/s (105MB/s-105MB/s), io=2000MiB (2097MB), run=20018-20018msec

Disk stats (read/write):
  nbd0: ios=7764/7861, merge=0/0, ticks=15019/299420, in_queue=314438, util=98.47%
```

## XFS

### Backend on S3 Standard, WAL disabled

```
# start nbd server
nbdkit -f --log=stderr --ipaddr=127.0.0.1 --port=7788 ./target/release/libhypernbd.so backend_uri=s3://MYS3GPBUCKET/nbdroot
```

```
# connect nbd deivce
$ nbd-client 127.0.0.1 7788 /dev/nbd0

# create filesystem 
$ mkfs.xfs -K /dev/nbd0

# mount filesystem
$ mount /dev/nbd0 /data

# run test
$ fio -group_reporting -numjobs=16 -iodepth=1 -direct=1 -ioengine=libaio -rw=randrw -bsrange=4k-512k -size=512M -time_based -runtime=20 -name=Fio -eta-newline=1 -directory=/data
Fio: (g=0): rw=randrw, bs=(R) 4096B-512KiB, (W) 4096B-512KiB, (T) 4096B-512KiB, ioengine=libaio, iodepth=1
...
fio-3.32
Starting 16 processes
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Jobs: 16 (f=16): [m(16)][20.0%][r=26.9MiB/s,w=21.7MiB/s][r=96,w=92 IOPS][eta 00m:16s]
Jobs: 16 (f=16): [m(16)][30.0%][r=45.6MiB/s,w=50.1MiB/s][r=194,w=207 IOPS][eta 00m:14s]
Jobs: 16 (f=16): [m(16)][40.0%][r=31.0MiB/s,w=22.0MiB/s][r=117,w=93 IOPS][eta 00m:12s]
Jobs: 16 (f=16): [m(16)][50.0%][r=8276KiB/s,w=11.2MiB/s][r=39,w=46 IOPS][eta 00m:10s]
Jobs: 16 (f=16): [m(16)][60.0%][r=12.3MiB/s,w=9228KiB/s][r=49,w=41 IOPS][eta 00m:08s]
Jobs: 16 (f=16): [m(16)][70.0%][r=15.3MiB/s,w=11.6MiB/s][r=65,w=50 IOPS][eta 00m:06s]
Jobs: 16 (f=16): [m(16)][80.0%][eta 00m:04s]
Jobs: 16 (f=16): [m(16)][90.0%][r=47.6MiB/s,w=39.1MiB/s][r=200,w=163 IOPS][eta 00m:02s]
Jobs: 16 (f=16): [m(16)][100.0%][r=31.2MiB/s,w=25.1MiB/s][r=136,w=119 IOPS][eta 00m:00s]
Jobs: 12 (f=11): [m(5),_(2),m(4),_(2),m(1),f(1),m(1)][22.0%][r=1648KiB/s][r=5 IOPS][eta 01m:18s]
Jobs: 3 (f=3): [_(1),m(1),E(1),_(7),m(1),_(4),m(1)][12.0%][r=2452KiB/s][r=8 IOPS][eta 02m:49s]
Fio: (groupid=0, jobs=16): err= 0: pid=233287: Sat Oct  4 06:45:13 2025
  read: IOPS=98, BW=23.7MiB/s (24.8MB/s)(541MiB/22839msec)
    slat (usec): min=10, max=1666, avg=37.00, stdev=49.16
    clat (msec): min=23, max=4089, avg=138.31, stdev=315.61
     lat (msec): min=23, max=4089, avg=138.35, stdev=315.61
    clat percentiles (msec):
     |  1.00th=[   32],  5.00th=[   35], 10.00th=[   42], 20.00th=[   51],
     | 30.00th=[   58], 40.00th=[   65], 50.00th=[   73], 60.00th=[   82],
     | 70.00th=[   97], 80.00th=[  125], 90.00th=[  239], 95.00th=[  317],
     | 99.00th=[ 2333], 99.50th=[ 2500], 99.90th=[ 3842], 99.95th=[ 4010],
     | 99.99th=[ 4077]
   bw (  KiB/s): min= 1920, max=86440, per=100.00%, avg=32561.28, stdev=1454.56, samples=540
   iops        : min=   32, max=  324, avg=132.21, stdev= 5.51, samples=540
  write: IOPS=95, BW=22.7MiB/s (23.8MB/s)(519MiB/22839msec); 0 zone resets
    slat (usec): min=11, max=1885, avg=126.20, stdev=107.63
    clat (usec): min=52, max=731814, avg=14721.93, stdev=89817.06
     lat (usec): min=64, max=731958, avg=14848.12, stdev=89816.09
    clat percentiles (usec):
     |  1.00th=[    71],  5.00th=[   103], 10.00th=[   125], 20.00th=[   161],
     | 30.00th=[   190], 40.00th=[   219], 50.00th=[   245], 60.00th=[   277],
     | 70.00th=[   310], 80.00th=[   367], 90.00th=[   619], 95.00th=[  1270],
     | 99.00th=[633340], 99.50th=[692061], 99.90th=[725615], 99.95th=[725615],
     | 99.99th=[734004]
   bw (  KiB/s): min= 2112, max=124800, per=100.00%, avg=38332.09, stdev=1935.11, samples=446
   iops        : min=   32, max=  490, avg=156.85, stdev= 7.47, samples=446
  lat (usec)   : 100=2.15%, 250=23.16%, 500=17.75%, 750=2.01%, 1000=0.75%
  lat (msec)   : 2=1.38%, 4=0.41%, 10=0.05%, 20=0.02%, 50=9.91%
  lat (msec)   : 100=26.64%, 250=10.00%, 500=3.60%, 750=1.38%, 1000=0.09%
  lat (msec)   : 2000=0.05%, >=2000=0.68%
  cpu          : usr=0.03%, sys=0.10%, ctx=5319, majf=0, minf=202
  IO depths    : 1=100.0%, 2=0.0%, 4=0.0%, 8=0.0%, 16=0.0%, 32=0.0%, >=64=0.0%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     issued rwts: total=2246,2176,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=1

Run status group 0 (all jobs):
   READ: bw=23.7MiB/s (24.8MB/s), 23.7MiB/s-23.7MiB/s (24.8MB/s-24.8MB/s), io=541MiB (567MB), run=22839-22839msec
  WRITE: bw=22.7MiB/s (23.8MB/s), 22.7MiB/s-22.7MiB/s (23.8MB/s-23.8MB/s), io=519MiB (544MB), run=22839-22839msec

Disk stats (read/write):
  nbd0: ios=2245/2187, merge=0/5, ticks=306566/42401, in_queue=350545, util=99.58%
```

### Backend on S3 Express One Zone, WAL disabled

```
# start nbd server
nbdkit -f --log=stderr --ipaddr=127.0.0.1 --port=7788 ./target/release/libhypernbd.so backend_uri=s3://MYS3DIRBUCKET/nbdroot
```

```
# connect nbd deivce
$ nbd-client 127.0.0.1 7788 /dev/nbd0

# create filesystem 
$ mkfs.xfs -K /dev/nbd0

# mount filesystem
$ mount /dev/nbd0 /data

# run test
$ fio -group_reporting -numjobs=16 -iodepth=1 -direct=1 -ioengine=libaio -rw=randrw -bsrange=4k-512k -size=512M -time_based -runtime=20 -name=Fio -eta-newline=1 -directory=/data
Fio: (g=0): rw=randrw, bs=(R) 4096B-512KiB, (W) 4096B-512KiB, (T) 4096B-512KiB, ioengine=libaio, iodepth=1
...
fio-3.32
Starting 16 processes
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Jobs: 16 (f=16): [m(16)][15.0%][r=114MiB/s,w=118MiB/s][r=486,w=500 IOPS][eta 00m:17s]
Jobs: 16 (f=16): [m(16)][20.0%][r=101MiB/s,w=97.7MiB/s][r=428,w=405 IOPS][eta 00m:16s]
Jobs: 16 (f=16): [m(16)][30.0%][r=93.9MiB/s,w=87.1MiB/s][r=402,w=380 IOPS][eta 00m:14s]
Jobs: 16 (f=16): [m(16)][35.0%][r=124MiB/s,w=121MiB/s][r=534,w=540 IOPS][eta 00m:13s]
Jobs: 16 (f=16): [m(16)][45.0%][r=116MiB/s,w=115MiB/s][r=520,w=506 IOPS][eta 00m:11s]
Jobs: 16 (f=16): [m(16)][55.0%][r=102MiB/s,w=107MiB/s][r=477,w=529 IOPS][eta 00m:09s]
Jobs: 16 (f=16): [m(16)][65.0%][r=103MiB/s,w=117MiB/s][r=512,w=564 IOPS][eta 00m:07s]
Jobs: 16 (f=16): [m(16)][75.0%][r=118MiB/s,w=114MiB/s][r=597,w=580 IOPS][eta 00m:05s]
Jobs: 16 (f=16): [m(16)][85.0%][r=111MiB/s,w=116MiB/s][r=585,w=635 IOPS][eta 00m:03s]
Jobs: 16 (f=16): [m(16)][95.0%][r=101MiB/s,w=110MiB/s][r=556,w=603 IOPS][eta 00m:01s]
Jobs: 16 (f=16): [m(16)][100.0%][r=99.2MiB/s,w=109MiB/s][r=573,w=630 IOPS][eta 00m:00s]
Fio: (groupid=0, jobs=16): err= 0: pid=232194: Sat Oct  4 06:38:23 2025
  read: IOPS=499, BW=105MiB/s (110MB/s)(2096MiB/20008msec)
    slat (usec): min=8, max=5306, avg=84.23, stdev=207.83
    clat (msec): min=4, max=840, avg=25.27, stdev=42.48
     lat (msec): min=4, max=841, avg=25.36, stdev=42.50
    clat percentiles (msec):
     |  1.00th=[    8],  5.00th=[   11], 10.00th=[   12], 20.00th=[   15],
     | 30.00th=[   17], 40.00th=[   19], 50.00th=[   20], 60.00th=[   22],
     | 70.00th=[   24], 80.00th=[   27], 90.00th=[   31], 95.00th=[   34],
     | 99.00th=[  368], 99.50th=[  376], 99.90th=[  384], 99.95th=[  388],
     | 99.99th=[  844]
   bw (  KiB/s): min= 5696, max=189512, per=99.39%, avg=106616.42, stdev=3163.10, samples=621
   iops        : min=   32, max=  878, avg=492.75, stdev=14.33, samples=621
  write: IOPS=514, BW=108MiB/s (113MB/s)(2153MiB/20008msec); 0 zone resets
    slat (usec): min=8, max=6928, avg=203.89, stdev=301.43
    clat (usec): min=2, max=367042, avg=6271.28, stdev=42014.78
     lat (usec): min=75, max=367287, avg=6475.17, stdev=42021.76
    clat percentiles (usec):
     |  1.00th=[   104],  5.00th=[   159], 10.00th=[   208], 20.00th=[   297],
     | 30.00th=[   375], 40.00th=[   469], 50.00th=[   594], 60.00th=[   766],
     | 70.00th=[  1012], 80.00th=[  1483], 90.00th=[  2540], 95.00th=[  4113],
     | 99.00th=[354419], 99.50th=[358613], 99.90th=[362808], 99.95th=[362808],
     | 99.99th=[367002]
   bw (  KiB/s): min=14136, max=258896, per=99.51%, avg=109666.99, stdev=3911.30, samples=620
   iops        : min=   60, max= 1166, avg=507.52, stdev=17.53, samples=620
  lat (usec)   : 4=0.02%, 50=0.01%, 100=0.39%, 250=7.04%, 500=14.19%
  lat (usec)   : 750=8.44%, 1000=5.22%
  lat (msec)   : 2=8.48%, 4=4.32%, 10=4.19%, 20=22.88%, 50=23.04%
  lat (msec)   : 100=0.33%, 250=0.11%, 500=1.34%, 1000=0.01%
  cpu          : usr=0.07%, sys=0.67%, ctx=40147, majf=0, minf=195
  IO depths    : 1=100.0%, 2=0.0%, 4=0.0%, 8=0.0%, 16=0.0%, 32=0.0%, >=64=0.0%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     issued rwts: total=9990,10294,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=1

Run status group 0 (all jobs):
   READ: bw=105MiB/s (110MB/s), 105MiB/s-105MiB/s (110MB/s-110MB/s), io=2096MiB (2198MB), run=20008-20008msec
  WRITE: bw=108MiB/s (113MB/s), 108MiB/s-108MiB/s (113MB/s-113MB/s), io=2153MiB (2258MB), run=20008-20008msec

Disk stats (read/write):
  nbd0: ios=9881/10184, merge=0/6, ticks=250311/67809, in_queue=318738, util=99.55%
```

### Backend on S3 General Purpose Bucket, WAL on S3 Express One Zone

```
# start nbd server
nbdkit -f --log=stderr --ipaddr=127.0.0.1 --port=7788 ./target/release/libhypernbd.so backend_uri=s3://MYS3GPBUCKET/nbdroot backend_wal_uri=s3://MYS3DIRBUCKET/nbdwalroot
```

```
# connect nbd deivce
$ nbd-client 127.0.0.1 7788 /dev/nbd0

# create filesystem 
$ mkfs.xfs -K /dev/nbd0

# mount filesystem
$ mount /dev/nbd0 /data

# run test
$ fio -group_reporting -numjobs=16 -iodepth=1 -direct=1 -ioengine=libaio -rw=randrw -bsrange=4k-512k -size=512M -time_based -runtime=20 -name=Fio -eta-newline=1 -directory=/data
Fio: (g=0): rw=randrw, bs=(R) 4096B-512KiB, (W) 4096B-512KiB, (T) 4096B-512KiB, ioengine=libaio, iodepth=1
...
fio-3.32
Starting 16 processes
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Jobs: 16 (f=16): [m(16)][15.0%][r=38.6MiB/s,w=31.7MiB/s][r=152,w=131 IOPS][eta 00m:17s]
Jobs: 16 (f=16): [m(16)][25.0%][r=54.9MiB/s,w=60.7MiB/s][r=242,w=256 IOPS][eta 00m:15s]
Jobs: 16 (f=16): [m(16)][35.0%][r=61.3MiB/s,w=58.4MiB/s][r=258,w=244 IOPS][eta 00m:13s]
Jobs: 16 (f=16): [m(16)][45.0%][r=53.5MiB/s,w=54.5MiB/s][r=225,w=218 IOPS][eta 00m:11s]
Jobs: 16 (f=16): [m(16)][55.0%][r=37.6MiB/s,w=37.8MiB/s][r=173,w=153 IOPS][eta 00m:09s]
Jobs: 16 (f=16): [m(16)][65.0%][r=30.7MiB/s,w=32.1MiB/s][r=137,w=143 IOPS][eta 00m:07s]
Jobs: 16 (f=16): [m(16)][75.0%][r=23.8MiB/s,w=21.5MiB/s][r=104,w=100 IOPS][eta 00m:05s]
Jobs: 16 (f=16): [m(16)][85.0%][eta 00m:03s]
Jobs: 16 (f=16): [m(16)][95.0%][r=84KiB/s,w=1637KiB/s][r=2,w=7 IOPS][eta 00m:01s]
Jobs: 16 (f=16): [m(16)][100.0%][eta 00m:00s]
Fio: (groupid=0, jobs=16): err= 0: pid=236935: Sat Oct  4 07:02:18 2025
  read: IOPS=136, BW=32.6MiB/s (34.1MB/s)(666MiB/20461msec)
    slat (usec): min=13, max=521, avg=39.87, stdev=29.90
    clat (msec): min=21, max=6243, avg=101.16, stdev=363.49
     lat (msec): min=21, max=6243, avg=101.20, stdev=363.49
    clat percentiles (msec):
     |  1.00th=[   27],  5.00th=[   31], 10.00th=[   33], 20.00th=[   35],
     | 30.00th=[   39], 40.00th=[   45], 50.00th=[   50], 60.00th=[   55],
     | 70.00th=[   62], 80.00th=[   77], 90.00th=[  117], 95.00th=[  163],
     | 99.00th=[ 2089], 99.50th=[ 3104], 99.90th=[ 6141], 99.95th=[ 6208],
     | 99.99th=[ 6275]
   bw (  KiB/s): min= 4760, max=87368, per=100.00%, avg=47131.69, stdev=1364.39, samples=460
   iops        : min=   36, max=  330, avg=192.97, stdev= 5.03, samples=460
  write: IOPS=133, BW=31.6MiB/s (33.1MB/s)(647MiB/20461msec); 0 zone resets
    slat (usec): min=20, max=1295, avg=141.07, stdev=89.68
    clat (msec): min=5, max=476, avg=16.23, stdev=32.63
     lat (msec): min=5, max=476, avg=16.37, stdev=32.63
    clat percentiles (msec):
     |  1.00th=[    6],  5.00th=[    7], 10.00th=[    7], 20.00th=[    8],
     | 30.00th=[    9], 40.00th=[    9], 50.00th=[   10], 60.00th=[   11],
     | 70.00th=[   12], 80.00th=[   14], 90.00th=[   19], 95.00th=[   30],
     | 99.00th=[  163], 99.50th=[  199], 99.90th=[  422], 99.95th=[  468],
     | 99.99th=[  477]
   bw (  KiB/s): min= 4824, max=120744, per=100.00%, avg=47625.22, stdev=1856.96, samples=445
   iops        : min=   38, max=  454, avg=196.24, stdev= 6.86, samples=445
  lat (msec)   : 10=27.71%, 20=17.60%, 50=28.51%, 100=17.85%, 250=6.85%
  lat (msec)   : 500=0.60%, 750=0.11%, 1000=0.27%, >=2000=0.51%
  cpu          : usr=0.02%, sys=0.16%, ctx=7161, majf=0, minf=195
  IO depths    : 1=100.0%, 2=0.0%, 4=0.0%, 8=0.0%, 16=0.0%, 32=0.0%, >=64=0.0%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     issued rwts: total=2788,2730,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=1

Run status group 0 (all jobs):
   READ: bw=32.6MiB/s (34.1MB/s), 32.6MiB/s-32.6MiB/s (34.1MB/s-34.1MB/s), io=666MiB (699MB), run=20461-20461msec
  WRITE: bw=31.6MiB/s (33.1MB/s), 31.6MiB/s-31.6MiB/s (33.1MB/s-33.1MB/s), io=647MiB (678MB), run=20461-20461msec

Disk stats (read/write):
  nbd0: ios=2772/2740, merge=0/5, ticks=222964/49691, in_queue=273390, util=99.53%
```

### Backend on S3 Express One Zone, WAL on S3 Express One Zone

```
# start nbd server
nbdkit -f --log=stderr --ipaddr=127.0.0.1 --port=7788 ./target/release/libhypernbd.so backend_uri=s3://MYS3DIRBUCKET/nbdroot backend_wal_uri=s3://MYS3DIRBUCKET/nbdwalroot
```

```
# connect nbd deivce
$ nbd-client 127.0.0.1 7788 /dev/nbd0

# create filesystem 
$ mkfs.xfs -K /dev/nbd0

# mount filesystem
$ mount /dev/nbd0 /data

# run test
$ fio -group_reporting -numjobs=16 -iodepth=1 -direct=1 -ioengine=libaio -rw=randrw -bsrange=4k-512k -size=512M -time_based -runtime=20 -name=Fio -eta-newline=1 -directory=/data
Fio: (g=0): rw=randrw, bs=(R) 4096B-512KiB, (W) 4096B-512KiB, (T) 4096B-512KiB, ioengine=libaio, iodepth=1
...
fio-3.32
Starting 16 processes
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Fio: Laying out IO file (1 file / 512MiB)
Jobs: 16 (f=16): [m(16)][19.0%][r=93.5MiB/s,w=94.4MiB/s][r=393,w=389 IOPS][eta 00m:17s]
Jobs: 16 (f=16): [m(16)][28.6%][r=78.4MiB/s,w=75.1MiB/s][r=337,w=306 IOPS][eta 00m:15s]
Jobs: 16 (f=16): [m(16)][38.1%][r=102MiB/s,w=108MiB/s][r=463,w=475 IOPS][eta 00m:13s]
Jobs: 16 (f=16): [m(16)][47.6%][r=71.3MiB/s,w=79.7MiB/s][r=328,w=371 IOPS][eta 00m:11s]
Jobs: 16 (f=16): [m(16)][57.1%][r=79.1MiB/s,w=76.5MiB/s][r=355,w=336 IOPS][eta 00m:09s]
Jobs: 16 (f=16): [m(16)][61.9%][r=83.4MiB/s,w=85.1MiB/s][r=412,w=403 IOPS][eta 00m:08s]
Jobs: 16 (f=16): [m(16)][71.4%][r=66.6MiB/s,w=70.7MiB/s][r=321,w=353 IOPS][eta 00m:06s]
Jobs: 16 (f=16): [m(16)][81.0%][r=97.7MiB/s,w=108MiB/s][r=497,w=525 IOPS][eta 00m:04s]
Jobs: 16 (f=16): [m(16)][90.5%][r=83.5MiB/s,w=80.7MiB/s][r=427,w=416 IOPS][eta 00m:02s]
Jobs: 16 (f=16): [m(16)][100.0%][r=67.8MiB/s,w=64.3MiB/s][r=337,w=338 IOPS][eta 00m:00s]
Fio: (groupid=0, jobs=16): err= 0: pid=238148: Sat Oct  4 07:06:58 2025
  read: IOPS=393, BW=85.7MiB/s (89.9MB/s)(1719MiB/20044msec)
    slat (usec): min=12, max=3217, avg=56.10, stdev=99.23
    clat (msec): min=4, max=279, avg=10.86, stdev=19.95
     lat (msec): min=4, max=279, avg=10.92, stdev=19.96
    clat percentiles (msec):
     |  1.00th=[    5],  5.00th=[    6], 10.00th=[    6], 20.00th=[    6],
     | 30.00th=[    7], 40.00th=[    7], 50.00th=[    8], 60.00th=[    9],
     | 70.00th=[   10], 80.00th=[   11], 90.00th=[   14], 95.00th=[   17],
     | 99.00th=[  121], 99.50th=[  163], 99.90th=[  268], 99.95th=[  275],
     | 99.99th=[  279]
   bw (  KiB/s): min=15928, max=190673, per=100.00%, avg=88331.19, stdev=2591.14, samples=633
   iops        : min=   90, max=  822, avg=394.42, stdev=11.07, samples=633
  write: IOPS=401, BW=87.2MiB/s (91.5MB/s)(1748MiB/20044msec); 0 zone resets
    slat (usec): min=17, max=2267, avg=160.06, stdev=118.28
    clat (msec): min=5, max=295, avg=28.93, stdev=34.95
     lat (msec): min=5, max=295, avg=29.09, stdev=34.95
    clat percentiles (msec):
     |  1.00th=[    6],  5.00th=[    7], 10.00th=[    8], 20.00th=[    9],
     | 30.00th=[   10], 40.00th=[   14], 50.00th=[   21], 60.00th=[   28],
     | 70.00th=[   35], 80.00th=[   43], 90.00th=[   54], 95.00th=[   65],
     | 99.00th=[  259], 99.50th=[  271], 99.90th=[  292], 99.95th=[  292],
     | 99.99th=[  296]
   bw (  KiB/s): min=22536, max=174336, per=100.00%, avg=89392.89, stdev=2416.91, samples=634
   iops        : min=  132, max=  750, avg=400.61, stdev=10.06, samples=634
  lat (msec)   : 10=53.11%, 20=19.71%, 50=19.95%, 100=5.39%, 250=1.04%
  lat (msec)   : 500=0.80%
  cpu          : usr=0.06%, sys=0.51%, ctx=27571, majf=0, minf=207
  IO depths    : 1=100.0%, 2=0.0%, 4=0.0%, 8=0.0%, 16=0.0%, 32=0.0%, >=64=0.0%
     submit    : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     complete  : 0=0.0%, 4=100.0%, 8=0.0%, 16=0.0%, 32=0.0%, 64=0.0%, >=64=0.0%
     issued rwts: total=7884,8049,0,0 short=0,0,0,0 dropped=0,0,0,0
     latency   : target=0, window=0, percentile=100.00%, depth=1

Run status group 0 (all jobs):
   READ: bw=85.7MiB/s (89.9MB/s), 85.7MiB/s-85.7MiB/s (89.9MB/s-89.9MB/s), io=1719MiB (1802MB), run=20044-20044msec
  WRITE: bw=87.2MiB/s (91.5MB/s), 87.2MiB/s-87.2MiB/s (91.5MB/s-91.5MB/s), io=1748MiB (1833MB), run=20044-20044msec

Disk stats (read/write):
  nbd0: ios=7880/8030, merge=0/0, ticks=85557/232774, in_queue=318331, util=99.57%
```
