# whoami
 implementation of traefik/whoami in rust



#### 打印日志效果：

``log
whoami  | Hostname: e2ed0c50e9db
whoami  | LocalTime: 2026.04.23 17:00:08 
whoami  | 
whoami  | IP: localhost
whoami  | IP: 127.0.0.1
whoami  | IP: 172.18.0.2
whoami  | 
whoami  | RemoteAddr: 172.18.0.1:49216
whoami  | 
whoami  | GET / HTTP/1.1
whoami  | 
whoami  | Headers:
whoami  | Host: 127.0.0.1:3000
whoami  | Connection: keep-alive
whoami  | Sec-Ch-Ua: "Chromium";v="146", "Not-A.Brand";v="24", "Google Chrome";v="146"
whoami  | Sec-Ch-Ua-Mobile: ?0
whoami  | Sec-Ch-Ua-Platform: "Windows"
whoami  | Upgrade-Insecure-Requests: 1
whoami  | User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36
whoami  | Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
whoami  | Sec-Fetch-Site: none
whoami  | Sec-Fetch-Mode: navigate
whoami  | Sec-Fetch-User: ?1
whoami  | Sec-Fetch-Dest: document
whoami  | Accept-Encoding: gzip, deflate, br, zstd
whoami  | Accept-Language: zh-CN,zh;q=0.9
``


# 浏览器访问效果
将 images/web.png 放到 whoami 容器中，访问 http://localhost:3000/ 即可查看效果