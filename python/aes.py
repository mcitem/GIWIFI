from Crypto.Cipher import AES
import base64


# node_modules\crypto-js\pad-zeropadding.js
def zero_pad(data, block_size):
    padding_len = block_size - (len(data) % block_size or block_size)
    return data + b'\0' * padding_len


def cryptoEncode(data, iv, key=b"1234567887654321"):
    ivv = iv.encode('utf-8')
    cipher = AES.new(key, AES.MODE_CBC, ivv)
    padded_data = zero_pad(data.encode('utf-8'), AES.block_size)
    encrypted = cipher.encrypt(padded_data)
    encrypted_data = base64.b64encode(encrypted).decode('utf-8')
    return {'data': encrypted_data, 'iv': iv}


if __name__ == '__main__':
    print("aes test")
    data = "Hello, World!"
    iv = "0123456789abcdef"
    key = b"0123456789abcdef"
    res = cryptoEncode(data, iv, key)
    print(res["data"] == "BY4GP9KAMVmefx9XMXA1Hg==")
    pass
