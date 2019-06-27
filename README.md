# lacbd

lacbd is a Python library written in Rust that implements the Aho Corasick algorithm for fast subsentence
matching of many keywords against one string.

You can find the actual Rust library as [nitros12/like-aho-corasick-but-different](https://github.com/nitros12/like-aho-corasick-but-different).

## Features

- Supports arbitrary values associated with each keyword
- Operates on Unicode word bounds, rather than naïve substring matching
- Case insensitive
- 10× faster than an equivalent regex

None of the existing python libraries fit my needs.

## License

This library is AGPLv3+ licensed. That may seem like an odd choice for a library. However, doing so ensures
that users of this code must make their application open source, even if run as a service (such as in a Discord bot).
If you want to use this to make proprietary software, look somewhere else.

Copyright © 2019 Ben Simms and Ben Mintz

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
