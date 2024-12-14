package main

import (
	"crypto/sha512"
	"encoding/base64"
	"fmt"
	"strings"
	"time"
)

func main() {
	key := "token_from_dashboard"
	url := "your_url" // e.g. https://spaceprotect.net/images/logo.png
	expiration := 3600 // In seconds

	expires := time.Now().Unix() + expiration
	base := fmt.Sprintf("%s%s%d", key, url, expires)

	// This client IP address is ONLY required when IP Validation is enabled in the dashboard, otherwise remove this value
	base += "1.1.1.1"

	hash := sha512.Sum512([]byte(base))
	token := base64.StdEncoding.EncodeToString(hash[:])

	token = strings.ReplaceAll(token, "+", "-")
	token = strings.ReplaceAll(token, "/", "_")
	token = strings.ReplaceAll(token, "=", "")

	output := fmt.Sprintf("%s?token=%s&expires=%d", url, token, expires)
	fmt.Println(output)
}
