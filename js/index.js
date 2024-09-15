const username = "username";
const password = "password";
const base = "http://100.100.9.2";
const login_doc = base + "/gportal/web/login";
const login_action = base + "/gportal/Web/loginAction";
const logout_doc = base + "/gportal/web/logout";
const logout_action = base + "/gportal/Web/logoutAction";
const headers = {
  "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
  "User-Agent":
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0",
};
import axios from "axios";
import * as cheerio from "cheerio";
import CryptoJS from "crypto-js";

function cryptoEncode(data, iv) {
  var key = CryptoJS.enc.Utf8.parse("1234567887654321");
  var ivv = CryptoJS.enc.Utf8.parse(iv);
  var encrypted = CryptoJS.AES.encrypt(data, key, {
    iv: ivv,
    mode: CryptoJS.mode.CBC,
    padding: CryptoJS.pad.ZeroPadding,
  });
  data = encrypted.toString();
  var msg = { data: data, iv: iv };
  return msg;
}

function login() {
  axios.get(login_doc, { headers: headers }).then((Response) => {
    const $ = cheerio.load(Response.data);
    const iv = $("input[name='iv']").val();
    $("input[name='user_account']").val(username);
    $("input[name='user_password']").val(password);
    axios
      .post(
        login_action,
        {
          data: cryptoEncode($("#loginForm").serialize(), iv).data,
          iv: iv,
        },
        { headers: headers }
      )
      .then((Response) => {
        console.log(Response.data);
      });
  });
}
function logout() {
  axios.get(logout_doc, { headers: headers }).then((Response) => {
    const $ = cheerio.load(Response.data);
    const si = $("input[name='si']").val();
    axios
      .post(
        logout_action,
        {
          si: si,
        },
        { headers: headers }
      )
      .then((Response) => {
        console.log(Response.data);
      });
  });
}
function main() {
  login();
  //   logout();
}
main();
