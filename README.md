
# Information about epass software

The epass software is a secure password management application that stores and encrypts information locally. With this console-based program, you can easily create or view new passwords after setting a master password.

The software uses an advanced AES256 encryption algorithm to protect the stored data, ensuring that no unauthorized person has access to it. Please keep in mind that if you lose your master password, there is no way to recover any of your saved passwords. Therefore, it is crucial to remember your master password in order to access your passwords.

## Features:

- Creating and managing passwords
- Creating encrypted backup copies
- Restoring backup copies

<br>

### [Download MacOS version](https://github.com/parsgit/epass/releases/download/1.0.6/epass_1.0.6_x86_64-apple-darwin.zip)

### [Download Windows version](https://github.com/parsgit/epass/releases/download/1.0.6/epass_1.0.6_x86_64-pc-windows-gnu.zip)

### [Download Linux version](https://github.com/parsgit/epass/releases/download/1.0.6/epass_1.0.6_x86_64-unknown-linux-musl.tar.gz)

<br>


## About Export :

When you choose the export option, a backup file is created containing all your encrypted passwords. This backup file is also encrypted again using the AES256 algorithm and to encrypt this backup version, we use a 30-character hash of your original password as the key.
This ensures that even if someone gains access to your backup file, they won't be able to view your passwords without the original passphrase.

However, in any case, you should never expose your sensitive files to the public or leave them unsecured where others may find them. It is always recommended to take necessary precautions such as storing backups in a secure location and using strong passwords to protect your important data.



