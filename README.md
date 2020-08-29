# ssh-manager
An ssh manager that can keep track of ssh connections that are used frequently, so that multiple connections with multiple users can be accessed quickly.


### Ideas

If this tool is best used as simply an ssh connection navigator and not a client or wrapper, then maybe when a server is selected, it would simply be copied to the clipboard for pasting into a different pane.

Maybe something even better though would be to cause a file to be updated when a server is selected. Essentially, when this program is used, it could be called from a shell script by default which takes parameters. If none are given, it runs the tui, but if -c is given, it simply runs a shell script to make the connection for you. For example, if the server "exampleserver" were selected with the user "default," then a shell script would be created with `ssh default@exampleserver` as the only contents. Then, when `$ ssh-manager -c` was run, the ssh script would be called instead of the rust program.

A riff off the previous idea would be to, instead of creating a script that would be called with `$ ssh-manager -c`, a script with the same contents, but called `connect.sh` would be created in the current user directory. That way, in the case of a user intending on sshing from a directory indended for the purpose of that connection, they wouldn't have to renavigate every time.

I'm thinking that proably both are good options here, however the second one's importance is lessened by there being fewer use cases for that due to the organization being driven by the client folder structure instead of the application.

Basially, this isn't meant to be an ssh window manager tool. There are way better tools out there like `screen` and `tmux`. Therefore, all this app needs to do is manage the connections themselves so that the multiplexers can do their jobs.


---

### Notes

Looks like a Layout ultimately allows for the quick definition of Rects. When multiple constraints are used, a vector of those Rects is created. The `split()` function is what determines the location and size of the layout. One way to nest (there may be a better way I have yet to find) layouts is to pass a new Rect to `split()` with `x, y, height, and width` set to those of the desired "parent" Block.