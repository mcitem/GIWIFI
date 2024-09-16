# LUN GIWIFI
为LNU新的giwifi适配的登录脚本

[ ![github](https://github.com/mcitem/GIWIFI/actions/workflows/build.yml/badge.svg)](https://github.com/mcitem/GIWIFI/actions/workflows/build.yml)

[![github](https://img.shields.io/badge/Download-x86_64_windows_msvc-blue)](https://github.com/mcitem/GIWIFI/actions/runs/10880590323/artifacts/1936566236)

```
PS E:\GIWIFI> .\giwifi.exe -h
Usage: giwifi.exe [OPTIONS]

Options:
  -b <BASE>          认证网页IP [default: http://100.100.9.2]
  -u <USERNAME>      账号 [default: ]
  -p <PASSWORD>      密码 [default: ]
  -q                 退出登录
      --key <KEY>    AES-CBC 加密密钥 [default: 1234567887654321]
      --ua <UA>      User-Agent [default: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0"]
  -h, --help         Print help
```

# 使用方式
## 退出登录
```
.\giwifi.exe -q
```
## 登录
```
.\giwifi.exe -u username -p password
```
