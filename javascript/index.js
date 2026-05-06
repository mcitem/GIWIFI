const base = "http://100.100.9.2";

const headers = {
  "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
  "User-Agent":
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0",
};

import axios from "axios";
import * as cheerio from "cheerio";
import CryptoJS from "crypto-js";

function cryptoEncode(data, iv) {
  let key = CryptoJS.enc.Utf8.parse("1234567887654321");
  let ivv = CryptoJS.enc.Utf8.parse(iv);
  let encrypted = CryptoJS.AES.encrypt(data, key, {
    iv: ivv,
    mode: CryptoJS.mode.CBC,
    padding: CryptoJS.pad.ZeroPadding,
  });
  data = encrypted.toString();
  let msg = { data: data, iv: iv };
  return msg;
}

export function login(username, password) {
  axios
    .get(base + "/gportal/web/login", { headers: headers })
    .then((Response) => {
      const $ = cheerio.load(Response.data);
      let iv = $("#loginForm input[name='iv']").val();

      if (typeof iv === "undefined") {
        console.log("iv is undefined");
        throw new Error("iv is undefined");
      } else if (Array.isArray(iv)) {
        iv = iv[0];
      }

      $("input[name='user_account']").val(username);
      $("input[name='user_password']").val(password);
      axios
        .post(
          base + "/gportal/Web/loginAction",
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

export function logout() {
  axios
    .get(base + "/gportal/web/logout", { headers: headers })
    .then((Response) => {
      const $ = cheerio.load(Response.data);
      const si = $("input[name='si']").val();
      axios
        .post(
          base + "/gportal/Web/logoutAction",
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
