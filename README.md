# monitor-avahi

Simple systemd service which monitors *avahi-daemon* for the current hostname in use and restarts it.
This helps if you have the issue where avahi sporadicallty thinks there is a conflict (when in fact there is none) and changes to *hostname-2*.

The the long lasting issue:
https://github.com/lathiat/avahi/issues/117
