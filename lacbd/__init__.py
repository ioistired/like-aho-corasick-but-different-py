import sys

from typing import List, Tuple, Collection, Any
from pathlib import Path

from cffi import FFI

ffi = FFI()
ffi.cdef("""
struct Searcher {
    char private[0];
};

struct SearchElement {
  const char *key;
  const void *val;
};

struct SearchResult {
    void *const *const values;
    size_t length;
};

struct Searcher *new_searcher(struct SearchElement *search_strings, size_t num_strings);
struct SearchResult search_searcher(const struct Searcher *searcher, const char* haystack);
size_t searcher_size(const struct Searcher *searcher);
void deallocate_result(struct SearchResult result);
void deallocate_searcher(struct Searcher *result);
""")

# XXX is this a hack?
C = ffi.dlopen(str(next(Path(__file__).parent.parent.glob("_lacbd*"))))

class Searcher:
    def __init__(self, elements: Collection[Tuple[str, Any]]):
        # make sure values are kept alive
        self.__keys = [ffi.new("char[]", k.lower().encode("utf8")) for k, _ in elements]
        self.__values = [ffi.new_handle(v) for _, v in elements]
        elements = ffi.new("struct SearchElement[]", len(elements))

        for idx, (key, val) in enumerate(zip(self.__keys, self.__values)):
            elements[idx].key = key
            elements[idx].val = val

        self.__searcher = ffi.gc(
            C.new_searcher(elements, len(elements)), C.deallocate_searcher
        )

    def search(self, haystack: str) -> List[str]:
        haystack = ffi.new("char[]", haystack.lower().encode("utf8"))
        results = ffi.gc(
            C.search_searcher(self.__searcher, haystack), C.deallocate_result
        )
        return [ffi.from_handle(results.values[i]) for i in range(results.length)]

    def __sizeof__(self):
        return (super().__sizeof__()
                + C.searcher_size(self.__searcher)
                + sum(sys.getsizeof(i) + sys.getsizeof(ffi.from_handle(i)) for i in self.__values)
                + sum(sys.getsizeof(i) + len(i) for i in self.__keys))

    def __len__(self):
        return len(self.__keys)

    def __class_getitem__(cls, item):
        return f"{cls.__name__}[{item!r}]"
