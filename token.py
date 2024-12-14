import hashlib
import base64
import time
import urllib.parse


def generate_url_with_token(key, url, expiration, client_ip=None):
    expires = int(time.time()) + expiration
    base = f"{key}{url}{expires}"
    if client_ip:
        base += client_ip

    hash_object = hashlib.sha512(base.encode())
    token = base64.urlsafe_b64encode(hash_object.digest()).decode().rstrip("=")

    output = f"{url}?token={token}&expires={expires}"
    return output


key = "token_from_dashboard"
url = "your_url" # e.g. https://spaceprotect.net/images/logo.png
expiration = 3600  # In seconds

# This client IP address is ONLY required when IP Validation is enabled in the dashboard, otherwise remove this value
client_ip = "1.1.1.1"

final_url = generate_url_with_token(key, url, expiration, client_ip)
print(final_url)
