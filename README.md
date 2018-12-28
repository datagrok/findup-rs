# findup-rs

_An implementation of [findup](https://github.com/datagrok/findup) in Rust. (Incomplete; I'm writing this while learning the Rust language. It "works," but needs better error handling and code organization.)_

**findup** ("find up") locates a given filename in the nearest ancestor directory.

```
Usage: findup FILENAME

Look for FILENAME in the current or given DIRECTORY and all of its
ancestors until found. Return the full path of the closest match on
standard output.
```

## License: AGPL-3.0+

All of the code herein is copyright 2017 [Michael F. Lamb](http://datagrok.org) and released under the terms of the [GNU Affero General Public License, version 3][AGPL-3.0+] (or, at your option, any later version.)

[AGPL-3.0+]: http://www.gnu.org/licenses/agpl.html
