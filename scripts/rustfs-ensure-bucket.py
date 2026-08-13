#!/usr/bin/env python3
"""Idempotently ensure the kache remote-cache bucket exists on the local rustfs
S3-compatible object store.

rustfs (the Rust S3 store used as kache's shared remote cache) exposes no bucket
management CLI, and neither the `aws` CLI nor `mc` are required by the committed
Mise toolchain. This script therefore issues a SigV4-signed `PUT /<bucket>`
(CreateBucket) using only the Python standard library, so it has no third-party
dependencies.

It is safe to run on every service start: rustfs returns 200 for an existing
bucket owned by the same credentials (MinIO-style), and any
BucketAlreadyOwnedByYou / BucketAlreadyExists response is treated as success.

Configuration is read from KACHE_S3_* environment variables. Shared service
passes localhost credentials only to this process, while endpoint, bucket, and
region match kache's remote configuration.
"""

import datetime
import hashlib
import hmac
import os
import sys
import time
import urllib.error
import urllib.request


def _sign(key: bytes, msg: str) -> bytes:
    return hmac.new(key, msg.encode(), hashlib.sha256).digest()


def _signing_key(secret: str, date: str, region: str, service: str) -> bytes:
    k = _sign(("AWS4" + secret).encode(), date)
    k = _sign(k, region)
    k = _sign(k, service)
    return _sign(k, "aws4_request")


def _put_bucket(endpoint: str, bucket: str, access_key: str, secret_key: str, region: str) -> int:
    """Send a SigV4-signed path-style CreateBucket request. Returns an exit code."""
    host = endpoint.split("://", 1)[1]
    service = "s3"
    now = datetime.datetime.now(datetime.timezone.utc)
    amz_date = now.strftime("%Y%m%dT%H%M%SZ")
    date_stamp = now.strftime("%Y%m%d")
    method = "PUT"
    canonical_uri = "/" + bucket
    payload_hash = hashlib.sha256(b"").hexdigest()
    canonical_headers = (
        f"host:{host}\n"
        f"x-amz-content-sha256:{payload_hash}\n"
        f"x-amz-date:{amz_date}\n"
    )
    signed_headers = "host;x-amz-content-sha256;x-amz-date"
    canonical_request = (
        f"{method}\n{canonical_uri}\n\n{canonical_headers}\n{signed_headers}\n{payload_hash}"
    )
    scope = f"{date_stamp}/{region}/{service}/aws4_request"
    string_to_sign = (
        f"AWS4-HMAC-SHA256\n{amz_date}\n{scope}\n"
        + hashlib.sha256(canonical_request.encode()).hexdigest()
    )
    signature = hmac.new(
        _signing_key(secret_key, date_stamp, region, service),
        string_to_sign.encode(),
        hashlib.sha256,
    ).hexdigest()
    authorization = (
        f"AWS4-HMAC-SHA256 Credential={access_key}/{scope}, "
        f"SignedHeaders={signed_headers}, Signature={signature}"
    )

    request = urllib.request.Request(f"{endpoint}/{bucket}", method="PUT", data=b"")
    request.add_header("Host", host)
    request.add_header("x-amz-date", amz_date)
    request.add_header("x-amz-content-sha256", payload_hash)
    request.add_header("Authorization", authorization)

    try:
        response = urllib.request.urlopen(request, timeout=10)
        print(f"rustfs bucket '{bucket}' created ({response.status})")
        return 0
    except urllib.error.HTTPError as error:
        body = error.read().decode(errors="replace")
        if error.code == 409 or "BucketAlreadyOwnedByYou" in body or "BucketAlreadyExists" in body:
            print(f"rustfs bucket '{bucket}' already exists")
            return 0
        print(f"rustfs bucket create HTTP {error.code}: {body[:400]}", file=sys.stderr)
        return 1
    except (urllib.error.URLError, OSError) as error:
        # Connection refused / not yet listening: signal retryable failure.
        print(f"rustfs bucket create not reachable yet: {error}", file=sys.stderr)
        return 3


def main() -> int:
    endpoint = os.environ.get("KACHE_S3_ENDPOINT", "http://127.0.0.1:19000")
    bucket = os.environ.get("KACHE_S3_BUCKET", "shared-local")
    access_key = os.environ.get("KACHE_S3_ACCESS_KEY", "kachelocal")
    secret_key = os.environ.get("KACHE_S3_SECRET_KEY", "kachelocalsecret")
    region = os.environ.get("KACHE_S3_REGION", "us-east-1")

    # rustfs needs a moment after launch before it accepts S3 requests; retry the
    # connection a bounded number of times so the service startup stays robust.
    attempts = int(os.environ.get("RUSTFS_BUCKET_INIT_ATTEMPTS", "30"))
    for attempt in range(1, attempts + 1):
        code = _put_bucket(endpoint, bucket, access_key, secret_key, region)
        if code != 3:
            return code
        if attempt < attempts:
            time.sleep(1)
    print("rustfs did not become reachable in time for bucket init", file=sys.stderr)
    return 3


if __name__ == "__main__":
    sys.exit(main())
