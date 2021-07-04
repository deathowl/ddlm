# dumb login manager

A stupidly simple graphical login manager. 
Uses framebuffer, so You wont have to run a wayland session to bootstrap your wayland session (unlike gtkgreet)

This is a greetd frontend.


My greetd config looks like :
```
[terminal]
# The VT to run the greeter on. Can be "next", "current" or a number
# designating the VT.
vt = 7

# The default session, also known as the greeter.
[default_session]

command = "ddlm --target /usr/bin/sway" 

# The user to run the command as. The privileges this user must have depends
# on the greeter. A graphical greeter may for example require the user to be
# in the `video` group.
user = "greetd"
```