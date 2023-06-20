
# Information about epass software

## Under development. ⚠️

The epass software is a console-based password management application that stores and encrypts information locally. To use the program, simply create a master password and then you can create or view new passwords.

This software uses the AES256 algorithm to encrypt the stored information. Please note: If you forget your master password, there is no way to recover the saved passwords in the program.


## About Export :

To create a backup of all your passwords, we use AES256 encryption algorithm to encrypt them all. When you choose the export option, a backup file is created containing all your encrypted passwords. This backup file is also encrypted again using the AES256 algorithm and to encrypt this backup version, we use a 30-character hash of your original password as the key.
This ensures that even if someone gains access to your backup file, they won't be able to view your passwords without the original passphrase.

However, in any case, you should never expose your sensitive files to the public or leave them unsecured where others may find them. It is always recommended to take necessary precautions such as storing backups in a secure location and using strong passwords to protect your important data.
