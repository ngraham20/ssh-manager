# ssh-manager
An ssh manager that can keep track of ssh connections that are used frequently, so that multiple connections with multiple users can be accessed quickly.


---

### Notes

Looks like a Layout ultimately allows for the quick definition of Rects. When multiple constraints are used, a vector of those Rects is created. The `split()` function is what determines the location and size of the layout. One way to nest (there may be a better way I have yet to find) layouts is to pass a new Rect to `split()` with `x, y, height, and width` set to those of the desired "parent" Block.