# Generate certificate and key for police system

```bash
openssl req -x509 -newkey rsa:4096 -nodes \
 -keyout backend/police-system/key.pem \
 -out backend/police-system/cert.pem \
 -days 365 \
 -subj "/CN=localhost"
 ```

# Generate certificate and key for hospital system

```bash
openssl req -x509 -newkey rsa:4096 -nodes \
 -keyout backend/hospital-system/key.pem \
 -out backend/hospital-system/cert.pem \
 -days 365 \
 -subj "/CN=localhost"
```