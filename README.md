# Rusty Keychain [![Build Status](https://travis-ci.org/MarceColl/rusty-keychain.svg?branch=master)](https://travis-ci.org/MarceColl/rusty-keychain)

Password manager using:

- Rust for the client
- gpg for encrypting
- git for syncronizing
- YAML to store the accounts for each site




> This is the current specification for the program, it is a build in progress so everything is subjected to change at any time. **Currently nothing is implemented**

## CLI

#### Initialize password repository

To initialize a new repository type:

```shell
$ rk init <gpg key id>
```

This initializes a new repository using the gpg key specified for encryption.



If you have a repository stored in git already you can use

```shell
$ rk init -g <repository>
```

to initialize it to the current machine. For example:

```shell
$ rk init -g MarceColl/passwords
```



#### Manage accounts

```shell
$ rk manage <site>
```

Opens a repl for managing the `site` passwords and accounts.

Example of usage below:

```shell
$ rk manage github.com
Rusty Keychain
> accounts
* (1) coll.marce@gmail.com
> create
User: marce@marce.com
> accounts
* (1) coll.marce@gmail.com
  (2) marce@marce.com
> switch 2
Switched to marce@marce.com
> generate 50
Generated new password for account marce@marce.com
> save
Saved site.
Goodbye!
```



#### Copy to clipboard

```shell
$ rk copy <site>
```

Copies the password of the site to the clipboard, if there are several accounts for the site it asks which one you want to copy

```shell
$ rk copy github.com
Two accounts found:
(1) coll.marce@gmail.com
(2) marce@marce.com
Which password do you want to copy? 2
Password for marce@marce.com copied to clipboard`
```



#### Sync with git

```shell
$ rk sync
```

Syncs the password database with git

