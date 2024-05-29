<p align="center">
	<a href="https://rustyvault.com/"><img src="website/assets/logo-512.webp" alt="Vault logo" width="256"></a><br>
	<a href="https://rustyvault.com/">rustyvault.com</a>
</p>

# Vault

> Discover this open source multi-platform password manager, crafted in Rust for
> top performance. It stays on your computer, featuring a heavily encrypted
> database that's easily accessible and boasts a user-friendly interface.

## General

- All edits and changes are non-destructive on secret fields
- On first start the app generates a `vault_config.toml` file and asks you to pick a password
- Once a password has been chosen the app creates a `vault_db.toml` file
- Both files are saved by default in your app config folder determined by [`dirs`](https://github.com/dirs-dev/dirs-rs) and a sub folder called `rusty-vault`
- The location of the `vault_db.toml` file can be changed in settings later (a change won't move the file but create a new file in the new location, the old file will persist in-line with our non-destructive policy)
- The framework we use to render the GUI is [floem](https://github.com/lapce/floem)

## Encryption

We generate a new salt for each new password, using `OsRng` and then take the
password + salt and hash it with 
[`argon2`](https://github.com/RustCrypto/password-hashes/tree/master/argon2).
We then use that hash to encrypt our database with
[`aes-gcm-siv`](https://github.com/RustCrypto/AEADs/tree/master/aes-gcm-siv),
a pure rust implementation of the `AES-GCM-SIV` (Misuse-Resistant Authenticated
Encryption Cipher) ([RFC 8452](https://datatracker.ietf.org/doc/html/rfc8452))
cypher. The `nonce` is also generated with the same library and prepended to
the cypher string before we base64 it and save it into the `vault_db.toml` file.

On lock we make sure we
[`zeroize`](https://github.com/RustCrypto/utils/tree/master/zeroize) all fields.

## How to run

The project comes with a dummy database to make testing easier with real data.
To make sure you use that db over the system config db that would otherwise be
installed automatically on the first run, run the app with the environment
variable `DEBUG` set.

```sh
λ DEBUG=true cargo run
```

This will also allow you to run the database unencrypted which is not possible
in "normal" mode.

## Clearing memory when locked

We verify that the memory is clean when locked by running the below commands and
looking at the output.

```sh
λ DEBUG=true cargo run
λ ps -e|grep vault
λ lldb --attach-pid <pip>
(lldb) process save-core <path>
(lldb) exit
λ cat <path> | strings | grep totally_secure_password
```

## How to contribute

Make sure you run the following commands before sending us a PR:

```sh
λ cargo fmt -- -l
λ cargo clippy
```

Make sure you address clippy warnings as it will fail CI.
It's ok to ignore clippy warnings where appropriate.

## License
Copyleft (c) 2023
Licensed under the [GNU GPL-3.0-or-later](https://github.com/dominikwilkowski/vault/blob/main/LICENSE).
