from Crypto.Cipher import AES
import base64
key = b"1234567887654321"


def zero_pad(data, block_size):
    padding_len = block_size - len(data) % block_size
    return data + b'\0' * padding_len


def zero_unpad(data):
    return data.rstrip(b'\0')


def cryptoDecode(encrypted_data, iv):
    ivv = iv.encode('utf-8')
    encrypted_data_bytes = base64.b64decode(encrypted_data)  # 解码Base64加密数据
    cipher = AES.new(key, AES.MODE_CBC, ivv)
    decrypted = cipher.decrypt(encrypted_data_bytes)
    return zero_unpad(decrypted).decode('utf-8')


def cryptoEncode(data, iv):
    ivv = iv.encode('utf-8')
    cipher = AES.new(key, AES.MODE_CBC, ivv)
    padded_data = zero_pad(data.encode('utf-8'), AES.block_size)
    encrypted = cipher.encrypt(padded_data)
    encrypted_data = base64.b64encode(encrypted).decode('utf-8')
    return {'data': encrypted_data, 'iv': iv}
