[![Docker-Image CI](https://github.com/SXUOJ/sandbox/actions/workflows/docker-publish.yml/badge.svg)](https://github.com/SXUOJ/sandbox/actions/workflows/docker-publish.yml)

# Sandbox

## gRPC

```bash
Run a grpc server

Usage: sandbox grpc [ADDR]

Argument
```

## 命令行工具

判题沙盒, 命令行工具.

### 参数说明

```bash
Command line tool

Usage: sandbox cmd [OPTIONS]

Options:
  -t, --code_type <CODE_TYPE>                    Code type.					# 源代码类型
  -b, --bin_path <BIN_PATH>                      Bin path.  				# 执行文件路径
  -i, --input_path <INPUT_PATH>                  Input path.				# 输入文件路径
  -o, --output_path <OUTPUT_PATH>                Output path.				# 输出文件路径
  -e, --error_path <ERROR_PATH>                  Error output path.			# 错误输出路径
  -r, --real_time_limit <REAL_TIME_LIMIT>        Real time limit.			# 实际耗时限制
  -c, --cpu_time_limit <CPU_TIME_LIMIT>          CPU time limit.			# CPU耗时限制
  -m, --max_memory <MAX_MEMORY>                  Max memory.				# 内存限制
  -s, --max_stack <MAX_STACK>                    Max stack.					# 栈内存限制
  -p, --max_process_number <max_process_number>  Max process number.		# 线程数限制
  -z, --max_output_size <MAX_OUTPUT_SIZE>        Max output size.			# 输出限制
      --arg <ARG>                                Args.						# 执行文件附带参数
      --env <ENV>                                Envs.						# 执行文件环境变量
  -h, --help                                     Print help information		
  -V, --version                                  Print version information
```
