# whoami

一个使用 Rust 实现的轻量级 `whoami` 服务，功能类似 `traefik/whoami`。

它会在收到请求后返回当前容器或主机的基础信息，便于排查反向代理、容器网络、请求头透传、来源地址等问题。

## 功能说明

- 返回主机名 `Hostname`
- 返回服务所在机器的本地时间 `LocalTime`
- 返回服务实例的本机 IP 列表 `IP`
- 返回客户端来源地址 `RemoteAddr`
- 返回请求方法、路径和 HTTP 版本
- 返回所有请求头 `Headers`
- 当请求体不为空时，返回请求体内容 `Body`

## 返回效果

访问服务后，会返回纯文本内容，类似下面这样：

```text
Hostname: af3ddef704b6
LocalTime: 2026.04.23 17:17:10 

IP: localhost
IP: 127.0.0.1
IP: 172.18.0.2

RemoteAddr: 172.18.0.1:48168

GET /api/hello?name=123&page=1 HTTP/1.1

Headers:
Token: adfasdf
Content-Type: application/json
Accept: */*
Host: 127.0.0.1:3000
```



```text
Hostname: af3ddef704b6
LocalTime: 2026.04.23 17:17:50 

IP: localhost
IP: 127.0.0.1
IP: 172.18.0.2

RemoteAddr: 172.18.0.1:48376

POST /api/hello HTTP/1.1

Headers:
Token: adfasdf
Content-Type: application/json
Accept: */*
Host: 127.0.0.1:3000
Content-Length: 50

Body:
{
  "name":"张三",
  "sex":"1",
  "old":25
}
```

## 快速开始

### 方式一：使用 Docker Compose

项目内置了 `docker-compose.yml`，可以直接启动：

```bash
docker-compose up -d
```

启动后访问：

```text
http://localhost:3000/
```

查看运行日志：

```bash
docker-compose logs -f
```

停止服务：

```bash
docker-compose down
```

### 方式二：本地运行 Rust 项目

如果你想直接在本地运行源码：

```bash
cargo run
```

服务默认监听：

```text
0.0.0.0:3000
```

## Docker 构建

如果你希望自行构建镜像，可以在项目根目录执行：

```bash
docker build -t whoami:1.0.0 .
```

然后运行：

```bash
docker run --rm -p 3000:3000 whoami:1.0.0
```

## 使用示例

### 浏览器访问

直接打开：

```text
http://localhost:3000/
```

页面会返回当前请求的调试信息。

如需查看截图示例，可参考：

`images/web.png`

### 使用 curl 发送 GET 请求

```bash
curl http://localhost:3000/
```

### 使用 curl 发送 POST 请求

```bash
curl -X POST http://localhost:3000/test \
  -H "Content-Type: application/json" \
  -d "{\"name\":\"whoami\"}"
```

服务会返回：

- 请求路径，例如 `/test`
- 请求头
- 请求体内容

## 路由说明

当前服务支持以下路由：

- `GET /`
- `POST /`
- `GET /任意路径`
- `POST /任意路径`

也就是说，无论访问 `/` 还是其他路径，都会返回当前请求的上下文信息。

## 适用场景

- 调试反向代理是否正确转发请求头
- 排查容器网络和客户端来源地址
- 验证网关、Ingress、负载均衡的转发行为
- 快速检查请求体是否已正确到达后端

## 项目结构

```text
.
├── src/
├── Dockerfile
├── docker-compose.yml
└── README.md
```

## 说明

- 默认端口为 `3000`
- 返回内容类型为 `text/plain; charset=utf-8`
- 日志中也会输出请求内容，方便通过容器日志排查问题

## License

本项目基于仓库中的 [LICENSE](LICENSE) 文件使用。
