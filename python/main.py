import requests
from aes import cryptoEncode
from pyquery import PyQuery as pq
from urllib.parse import quote
base = "http://100.100.9.2"
hd = {
    "content-type": "application/x-www-form-urlencoded; charset=UTF-8",
    "user-agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3",
}

se = requests.session()
se.headers.update(hd)


def login(u, p):
    res = se.get(base + "/gportal/web/login")
    doc = pq(res.text)
    doc("#loginForm input[name=user_account]").val(u)
    doc("#loginForm input[name=user_password]").val(p)
    data = "&".join([
        f"{el.attr('name')}={quote(el.val())}"
        for el in doc("#loginForm input").items()
    ])
    msg = cryptoEncode(data, doc("input[name=iv]").attr("value"))
    msg = "&".join([f"{k}={quote(v)}" for k, v in msg.items()])
    res = se.post(base + "/gportal/Web/loginAction", data=msg)
    print(res.text)


def logout():
    si = get_si()
    data = {"si": si}
    res = se.post(base + "/gportal/Web/logoutAction", data=data)
    print(res.text)


def get_si():
    res = se.get(base + "/gportal/web/logout")
    doc = pq(res.text)
    si = doc("input[name=si]").attr("value")
    if not si:
        raise ValueError("Failed to retrieve 'si'")
    return si


if __name__ == "__main__":
    # login(username, password)
    # logout()
    pass
