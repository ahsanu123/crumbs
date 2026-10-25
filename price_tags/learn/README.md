### Learn Real Shape of Http Request 

- tool used, `httpie` to inspect request offline 
  ```shell
  http --offline --print=HB POST \
    http://192.168.1.100/upload \
    Content-Type:application/octet-stream \
    Content-Length:8 \
    < test.txt

    POST /upload HTTP/1.1
    Accept: application/json, */*;q=0.5
    Accept-Encoding: gzip, deflate, zstd
    Connection: keep-alive
    Content-Length: 8
    Content-Type: application/octet-stream
    Host: 192.168.1.100
    User-Agent: HTTPie/3.2.4

    +-----------------------------------------+
    | NOTE: binary data not shown in terminal |
    +-----------------------------------------+
  ```

- to see hex use `hexyl`
  
  ```shell
  (main)> printf '\x48\x65\x00\x6C\x6F\xFF\x0A\x42' > test.bin

  (main)> ls
   README.md  justfile  test.bin

  (main)> hexyl test.bin
  ┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐
  │00000000│ 48 65 00 6c 6f ff 0a 42 ┊                         │He⋄lo×_B┊        │
  └────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘
  ```

