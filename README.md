# Sherlly_password_manager
An exceedingly simple password manager I wrote in rust.

This is designed to work with my dotfiles and the sherlly windows manager.
Some emoji are off limits for passwords. Check source code.

It will store passwords and info as pseduojson in an encrypted bytes file.
It has some basic functionality.
You must provide the password file passphrase for most functions.

Using dmenu you can select a entry title and have the password copies to your clipboard.

Wipe database / make new database with with another passphrase.

Delete entries on the database.

Add entries with a set or generated password.

Mass import entries with a csv.

In the suckless approach, there is no config file. Set the path you would like the program to use for the database file in the source code.
