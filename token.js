const crypto = require('crypto');

function generateUrlWithToken(key, url, expiration, clientIp = null) {
    const expires = Math.floor(Date.now() / 1000) + expiration;
    let base = `${key}${url}${expires}`;

    if (clientIp) {
        base += clientIp;
    }

    const hash = crypto.createHash('sha512').update(base).digest();
    let token = hash.toString('base64')
        .replace(/\+/g, '-')
        .replace(/\//g, '_')
        .replace(/=+$/, '');

    const output = `${url}?token=${token}&expires=${expires}`;
    return output;
}

const key = "token_from_dashboard";
const url = "your_url"; // e.g. https://spaceprotect.net/images/logo.png
const expiration = 3600; // In seconds

// This client IP address is ONLY required when IP Validation is enabled in the dashboard, otherwise remove this value
const clientIp = "1.1.1.1";

const finalUrl = generateUrlWithToken(key, url, expiration, clientIp);
console.log(finalUrl);
