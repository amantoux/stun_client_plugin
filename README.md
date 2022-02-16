# stun_client

A flutter plugin requesting mapped address to a STUN server

### NB
In debug mode, Android version must be launched from Android Studio. From Intellij it doesn't work properly.

### NB 2
For some reason, COTURN server doesn't like setting `software` attibute to some values.
eg "C client" is OK, but "stunc" is not /\o/\
