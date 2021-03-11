Sizing test results
-------------------

### Data model with large partitions

This is the test results for the first iteration of the data
model for this project that has the following characteristics:

- a sensor stores its data in a large partition
- a sensor does a measurement once a minute
- a sensor stores a month of data in a partition

    measurement * 60 minutes / hour * 24 hours * 30 days =
    = 43,200 rows / partition

#### Considerations

There are 4 kinds of workloads:

- write a sensor measurement update - simple insert with the newest
  timestamp
- read a day of data
- read a week of data
- read a month of data

A pet usually would have 4 sensors so that an average dashboard
is ecpected to show 4 pets measurements for last day / week +
some average for the last month on a per day basis.

The table uses TimeWindowCompactionStrategy (TWCS) that has by default
1 day size of the time window. That means that the measurements from
different days would not be stored in the same SSTables. With
a 43,200 rows / month / partition a partition would be split
across 30 SSTables.

A default columns width size for promoted index is 64kb. 43,200,000
rows across 1,000 partitions weight 412MB (10 bytes per row with LZ4).
That means a partition of a month data for a sensor will take

    43,200 * 10 / 1024 = 421KB / partition / month
                                 sensor    / month

or

    421KB a partition / 30 SSTables = 14KB / SSTable.

14KB is less than 64KB and that means that there will be no promoted
index for a whole sensor data.

Current, compression chunk size is 4KB, compression ratio is about 2.
Than means there will be about 14KB / 2KB = 7 chunks per SSTable per
partition. So to read a day of data Scylla would have to uncompress
7 chunks.

SSTables index does not contain clustering keys. That means in order
to read a day of data (1440 rows) Scylla will have to scan through
all SSTables in a month because each SSTable will contain a day of
a partition. So it will have to scan through all 30.

TWCS requires constant TTL per row. The program shall be updated to
insert data with TTL = 30 days.

For the most of the data reads it does not make sense to use cache.
So that BYPASS CACHE shall be default for reading measurements besides
the latest day of the data. Maybe even they shall not be promoted.

#### Evaluation

I have used `sizing_spec_v1.yaml` spec for `cassandra-stress` to
measure parameters of this model with `https://github.com/sitano/scylla-cluster-tests/tree/ycsb_empty`:

    $ cassandra-stress user profile=./spec.yaml n=1000 "ops(insert=1)"
        -node $(cat node1) -rate threads=1
        -mode native cql3 user=cassandra password=cassandra
        -log level=verbose hdrfile=store.hdr
        -graph file=store.html title=store revision=benchmark

    $ cassandra-stress user profile=./spec.yaml n=1000 "ops(select1234=1)"
        -node $(cat node1) -rate threads=1
        -mode native cql3 user=cassandra password=cassandra
        -log level=verbose hdrfile=load.hdr
        -graph file=load.html title=load revision=benchmark

I ignored the Coordinated Omission problem because I almost do not
measure latency.

The test have found out that `cassandra-stress` has an inefficient
implementation of a `PartitionIterator$MultiRowIterator` so that it
takes about 2-3ms just to `fill()` a partition of 43,200 rows just
to pick rows for the next request. Thus that limits a thread
throughput to 300.

So that it is impossible to generate a pattern of data insertions
with `cassandra_stress` of the following form for a month of data:

    fixed(1)/43200

Writing an UNLOGGED batch of `43200/43200` (43,200 rows per a single
partition) takes about 300-400ms in Scylla so that it also makes it
unfeasable to do the testing.

Thus, I had to shrink the dataset size to make my initial estimations.
I shrinked the test size from 1M pets / 4M sensors to just 1,000
sensors of a month of data to verify:

- space estimations
- workloads average latency

Here the results:

An expected number of rows per month per 1M pets / 4M sensors is:

    172,800,000,000 records/month = 173B rows/month

1,000 partitions of 43,200,000 rows took:

    433,558,012 bytes of compressed data in all SSTables

    =>

    433558012 / 43200 / 1000 = 10 bytes per row

    =>

    10 bytes per 4 bytes float (ts) cell with cluster key 8 bytes (timestamp)

    =>

    Scylla compressed 12 bytes row into 10 bytes on avg.

Expected cluster size then is:

    172,800,000,000 rows * 10 bytes * 3 RF / 1024^4 = 4.7 TB

This is even 3 times better than what I expected in my back of
the envelope calculations. Basically, its about 1.6GB a node. 

The amount of data it took for the commitlog to write these 43,200,000
rows is:

    sum(scylla_io_queue_total_bytes{class="commitlog",instance="[10.0.2.57]"}):

    (14584905728 - 1142632448) = 13442273280 bytes = 12 GB

or 

    12 GB / 43,200,000 = 311 bytes per row

This is 30 times worse than the resulting storage.

Measurements for the uncompressed table:

    907764 KB * 1024 bytes / 43,200 measurements / 1000 partitions =
    22 bytes / row

    =>

    projected cluster size 10TB.

`select1` results are abount 4.2ms p99 for cached data:

                              ops            rows/s                                 99%%    99.9%     max
    total,          8314,     252,     250,  225698,     2.1,     2.8,     3.5,     3.8,    20.7,    20.7,   38.0,  0.06598
    total,          8566,     252,     246,  241052,     2.2,     2.8,     3.5,     3.9,     4.3,     4.3,   39.0,  0.06419
    total,          8832,     266,     255,  213869,     1.8,     2.6,     3.4,     3.7,     4.4,     4.4,   40.0,  0.06247
    total,          9076,     244,     240,  214651,     2.1,     2.7,     3.7,     4.2,    23.8,    23.8,   41.0,  0.06084
    total,          9331,     255,     250,  228645,     2.1,     2.7,     3.6,     4.1,     4.4,     4.4,   42.0,  0.05929
    total,          9585,     254,     247,  226328,     2.0,     2.8,     3.7,     4.2,     4.5,     4.5,   43.0,  0.05781

`select2` results are abount 5ms p99 for uncached data:

    total,          5237,     234,     229,  206678,     2.4,     2.7,     4.0,     5.0,    13.6,    13.6,   23.0,  0.02892
    total,          5487,     250,     246,  217087,     2.1,     2.5,     3.9,     4.2,     4.8,     4.8,   24.0,  0.02783
    total,          5730,     243,     238,  212432,     2.3,     2.5,     4.0,     4.9,    23.6,    23.6,   25.0,  0.02680
    total,          5962,     232,     227,  218342,     2.5,     2.9,     4.2,     4.8,     5.2,     5.2,   26.0,  0.02587

`select3` 20-30ms p99 per week scan for cached data.

`select4` 20ms p99 per week scan for uncached data.

#### Conclusion

Space estimations were correct at least on the small dataset.

P99 latency for a read of a day of the data (`1440 rows / partition`)
is about 4-5ms is satisfying. P99 latency for the rest of the workloads,
and, specifically, for reading a week of the data (`60 * 24 * 7 = 10080`
rows / partition or per sensor) is unsatisfying in current model.

The data model that has a large partition of a sensor month data of
43,200 measurement is inefficient (with TWCS even more - it splits
the partition among 30 files without promoted index).

Desired model correction:

- Try to keep partition size small. Minimize rows to read.

- Pick the partition size that does not span across many TWCS windows.
  By default a day. For example: ((sensor, date), value).

- A day of data is 1440 rows. They can be split into hours
  ((sensor, date, hours), value). So it will be 60 rows / partition.

- A day of the data maybe stored in a single row. Use fixed 1-2 byte
  numbers with delta-encoding and compression on the app level.
  This will give even better packing density.

- A week and a month of data shall be aggregated.
