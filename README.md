# ip_export

ip_export 自定义的ddns程序。


业务逻辑如下：

- 首先检测当前外网ip地址是否改变。
- 如果发生改变，将当前ip地址缓存到本地，并且利用dnspod提供的修改二级域名的方式更新指向的服务器ip地址。
- 并且向自己的邮箱发送最新的ip地址。

备注：
- 域名用二级的，不影响自己一级域名的指向位置
- 域名是godaddy上面申请的，dns用dnspod来做解析。毕竟是国内的，比较方便管理。
- 用域名的好处有很多，这里不再说明了。

## deploy

启动的时候指定一个缓存上一次`ip`地址的文件，与检测时间周期，这里是`3600`秒去检查一次。

**手动**启动： `ip_export -r ./record -t 3600` 

**systemd**服务启动：

准备好**systemd**需要用的`ip_export.service`文件，文件内容如下：
```service
[Unit]
Description=ip export to email
After=network.target auditd.service

[Service]
ExecStart=/usr/local/ip_export/ip_export -r /usr/local/ip_export/record -t 3600
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

```bash
# 创建目录，复制程序
sudo mkdir /usr/local/ip_export
sudo cp ip_export /usr/local/ip_export/ip_export
sudo cp ip_export.service /lib/systemd/system/ip_export.service
# 重新加载服务
sudo systemctl daemon-reload
sudo systemctl enable ip_export.service
sudo systemctl start ip_export.service
```
