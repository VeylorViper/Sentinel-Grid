# Sentinel Grid Blockchain File Format Specification

## Overview
The Sentinel Grid blockchain file format stores blocks containing transactions, nonce placeholders, and networking metadata. It supports compression and encryption as optional features.

## File Structure

| Section        | Description                                      |
|----------------|------------------------------------------------|
| Header         | Fixed-size metadata about the file               |
| Block Data     | Serialized blocks with transactions and metadata |
| Footer (opt.)  | Checksum or hash for file integrity              |

## 1. Header (fixed 32 bytes)

| Offset | Size (bytes) | Description                                  |
|--------|--------------|----------------------------------------------|
| 0      | 4            | Magic Number (e.g. "SGCH" = 0x53 0x47 0x43 0x48) |
| 4      | 2            | Version number (major.minor)                 |
| 6      | 2            | Compression flag (0 = none, 1 = gzip, 2 = zstd) |
| 8      | 2            | Encryption flag (0 = none, 1 = AES-GCM)     |
| 10     | 6            | Reserved for future use                       |
| 16     | 8            | Timestamp (Unix epoch ms)                     |
| 24     | 8            | Number of blocks in the file                  |

## 2. Block Data (variable length)

Each block consists of:

### 2.1 Block Header (76 bytes)

| Size (bytes) | Description                     |
|--------------|---------------------------------|
| 32           | Previous block hash (SHA-256)   |
| 32           | Merkle root of transactions     |
| 8            | Timestamp (Unix epoch ms)        |
| 4            | Difficulty target (bits)         |

### 2.2 Transactions Section

| Size (bytes)    | Description                             |
|-----------------|-----------------------------------------|
| 4               | Number of transactions in the block     |
| Variable        | Transactions serialized consecutively   |

### 2.3 Transaction Format

Each transaction:

| Size (bytes)    | Description                           |
|-----------------|-------------------------------------|
| 4               | Transaction length (N)               |
| N               | Transaction data (arbitrary bytes)  |

### 2.4 Nonce (8 bytes)

| Size (bytes) | Description                  |
|--------------|------------------------------|
| 8            | Nonce (uint64), mining placeholder |

### 2.5 Networking Metadata (variable)

| Size (bytes) | Description                           |
|--------------|-------------------------------------|
| 4            | Number of peers                      |
| Variable     | For each peer:                      |
|              | - 16 bytes: IP (IPv6 or padded IPv4) |
|              | - 2 bytes: Port number               |

### 2.6 Block Footer (32 bytes)

| Size (bytes) | Description                         |
|--------------|-----------------------------------|
| 32           | Block hash (SHA-256 of block data) |

## 3. Compression & Encryption

- If the compression flag is set, the block data is compressed.
- The following algorithms are supported: none (0), zstd (2), and gzip (1).
- If an encryption flag is set, encryption is applied after compression.
- Encryption supported: AES-GCM (1), none (0).

## 4. Footer (optional)

| Size (bytes) | Description                       |
|--------------|---------------------------------|
| 32           | File checksum or hash for integrity |

## Notes

- Transaction data is flexible and can hold any payload.
- Nonce is a placeholder for mining logic.
- Networking metadata facilitates peer discovery and sync.
- Reserved header bytes allow future extensions.

# End of Specification

