# Benchmark Specifications

## Category 1: Project Operations (2,500 pts max)

### 1.1 File Enumeration (500 pts)
- Simulates: VS solution load, git status
- Method: Create 30,000 files in 500 dirs, enumerate all
- Measure: files/sec (median of 5 runs after warmup)
- Scoring:
  - 60,000+/s = 500 pts
  - 30,000/s = 300 pts
  - 5,000/s = 50 pts

### 1.2 Small File Random Read (600 pts)
- Simulates: Loading source files
- Method: 10,000 random 4KB reads from 1GB file
- Measure: P99 latency in ms
- Scoring (P99):
  - <0.5ms = 600 pts
  - <2ms = 400 pts
  - <10ms = 150 pts
  - >50ms = 10 pts

### 1.3 Metadata Operations (500 pts)
- Simulates: Build temp files, npm install
- Method: Create/write/close/delete 5,000 small files
- Measure: ops/sec
- Scoring:
  - 5,000+/s = 500 pts
  - 1,500/s = 200 pts
  - <200/s = 25 pts

### 1.4 Directory Traversal with Content (400 pts)
- Simulates: Search in files
- Method: Enumerate + read first 1KB of 30,000 files
- Measure: files/sec
- Scoring:
  - 20,000+/s = 400 pts
  - 5,000/s = 150 pts
  - <1,000/s = 25 pts

### 1.5 Large File Sequential Read (500 pts)
- Simulates: Opening large CAD files
- Method: Read 2GB file in 1MB chunks
- Measure: MB/s
- Scoring:
  - 3,000+ MB/s = 500 pts
  - 1,000 MB/s = 250 pts
  - <100 MB/s = 25 pts

## Category 2: Build Performance (2,500 pts max)

### 2.1 Single-Thread Compute (600 pts)
- Simulates: Single-file compilation
- Method: LZ4 compress/decompress loop
- Measure: MB/s throughput

### 2.2 Multi-Thread Compute (600 pts)
- Simulates: Parallel build
- Method: Same workload across all cores
- Measure: Total throughput + scaling efficiency

### 2.3 Mixed Read-Compute-Write (700 pts)
- Simulates: Full build cycle
- Method: Read → compress → write files
- Measure: End-to-end throughput

### 2.4 Sustained Write Performance (600 pts)
- Simulates: Build output
- Method: Write 4GB with periodic fsync
- Measure: Sustained MB/s
- Scoring:
  - 2,500+ MB/s = 600 pts
  - 800 MB/s = 300 pts
  - <200 MB/s = 50 pts

## Category 3: Responsiveness (2,500 pts max)

### 3.1 Storage Latency Distribution (700 pts)
- Simulates: Every disk access
- Method: 10,000 random 4KB reads with timing
- Measure: P50, P95, P99, P99.9 latency
- Output: Histogram data
- Scoring: Based on P99 (same as 1.2)

### 3.2 Memory Latency (400 pts)
- Simulates: App memory access
- Method: Pointer-chasing through large buffer
- Measure: Nanoseconds per access
- Scoring:
  - <70ns = 400 pts
  - <120ns = 200 pts
  - >150ns = 50 pts

### 3.3 Process Spawn Time (500 pts)
- Simulates: Running build tools
- Method: Spawn cmd.exe 100 times
- Measure: Average and P99 time
- Scoring:
  - <30ms = 500 pts
  - <100ms = 250 pts
  - >500ms = 10 pts

### 3.4 Thread Wake Latency (400 pts)
- Simulates: Async operations
- Method: Signal sleeping thread 1,000 times
- Measure: Wake time in microseconds
- Scoring:
  - <50μs = 400 pts
  - <200μs = 200 pts
  - >500μs = 50 pts

### 3.5 Memory Bandwidth (500 pts)
- Simulates: Large data processing
- Method: Multi-threaded memory copy
- Measure: GB/s
- Scoring:
  - 50+ GB/s = 500 pts
  - 30 GB/s = 300 pts
  - <15 GB/s = 100 pts

## Category 4: Graphics (2,500 pts max, optional)

### 4.1 GPU Detection (300 pts)
- Dedicated GPU = 300 pts
- Integrated = 200 pts
- Software = 50 pts

### 4.2 2D Vector Rendering (500 pts)
- Method: Render 500,000 lines
- Measure: FPS

### 4.3 3D Mesh Rendering (600 pts)
- Method: Render 100K triangle mesh
- Measure: FPS

### 4.4 Frame Consistency (600 pts)
- Method: Measure frame time variance
- Scoring: Low variance = high score

### 4.5 Texture Upload (500 pts)
- Method: Upload large textures
- Measure: GB/s
