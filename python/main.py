import requests
from aes import cryptoEncode
from pyquery import PyQuery as pq
from urllib.parse import unquote, quote
base = "http://100.100.9.2"

hd = {
    "content-type": "application/x-www-form-urlencoded; charset=UTF-8",
    "user-agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3",
}

se = requests.session()
se.headers.update(hd)

res = se.get(base + "/gportal/web/login")

doc = pq(res.text)
data = {
    "sign": unquote(doc("input[name=sign]").attr("value")),  # -
    "sta_vlan": doc("input[name=sta_vlan]").attr("value"),
    "sta_port": doc("input[name=sta_port]").attr("value"),
    "sta_ip": doc("input[name=sta_ip]").attr("value"),  # -
    "nas_ip": doc("input[name=nas_ip]").attr("value"),
    "nas_name": doc("input[name=nas_name]").attr("value"),  # -
    "last_url": doc("input[name=last_url]").attr("value"),
    "request_ip": doc("input[name=request_ip]").attr("value"),
    "device_mode": doc("input[name=device_mode]").attr("value"),
    "device_type": doc("input[name=device_type]").attr("value"),
    "device_os_type": doc("input[name=device_os_type]").attr("value"),
    "is_mobile": doc("input[name=is_mobile]").attr("value"),
    "iv": doc("input[name=iv]").attr("value"),
    "login_type": doc("input[name=login_type]").attr("value"),
    "account_type": doc("input[name=account_type]").attr("value"),  # -
    "user_account": "username",
    "user_password": "password",
}
''' error
sign:{"data":99,"info":"error: sign is empty","status":0}
sta_ip:{"data":0,"info":"error: staIp is empty","status":0}
account_type:{"data":0,"info":"error: at is empty","status":0}
nas_name:{"data":0,"info":"error: nan is empty","status":0}
'''


data = "&".join([f"{k}={quote(v)}" for k, v in data.items()])
print(data)
print(doc("input[name=iv]").attr("value"))
msg = cryptoEncode(data, doc("input[name=iv]").attr("value"))
print(msg)
msg = "&".join([f"{k}={quote(v)}" for k, v in msg.items()])
res = se.post(base + "/gportal/Web/loginAction", data=msg)

print(res.text)
