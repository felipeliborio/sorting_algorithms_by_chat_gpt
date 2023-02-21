#include <algorithm>

template<typename RandomAccessIterator>
void pdqsort(RandomAccessIterator begin, RandomAccessIterator end) {
    const size_t len = end - begin;
    if (len < 32) {
        std::sort(begin, end);
        return;
    }
    RandomAccessIterator l = begin, r = end - 1;
    size_t sz = len / 8;
    size_t m1 = len / 2;
    size_t m2 = m1 - sz;
    size_t m3 = m1 + sz;
    med3(begin + m1, begin + m2, begin + m3);
    med3(begin + (m1 - sz), begin + m1, begin + (m1 + sz));
    med3(begin, begin + m1, begin + (len - 1));
    typename std::iterator_traits<RandomAccessIterator>::value_type pivot = *(begin + m1);
    RandomAccessIterator ml = begin, mr = end;
    for (;;) {
        while (ml <= mr && *ml < pivot) {
            ml++;
        }
        while (ml <= mr && *mr > pivot) {
            mr--;
        }
        if (ml > mr) {
            break;
        }
        std::iter_swap(ml++, mr--);
    }
    RandomAccessIterator mp = begin;
    RandomAccessIterator mq = end;
    for (RandomAccessIterator i = begin + 1; i < ml; i++) {
        if (*i == pivot) {
            std::iter_swap(i, mp++);
        }
    }
    for (RandomAccessIterator i = end - 1; i > mr; i--) {
        if (*i == pivot) {
            std::iter_swap(i, --mq);
        }
    }
    while (mp < mq) {
        std::iter_swap(mp++, --mq);
    }
    pdqsort(begin, ml - 1);
    pdqsort(mr + 1, end);
    if (pivot != *(ml - 1)) {
        pdqsort(ml, mr);
    }
}
