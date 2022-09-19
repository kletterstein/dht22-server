# REST Server for the Nestbox

## DHT22

This project uses the [dht22_pi crate by michaelfletchercgy](https://github.com/michaelfletchercgy/dht22_pi).

The server requires the capability to `nice` the thread. So you have to run this once, before you
start the server (taken from https://stackoverflow.com/questions/7635515/how-to-set-cap-sys-nice-capability-to-a-linux-user):

```bash
$ sudo setcap 'cap_sys_nice=eip' restserver
```

You can confirm what capabilities are on an application using getcap:

```bash
$ getcap restserver
restserver = cap_sys_nice+eip
 `cap_sys_nice` 
```

