from typing import Tuple, List, Any
from pathlib import Path

from cffi import FFI

ffi = FFI()
ffi.cdef(
    """
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
void deallocate_result(struct SearchResult result);
void deallocate_searcher(struct Searcher *result);
"""
)

C = ffi.dlopen(str(Path(__file__).parent / "liblike_aho_corasick_but_different.so"))


class Searcher:
    def __init__(self, elements: List[Tuple[str, Any]]):
        # make sure values are kept alive
        self.__keys = [ffi.new("char[]", k.lower().encode("utf8")) for k, _ in elements]
        self.__values = [ffi.new_handle(v) for _, v in elements]
        self.__elements = ffi.new("struct SearchElement[]", len(elements))

        for idx, (key, val) in enumerate(zip(self.__keys, self.__values)):
            self.__elements[idx].key = key
            self.__elements[idx].val = val

        self.__searcher = ffi.gc(
            C.new_searcher(self.__elements, len(elements)), C.deallocate_searcher
        )

    def search(self, haystack: str) -> List[str]:
        haystack = ffi.new("char[]", haystack.lower().encode("utf8"))
        results = ffi.gc(
            C.search_searcher(self.__searcher, haystack), C.deallocate_result
        )
        return [ffi.from_handle(results.values[i]) for i in range(results.length)]
